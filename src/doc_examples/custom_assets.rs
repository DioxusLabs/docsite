use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            img { src: "examples/assets/logo.png" }
        }
    }
}
