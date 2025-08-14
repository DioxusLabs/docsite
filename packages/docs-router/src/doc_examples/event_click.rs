#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::doc_examples::{ComponentWithLogs, log};

pub fn App() -> Element {
    // ANCHOR: rsx
    rsx! {
        button { width: "100%", height: "100%",
            // This event handler will be called when the button is clicked
            onclick: move |event| log!("Clicked! Event: {event:#?}"),
            "click me!"
        }
    }
    // ANCHOR_END: rsx
}

#[component]
pub fn AppDemo() -> Element {
    rsx! {
        ComponentWithLogs { App {} }
    }
}
