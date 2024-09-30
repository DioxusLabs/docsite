#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: usage
    rsx! {
        FancyButton { 
            props: move |event| println!("Clicked! {event:?}"), 
        }
    }
    // ANCHOR_END: usage
}

// ANCHOR: component_with_handler
pub fn FancyButton(props: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |evt| props.call(evt),
            "click me pls."
        }
    }
}
// ANCHOR_END: component_with_handler

// ANCHOR: custom_data
pub struct ComplexData(i32);

pub fn CustomFancyButton(props: EventHandler<ComplexData>) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |_| props.call(ComplexData(0)),
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
            props: move |event| {
                spawn(async move {
                    println!("Clicked! {event:?}");
                });
            },
        }
    }
    // ANCHOR_END: async
}
