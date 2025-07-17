#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home(cx: Scope) -> Element {
    todo!()
}

// ANCHOR: history_buttons
fn HistoryNavigation(cx: Scope) -> Element {
    render! {
        GoBackButton {
            "Back to the Past"
        }
        GoForwardButton {
            "Back to the Future" /* You see what I did there? 😉 */
        }
    }
}
// ANCHOR_END: history_buttons

fn main() {}
