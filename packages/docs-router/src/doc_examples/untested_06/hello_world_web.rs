#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

fn main() {
    // launch the web app
    launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
