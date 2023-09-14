use dioxus::prelude::*;

#[component]
pub fn HelloWorldCounter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
