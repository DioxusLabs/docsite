// ANCHOR: router
#![allow(non_snake_case)]
use dioxus::prelude::*;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
}
// ANCHOR_END: router

// ANCHOR: app
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> { config: || RouterConfig::default() }
    }
}
// ANCHOR_END: app

// ANCHOR: home
#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
// ANCHOR_END: home

fn main() {}
