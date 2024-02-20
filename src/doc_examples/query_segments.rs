#![allow(non_snake_case, unused)]
use std::fmt::Display;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // segments that start with ?: are query segments
    #[route("/blog?:query_params")]
    BlogPost {
        // You must include query segments in child variants
        name: String,
        surname: String,
    },
}

#[component]
fn BlogPost(query_params: BlogQuerySegments) -> Element {
    rsx! {
        div { "This is your blogpost with a query segment:" }
        div { "{query_params:?}" }
    }
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

fn main() {}
