# Data Fetching

One of the most common asynchronous operations in applications is making network requests. This guide will cover how fetch data in dioxus, avoiding waterfalls, and using libraries to manage caching and invalidating requests.

## Dependencies

While dioxus does not provide a built-in HTTP client, you can use the popular [reqwest](https://docs.rs/reqwest/latest/reqwest/) library to make asynchronous network requests. We will be using the reqwest library throughout the examples in this page. Before we start, make sure to add the `reqwest` and `serde` libraries to your `Cargo.toml`:

```sh
cargo add reqwest --features json
cargo add serde --features derive
```

## Requests with `use_resource`

The easiest way to fetch data in dioxus is to fetch your data in a [`use_resource`](./resources.md) hook. The async closure you pass to `use_resource` will be called when the component mounts, and it will automatically rerun when the dependencies change. For example, we can fetch a dog from the Dog API like this:

```rust
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let img_src = use_resource(|| async {
        let response = reqwest::get("https://dog.ceo/api/breed/poodle/images/random")
            .await?
            .json::<DogApi>()
            .await?;
        response.message
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
```

Most requests will depend on state in your application. `use_resource` is reactive so it will automatically rerun when the dependencies change. For example, we can fetch a dog from the Dog API based on a breed selected by the user:

```rust
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogView() -> Element {
    let mut breed = use_signal(|| "poodle".to_string());

    let img_src = use_resource(move || async move {
        // The resource depends on the breed signal since it is read in the async closure
        let response = reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?
            .json::<DogApi>()
            .await?;
        response.message
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
            option { value: "poodle", "poodle" },
            option { value: "golden retriever", "golden retriever" },
        }
        {img_element}
    }
}
```

## Avoiding Waterfalls

One common issue when fetching data is the "waterfall" effect, where each request depends on the previous one. This can lead to slow loading times and a poor user experience. To avoid waterfalls, you can hoist your data loading logic to a higher level in your component tree and avoid returning early before unrelated requests.

Lets look at at an app that causes a waterfall effect:

```rust
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn fetch_dog_image(breed: impl Display) -> impl Future<Output = Result<String, reqwest::Error>> {
    async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?
            .json::<DogApi>()
            .await
            .map(|response| response.message)
    }
}

#[component]
fn DogView() -> Element {
    let poodle_img = use_resource(|| fetch_dog_image("poodle"));

    let poodle_img = match poodle_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
    };

    let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));

    let golden_retriever_img = match golden_retriever_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
    };

    let pug_img = use_resource(|| fetch_dog_image("pug"));

    let pug_img = match pug_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
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
```

In this example, we return early from the component if any of the requests are still loading or if there is an error. The request for the golden retriever and pug images will not start until the poodle image is loaded, causing a waterfall effect.

![waterfall effect](/assets/07/waterfall_effect.png)

We can avoid this issue by moving all of the early returns after the data fetching for all three images has started. This way, all requests will start at the same time which means they can execute in parallel:

```rust
use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

fn fetch_dog_image(breed: impl Display) -> impl Future<Output = Result<String, reqwest::Error>> {
    async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?
            .json::<DogApi>()
            .await
            .map(|response| response.message)
    }
}

#[component]
fn DogView() -> Element {
    let poodle_img = use_resource(|| fetch_dog_image("poodle"));
    let golden_retriever_img = use_resource(|| fetch_dog_image("golden retriever"));
    let pug_img = use_resource(|| fetch_dog_image("pug"));

    let poodle_img = match poodle_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
    };
    let golden_retriever_img = match golden_retriever_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
    };
    let pug_img = match pug_img() {
        Some(Ok(src)) => src,
        _ => return rsx! {
            p { "Loading or error..." }
        },
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
```

![no waterfall effect](/assets/07/no_waterfall_effect.png)

## Libraries for Data Fetching

Libraries like [dioxus query](https://docs.rs/dioxus-query/latest/dioxus_query/) provide more advanced features for data fetching, such as caching, invalidation, and polling. These libraries can help you manage complex data fetching scenarios and improve the performance of your application.
