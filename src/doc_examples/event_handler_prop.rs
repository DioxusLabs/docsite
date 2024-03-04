#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: usage
    rsx! { FancyButton { on_click: move |event| println!("Clicked! {event:?}") } }
    // ANCHOR_END: usage
}

// ANCHOR: component_with_handler
#[derive(PartialEq, Clone, Props)]
pub struct FancyButtonProps {
    on_click: EventHandler<MouseEvent>,
}

pub fn FancyButton(props: FancyButtonProps) -> Element {
    rsx!(
        button {
            class: "fancy-button",
            onclick: move |evt| props.on_click.call(evt),
            "click me pls."
        }
    )
}
// ANCHOR_END: component_with_handler

// ANCHOR: custom_data
struct ComplexData(i32);

#[derive(PartialEq, Clone, Props)]
pub struct CustomFancyButtonProps {
    on_click: EventHandler<ComplexData>,
}

pub fn CustomFancyButton(props: CustomFancyButtonProps) -> Element {
    rsx!(
        button {
            class: "fancy-button",
            onclick: move |_| props.on_click.call(ComplexData(0)),
            "click me pls."
        }
    )
}
// ANCHOR_END: custom_data
