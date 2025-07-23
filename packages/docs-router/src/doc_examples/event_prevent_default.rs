#![allow(non_snake_case)]
use crate::doc_examples::{ComponentWithLogs, log};
use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: prevent_default
    rsx! {
        a {
            href: "https://example.com",
            onclick: |evt| {
                evt.prevent_default();
                log!("link clicked")
            },
            "example.com"
        }
    }
    // ANCHOR_END: prevent_default
}

#[component]
pub fn AppDemo() -> Element {
    rsx! {
        ComponentWithLogs { App {} }
    }
}
