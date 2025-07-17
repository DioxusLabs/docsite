#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    // ANCHOR: usage
    cx.render(rsx! {
        FancyButton {
            on_click: move |event| println!("Clicked! {event:?}")
        }
    })
    // ANCHOR_END: usage
}

// ANCHOR: component_with_handler
#[derive(Props)]
pub struct FancyButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn FancyButton<'a>(cx: Scope<'a, FancyButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(button {
        class: "fancy-button",
        onclick: move |evt| cx.props.on_click.call(evt),
        "click me pls."
    }))
}
// ANCHOR_END: component_with_handler

// ANCHOR: custom_data
struct ComplexData(i32);

#[derive(Props)]
pub struct CustomFancyButtonProps<'a> {
    on_click: EventHandler<'a, ComplexData>,
}

pub fn CustomFancyButton<'a>(cx: Scope<'a, CustomFancyButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(button {
        class: "fancy-button",
        onclick: move |_| cx.props.on_click.call(ComplexData(0)),
        "click me pls."
    }))
}
// ANCHOR_END: custom_data
