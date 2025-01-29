#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use std::collections::HashSet;
use super::{log, ComponentWithLogs};

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
    let mut url = use_signal(|| "https://dioxuslabs.com".to_string());
    let resource = use_resource(move || async move {
        let resp = reqwest::Client::new()
            .get(url())
            .send()
            .await;

        match resp {
            Ok(_data) => format!("{url} responded!"), 
            Err(err) => format!("Request failed with error: {err:?}"),
        }
    });

    rsx! {
        input {
            value: "{url}",
            oninput: move |evt| url.set(evt.value()),
        }

        if let Some(response) = resource() {
            {response}
        } else {
            "Loading..."
        }
    }
    // ANCHOR_END: use_resource
}

pub fn NotCancelSafe() -> Element {
    // ANCHOR: not_cancel_safe
    static RESOURCES_RUNNING: GlobalSignal<HashSet<String>> = Signal::global(|| HashSet::new());
    let mut url = use_signal(|| "https://dioxuslabs.com".to_string());
    let _ = use_resource(move || async move {
        // Modify some global state
        RESOURCES_RUNNING.write().insert(url());

        // Wait for a future to finish. The resource may cancel
        // without warning if url is changed while the future is running. If
        // it does, then the url pushed to RESOURCES_RUNNING will never be popped
        _ = reqwest::Client::new()
            .get(url())
            .send()
            .await;

        // Restore some global state
        RESOURCES_RUNNING.write().remove(&url());
    });

    rsx! {
        input {
            value: "{url}",
            oninput: move |evt| url.set(evt.value()),
        }

        for url in RESOURCES_RUNNING.read().iter() {
            "{url}"
        }
    }
    // ANCHOR_END: not_cancel_safe
}

pub fn CancelSafe() -> Element {
    // ANCHOR: cancel_safe
    static RESOURCES_RUNNING: GlobalSignal<HashSet<String>> = Signal::global(|| HashSet::new());
    let mut url = use_signal(|| "https://dioxuslabs.com".to_string());
    let _ = use_resource(move || async move {
        // Modify some global state
        RESOURCES_RUNNING.write().insert(url());
        
        // Automatically restore the global state when the future is dropped, even if
        // isn't finished
        struct DropGuard(String);
        impl Drop for DropGuard {
            fn drop(&mut self) {
                RESOURCES_RUNNING.write().remove(&self.0);
            }
        }
        let _guard = DropGuard(url());

        // Wait for a future to finish. The resource may cancel
        // without warning if url is changed while the future is running. If
        // it does, then it will be dropped ant the url pushed to RESOURCES_RUNNING
        // will be popped
        _ = reqwest::Client::new()
            .get(url())
            .send()
            .await;
    });

    rsx! {
        input {
            value: "{url}",
            oninput: move |evt| url.set(evt.value()),
        }

        for url in RESOURCES_RUNNING.read().iter() {
            "{url}"
        }
    }
    // ANCHOR_END: cancel_safe
}

pub fn UseResourceDemo() -> Element {
    rsx! {
        ComponentWithLogs {
            UseResource {}
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
    let halved_resource = use_resource(move || async move {
        number() / 2
    });

    log!("Component reran");

    rsx! {
        button { onclick: move |_| number += 1, "Increment" }
        "Halved: {halved_resource:?}"
    }
    // ANCHOR_END: use_resource_memo
}