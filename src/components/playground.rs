use dioxus::prelude::*;

#[component]
pub fn Playground() -> Element {
    rsx! {
        dioxus_playground::Playground {
            socket_uri: "ws://localhost:3000/ws".to_string(),
            built_uri: "http://localhost:3000/built/".to_string(),
        },
    }
}