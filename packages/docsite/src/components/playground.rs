use dioxus::prelude::*;
use dioxus_playground::PlaygroundUrls;

#[cfg(not(feature = "production"))]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "ws://localhost:3000/ws",
    server: "http://localhost:3000/built/",
    location: "http://localhost:8080",
};

#[cfg(feature = "production")]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "wss://docsite-playground.fly.dev/ws",
    built: "https://docsite-playground.fly.dev/built/",
    location: "https://dioxuslabs.com/playground",
};

#[component]
pub fn Playground() -> Element {
    // Only render playground on client.
    let mut on_client = use_signal(|| false);
    use_effect(move || on_client.set(true));

    if on_client() {
        rsx! {
            dioxus_playground::Playground {
                urls: URLS,
                class: "playground-container max-w-screen-2xl mx-auto mt-8",
            }
        }
    } else {
        rsx! {}
    }
}
