use dioxus::prelude::*;

pub fn Learn(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full",
            iframe {
                class: "w-full",
                style: "overflow: hidden; height: 100%; width: 100%; position: absolute;",
                src: "https://dioxuslabs.com/guide/"
            }
        }
    })
}
