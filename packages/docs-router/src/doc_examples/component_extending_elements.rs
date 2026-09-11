#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    // ANCHOR: ExtendingPanelUsage
    rsx! {
        Panel {
            style: "border: 1px solid #ccc; padding: 1em;",
            id: "main-panel",
            class: "highlight"
        }
    }
    // ANCHOR_END: ExtendingPanelUsage
}

// ANCHOR: ExtendingPanel
#[component]
fn Panel(#[props(extends = GlobalAttributes)] attrs: Vec<Attribute>) -> Element {
    rsx! {
        div {
            ..attrs,
            "This is a panel"
        }
    }
}
// ANCHOR_END: ExtendingPanel

#[component]
pub fn ActionApp() -> Element {
    // ANCHOR: ExtendingButtonUsage
    rsx! {
        ActionButton {
            // TODO: File an issue to add this callback to `ActionButtonPropsBuilder<()>`
            // cause button has `onclick` event handler and should be extended
            // onclick: move |_| {},
            disabled: true,
            title: "Click to execute",
            class: "btn"
        }
    }
    // ANCHOR_END: ExtendingButtonUsage
}

// ANCHOR: ExtendingButton
#[component]
fn ActionButton(
    #[props(extends = GlobalAttributes, extends = button)] attrs: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            ..attrs,
        }
    }
}
// ANCHOR_END: ExtendingButton
