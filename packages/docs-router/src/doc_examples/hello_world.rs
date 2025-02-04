#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn HelloWorldCounter() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
