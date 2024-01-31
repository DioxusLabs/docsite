#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut list = use_signal(Vec::new);

    rsx!(
        p { "Current list: {list.read():?}" }
        button {
            onclick: move |event| {
                list.with_mut(|list| list.push(event));
            },
            "Click me!"
        }
    )
}
// ANCHOR_END: component
