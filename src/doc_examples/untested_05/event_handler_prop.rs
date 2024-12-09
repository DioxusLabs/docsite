#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: usage
    rsx! {
        FancyButton {
            onclick: move |event| println!("Clicked! {event:?}"),
        }
    }
    // ANCHOR_END: usage
}

// ANCHOR: component_with_handler
#[derive(PartialEq, Clone, Props)]
pub struct FancyButtonProps {
    onclick: EventHandler<MouseEvent>,
}

pub fn FancyButton(props: FancyButtonProps) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |evt| props.onclick.call(evt),
            "click me pls."
        }
    }
}
// ANCHOR_END: component_with_handler

// ANCHOR: custom_data
struct ComplexData(i32);

#[derive(PartialEq, Clone, Props)]
pub struct CustomFancyButtonProps {
    onclick: EventHandler<ComplexData>,
}

pub fn CustomFancyButton(props: CustomFancyButtonProps) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |_| props.onclick.call(ComplexData(0)),
            "click me pls."
        }
    }
}
// ANCHOR_END: custom_data

pub fn MyComponent() -> Element {
    // ANCHOR: async
    rsx! {
        FancyButton {
            // This does not work!
            // onclick: move |event| async move {
            //      println!("Clicked! {event:?}");
            // },

            // This does work!
            onclick: move |event| {
                spawn(async move {
                    println!("Clicked! {event:?}");
                });
            },
        }
    }
    // ANCHOR_END: async
}
