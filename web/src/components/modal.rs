use dioxus::prelude::*;

#[component]
pub fn Modal(icon_src: Option<String>, title: String, text: String, on_ok: EventHandler) -> Element {
    rsx! {
        // Background
        div {
            id: "dxp-modal-bg",

            div {
                id: "dxp-modal",

                // Modal header with optional icon
                div {
                    id: "dxp-modal-header",
                    if let Some(icon_src) = icon_src {
                        img {
                            src: icon_src
                        }
                    }
                    h4 { "{title}" }
                }

                // Modal description text
                p { "{text}" }

                // ok button
                button {
                    onclick: move |_| on_ok.call(()),
                    "I understand"
                }
            }
        }
    }
}
