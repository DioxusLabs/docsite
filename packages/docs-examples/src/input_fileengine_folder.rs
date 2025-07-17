#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let filenames: Signal<Vec<String>> = use_signal(Vec::new);
    rsx! {
        // ANCHOR: rsx
        input {
            r#type: "file",
            // Select a folder by setting the directory attribute
            directory: true,
            onchange: move |evt| {
                if let Some(file_engine) = evt.files() {
                    let files = file_engine.files();
                    for file_name in files {
                        println!("{}", file_name);
                    }
                }
            }
        }
        // ANCHOR_END: rsx
    }
}
// ANCHOR_END: component
