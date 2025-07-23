// // TODO: Remove public folder with monaco in it (once manganis folder dir works)

// use dioxus::logger::tracing::Level;
// use dioxus::prelude::*;
// use dioxus_playground::{Playground, PlaygroundUrls};

// #[cfg(not(feature = "real-server"))]
// const URLS: PlaygroundUrls = PlaygroundUrls {
//     socket: "ws://localhost:3000/ws",
//     server: "http://localhost:3000",
//     location: "http://localhost:8080",
// };

// #[cfg(feature = "real-server")]
// const URLS: PlaygroundUrls = PlaygroundUrls {
//     socket: "wss://docsite-playground.fly.dev/ws",
//     server: "https://docsite-playground.fly.dev",
//     location: "https://dioxuslabs.com/playground",
// };

// // Runner-only styling
// const MAIN_CSS: Asset = asset!("/src/main.css");

// #[derive(Routable, PartialEq, Clone)]
// enum Route {
//     #[route("/")]
//     DefaultPlayground {},

//     #[route("/shared/:share_code")]
//     SharePlayground { share_code: String },
// }

// fn main() {
//     dioxus::logger::init(Level::INFO).expect("failed to start logger");
//     dioxus::launch(App);
// }

// #[component]
// fn App() -> Element {
//     rsx! {
//         document::Link { rel: "stylesheet", href: MAIN_CSS }
//         Router::<Route> {}
//     }
// }

// #[component]
// fn DefaultPlayground() -> Element {
//     rsx! {
//         Playground { urls: URLS, class: "playground-container" }
//     }
// }

// #[component]
// fn SharePlayground(share_code: ReadOnlySignal<Option<String>>) -> Element {
//     rsx! {
//         Playground { urls: URLS, share_code, class: "playground-container" }
//     }
// }

fn main() {}
