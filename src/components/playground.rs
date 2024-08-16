use dioxus::prelude::*;

#[cfg(not(debug_assertions))]
const SOCKET_URL: &str = "wss://docsite-playground.fly.dev/ws"; // todo: switch to play.dioxuslabs.com when the subdomain becomes live

#[cfg(debug_assertions)]
const SOCKET_URL: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URL: &str = "https://docsite-playground.fly.dev/built/";

#[cfg(debug_assertions)]
const BUILT_URL: &str = "http://localhost:3000/built/";

#[component]
pub fn Playground() -> Element {
    rsx! {
        dioxus_playground::Playground { socket_url: SOCKET_URL, built_url: BUILT_URL }
    }
}
