// ANCHOR: all
use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { "Hello, world!" }
    }
}
// ANCHOR_END: all
