#![allow(non_snake_case, unused)]
use std::fmt::Display;

use dioxus::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // segments that start with ?: are query segments
    #[route("/blog?:name&:surname")]
    BlogPost {
        // You must include query segments in child variants
        name: String,
        surname: String,
    },
}

#[component]
fn BlogPost(name: String, surname: String) -> Element {
    rsx! {
        div { "This is your blogpost with a query segment:" }
        div { "Name: {name}" }
        div { "Surname: {surname}" }
    }
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

fn main() {}
