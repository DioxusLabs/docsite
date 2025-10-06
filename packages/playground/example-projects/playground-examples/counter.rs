//! A basic counter example demonstrating signals,
//! event handlers, and basic rendering.

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        p { "Count: {count}" }
        div { style: "display: flex;",
            button { onclick: move |_| count -= 1, "-" }
            button { onclick: move |_| count += 1, "+" }
        }
    }
}
