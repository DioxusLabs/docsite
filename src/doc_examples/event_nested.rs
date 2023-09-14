use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
    // ANCHOR: rsx
    cx.render(rsx! {
        div {
            onclick: move |_event| {},
            "outer",
            button {
                onclick: move |event| {
                    // now, outer won't be triggered
                    event.stop_propagation();
                },
                "inner"
            }
        }
    })
    // ANCHOR_END: rsx
}
