// TODO: Remove public folder with monaco in it (once manganis folder dir works)

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_playground::{Playground, PlaygroundUrls};

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

// Runner-only styling
const MAIN_CSS: &str = asset!("/src/main.css");

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    DefaultPlayground { },

    #[route("/:share_code")]
    SharePlayground { share_code: String }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to start logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        head::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn DefaultPlayground() -> Element {
    rsx! {
        Playground { urls: URLS, share_code: None, }
    }
}

#[component]
fn SharePlayground(share_code: String) -> Element {
    rsx! {
        Playground { urls: URLS, share_code }
    }
}
