#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    todo!()
}

fn main() {}

// ANCHOR: component
fn GoToDioxus() -> Element {
    rsx! {
        Link { to: "https://dioxuslabs.com", "ExternalTarget target" }
    }
}
// ANCHOR_END: component
