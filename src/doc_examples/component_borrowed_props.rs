#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    let hello = "Hello Dioxus!";

    cx.render(rsx!(TitleCard { title: hello }))
}
// ANCHOR_END: App

// ANCHOR: TitleCard
#[derive(Props)]
struct TitleCardProps<'a> {
    title: String,
}

fn TitleCard(props: TitleCardProps) -> Element {
    cx.render(rsx! {
        h1 { "{props.title}" }
    })
}
// ANCHOR_END: TitleCard
