// ANCHOR: dog_api
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}
// ANCHOR_END: dog_api

mod dog_view_reqwest {
    use super::DogApi;
    use dioxus::prelude::*;

    // ANCHOR: dog_view_reqwest
    #[component]
    fn DogView() -> Element {
        let mut img_src = use_signal(|| "".to_string());

        let fetch_new = move |_| async move {
            let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
                .await
                .unwrap()
                .json::<DogApi>()
                .await
                .unwrap();

            img_src.set(response.message);
        };

        // ..

        rsx! {
            div { id: "dogview",
                img { src: "{img_src}" }
            }
            div { id: "buttons",
                // ..
                button { onclick: fetch_new, id: "save", "save!" }
            }
        }
    }
    // ANCHOR_END: dog_view_reqwest
}

mod dog_view_resource {
    use super::DogApi;
    use dioxus::prelude::*;

    // ANCHOR: dog_view_resource
    #[component]
    fn DogView() -> Element {
        let mut img_src = use_resource(|| async move {
            reqwest::get("https://dog.ceo/api/breeds/image/random")
                .await
                .unwrap()
                .json::<DogApi>()
                .await
                .unwrap()
                .message
        });

        rsx! {
            div { id: "dogview",
                img { src: img_src.cloned().unwrap_or_default() }
            }
            div { id: "buttons",
                button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
                button { onclick: move |_| img_src.restart(), id: "save", "save!" }
            }
        }
    }
    // ANCHOR_END: dog_view_resource
}

mod spawn {
    use dioxus::prelude::*;

    #[component]
    fn DogView() -> Element {
        // ANCHOR: spawn
        rsx! {
            button {
                onclick: move |_| {
                    spawn(async move {
                        // do some async work...
                    });
                }
            }
        }
        // ANCHOR_END: spawn
    }
}
