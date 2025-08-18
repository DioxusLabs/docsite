
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
