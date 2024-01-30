#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Index {},
    #[route("/home")]
    Home {},
}

#[component]
fn Home() -> Element {
    render! {
        p { "Home" }
    }
}

#[component]
fn Index() -> Element {
    render! {
        p { "Index" }
    }
}

fn app() -> Element {
    render! {
        Router::<Route> {
            config: || RouterConfig::default().on_update(|state|{
                (state.current() == Route::Index {}).then_some(
                    NavigationTarget::Internal(Route::Home {})
                )
            })
        }
    }
}
// ANCHOR_END: router

fn main() {}
