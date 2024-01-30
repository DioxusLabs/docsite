#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: App
pub fn App() -> Element {
    rsx! {
        Likes {
            score: 42,
        },
    }
}
// ANCHOR_END: App

// ANCHOR: Likes
// Remember: Owned props must implement `PartialEq`!
#[derive(PartialEq, Props)]
struct LikesProps {
    score: i32,
}

fn Likes(props: LikesProps) -> Element {
    rsx! {
        div {
            "This post has ",
            b { "{cx.props.score}" },
            " likes"
        }
    }
}
// ANCHOR_END: Likes
