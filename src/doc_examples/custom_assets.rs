use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            img { src: asset!("/assets/static/scanner.png") }
        }
    }
}
