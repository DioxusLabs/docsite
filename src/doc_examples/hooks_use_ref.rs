#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    let list = use_ref(cx, Vec::new);

    cx.render(rsx!(
        p { "Current list: {list.read():?}" }
        button {
            onclick: move |event| {
                list.with_mut(|list| list.push(event));
            },
            "Click me!"
        }
    ))
}
// ANCHOR_END: component
