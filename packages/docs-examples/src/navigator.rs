#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}

// ANCHOR: nav
#[component]
fn Home() -> Element {
    let nav = navigator();

    // push
    nav.push(Route::PageNotFound { route: vec![] });

    // replace
    nav.replace(Route::Home {});

    // go back
    nav.go_back();

    // go forward
    nav.go_forward();

    rsx! { h1 { "Welcome to the Dioxus Blog!" } }
}
// ANCHOR_END: nav

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

fn main() {}
