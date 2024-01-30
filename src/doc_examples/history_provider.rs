#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

// ANCHOR: app
#[component]
fn App() -> Element {
    render! {
        Router::<Route> {
            config: || RouterConfig::default().history(WebHistory::default())
        }
    }
}
// ANCHOR_END: app

#[component]
fn Home() -> Element {
    render! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}

fn main() {}
