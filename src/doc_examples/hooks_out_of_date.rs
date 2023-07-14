#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    // count will be initialized to 0 the first time the component is rendered
    let mut count = use_state(cx, || 0);

    // Increase the count
    count += 1;

    cx.render(rsx!(
        // This uses the deref value
        h1 { "High-Five counter: {count}" }
    ))
}
// ANCHOR_END: component

pub mod fixed {
    use dioxus::prelude::*;
    // ANCHOR: fixed
    pub fn App(cx: Scope) -> Element {
        let mut count = use_state(cx, || 0);

        // Increase the count
        count += 1;

        cx.render(rsx!(
            // Use .current to get the real current value
            h1 { "High-Five counter: {count.current()}" }
        ))
    }
    // ANCHOR_END: fixed
}