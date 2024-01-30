#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let name = use_state(|| "bob".to_string());

    rsx! {
        input {
            // we tell the component what to render
            value: "{name}",
            // and what to do when the value changes
            oninput: move |evt| name.set(evt.value.clone()),
        }
    }
}
// ANCHOR_END: component
