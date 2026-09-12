//! Per-page SEO/social metadata (Open Graph, Twitter cards) derived from the route.

use crate::components::blog::page_to_meta;
use crate::docs::AnyBookRoute;
use crate::Route;
use mdbook_shared::SummaryItem;

pub const SITE_URL: &str = "https://dioxuslabs.com";
/// Fallback path for the default OG image. Prefer `asset!("/assets/static/opengraph.png")`
/// at the call site since bundled assets get a hash suffix in their served path.
pub const DEFAULT_OG_IMAGE: &str = "/assets/static/opengraph.png";

const HOMEPAGE_TITLE: &str = "Dioxus | Fullstack crossplatform app framework for Rust";
const HOMEPAGE_DESCRIPTION: &str = "Dioxus | A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.";

pub enum PageKind {
    Website,
    Article {
        published: Option<chrono::NaiveDate>,
        author: &'static str,
    },
}

pub struct PageMeta {
    /// full <title>
    pub title: String,
    pub description: String,
    /// absolute canonical URL, no fragment/query
    pub url: String,
    /// absolute URL of the preview image
    pub image: String,
    pub kind: PageKind,
    /// docs && !latest
    pub noindex: bool,
}

impl PageMeta {
    pub fn for_route(route: &Route) -> PageMeta {
        let noindex = route.is_docs() && !route.is_latest_docs();
        let url = canonical_url(route);
        let image = format!("{SITE_URL}{}", og_image_path(route));

        let (title, description, kind) = match route {
            Route::Homepage {} => (
                HOMEPAGE_TITLE.to_string(),
                HOMEPAGE_DESCRIPTION.to_string(),
                PageKind::Website,
            ),
            Route::Docs07 { child } => docs_meta(*child),
            Route::Docs06 { child } => docs_meta(*child),
            Route::Docs05 { child } => docs_meta(*child),
            Route::Docs04 { child } => docs_meta(*child),
            Route::Docs03 { child } => docs_meta(*child),
            Route::BlogPost { child } => {
                let meta = page_to_meta(child.page());
                let published =
                    parse_blog_date(meta.date).or_else(|| parse_blog_date(meta.category));
                (
                    format!("{} – Dioxus Blog", meta.title),
                    meta.description.to_string(),
                    PageKind::Article {
                        published,
                        author: meta.author,
                    },
                )
            }
            Route::BlogList {} => (
                "Blog – Dioxus".to_string(),
                "News, release notes, and articles from the Dioxus team.".to_string(),
                PageKind::Website,
            ),
            Route::Awesome {} => (
                "Awesome Dioxus".to_string(),
                "Community-built libraries, tools, and apps for Dioxus.".to_string(),
                PageKind::Website,
            ),
            Route::Deploy {} => (
                "Deploy Dioxus".to_string(),
                "Deploy Dioxus web, fullstack, desktop, and mobile apps.".to_string(),
                PageKind::Website,
            ),
            Route::Components {} => (
                "Components – Dioxus".to_string(),
                "A gallery of components built with Dioxus.".to_string(),
                PageKind::Website,
            ),
            Route::Err404 { .. } => (
                "Page not found – Dioxus".to_string(),
                HOMEPAGE_DESCRIPTION.to_string(),
                PageKind::Website,
            ),
        };

        PageMeta {
            title,
            description,
            url,
            image,
            kind,
            noindex,
        }
    }
}

fn canonical_url(route: &Route) -> String {
    let path = route.to_string();
    let path = path.split(['#', '?']).next().unwrap_or("/");
    format!("{SITE_URL}{path}")
}

/// Page titles that don't say anything on their own ("Overview") and need their chapter for context.
pub fn is_generic_title(title: &str) -> bool {
    matches!(title, "Overview" | "Index" | "Introduction" | "Summary")
}

/// Chapter pages above this page, outermost first, excluding the book index and pages with generic titles.
pub fn chapter_titles<R: AnyBookRoute>(route: R) -> Vec<String> {
    let mut titles = Vec::new();
    let mut cur = route;
    while let Some(parent) = cur.parent() {
        // The book index renders as "/" relative to its nest
        if parent == cur || parent.to_string() == "/" {
            break;
        }
        let title = &parent.page().title;
        if !is_generic_title(title) {
            titles.push(title.clone());
        }
        cur = parent;
    }
    titles.reverse();
    titles
}

/// The SUMMARY.md part heading (e.g. "Core Concepts") the page is listed under, if any.
pub fn part_title<R: AnyBookRoute>(route: R) -> Option<String> {
    fn contains<R: AnyBookRoute>(items: &[SummaryItem<R>], route: &R) -> bool {
        items.iter().any(|item| match item {
            SummaryItem::Link(link) => {
                link.location.as_ref() == Some(route) || contains(&link.nested_items, route)
            }
            _ => false,
        })
    }

    let summary = &R::book().summary;
    let mut current: Option<&str> = None;
    for item in summary
        .prefix_chapters
        .iter()
        .chain(&summary.numbered_chapters)
        .chain(&summary.suffix_chapters)
    {
        match item {
            SummaryItem::PartTitle(title) => current = Some(title),
            SummaryItem::Link(link)
                if link.location.as_ref() == Some(&route)
                    || contains(&link.nested_items, &route) =>
            {
                return current.map(str::to_string);
            }
            _ => {}
        }
    }
    None
}

