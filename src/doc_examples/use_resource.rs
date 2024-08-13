#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    #[serde(rename = "message")]
    image_url: String,
}

pub fn App() -> Element {
    // ANCHOR: use_resource
    let mut future = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });
    // ANCHOR_END: use_resource

    // ANCHOR: render
    match &*future.read_unchecked() {
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
    // ANCHOR_END: render
}

#[component]
fn RandomDog(breed: ReadOnlySignal<String>) -> Element {
    // ANCHOR: dependency
    let future = use_resource(move || async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    // You can also add non-reactive state to the resource hook with the use_reactive method
    let non_reactive_state = "poodle";
    use_resource(use_reactive!(|(non_reactive_state,)| async move {
        reqwest::get(format!(
            "https://dog.ceo/api/breed/{non_reactive_state}/images/random"
        ))
        .await
        .unwrap()
        .json::<ApiResponse>()
        .await
    }));
    // ANCHOR_END: dependency

    rsx!()
}
