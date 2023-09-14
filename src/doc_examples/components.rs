use dioxus::prelude::*;

// ANCHOR: App
#[component]
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        About {},
        About {},
    })
}
// ANCHOR_END: App

// ANCHOR: About
#[component]
pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        " An Open Source project dedicated to making Rust UI wonderful."
    }))
}
// ANCHOR_END: About
