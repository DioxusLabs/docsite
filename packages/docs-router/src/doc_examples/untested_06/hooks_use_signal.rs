#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    let mut list = use_signal(Vec::new);

    rsx! {
        p { "Current list: {list:?}" }
        button {
            onclick: move |event| {
                let list_len = list.len();
                list.push(list_len);
                list.push(list_len);
            },
            "Add two elements!"
        }
    }
}
// ANCHOR_END: component
