#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: rsx
    rsx! {
        button { onclick: move |event| log::info!("Clicked! Event: {event:?}"), "click me!" }
    }
    // ANCHOR_END: rsx
}
