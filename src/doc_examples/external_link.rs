#![allow(unused)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home(cx: Scope) -> Element {
    todo!()
}

fn main() {}

// ANCHOR: component
#[component]
fn GoToDioxus(cx: Scope) -> Element {
    render! {
        Link {
            to: "https://dioxuslabs.com",
            "ExternalTarget target"
        }
    }
}
// ANCHOR_END: component
