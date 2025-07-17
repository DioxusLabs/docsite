#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

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
    rsx! { p { "Home" } }
}

#[component]
fn Index() -> Element {
    rsx! { p { "Index" } }
}

fn app() -> Element {
    rsx! {
        Router::<Route> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        (state.current() == Route::Index {})
                            .then_some(NavigationTarget::Internal(Route::Home {}))
                    })
            }
        }
    }
}
// ANCHOR_END: router

fn main() {}
