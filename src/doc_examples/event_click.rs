use dioxus::prelude::*;

#[component]
pub fn App(cx: Scope) -> Element {
    // ANCHOR: rsx
    cx.render(rsx! {
        button {
            onclick: move |event| log::info!("Clicked! Event: {event:?}"),
            "click me!"
        }
    })
    // ANCHOR_END: rsx
}
