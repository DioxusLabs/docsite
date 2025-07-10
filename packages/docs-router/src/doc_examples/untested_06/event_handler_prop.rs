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

// ANCHOR: callback
#[derive(PartialEq, Clone, Props)]
pub struct CounterProps {
    modify: Callback<u32, u32>,
}

pub fn Counter(props: CounterProps) -> Element {
    let mut count = use_signal(|| 1);

    rsx! {
        button {
            onclick: move |_| count.set(props.modify.call(count())),
            "double"
        }
        div { "count: {count}" }
    }
}
// ANCHOR_END: callback
