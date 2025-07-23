#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::doc_examples::{ComponentWithLogs, log};

pub fn App() -> Element {
    // ANCHOR: rsx
    rsx! {
        button { onclick: move |event| log!("Clicked! Event: {event:?}"), "click me!" }
    }
    // ANCHOR_END: rsx
}

#[component]
pub fn AppDemo() -> Element {
    rsx! {
        ComponentWithLogs { App {} }
    }
}
