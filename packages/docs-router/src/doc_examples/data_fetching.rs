mod waterfall_effect {
    use std::fmt::Display;

    use dioxus::prelude::*;

    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    // ANCHOR: waterfall_effect
    async fn fetch_dog_image(breed: impl Display) -> dioxus::Result<String> {
        let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?
            .json::<DogApi>()
            .await?;
        dioxus::Ok(response.message)
    }

    #[component]
    fn DogView() -> Element {
        let poodle_img = use_resource(|| fetch_dog_image("poodle"));

        let read = poodle_img.read();
        let poodle_img = match read.as_ref() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));

        let read = golden_retriever_img.read();
        let golden_retriever_img = match read.as_ref() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };

        let pug_img = use_resource(|| fetch_dog_image("pug"));

        let read = pug_img.read();
        let pug_img = match read.as_ref() {
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

    async fn fetch_dog_image(breed: impl Display) -> dioxus::Result<String> {
        let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?
            .json::<DogApi>()
            .await?;
        dioxus::Ok(response.message)
    }

    #[component]
    fn DogView() -> Element {
        // ANCHOR: no_waterfall_effect
        let poodle_img = use_resource(|| fetch_dog_image("poodle"));
        let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));
        let pug_img = use_resource(|| fetch_dog_image("pug"));

        let read = poodle_img.read();
        let poodle_img = match read.as_ref() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };
        let read = golden_retriever_img.read();
        let golden_retriever_img = match read.as_ref() {
            Some(Ok(src)) => src,
            _ => {
                return rsx! {
                    p { "Loading or error..." }
                };
            }
        };
        let read = pug_img.read();
        let pug_img = match read.as_ref() {
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
