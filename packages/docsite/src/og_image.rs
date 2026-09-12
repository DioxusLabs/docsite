//! Build-time generation of per-page Open Graph preview images.
//!
//! Every route that `seo::has_generated_og_image` reports gets a 1200x630 PNG rendered from an
//! SVG template (title, breadcrumb-style eyebrow, description) and written to
//! `static_dir()/og/<slug>.png`, matching the URLs emitted by `seo::og_image_path`.

pub fn generate_og_images() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::{seo, static_dir, Route};
        use dioxus::prelude::Routable;

        let out_dir = static_dir().join("og");
        if let Err(err) = std::fs::create_dir_all(&out_dir) {
            eprintln!("Failed to create OG image directory: {err}");
            return;
        }

        let renderer = native::Renderer::new();
        let mut count = 0;
        for route in Route::static_routes() {
            if !seo::has_generated_og_image(&route) {
                continue;
            }
            let card = native::card_for_route(&route);
            let png = renderer.render(&card);
            let path = out_dir.join(format!("{}.png", seo::og_image_slug(&route)));
            if let Err(err) = std::fs::write(&path, png) {
                eprintln!("Failed to write {}: {err}", path.display());
                continue;
            }
            count += 1;
        }
        println!("Generated {count} OG images in {}", out_dir.display());
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::components::blog::page_to_meta;
    use crate::docs::AnyBookRoute;
    use crate::{seo, Route};
    use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
    use base64::Engine;
    use resvg::usvg;

    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 630;

    const INTER_BOLD: &[u8] = include_bytes!("../og/fonts/Inter-Bold.ttf");
    const INTER_MEDIUM: &[u8] = include_bytes!("../og/fonts/Inter-Medium.ttf");
    const INTER_REGULAR: &[u8] = include_bytes!("../og/fonts/Inter-Regular.ttf");
    const LOGO_PNG: &[u8] = include_bytes!("../assets/static/smalllogo.png");

    pub struct Card {
        pub eyebrow: String,
        pub title: String,
        pub description: String,
        pub footer_right: String,
    }

    pub fn card_for_route(route: &Route) -> Card {
        let meta = seo::PageMeta::for_route(route);
        match route {
            Route::Docs07 { child } => docs_card(*child),
            Route::Docs06 { child } => docs_card(*child),
            Route::Docs05 { child } => docs_card(*child),
            Route::Docs04 { child } => docs_card(*child),
            Route::Docs03 { child } => docs_card(*child),
            Route::BlogPost { child } => {
                let blog = page_to_meta(child.page());
                let eyebrow = ["Blog", blog.category, blog.date]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .copied()
                    .collect::<Vec<&str>>()
                    .join(" · ");
                Card {
                    eyebrow,
                    title: blog.title.to_string(),
                    description: blog.description.to_string(),
                    footer_right: blog.author.to_string(),
                }
            }
            Route::BlogList {} => Card {
                eyebrow: "Blog".into(),
                title: "The Dioxus Blog".into(),
                description: meta.description,
                footer_right: String::new(),
            },
            _ => Card {
                eyebrow: "Dioxus".into(),
                title: meta.title,
                description: meta.description,
                footer_right: String::new(),
            },
        }
    }

    fn docs_card<R: AnyBookRoute>(route: R) -> Card {
        let page = route.page();

        let mut eyebrow = vec!["Docs".to_string(), R::short_version().to_string()];
        eyebrow.extend(seo::part_title(route));
        let mut chapters = seo::chapter_titles(route);

        // "Overview" says nothing on its own: promote the chapter (or part) to the title.
        let title = if seo::is_generic_title(&page.title) {
            let promoted = chapters
                .pop()
                .or_else(|| (eyebrow.len() > 2).then(|| eyebrow.remove(2)));
            eyebrow.extend(chapters);
            eyebrow.push(page.title.clone());
            promoted.unwrap_or_else(|| page.title.clone())
        } else {
            eyebrow.extend(chapters);
            page.title.clone()
        };

        let description = seo::markdown_description(R::page_markdown(route.page_id()));

        Card {
            eyebrow: eyebrow.join(" · "),
            title,
            description,
            footer_right: format!("Dioxus {} Docs", R::short_version()),
        }
    }

    pub struct Renderer {
        bold: FontRef<'static>,
        regular: FontRef<'static>,
        opts: usvg::Options<'static>,
        logo_b64: String,
    }

    impl Renderer {
        pub fn new() -> Self {
            let mut fontdb = usvg::fontdb::Database::new();
            fontdb.load_font_data(INTER_BOLD.to_vec());
            fontdb.load_font_data(INTER_MEDIUM.to_vec());
            fontdb.load_font_data(INTER_REGULAR.to_vec());
            let opts = usvg::Options {
                fontdb: std::sync::Arc::new(fontdb),
                ..Default::default()
            };
            Self {
                bold: FontRef::try_from_slice(INTER_BOLD).expect("valid font"),
                regular: FontRef::try_from_slice(INTER_REGULAR).expect("valid font"),
                opts,
                logo_b64: base64::engine::general_purpose::STANDARD.encode(LOGO_PNG),
            }
        }

        pub fn render(&self, card: &Card) -> Vec<u8> {
            let svg = self.build_svg(card);
            let tree = usvg::Tree::from_str(&svg, &self.opts).expect("generated svg is valid");
            let mut pixmap = resvg::tiny_skia::Pixmap::new(WIDTH, HEIGHT).expect("non-zero size");
            resvg::render(
                &tree,
                resvg::tiny_skia::Transform::identity(),
                &mut pixmap.as_mut(),
            );
            pixmap.encode_png().expect("png encoding")
        }

        fn build_svg(&self, card: &Card) -> String {
            let margin = 72.0;
            let content_width = WIDTH as f32 - margin * 2.0;

            // Vertical stack: eyebrow, title, description. Shrink the title, then drop
            // description lines, until everything fits above the footer.
            let stack_top = 196.0_f32;
            let stack_bottom = 530.0_f32;
            let eyebrow_gap = 40.0;
            let desc_gap = 36.0;
            let desc_px = 30.0;
            let desc_lh = desc_px * 1.4;

            let mut layout = None;
            'outer: for desc_max_lines in [2usize, 1, 0] {
                for &title_px in &[76.0_f32, 68.0, 60.0, 52.0] {
                    let Some(title_lines) =
                        wrap(&self.bold, title_px, &card.title, content_width, 3)
                    else {
                        continue;
                    };
                    let desc_lines = if desc_max_lines == 0 || card.description.is_empty() {
                        Vec::new()
                    } else {
                        fit(
                            &self.regular,
                            &[desc_px],
                            &card.description,
                            content_width,
                            desc_max_lines,
                        )
                        .1
                    };
                    let title_lh = title_px * 1.12;
                    let mut height =
                        eyebrow_gap + title_px + (title_lines.len() as f32 - 1.0) * title_lh;
                    if !desc_lines.is_empty() {
                        height += desc_gap + desc_px + (desc_lines.len() as f32 - 1.0) * desc_lh;
                    }
                    if stack_top + height <= stack_bottom {
                        layout = Some((title_px, title_lines, desc_lines));
                        break 'outer;
                    }
                }
            }
            let (title_px, title_lines, desc_lines) = layout.unwrap_or_else(|| {
                let (px, lines) = fit(&self.bold, &[52.0], &card.title, content_width, 3);
                (px, lines, Vec::new())
            });
            let title_lh = title_px * 1.12;

            let title_first_baseline = stack_top + eyebrow_gap + title_px;
            let title_block_h = title_px + (title_lines.len() as f32 - 1.0) * title_lh;
            let desc_first_baseline = stack_top + eyebrow_gap + title_block_h + desc_gap + desc_px;

            // Drop inner breadcrumb segments until the eyebrow fits on one line.
            let eyebrow_px = 24.0;
            let mut eyebrow_parts: Vec<&str> = card.eyebrow.split(" · ").collect();
            let eyebrow_width = |parts: &[&str]| {
                let text = parts.join(" · ").to_uppercase();
                text_width(&self.regular, eyebrow_px, &text)
                    + text.chars().count() as f32 * eyebrow_px * 0.08
            };
            while eyebrow_parts.len() > 2 && eyebrow_width(&eyebrow_parts) > content_width {
                eyebrow_parts.remove(2);
            }
            let eyebrow = eyebrow_parts.join(" · ").to_uppercase();

            let title_text = text_lines(
                &title_lines,
                "title",
                margin,
                title_first_baseline,
                title_lh,
            );
            let desc_text = text_lines(&desc_lines, "desc", margin, desc_first_baseline, desc_lh);
            let footer_y = HEIGHT as f32 - 56.0;

            format!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">
<defs>
  <linearGradient id="accent" x1="0" y1="0" x2="1" y2="0">
    <stop offset="0" stop-color="#e96b26"/>
    <stop offset="1" stop-color="#00a8d6"/>
  </linearGradient>
  <radialGradient id="glow" cx="1" cy="0" r="1.1">
    <stop offset="0" stop-color="#00a8d6" stop-opacity="0.22"/>
    <stop offset="0.55" stop-color="#e96b26" stop-opacity="0.06"/>
    <stop offset="1" stop-color="#0b0b0f" stop-opacity="0"/>
  </radialGradient>
  <style>
    .title {{ font-family: 'Inter'; font-weight: 700; font-size: {title_px}px; fill: #ffffff; letter-spacing: -0.02em; }}
    .desc {{ font-family: 'Inter'; font-weight: 400; font-size: {desc_px}px; fill: #a3a9b3; }}
    .eyebrow {{ font-family: 'Inter'; font-weight: 500; font-size: {eyebrow_px}px; fill: #00a8d6; letter-spacing: 0.08em; }}
    .brand {{ font-family: 'Inter'; font-weight: 700; font-size: 34px; fill: #ffffff; letter-spacing: -0.01em; }}
    .footer {{ font-family: 'Inter'; font-weight: 500; font-size: 24px; fill: #6b7280; }}
  </style>
</defs>
<rect width="{WIDTH}" height="{HEIGHT}" fill="#0b0b0f"/>
<rect width="{WIDTH}" height="{HEIGHT}" fill="url(#glow)"/>
<rect x="0" y="0" width="{WIDTH}" height="8" fill="url(#accent)"/>
<image x="{margin}" y="72" width="64" height="64" xlink:href="data:image/png;base64,{logo}"/>
<text x="{brand_x}" y="116" class="brand">Dioxus</text>
<text x="{margin}" y="{stack_top}" class="eyebrow">{eyebrow}</text>
{title_text}
{desc_text}
<text x="{margin}" y="{footer_y}" class="footer">dioxuslabs.com</text>
<text x="{right_x}" y="{footer_y}" class="footer" text-anchor="end">{footer_right}</text>
</svg>"##,
                logo = self.logo_b64,
                brand_x = margin + 80.0,
                right_x = WIDTH as f32 - margin,
                eyebrow = escape(&eyebrow),
                footer_right = escape(&card.footer_right),
            )
        }
    }

    fn escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn text_lines(lines: &[String], class: &str, x: f32, first_y: f32, line_height: f32) -> String {
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    r#"<text x="{x}" y="{y}" class="{class}">{t}</text>"#,
                    y = first_y + i as f32 * line_height,
                    t = escape(line)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Advance width of `text` at a CSS font-size of `px`.
    fn text_width(font: &FontRef, px: f32, text: &str) -> f32 {
        // ab_glyph scales by the font's height rather than its em size.
        let scale =
            PxScale::from(px * font.height_unscaled() / font.units_per_em().unwrap_or(1000.0));
        let scaled = font.as_scaled(scale);
        let mut width = 0.0;
        let mut prev: Option<ab_glyph::GlyphId> = None;
        for ch in text.chars() {
            let id = scaled.glyph_id(ch);
            if let Some(p) = prev {
                width += scaled.kern(p, id);
            }
            width += scaled.h_advance(id);
            prev = Some(id);
        }
        width
    }

    /// Greedy word wrap. Returns None if the text doesn't fit in `max_lines`.
    fn wrap(
        font: &FontRef,
        px: f32,
        text: &str,
        max_width: f32,
        max_lines: usize,
    ) -> Option<Vec<String>> {
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if text_width(font, px, &candidate) <= max_width {
                current = candidate;
            } else {
                if !current.is_empty() {
                    lines.push(std::mem::take(&mut current));
                }
                current = word.to_string();
                if lines.len() >= max_lines {
                    return None;
                }
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        (lines.len() <= max_lines).then_some(lines)
    }

    /// Wrap text, trying each font size in order. If none fit, use the last size and
    /// truncate the final line with an ellipsis.
    fn fit(
        font: &FontRef,
        sizes: &[f32],
        text: &str,
        max_width: f32,
        max_lines: usize,
    ) -> (f32, Vec<String>) {
        for &px in sizes {
            if let Some(lines) = wrap(font, px, text, max_width, max_lines) {
                return (px, lines);
            }
        }
        let px = *sizes.last().expect("at least one size");
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for word in text.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if text_width(font, px, &candidate) <= max_width {
                current = candidate;
            } else {
                lines.push(std::mem::take(&mut current));
                current = word.to_string();
                if lines.len() == max_lines {
                    break;
                }
            }
        }
        if lines.len() < max_lines && !current.is_empty() {
            lines.push(current);
        }
        lines.truncate(max_lines);
        if let Some(last) = lines.last_mut() {
            while text_width(font, px, &format!("{last}…")) > max_width {
                match last.rfind(' ') {
                    Some(idx) => last.truncate(idx),
                    None => break,
                }
            }
            let trimmed = last
                .trim_end_matches(|c: char| c.is_ascii_punctuation())
                .len();
            last.truncate(trimmed);
            last.push('…');
        }
        (px, lines)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use dioxus::prelude::Routable;

        /// Renders one card per generated-image route kind. Set `OG_PREVIEW_DIR` to also
        /// write the PNGs out for eyeballing.
        #[test]
        fn renders_cards_for_every_route_kind() {
            let renderer = Renderer::new();
            let preview_dir = std::env::var_os("OG_PREVIEW_DIR").map(std::path::PathBuf::from);
            if let Some(dir) = &preview_dir {
                std::fs::create_dir_all(dir).unwrap();
            }
            for route in Route::static_routes() {
                if !seo::has_generated_og_image(&route) {
                    continue;
                }
                let card = card_for_route(&route);
                assert!(!card.title.is_empty(), "empty title for {route}");
                let png = renderer.render(&card);
                assert!(png.starts_with(b"\x89PNG"));
                if let Some(dir) = &preview_dir {
                    std::fs::write(dir.join(format!("{}.png", seo::og_image_slug(&route))), png)
                        .unwrap();
                }
            }
        }

        #[test]
        fn wraps_and_truncates() {
            let bold = FontRef::try_from_slice(INTER_BOLD).unwrap();
            assert_eq!(
                wrap(&bold, 76.0, "Introducing RSX", 1056.0, 3)
                    .unwrap()
                    .len(),
                1
            );
            assert!(wrap(&bold, 76.0, &"word ".repeat(40), 1056.0, 3).is_none());
            let (_, lines) = fit(&bold, &[76.0], &"word ".repeat(40), 1056.0, 2);
            assert_eq!(lines.len(), 2);
            assert!(lines[1].ends_with('…'));
        }
    }
}
