// ANCHOR: router
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
}
// ANCHOR_END: router

// ANCHOR: app
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
// ANCHOR_END: app

// ANCHOR: home
#[component]
fn Home() -> Element {
    rsx! { h1 { "Welcome to the Dioxus Blog!" } }
}
// ANCHOR_END: home

fn main() {}
