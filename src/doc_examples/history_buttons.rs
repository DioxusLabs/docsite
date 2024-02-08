#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    todo!()
}

// ANCHOR: history_buttons
fn HistoryNavigation() -> Element {
    rsx! {
        GoBackButton { "Back to the Past" }
        GoForwardButton { "Back to the Future" }
    }
}
// ANCHOR_END: history_buttons

fn main() {}
