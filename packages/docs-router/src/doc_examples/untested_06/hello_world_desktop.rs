// ANCHOR: all
#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

fn main() {
    // launch the dioxus app in a webview
    launch(App);
}

// ANCHOR: component
// define a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
// ANCHOR_END: component
// ANCHOR_END: all
