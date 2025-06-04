#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        form {
            onsubmit: move |event| {
                log::info!("Submitted! {event:?}")
            },
            input { name: "name", },
            input { name: "age", },
            input { name: "date", },
            input { r#type: "submit", },
        }
    })
}
// ANCHOR_END: component
