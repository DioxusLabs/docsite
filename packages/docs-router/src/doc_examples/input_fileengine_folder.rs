#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let filenames: Signal<Vec<String>> = use_signal(Vec::new);
    rsx! {
        // ANCHOR: rsx
        input {
            type: "file",
            // Select a folder by setting the directory attribute
            directory: true,
            onchange: move |evt| {
                for file in evt.files() {
                    println!("{}", file.name());
                }
            }
        }
        // ANCHOR_END: rsx
    }
}
// ANCHOR_END: component
