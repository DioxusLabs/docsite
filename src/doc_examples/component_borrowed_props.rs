#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    let hello = "Hello Dioxus!";

    rsx!(TitleCard { title: hello })
}
// ANCHOR_END: App

// ANCHOR: TitleCard
#[derive(Props)]
struct TitleCardProps {
    title: String,
}

fn TitleCard(props: TitleCardProps) -> Element {
    rsx! {
        h1 { "{props.title}" }
    }
}
// ANCHOR_END: TitleCard
