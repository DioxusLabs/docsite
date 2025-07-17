//! Simplified hot reloading for a single main.rs file.
use dioxus::{logger::tracing::error, prelude::*};
use dioxus_core::internal::{
    HotReloadTemplateWithLocation, HotReloadedTemplate, TemplateGlobalKey,
};
use dioxus_devtools::HotReloadMsg;
use dioxus_document::eval;
use dioxus_html::HtmlCtx;
use dioxus_rsx::CallBody;
use dioxus_rsx_hotreload::{diff_rsx, ChangedRsx};
use std::{collections::HashMap, fmt::Display, path::Path};
use syn::spanned::Spanned as _;

/// Atempts to hot reload and returns true if a full rebuild is needed.
pub fn attempt_hot_reload(mut hot_reload: HotReload, new_code: &str) {
    // Process any potential hot -eloadable changes and send them to the iframe web client.
    let result = hot_reload.process_file_change(new_code.to_string());
    match result {
        Ok(templates) => {
            let hr_msg = HotReloadMsg {
                templates,
                assets: Vec::new(),
                ms_elapsed: Default::default(),
                jump_table: Default::default(),
                for_build_id: Default::default(),
                for_pid: Default::default(),
                // unknown_files: Vec::new(),
            };

            let e = eval(
                r#"
                const hrMsg = await dioxus.recv();
                const iframeElem = document.getElementById("dxp-iframe");
                const hrMsgJson = JSON.stringify(hrMsg);

                if (iframeElem) {
                    iframeElem.contentWindow.postMessage(hrMsgJson, "*");
                }
                "#,
            );
            _ = e.send(hr_msg);
        }
        Err(HotReloadError::NeedsRebuild) => hot_reload.set_needs_rebuild(true),
        Err(e) => {
            hot_reload.set_needs_rebuild(true);
            error!("hot reload error occured: {:?}", e);
        }
    }
}

#[derive(Clone, Copy)]
pub struct HotReload {
    needs_rebuild: Signal<bool>,
    cached_parse: Signal<CachedParse>,
}

struct CachedParse {
    raw: String,
    templates: HashMap<TemplateGlobalKey, HotReloadedTemplate>,
}

impl HotReload {
    pub fn new() -> Self {
        Self {
            cached_parse: {
                Signal::new(CachedParse {
                    raw: String::new(),
                    templates: HashMap::new(),
                })
            },
            needs_rebuild: Signal::new(true),
        }
    }

    pub fn set_needs_rebuild(&mut self, needs_rebuild: bool) {
        self.needs_rebuild.set(needs_rebuild);
    }

    pub fn set_starting_code(&mut self, code: &str) {
        *self.cached_parse.write() = CachedParse {
            raw: code.to_string(),
            templates: HashMap::new(),
        };
    }

    fn full_rebuild(&mut self, code: String) -> HotReloadError {
        *self.cached_parse.write() = CachedParse {
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

        let cached_file = {
            let cached = &mut self.cached_parse.read();
            syn::parse_file(&cached.raw).map_err(|_err| HotReloadError::Parse)?
        };

        let changes = match diff_rsx(&new_file, &cached_file) {
            Some(rsx_calls) => rsx_calls,
            None => return Err(self.full_rebuild(new_code)),
        };

        let mut out_templates = Vec::new();

        for ChangedRsx { old, new } in changes {
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
            let Some(results) = hotreload_result else {
                return Err(self.full_rebuild(new_code));
            };

            let mut cached = self.cached_parse.write();
            for (index, template) in results.templates {
                if template.roots.is_empty() {
                    continue;
                }

                // Create the key we're going to use to identify this template
                let key = TemplateGlobalKey {
                    file: "src/main.rs".to_string(),
                    line: old_start.line,
                    column: old_start.column + 1,
                    index,
                };

                // if the template is the same, don't send it
                if cached.templates.get(&key) == Some(&template) {
                    continue;
                };

                cached.templates.insert(key.clone(), template.clone());
                out_templates.push(HotReloadTemplateWithLocation { key, template })
            }
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

#[derive(Debug, PartialEq)]
pub enum HotReloadError {
    NeedsRebuild,
    Parse,
}

impl Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NeedsRebuild => write!(f, "needs rebuild"),
            Self::Parse => write!(f, "failed to parse rust file"),
        }
    }
}
