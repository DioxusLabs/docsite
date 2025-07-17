#![allow(non_snake_case)]
use crate::doc_examples::{ComponentWithLogs, log};
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    rsx! {
        About {}
        About {}
    }
}
// ANCHOR_END: App

// ANCHOR: About
pub fn About() -> Element {
    rsx! {
        p {
            b { "Dioxus Labs" }
            " An Open Source project dedicated to making Rust UI wonderful."
        }
    }
}
// ANCHOR_END: About

// ANCHOR: MyComponent
#[component]
pub fn MyComponent(name: String) -> Element {
    rsx! {
        div {
            h1 { "Hello, {name}!" }
        }
    }
}
// ANCHOR_END: MyComponent


#[component]
pub fn MyComponentCall() -> Element {
    // ANCHOR: MyComponentCall
    rsx! {
        MyComponent { name: "World" }
    }
    // ANCHOR_END: MyComponentCall
}


// ANCHOR: MyStatefulComponent
#[component]
pub fn MyStatefulComponent() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "Count: {count}" }
            button { onclick: move |_| count += 1, "Increment" }
        }
    }
}
// ANCHOR_END: MyStatefulComponent

// ANCHOR: double
fn double(x: i32) -> i32 {
    x * 2
}
// ANCHOR_END: double

use std::sync::atomic::{AtomicI32, Ordering};

// ANCHOR: increment_global_count
static GLOBAL_COUNT: AtomicI32 = AtomicI32::new(0);

fn increment_global_count() -> i32 {
    GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst)
}
// ANCHOR_END: increment_global_count


// ANCHOR: MyImpureComponent
#[component]
fn MyImpureComponent() -> Element {
    let mut count = use_signal(|| 0);

    // ❌ Modifying the count signal from a hook is a side effect.
    // Dioxus may try to rerender the component with the new value,
    // which can lead to unexpected behavior or infinite loops.
    count += 1;

    rsx! {
        div {
            h1 { "Count: {count}" }
        }
    }
}
// ANCHOR_END: MyImpureComponent

// ANCHOR: MyPureComponent
#[component]
fn MyPureComponent() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "Count: {count}" }
            button {
                // ✅ Event handlers can modify state and have side effects.
                onclick: move |_| count += 1,
                "Increment"
            }
        }
    }
}
// ANCHOR_END: MyPureComponent

// ANCHOR: Button
// When the name property changes, the component will rerender
#[component]
fn Button(name: String) -> Element {
    let mut count = use_signal(|| 0);
    log!("Component rerendered with name: {name} count: {count}");

    rsx! {
        h1 { "Hello, {name}!" }
        // MyComponent reads the `count` signal, so it will rerender
        // whenever `count` changes.
        "Count: {count}"
        button {
            // When the button is clicked, it increments the count signal
            onclick: move |_| count += 1,
            "Increment"
        }
    }
}
// ANCHOR_END: Button

pub fn ButtonDemo() -> Element {
    let mut name = use_signal(|| "World".to_string());
    rsx! {
        ComponentWithLogs {
            input {
                color: "black",
                padding_bottom: "0.5rem",
                value: "{name}",
                oninput: move |e| name.set(e.value()),
            }
            Button { name }
        }
    }
}
