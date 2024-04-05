#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut files_uploaded: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        input {
            r#type: "file",
            accept: ".txt,.rs",
            multiple: true,
            // ANCHOR: onchange_event
            onchange: move |evt| {
                async move {
                    if let Some(file_engine) = evt.files() {
                        let files = file_engine.files();
                        for file_name in &files {
                            if let Some(file) = file_engine.read_file_to_string(file_name).await
                            {
                                files_uploaded.write().push(file);
                            }
                        }
                    }
                }
            }
            // ANCHOR_END: onchange_event
        }
    }
}
// ANCHOR_END: component
