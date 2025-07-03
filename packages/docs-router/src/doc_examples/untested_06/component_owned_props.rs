#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    rsx! { Likes { score: 42 } }
}
// ANCHOR_END: App

// ANCHOR: Likes
#[derive(PartialEq, Props, Clone)]
struct LikesProps {
    score: i32,
}

fn Likes(props: LikesProps) -> Element {
    rsx! {
        div {
            "This post has "
            b { "{props.score}" }
            " likes"
        }
    }
}
// ANCHOR_END: Likes
