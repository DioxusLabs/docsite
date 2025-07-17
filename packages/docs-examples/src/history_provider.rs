#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::router::RouterConfig;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

// ANCHOR: app
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> { config: || RouterConfig::default() }
    }
}
// ANCHOR_END: app

#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

fn main() {}
