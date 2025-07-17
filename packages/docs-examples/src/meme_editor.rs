// ANCHOR: all
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(MemeEditor);
}

// ANCHOR: meme_editor
fn MemeEditor() -> Element {
    let container_style = r"
        display: flex;
        flex-direction: column;
        gap: 16px;
        margin: 0 auto;
        width: fit-content;
    ";

    let mut caption = use_signal(|| "me waiting for my rust code to compile".to_string());

    rsx! {
        div { style: "{container_style}",
            h1 { "Meme Editor" }
            Meme { caption: caption }
            CaptionEditor { caption: caption, oninput: move |event: FormEvent| caption.set(event.value()) }
        }
    }
}
// ANCHOR_END: meme_editor

// ANCHOR: meme_component
#[component]
fn Meme(caption: String) -> Element {
    let container_style = r#"
        position: relative;
        width: fit-content;
    "#;

    let caption_container_style = r#"
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 16px 8px;
    "#;

    let caption_style = r"
        font-size: 32px;
        margin: 0;
        color: white;
        text-align: center;
    ";

    rsx! {
        div { style: "{container_style}",
            img { src: "https://i.imgflip.com/2zh47r.jpg", height: "500px" }
            div { style: "{caption_container_style}", p { style: "{caption_style}", "{caption}" } }
        }
    }
}
// ANCHOR_END: meme_component

// ANCHOR: caption_editor
#[component]
fn CaptionEditor(caption: String, oninput: EventHandler<FormEvent>) -> Element {
    let input_style = r"
        border: none;
        background: cornflowerblue;
        padding: 8px 16px;
        margin: 0;
        border-radius: 4px;
        color: white;
    ";

    rsx! {
        input {
            style: "{input_style}",
            value: "{caption}",
            oninput: move |event| oninput.call(event)
        }
    }
}
// ANCHOR_END: caption_editor

// ANCHOR_END: all
