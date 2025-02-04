#![allow(non_snake_case, unused)]

use super::{log, ComponentWithLogs};
use dioxus::prelude::*;
use std::collections::HashSet;

#[derive(serde::Deserialize, serde::Serialize)]
struct BreedResponse {
    message: Vec<String>,
}

impl std::ops::Deref for BreedResponse {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

pub fn SpawnButton() -> Element {
    // ANCHOR: spawn
    let mut response = use_signal(|| "Click to start a request".to_string());

    rsx! {
        button {
            onclick: move |_| {
                response.set("...".into());
                // Spawn will start a task running in the background
                spawn(async move {
                    let resp = reqwest::Client::new()
                        .get("https://dioxuslabs.com")
                        .send()
                        .await;

                    if resp.is_ok() {
                        response.set("dioxuslabs.com responded!".into());
                    } else  {
                        response.set("failed to fetch response!".into());
                    }
                });
            },
            "{response}"
        }
    }
    // ANCHOR_END: spawn
}

pub fn SpawnButtonSimplified() -> Element {
    // ANCHOR: spawn_simplified
    let mut response = use_signal(|| "Click to start a request".to_string());

    rsx! {
        button {
            // Async closures passed to event handlers are automatically spawned
            onclick: move |_| async move {
                response.set("...".into());
                let resp = reqwest::Client::new()
                    .get("https://dioxuslabs.com")
                    .send()
                    .await;

                if resp.is_ok() {
                    response.set("dioxuslabs.com responded!".into());
                } else  {
                    response.set("failed to fetch response!".into());
                }
            },
            "{response}"
        }
    }
    // ANCHOR_END: spawn_simplified
}

pub fn UseResource() -> Element {
    // ANCHOR: use_resource
    let mut breed = use_signal(|| "hound".to_string());
    let dogs = use_resource(move || async move {
        reqwest::Client::new()
            // Since breed is read inside the async closure, the resource will subscribe to the signal
            // and rerun when the breed is written to
            .get(format!("https://dog.ceo/api/breed/{breed}/images"))
            .send()
            .await?
            .json::<BreedResponse>()
            .await
    });

    rsx! {
        input {
            value: "{breed}",
            // When the input is changed and the breed is set, the resource will rerun
            oninput: move |evt| breed.set(evt.value()),
        }

        div {
            display: "flex",
            flex_direction: "row",
            // You can read resource just like a signal. If the resource is still
            // running, it will return None
            if let Some(response) = &*dogs.read() {
                match response {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            } else {
                "Loading..."
            }
        }
    }
    // ANCHOR_END: use_resource
}

pub fn NotCancelSafe() -> Element {
    // ANCHOR: not_cancel_safe
    static RESOURCES_RUNNING: GlobalSignal<HashSet<String>> = Signal::global(|| HashSet::new());
    let mut breed = use_signal(|| "hound".to_string());
    let dogs = use_resource(move || async move {
        // Modify some global state
        RESOURCES_RUNNING.write().insert(breed());

        // Wait for a future to finish. The resource may cancel
        // without warning if breed is changed while the future is running. If
        // it does, then the breed pushed to RESOURCES_RUNNING will never be popped
        let response = reqwest::Client::new()
            .get(format!("https://dog.ceo/api/breed/{breed}/images"))
            .send()
            .await?
            .json::<BreedResponse>()
            .await;

        // Restore some global state
        RESOURCES_RUNNING.write().remove(&breed());

        response
    });
    // ANCHOR_END: not_cancel_safe

    rsx! {
        h4 { "RESOURCES_RUNNING:" }
        div {
            height: "10em",
            ul {
                for resource in RESOURCES_RUNNING.read().iter() {
                    li { "{resource}" }
                }
            }
        }

        input {
            value: "{breed}",
            // When the input is changed and the breed is set, the resource will rerun
            oninput: move |evt| breed.set(evt.value()),
        }

        div {
            display: "flex",
            flex_direction: "row",
            // You can read resource just like a signal. If the resource is still
            // running, it will return None
            if let Some(response) = &*dogs.read() {
                match response {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            } else {
                "Loading..."
            }
        }
    }
}

pub fn CancelSafe() -> Element {
    // ANCHOR: cancel_safe
    static RESOURCES_RUNNING: GlobalSignal<HashSet<String>> = Signal::global(|| HashSet::new());
    let mut breed = use_signal(|| "hound".to_string());
    let dogs = use_resource(move || async move {
        // Modify some global state
        RESOURCES_RUNNING.write().insert(breed());

        // Automatically restore the global state when the future is dropped, even if
        // isn't finished
        struct DropGuard(String);
        impl Drop for DropGuard {
            fn drop(&mut self) {
                RESOURCES_RUNNING.write().remove(&self.0);
            }
        }
        let _guard = DropGuard(breed());

        // Wait for a future to finish. The resource may cancel
        // without warning if breed is changed while the future is running. If
        // it does, then it will be dropped and the breed will be popped
        reqwest::Client::new()
            .get(format!("https://dog.ceo/api/breed/{breed}/images"))
            .send()
            .await?
            .json::<BreedResponse>()
            .await
    });
    // ANCHOR_END: cancel_safe

    rsx! {
        h4 { "RESOURCES_RUNNING:" }
        div {
            height: "10em",
            ul {
                for resource in RESOURCES_RUNNING.read().iter() {
                    li { "{resource}" }
                }
            }
        }

        input {
            value: "{breed}",
            // When the input is changed and the breed is set, the resource will rerun
            oninput: move |evt| breed.set(evt.value()),
        }

        div {
            display: "flex",
            flex_direction: "row",
            // You can read resource just like a signal. If the resource is still
            // running, it will return None
            if let Some(response) = &*dogs.read() {
                match response {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            } else {
                "Loading..."
            }
        }
    }
}

pub fn UseResourceDemo() -> Element {
    rsx! {
        ComponentWithLogs {
            UseResourceMemo {}
        }
    }
}

#[component]
pub fn UseResourceMemo() -> Element {
    // ANCHOR: use_resource_memo
    let mut number = use_signal(|| 0);

    // Resources rerun any time their dependencies change. They will
    // rerun any reactive scopes that read the resource when they finish
    // even if the value hasn't changed
    let halved_resource = use_resource(move || async move { number() / 2 });

    log!("Component reran");

    rsx! {
        button {
            onclick: move |_| number += 1,
            "Increment"
        }
        p {
            if let Some(halved) = halved_resource() {
                "Halved: {halved}"
            } else {
                "Loading..."
            }
        }
    }
    // ANCHOR_END: use_resource_memo
}

pub use suspense_boundary::DogGridView;

/// Force the children to only render on the client
#[component]
fn ClientOnly(children: Element) -> Element {
    let mut on_client = use_signal(|| false);

    use_effect(move || on_client.set(true));

    if on_client() {
        children
    } else {
        rsx! {}
    }
}

mod suspense_boundary {
    use super::{BreedResponse, ClientOnly};
    use dioxus::prelude::*;

    pub fn DogGridView() -> Element {
        let mut uuid = use_signal(|| 0);
        rsx! {
            div {
                height: "20em",
                ClientOnly {
                    button { onclick: move |_| uuid += 1, "🔄" }
                    {std::iter::once(rsx! {
                        DogGrid { key: "{uuid}" }
                    })}
                }
            }
        }
    }

    // ANCHOR: suspense_boundary
    fn DogGrid() -> Element {
        rsx! {
            SuspenseBoundary {
                // When any child components (like BreedGallery) are suspended, this closure will
                // be called and the loading view will be rendered instead of the children
                fallback: |_| rsx! {
                    div {
                        width: "100%",
                        height: "100%",
                        display: "flex",
                        align_items: "center",
                        justify_content: "center",
                        "Loading..."
                    }
                },
                div {
                    display: "flex",
                    flex_direction: "column",
                    BreedGallery {
                        breed: "hound"
                    }
                    BreedGallery {
                        breed: "poodle"
                    }
                    BreedGallery {
                        breed: "beagle"
                    }
                }
            }
        }
    }

    #[component]
    fn BreedGallery(breed: ReadOnlySignal<String>) -> Element {
        let response = use_resource(move || async move {
            // Artificially slow down the request to make the loading indicator easier to seer
            gloo_timers::future::TimeoutFuture::new(1000).await;
            reqwest::Client::new()
                .get(format!("https://dog.ceo/api/breed/{breed}/images"))
                .send()
                .await?
                .json::<BreedResponse>()
                .await
        })
        // Calling .suspend()? will suspend the component and return early while the future is running
        .suspend()?;

        // Then you can just handle the happy path with the resolved future
        rsx! {
            div {
                display: "flex",
                flex_direction: "row",
                match &*response.read() {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            }
        }
    }
    // ANCHOR_END: suspense_boundary
}

pub use suspense_boundary_with_loading_placeholder::DogGridViewWithLoadingPlaceholder;

mod suspense_boundary_with_loading_placeholder {
    use super::{BreedResponse, ClientOnly};
    use dioxus::prelude::*;

    pub fn DogGridViewWithLoadingPlaceholder() -> Element {
        let mut uuid = use_signal(|| 0);
        rsx! {
            div {
                height: "20em",
                ClientOnly {
                    button { onclick: move |_| uuid += 1, "🔄" }
                    {std::iter::once(rsx! {
                        DogGrid { key: "{uuid}" }
                    })}
                }
            }
        }
    }

    // ANCHOR: suspense_boundary_with_loading_placeholder
    fn DogGrid() -> Element {
        rsx! {
            SuspenseBoundary {
                // The fallback closure accepts a SuspenseContext which contains
                // information about the suspended component
                fallback: |suspense_context: SuspenseContext| if let Some(view) = suspense_context.suspense_placeholder() {
                    view
                } else {
                    rsx! {
                        div {
                            width: "100%",
                            height: "100%",
                            display: "flex",
                            align_items: "center",
                            justify_content: "center",
                            "Loading..."
                        }
                    }
                },
                div {
                    display: "flex",
                    flex_direction: "column",
                    BreedGallery {
                        breed: "hound"
                    }
                    BreedGallery {
                        breed: "poodle"
                    }
                    BreedGallery {
                        breed: "beagle"
                    }
                }
            }
        }
    }

    #[component]
    fn BreedGallery(breed: ReadOnlySignal<String>) -> Element {
        let response = use_resource(move || async move {
            gloo_timers::future::TimeoutFuture::new(breed().len() as u32 * 100).await;
            reqwest::Client::new()
                .get(format!("https://dog.ceo/api/breed/{breed}/images"))
                .send()
                .await?
                .json::<BreedResponse>()
                .await
        })
        .suspend()
        // You can pass up a loading placeholder to the nearest SuspenseBoundary
        // with the with_loading_placeholder method
        .with_loading_placeholder(move || {
            rsx! {
                div {
                    width: "100%",
                    height: "100%",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Loading {breed}..."
                }
            }
        })?;

        // Then you can just handle the happy path with the resolved future
        rsx! {
            div {
                display: "flex",
                flex_direction: "row",
                match &*response.read() {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            }
        }
    }
    // ANCHOR_END: suspense_boundary_with_loading_placeholder
}

#[cfg(not(feature = "fullstack"))]
pub use suspense_boundary::DogGridView as DogGridFullstack;
#[cfg(feature = "fullstack")]
pub use use_server_future::DogGridFullstack;

#[cfg(feature = "fullstack")]
mod use_server_future {
    use super::{BreedResponse, ClientOnly};
    use dioxus::prelude::*;

    pub fn DogGridFullstack() -> Element {
        rsx! {
            div {
                height: "20em",
                DogGrid {}
            }
        }
    }

    fn DogGrid() -> Element {
        rsx! {
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div {
                        width: "100%",
                        height: "100%",
                        display: "flex",
                        align_items: "center",
                        justify_content: "center",
                        "Loading..."
                    }
                },
                div {
                    display: "flex",
                    flex_direction: "column",
                    BreedGallery {
                        breed: "hound"
                    }
                    BreedGallery {
                        breed: "poodle"
                    }
                    BreedGallery {
                        breed: "beagle"
                    }
                }
            }
        }
    }

    // ANCHOR: use_server_future
    #[component]
    fn BreedGallery(breed: ReadOnlySignal<String>) -> Element {
        // use_server_future is very similar to use_resource, but the value returned from the future
        // must implement Serialize and Deserialize and it is automatically suspended
        let response = use_server_future(move || async move {
            // The future will run on the server during SSR and then get sent to the client
            reqwest::Client::new()
                .get(format!("https://dog.ceo/api/breed/{breed}/images"))
                .send()
                .await
                // reqwest::Result does not implement Serialize, so we need to map it to a string which
                // can be serialized
                .map_err(|err| err.to_string())?
                .json::<BreedResponse>()
                .await
                .map_err(|err| err.to_string())
            // use_server_future calls `suspend` internally, so you don't need to call it manually, but you
            // do need to bubble up the suspense variant with `?`
        })?;

        // If the future was still pending, it would have returned suspended with the `?` above
        // we can unwrap the None case here to get the inner result
        let response_read = response.read();
        let response = response_read.as_ref().unwrap();

        // Then you can just handle the happy path with the resolved future
        rsx! {
            div {
                display: "flex",
                flex_direction: "row",
                match response {
                    Ok(urls) => rsx! {
                        for image in urls.iter().take(3) {
                            img {
                                src: "{image}",
                                width: "100px",
                                height: "100px",
                            }
                        }
                    },
                    Err(err) => rsx! { "Failed to fetch response: {err}" },
                }
            }
        }
    }
    // ANCHOR_END: use_server_future
}

#[cfg(feature = "fullstack")]
mod use_server_future_reactive {
    use dioxus::prelude::*;

    fn main() {
        // ANCHOR: use_server_future_reactive
        let id = use_signal(|| 0);
        // ❌ The future inside of use_server_future is not reactive
        use_server_future(move || {
            async move {
                // But the future is not reactive which means that the future will not subscribe to any reads here
                println!("{id}");
            }
        });
        // ✅ The closure that creates the future for use_server_future is reactive
        use_server_future(move || {
            // The closure itself is reactive which means the future will subscribe to any signals you read here
            let cloned_id = id();
            async move {
                // But the future is not reactive which means that the future will not subscribe to any reads here
                println!("{cloned_id}");
            }
        });
        // ANCHOR_END: use_server_future_reactive
    }
}

#[cfg(feature = "fullstack")]
mod use_server_future_streaming {
    use dioxus::prelude::*;

    // ANCHOR: use_server_future_streaming
    fn main() {
        dioxus::LaunchBuilder::new()
            .with_context(server_only! {
                // Enable out of order streaming during SSR
                dioxus::fullstack::ServeConfig::builder().enable_out_of_order_streaming()
            })
            .launch(DogGrid);
    }
    // ANCHOR_END: use_server_future_streaming

    fn DogGrid() -> Element {
        todo!()
    }
}
