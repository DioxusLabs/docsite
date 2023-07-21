#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    // count will be initialized to 0 the first time the component is rendered
    let mut count = use_state(cx, || 0);
    let first_count_read = use_state(cx, || 0);

    // Increase the count
    if *count == 0 {
        count += 1;
        first_count_read.set(**count);
    }

    cx.render(rsx!(
        // This uses the deref value
        h1 { "High-Five counter: {first_count_read}" }
    ))
}
// ANCHOR_END: component

pub mod fixed {
    use dioxus::prelude::*;
    // ANCHOR: fixed
    pub fn App(cx: Scope) -> Element {
        let mut count = use_state(cx, || 0);
        let first_count_read = use_state(cx, || 0);

        // Increase the count
        if *count == 0 {
            count += 1;
            first_count_read.set(*count.current());
        }

        cx.render(rsx!(
            // Use .current to get the real current value
            h1 { "High-Five counter: {first_count_read}" }
        ))
    }
    // ANCHOR_END: fixed
}
