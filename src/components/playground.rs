use dioxus::prelude::*;

#[cfg(not(debug_assertions))]
const SOCKET_URL: &str = "wss://docsite-playground.fly.dev/ws"; // todo: switch to play.dioxuslabs.com when the subdomain becomes live

#[cfg(debug_assertions)]
const SOCKET_URL: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URL: &str = "https://docsite-playground.fly.dev/built/";

#[cfg(debug_assertions)]
const BUILT_URL: &str = "http://localhost:3000/built/";

use dioxus_playground::PlaygroundUrls;

#[cfg(not(feature = "real-server"))]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "ws://localhost:3000/ws",
    built: "http://localhost:3000/built/",
    location: "http://localhost:8080",
};

#[cfg(feature = "real-server")]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "ws://play.dioxuslabs.com/ws",
    built: "https://play.dioxuslabs.com/built/",
    location: "http://localhost:8080",
};

#[component]
pub fn Playground() -> Element {
    rsx! {
        dioxus_playground::Playground {
            urls: URLS,
            class: "playground-container max-w-screen-2xl mx-auto mt-8",
        }
    }
}
