#![allow(non_snake_case, unused)]
use std::fmt::Display;

use dioxus::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // segments that start with #: are hash segments
    #[route("/blog#:name")]
    BlogPost {
        // You must include hash segments in child variants
        name: String,
    },
}

#[component]
fn BlogPost(name: String) -> Element {
    rsx! {
        div { "This is your blogpost with a query segment:" }
        div { "Name: {name}" }
    }
}
// ANCHOR_END: route

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

fn main() {}
