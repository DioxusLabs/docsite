use crate::dx_components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use dioxus::prelude::*;

#[component]
pub fn Modal(
    icon: Element,
    title: String,
    text: String,
    ok_text: Option<String>,
    on_ok: EventHandler,
) -> Element {
    let ok_text = ok_text.unwrap_or("Ok".to_string());
    let mut open = use_signal(|| true);

    rsx! {
        DialogRoot {
            open: open(),
            DialogContent {
                button {
                    class: "dialog-close",
                    r#type: "button",
                    aria_label: "Close",
                    tabindex: if open() { "0" } else { "-1" },
                    onclick: move |_| open.set(false),
                    "×"
                }
                DialogTitle {
                    {icon}
                    "{title}"
                }
                DialogDescription { "{text}" }
            }
        }
    }
}
