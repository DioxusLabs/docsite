// ANCHOR: all
#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    // ANCHOR: Clickable_usage
    rsx! {
        Clickable { href: "https://www.youtube.com/watch?v=C-M2hs3sXGo",
            "How to "
            i { "not" }
            " be seen"
        }
    }
    // ANCHOR_END: Clickable_usage
}

#[derive(PartialEq, Clone, Props)]
struct ClickableProps {
    href: String,
    children: Element,
}

// ANCHOR: Clickable
fn Clickable(props: ClickableProps) -> Element {
    match props.children {
        Some(VNode { .. }) => {
            todo!("render some stuff")
        }
        _ => {
            todo!("render some other stuff")
        }
    }
}
// ANCHOR_END: Clickable
