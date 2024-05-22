use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            "Build coolgh stuff!"
        }a
    }
}