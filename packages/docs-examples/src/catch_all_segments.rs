#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // segments that start with :.. are catch all segments
    #[route("/blog/:..segments")]
    BlogPost {
        // You must include catch all segment in child variants
        segments: Vec<String>,
    },
}

// Components must contain the same catch all segments as their corresponding variant
#[component]
fn BlogPost(segments: Vec<String>) -> Element {
    todo!()
}
// ANCHOR_END: route

fn main() {}
