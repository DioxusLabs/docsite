#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    cx.render(rsx! {
        About {},
        About {},
    })
}
// ANCHOR_END: App

// ANCHOR: About
pub fn About() -> Element {
    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        " An Open Source project dedicated to making Rust UI wonderful."
    }))
}
// ANCHOR_END: About
