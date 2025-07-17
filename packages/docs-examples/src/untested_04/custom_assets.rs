use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            img { src: "examples/assets/logo.png" }
        }
    })
}
