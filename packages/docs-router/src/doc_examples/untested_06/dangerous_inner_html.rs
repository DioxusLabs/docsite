#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: dangerous_inner_html
    // this should come from a trusted source
    let contents = "live <b>dangerously</b>";

    rsx! {
        div { dangerous_inner_html: "{contents}" }
    }
    // ANCHOR_END: dangerous_inner_html
}
