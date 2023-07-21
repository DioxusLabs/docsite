#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    // ANCHOR: boolean_attribute
    cx.render(rsx! {
        div {
            hidden: false,
            "hello"
        }
    })
    // ANCHOR_END: boolean_attribute
}
