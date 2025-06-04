#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    let filenames: &UseRef<Vec<String>> = use_ref(cx, Vec::new);
    cx.render(rsx! {
        // ANCHOR: rsx
        input {
            r#type:"file",
            // Select a folder by setting the directory attribute
            directory: true,
            onchange: |evt| {
                if let Some(file_engine) = &evt.files {
                    let files = file_engine.files();
                    for file_name in files {
                        println!("{}", file_name);
                        // Do something with the folder path
                    }
                }
            }
        }
        // ANCHOR_END: rsx
    })
}
// ANCHOR_END: component
