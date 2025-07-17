#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: route
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // Routes always start with a slash
    #[route("/")]
    Home {},
    // You can have multiple segments in a route
    #[route("/hello/world")]
    HelloWorld {},
}

#[component]
fn Home() -> Element {
    todo!()
}

#[component]
fn HelloWorld() -> Element {
    todo!()
}
// ANCHOR_END: route

fn main() {}
