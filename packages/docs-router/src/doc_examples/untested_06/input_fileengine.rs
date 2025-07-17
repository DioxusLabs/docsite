#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    // ANCHOR: rsx
    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);
    rsx! {
        input {
            // tell the input to pick a file
            r#type: "file",
            // list the accepted extensions
            accept: ".txt,.rs",
            // pick multiple files
            multiple: true,
            onchange: move |evt| {
                if let Some(file_engine) = &evt.files() {
                    let files = file_engine.files();
                    for file_name in files {
                        filenames.write().push(file_name);
                    }
                }
            }
        }
    }
    // ANCHOR_END: rsx
}
// ANCHOR_END: component
