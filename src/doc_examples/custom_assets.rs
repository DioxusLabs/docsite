use dioxus::prelude::*;
use manganis::mg;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { img { src: mg!(file("public/static/scanner.png")) } }
    }
}
