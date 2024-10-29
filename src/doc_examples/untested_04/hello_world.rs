#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn HelloWorldCounter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
