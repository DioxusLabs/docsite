#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: rsx
    rsx! {
        div { onclick: move |_event| {},
            "outer"
            button {
                onclick: move |event| {
                    event.stop_propagation();
                },
                "inner"
            }
        }
    }
    // ANCHOR_END: rsx
}
