use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            img { src: "/public/static/scanner.png" }
        }
    }
}
