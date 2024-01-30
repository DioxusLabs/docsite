#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn HelloWorldCounter() -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
