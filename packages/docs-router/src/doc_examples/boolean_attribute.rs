#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: boolean_attribute
    rsx! {
        div { hidden: false, "hello" }
    }
    // ANCHOR_END: boolean_attribute
}
