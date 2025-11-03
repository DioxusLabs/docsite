use crate::dx_components::dialog::{DialogContent, DialogDescription, DialogRoot, DialogTitle};
use dioxus::prelude::*;

#[derive(Clone)]
struct ModalContext {
    open: Signal<bool>,
    on_ok: EventHandler,
}

#[component]
pub fn Modal(on_ok: EventHandler, open: ReadSignal<bool>, children: Element) -> Element {
    let mut internal_open = use_signal(|| false);
    use_effect(move || internal_open.set(open()));
    use_context_provider(move || ModalContext {
        open: internal_open,
        on_ok,
    });
    rsx! {
        DialogRoot {
            open: internal_open(),
            on_open_change: move |_| {
                on_ok(());
            },
            {children}
        }
    }
}

#[component]
pub fn ModalContent(
    icon: Element,
    title: String,
    text: String,
    ok_text: Option<String>,
) -> Element {
    let ModalContext { open, on_ok } = use_context();
    rsx! {
        DialogContent {
            button {
                class: "dialog-close",
                r#type: "button",
                aria_label: "Close",
                tabindex: if open() { "0" } else { "-1" },
                onclick: move |_| on_ok(()),
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
