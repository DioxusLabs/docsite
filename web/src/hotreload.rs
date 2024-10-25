//! This is simplified  hot reloading for a single main.rs file.
use dioxus_core::internal::{HotReloadTemplateWithLocation, HotReloadedTemplate};
use dioxus_html::HtmlCtx;
use dioxus_rsx::CallBody;
use dioxus_rsx_hotreload::{diff_rsx, ChangedRsx};
use std::{collections::HashMap, fmt::Display, path::Path};
use syn::spanned::Spanned as _;

pub struct HotReload {
    cached_parse: CachedParse,
}

struct CachedParse {
    raw: String,
    templates: HashMap<String, HotReloadedTemplate>,
}

impl HotReload {
    pub fn new() -> Self {
        Self {
            cached_parse: {
                CachedParse {
                    raw: String::new(),
                    templates: HashMap::new(),
                }
            },
        }
    }

    pub fn set_starting_code(&mut self, code: &str) {
        self.cached_parse = CachedParse {
            raw: code.to_string(),
            templates: HashMap::new(),
        };
    }

    fn full_rebuild(&mut self, code: String) -> HotReloadError {
        self.cached_parse = CachedParse {
            raw: code,
            templates: HashMap::new(),
        };

        HotReloadError::NeedsRebuild
    }

    pub fn process_file_change(
        &mut self,
        new_code: String,
    ) -> Result<Vec<HotReloadTemplateWithLocation>, HotReloadError> {
        let new_file = syn::parse_file(&new_code).map_err(|_err| HotReloadError::Parse)?;

        let cached = &mut self.cached_parse;
        let cached_file = syn::parse_file(&cached.raw).map_err(|_err| HotReloadError::Parse)?;

        let changes = match diff_rsx(&new_file, &cached_file) {
            Some(rsx_calls) => rsx_calls,
            None => return Err(self.full_rebuild(new_code)),
        };

        let mut out_templates = Vec::new();

        for calls in changes.into_iter() {
            let ChangedRsx { old, new } = calls;

            let old_start = old.span().start();

            let old_parsed = syn::parse2::<CallBody>(old.tokens);
            let new_parsed = syn::parse2::<CallBody>(new.tokens);
            let (Ok(old_call_body), Ok(new_call_body)) = (old_parsed, new_parsed) else {
                continue;
            };

            let template_location = template_location(old_start);

            // Returns a list of templates that are hotreloadable
            let hotreload_result = dioxus_rsx_hotreload::HotReloadResult::new::<HtmlCtx>(
                &old_call_body.body,
                &new_call_body.body,
                template_location.clone(),
            );

            // if the template is not hotreloadable, we need to do a full rebuild
            let Some(mut results) = hotreload_result else {
                return Err(self.full_rebuild(new_code));
            };

            // Be careful to not send the bad templates
            results.templates.retain(|idx, template| {
                // dioxus cannot handle empty templates...
                if template.roots.is_empty() {
                    return false;
                }
                let template_location = format_template_name(&template_location, *idx);

                // if the template is the same, don't send its
                if cached.templates.get(&template_location) == Some(&*template) {
                    return false;
                };

                // Update the most recent idea of the template
                // This lets us know if the template has changed so we don't need to send it
                cached.templates.insert(template_location, template.clone());

                true
            });

            out_templates.extend(results.templates.into_iter().map(|(idx, template)| {
                HotReloadTemplateWithLocation {
                    location: format_template_name(&template_location, idx),
                    template,
                }
            }));
        }
        Ok(out_templates)
    }
}

fn template_location(old_start: proc_macro2::LineColumn) -> String {
    let file = Path::new("src/main.rs");
    let line = old_start.line;
    let column = old_start.column + 1;

    // Always ensure the path components are separated by `/`.
    let path = file
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");

    path + ":" + line.to_string().as_str() + ":" + column.to_string().as_str()
}

fn format_template_name(name: &str, index: usize) -> String {
    format!("{}:{}", name, index)
}

#[derive(Debug, PartialEq)]
pub enum HotReloadError {
    NeedsRebuild,
    Parse,
}

impl Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NeedsRebuild => write!(f, "needs rebuild"),
            Self::Parse => write!(f, "failed to parse rust file")
        }
    }
}
