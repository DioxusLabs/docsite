#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    let filenames: &UseRef<Vec<String>> = use_ref(cx, Vec::new);

    cx.render(rsx! {
        input {
            // tell the input to pick a file
            r#type:"file",
            // list the accepted extensions
            accept: ".txt, .rs",
            multiple: true,
            onchange: |evt| {
                if let Some(file_engine) = &evt.files {
                    let files = file_engine.files();
                    for file_name in &files {
                        filenames.write().push(file_name);
                    }
                }
            }
        }
    })
}
// ANCHOR_END: component
