//! this file exists to make the old 04 examples work even if they wouldn't compile with newer versions
#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

pub fn Empty() -> Element {
    rsx! {
        div {}
    }
}
pub fn hooks_out_of_date() -> Element {
    rsx! {
        h1 { "High-Five counter: 0" }
    }
}
pub fn hooks_out_of_date_fixed() -> Element {
    rsx! {
        h1 { "High-Five counter: 0" }
    }
}
pub fn component_borrowed_props() -> Element {
    rsx! {
        h1 { "Hello Dioxus!" }
    }
}

pub fn hooks_use_ref() -> Element {
    let mut list = use_signal(|| Vec::new());

    rsx! {
        p { "Current list: {list:?}" }
        button { onclick: move |event| list.push(event), "Click me!" }
    }
}

pub fn use_future_() -> Element {
    #[derive(serde::Deserialize, Debug, Clone)]
    struct ApiResponse {
        #[serde(rename = "message")]
        image_url: String,
    }

    let mut future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    match &*future.value().read_unchecked() {
        Some(Ok(response)) => rsx! {
            button { onclick: move |_| future.restart(), "Click to fetch another doggo" }
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{response.image_url}",
                }
            }
        },
        Some(Err(_)) => rsx! {
            div { "Loading dogs failed" }
        },
        None => rsx! {
            div { "Loading dogs..." }
        },
    }
}
