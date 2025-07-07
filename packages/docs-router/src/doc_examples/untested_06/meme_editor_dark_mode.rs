// ANCHOR: all
#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

// ANCHOR: DarkMode_struct
#[derive(Clone, Copy)]
struct DarkMode(bool);
// ANCHOR_END: DarkMode_struct

pub fn App() -> Element {
    // ANCHOR: context_provider
    use_context_provider(|| Signal::new(DarkMode(false)));
    // ANCHOR_END: context_provider

    let is_dark_mode = use_is_dark_mode();

    let wrapper_style = if is_dark_mode {
        r"
            background: #222;
            min-height: 100vh;
        "
    } else {
        r""
    };

    rsx! {
        div { style: "{wrapper_style}",
            DarkModeToggle {}
            MemeEditor {}
        }
    }
}

pub fn use_is_dark_mode() -> bool {
    // ANCHOR: use_context
    let dark_mode_context = use_context::<Signal<DarkMode>>();
    // ANCHOR_END: use_context

    dark_mode_context().0
}

// ANCHOR: toggle
pub fn DarkModeToggle() -> Element {
    let mut dark_mode = use_context::<Signal<DarkMode>>();

    let style = if dark_mode().0 { "color:white" } else { "" };

    rsx! {
        label { style: "{style}",
            "Dark Mode"
            input {
                r#type: "checkbox",
                oninput: move |event| {
                    let is_enabled = event.value() == "true";
                    dark_mode.write().0 = is_enabled;
                }
            }
        }
    }
}
// ANCHOR_END: toggle

// ANCHOR: meme_editor
fn MemeEditor() -> Element {
    let is_dark_mode = use_is_dark_mode();
    let heading_style = if is_dark_mode { "color: white" } else { "" };

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
            h1 { style: "{heading_style}", "Meme Editor" }
            Meme { caption: caption }
            CaptionEditor { caption: caption, oninput: move |event: FormEvent| caption.set(event.value()) }
        }
    }
}
// ANCHOR_END: meme_editor

// ANCHOR: meme_component
#[component]
fn Meme(caption: String) -> Element {
    let container_style = r"
        position: relative;
        width: fit-content;
    ";

    let caption_container_style = r"
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        padding: 16px 8px;
    ";

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
    let is_dark_mode = use_is_dark_mode();

    let colors = if is_dark_mode {
        r"
            background: cornflowerblue;
            color: white;
        "
    } else {
        r"
            background: #def;
            color: black;
        "
    };

    let input_style = r"
        border: none;
        padding: 8px 16px;
        margin: 0;
        border-radius: 4px;
    ";

    rsx! {
        input {
            style: "{input_style}{colors}",
            value: "{caption}",
            oninput: move |event| oninput.call(event)
        }
    }
}
// ANCHOR_END: caption_editor

// ANCHOR_END: all
