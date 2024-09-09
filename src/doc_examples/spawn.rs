#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

pub fn App() -> Element {
    // ANCHOR: spawn
    let mut response = use_signal(|| String::from("..."));

    let log_in = move |_| {
        spawn(async move {
            let resp = reqwest::Client::new()
                .get("https://dioxuslabs.com")
                .send()
                .await;

            match resp {
                Ok(_data) => {
                    log::info!("dioxuslabs.com responded!");
                    response.set("dioxuslabs.com responded!".into());
                }
                Err(err) => {
                    log::info!("Request failed with error: {err:?}")
                }
            }
        });
    };

    rsx! {
        button { onclick: log_in, "Response: {response}" }
    }
    // ANCHOR_END: spawn
}

#[cfg(feature = "doc_test")]
pub fn Tokio() -> Element {
    let _ = || {
        // ANCHOR: tokio
        spawn(async {
            let _ = tokio::spawn(async {}).await;

            let _ = tokio::task::spawn_local(async {
                // some !Send work
            })
            .await;
        });
        // ANCHOR_END: tokio
    };

    None
}

pub fn ToOwnedMacro() -> Element {
    let count = use_signal(|| 0);
    let age = use_signal(|| 0);
    let name = use_signal(|| 0);
    let description = use_signal(|| 0);

    let _ = || {
        // ANCHOR: to_owned_macro
        use dioxus::hooks::to_owned;

        spawn(async move {
            // ...
        });
        // ANCHOR_END: to_owned_macro
    };

    rsx! {}
}
