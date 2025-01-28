//! The simplest Dioxus app

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { "Build cool stuff!" }
    }
}
