#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: prevent_default
    rsx! {
        a {
            href: "https://example.com",
            onclick: |evt| {
                evt.prevent_default();
                log::info!("link clicked")
            },
            "example.com"
        }
    }
    // ANCHOR_END: prevent_default
}
