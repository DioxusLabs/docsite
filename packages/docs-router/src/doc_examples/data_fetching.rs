mod use_resource {
    // ANCHOR: fetch_resource
    use dioxus::prelude::*;

    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    #[component]
    pub fn DogViewResource() -> Element {
        let img_src = use_resource(|| async {
            let response = reqwest::get("https://dog.ceo/api/breed/poodle/images/random")
                .await?
                .json::<DogApi>()
                .await?;
            dioxus::Ok(response.message)
        });

        match img_src() {
            // The resource successfully loaded
            Some(Ok(src)) => rsx! {
                img { src: "{src}" }
            },
            // The resource finished loading, but there was an error
            Some(Err(err)) => rsx! {
                p { "Error fetching dog image: {err}" }
            },
            // The resource is still loading
            None => rsx! {
                p { "Loading..." }
            },
        }
    }
    // ANCHOR_END: fetch_resource
}

pub use use_resource::DogViewResource;

mod fetch_resource_reactive {

    use dioxus::prelude::*;

    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    #[component]
    pub fn DogViewResourceReactive() -> Element {
        // ANCHOR: fetch_resource_reactive
        let mut breed = use_signal(|| "poodle".to_string());

        let img_src = use_resource(move || async move {
            // The resource depends on the breed signal since it is read in the async closure
            let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
                .await?
                .json::<DogApi>()
                .await?;
            dioxus::Ok(response.message)
        });

        let img_element = match img_src() {
            // The resource successfully loaded
            Some(Ok(src)) => rsx! {
                img { src: "{src}" }
            },
            // The resource finished loading, but there was an error
            Some(Err(err)) => rsx! {
                p { "Error fetching dog image: {err}" }
            },
            // The resource is still loading
            None => rsx! {
                p { "Loading..." }
            },
        };

        // A simple dropdown to select the breed
        rsx! {
            select {
                // The resource will rerun when the breed is set since it is a dependency
                onchange: move |event| breed.set(event.value()),
                option { value: "poodle", "Poodle" },
                option { value: "retriever/golden", "Golden Retriever" },
            }
            {img_element}
        }
        // ANCHOR_END: fetch_resource_reactive
    }
}

pub use fetch_resource_reactive::DogViewResourceReactive;

mod waterfall_effect {
    use std::fmt::Display;

    use dioxus::prelude::*;

    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    // ANCHOR: waterfall_effect
    fn fetch_dog_image(breed: impl Display) -> impl Future<Output = dioxus::Result<String>> {
        async move {
            let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
                .await?
                .json::<DogApi>()
                .await?;
            dioxus::Ok(response.message)
        }
    }

    #[component]
    fn DogView() -> Element {
        let poodle_img = use_resource(|| fetch_dog_image("poodle"));

        let poodle_img = match poodle_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));

        let golden_retriever_img = match golden_retriever_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        let pug_img = use_resource(|| fetch_dog_image("pug"));

        let pug_img = match pug_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        rsx! {
            div {
                h1 { "Dog Images" }
                img { src: "{poodle_img}" }
                img { src: "{golden_retriever_img}" }
                img { src: "{pug_img}" }
            }
        }
    }
    // ANCHOR_END: waterfall_effect
}

mod no_waterfall_effect {
    use std::fmt::Display;

    use dioxus::prelude::*;

    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    fn fetch_dog_image(
        breed: impl Display,
    ) -> impl Future<Output = dioxus::Result<String>> {
        async move {
            let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
                .await?
                .json::<DogApi>()
                .await?;
            dioxus::Ok(response.message)
        }
    }

    #[component]
    fn DogView() -> Element {
        // ANCHOR: no_waterfall_effect
        let poodle_img = use_resource(|| fetch_dog_image("poodle"));
        let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));
        let pug_img = use_resource(|| fetch_dog_image("pug"));

        let poodle_img = match poodle_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };
        let golden_retriever_img = match golden_retriever_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };
        let pug_img = match pug_img() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        rsx! {
            div {
                h1 { "Dog Images" }
                img { src: "{poodle_img}" }
                img { src: "{golden_retriever_img}" }
                img { src: "{pug_img}" }
            }
        }
        // ANCHOR_END: no_waterfall_effect
    }
}
