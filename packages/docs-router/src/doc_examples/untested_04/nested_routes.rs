#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        Home {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
// ANCHOR_END: router

// ANCHOR: nav
#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li { "links" }
            }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    }
}
// ANCHOR_END: nav

// ANCHOR: app
#[component]
fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
// ANCHOR_END: app

// ANCHOR: home
#[component]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcome to the Dioxus Blog!" }
    }
}
// ANCHOR_END: home

// ANCHOR: fallback
#[component]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
// ANCHOR_END: fallback

fn main() {}