fn docs_meta<R: AnyBookRoute>(route: R) -> (String, String, PageKind) {
    let page = route.page();

    let context = if is_generic_title(&page.title) {
        chapter_titles(route).pop().or_else(|| part_title(route))
    } else {
        None
    };
    let title = match context {
        Some(context) => format!(
            "{context}: {} – Dioxus {} Docs",
            page.title,
            R::short_version()
        ),
        None => format!("{} – Dioxus {} Docs", page.title, R::short_version()),
    };

    let description = {
        let desc = markdown_description(R::page_markdown(route.page_id()));
        if desc.is_empty() {
            HOMEPAGE_DESCRIPTION.to_string()
        } else {
            desc
        }
    };

    (title, description, PageKind::Website)
}

fn parse_blog_date(date: &str) -> Option<chrono::NaiveDate> {
    let collapsed = date.split_whitespace().collect::<Vec<_>>().join(" ");
    ["%b %e, %Y", "%B %e, %Y", "%b %e %Y", "%B %e %Y"]
        .iter()
        .find_map(|fmt| chrono::NaiveDate::parse_from_str(&collapsed, fmt).ok())
}

/// Stable slug used to name generated OG images: route path with fragment/query stripped, trimmed of '/', '/' replaced by '-', '.' kept; root => "index".
/// e.g. "/learn/0.7/essentials/ui/rsx" -> "learn-0.7-essentials-ui-rsx", "/blog/release-070" -> "blog-release-070"
pub fn og_image_slug(route: &Route) -> String {
    let path = route.to_string();
    let path = path.split(['#', '?']).next().unwrap_or("/");
    let slug = path.trim_matches('/').replace('/', "-");
    if slug.is_empty() {
        "index".to_string()
    } else {
        slug
    }
}

/// Routes that get a generated per-page image: Docs07 (all pages), BlogPost (all), BlogList, Awesome, Deploy.
/// Everything else (Homepage, older docs, Components, Err404) uses the default image.
pub fn has_generated_og_image(route: &Route) -> bool {
    matches!(
        route,
        Route::Docs07 { .. }
            | Route::BlogPost { .. }
            | Route::BlogList {}
            | Route::Awesome {}
            | Route::Deploy {}
    )
}

/// "/og/<slug>.png" when has_generated_og_image, else DEFAULT_OG_IMAGE
pub fn og_image_path(route: &Route) -> String {
    if has_generated_og_image(route) {
        format!("/og/{}.png", og_image_slug(route))
    } else {
        DEFAULT_OG_IMAGE.to_string()
    }
}

/// First paragraph of a markdown doc as plain text: skips headings, html, code blocks and
/// images; collects the Text and inline Code of the first Paragraph, collapses whitespace
/// and truncates to ~200 chars at a word boundary with "…". Returns "" if there is none.
pub fn markdown_description(md: &str) -> String {
    use pulldown_cmark::{Event, Parser, TagEnd};

    let mut in_paragraph = false;
    let mut text = String::new();

    for event in Parser::new(md) {
        match event {
            Event::Start(pulldown_cmark::Tag::Paragraph) => in_paragraph = true,
            Event::End(TagEnd::Paragraph) if in_paragraph => {
                in_paragraph = false;
                // Skip mdbook directives like `{{#include ...}}` that pulldown
                // sees as plain paragraphs
                if !text.trim_start().starts_with("{{#") {
                    break;
                }
                text.clear();
            }
            Event::Text(t) | Event::Code(t) if in_paragraph => {
                text.push_str(&t);
            }
            Event::SoftBreak | Event::HardBreak if in_paragraph => text.push(' '),
            _ => {}
        }
    }

    // Collapse whitespace
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    const MAX_LEN: usize = 200;
    if text.len() <= MAX_LEN {
        return text;
    }

    // Truncate at a word boundary
    let mut end = MAX_LEN;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    match text[..end].rfind(char::is_whitespace) {
        Some(boundary) => format!("{}…", text[..boundary].trim_end()),
        None => format!("{}…", &text[..end]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::Routable;

    #[test]
    fn description_after_heading() {
        let md = "# Heading\n\nThe first paragraph of the page.\n\nSecond paragraph.";
        assert_eq!(markdown_description(md), "The first paragraph of the page.");
    }

    #[test]
    fn description_with_inline_code_and_links() {
        let md = "Use `dioxus::launch` to start [your app](https://example.com) quickly.";
        assert_eq!(
            markdown_description(md),
            "Use dioxus::launch to start your app quickly."
        );
    }

    #[test]
    fn description_joins_inline_events_without_extra_spaces() {
        let md = "See the [GitHub repository](https://example.com).\nMore on the\nnext line.";
        assert_eq!(
            markdown_description(md),
            "See the GitHub repository. More on the next line."
        );
    }

    #[test]
    fn description_skips_include_and_html() {
        let md =
            "{{#include some_file.md}}\n\n<div class=\"banner\">html</div>\n\nReal content here.";
        assert_eq!(markdown_description(md), "Real content here.");
    }

    #[test]
    fn description_truncates_long_paragraph() {
        let words = "word ".repeat(100);
        let md = format!("# Intro\n\n{words}");
        let desc = markdown_description(&md);
        assert!(desc.ends_with('…'));
        assert!(desc.len() <= 205);
        assert!(!desc[..desc.len() - '…'.len_utf8()].ends_with(' '));
    }

    #[test]
    fn description_empty() {
        assert_eq!(
            markdown_description("# Only a heading\n\n```\ncode\n```"),
            ""
        );
    }

    #[test]
    fn slug_examples() {
        assert_eq!(og_image_slug(&Route::Homepage {}), "index");
        assert_eq!(og_image_slug(&Route::BlogList {}), "blog");
        assert_eq!(
            og_image_slug(&Route::BlogPost {
                child: crate::docs::router_blog::BookRoute::static_routes()
                    .into_iter()
                    .next()
                    .unwrap()
            })
            .starts_with("blog-"),
            true
        );
    }
}
