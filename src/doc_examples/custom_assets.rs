use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

#[component]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            img { src: "examples/assets/logo.png" }
        }
    })
}
