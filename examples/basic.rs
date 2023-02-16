use dioxus::prelude::*;

fn main() {
    let p = rsx!(div {
        div {
            div { "asd" }
            h2 { "" }
        }
    });
}
