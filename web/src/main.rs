use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_playground::Playground;


#[cfg(not(debug_assertions))]
const SOCKET_URI: &str = "ws://localhost:3000/ws";

#[cfg(debug_assertions)]
const SOCKET_URI: &str = "ws://localhost:3000/ws";

#[cfg(not(debug_assertions))]
const BUILT_URI: &str = "https://play.dioxuslabs.com/built/";

#[cfg(debug_assertions)]
const BUILT_URI: &str = "http://localhost:3000/built/";

fn main() {
    dioxus_logger::init(Level::WARN).expect("failed to start logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Playground {
            socket_uri: SOCKET_URI.to_string(),
            built_uri: BUILT_URI.to_string(),
        }
    }
}
