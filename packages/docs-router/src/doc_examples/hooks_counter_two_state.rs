#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    // ANCHOR: use_signal_calls
    let mut count_a = use_signal(|| 0);
    let mut count_b = use_signal(|| 0);
    // ANCHOR_END: use_signal_calls

    rsx! {
        h1 { "Counter_a: {count_a}" }
        button { onclick: move |_| count_a += 1, "a++" }
        button { onclick: move |_| count_a -= 1, "a--" }
        h1 { "Counter_b: {count_b}" }
        button { onclick: move |_| count_b += 1, "b++" }
        button { onclick: move |_| count_b -= 1, "b--" }
    }
}
// ANCHOR_END: component
