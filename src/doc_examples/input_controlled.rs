#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut name = use_signal(|| "bob".to_string());

    rsx! {
        input {
            // we tell the component what to render
            value: "{name}",
            // and what to do when the value changes
            oninput: move |event| name.set(event.value())
        }
    }
}
// ANCHOR_END: component
