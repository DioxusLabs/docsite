#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: component
pub fn App() -> Element {
    rsx! {
        form {
            onsubmit: move |event| {
                event.prevent_default(); // Prevent the default form submission behavior
                log::info!("Submitted! {event:?}");
            },
            input { name: "name" }
            input { name: "age" }
            input { name: "date" }
            input { r#type: "submit" }
        }
    }
}
// ANCHOR_END: component
