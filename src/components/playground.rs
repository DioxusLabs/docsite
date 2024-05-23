use dioxus::prelude::*;

#[cfg(not(debug_assertions))]
const SOCKET_URI: &str = "ws://play.dioxuslabs.com/ws";

#[cfg(debug_assertions)]
const SOCKET_URI: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URI: &str = "https://play.dioxuslabs.com/built/";

#[cfg(debug_assertions)]
const BUILT_URI: &str = "http://localhost:3000/built/";

#[component]
pub fn Playground() -> Element {
    rsx! {
        dioxus_playground::Playground {
            socket_uri: SOCKET_URI,
            built_uri: BUILT_URI,
        },
    }
}