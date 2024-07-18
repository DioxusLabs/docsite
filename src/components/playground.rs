use dioxus::prelude::*;

#[cfg(not(debug_assertions))]
const SOCKET_URL: &str = "ws://play.dioxuslabs.com/ws";

#[cfg(debug_assertions)]
const SOCKET_URL: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URL: &str = "https://play.dioxuslabs.com/built/";

#[cfg(debug_assertions)]
const BUILT_URL: &str = "http://localhost:3000/built/";

#[component]
pub fn Playground() -> Element {
    rsx! {
        dioxus_playground::Playground {
            socket_url: SOCKET_URL,
            built_url: BUILT_URL,
        },
    }
}