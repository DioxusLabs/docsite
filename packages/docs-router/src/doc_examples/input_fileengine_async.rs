#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        input {
            type: "file",
            accept: ".txt,.rs",
            multiple: true,
            // ANCHOR: onchange_event
            onchange: move |evt| {
                async move {
                    for file in evt.files() {
                        if let Ok(file) = file.read_string().await {
                            files_uploaded.write().push(file);
                        }
                    }
                }
            }
            // ANCHOR_END: onchange_event
        }
    }
}
// ANCHOR_END: component
