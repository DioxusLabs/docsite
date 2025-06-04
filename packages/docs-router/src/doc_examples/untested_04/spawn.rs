#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    // ANCHOR: spawn
    let response = use_state(cx, || String::from("..."));

    let log_in = move |_| {
        cx.spawn({
            to_owned![response];

            async move {
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
            }
        });
    };

    render! {
        button {
            onclick: log_in,
            "Response: {response}",
        }
    }
    // ANCHOR_END: spawn
}

#[cfg(feature = "doc_test")]
pub fn Tokio(cx: Scope) -> Element {
    let _ = || {
        // ANCHOR: tokio
        cx.spawn(async {
            let _ = tokio::spawn(async {}).await;

            let _ = tokio::task::spawn_local(async {
                // some !Send work
            })
            .await;
        });
        // ANCHOR_END: tokio
    };

    cx.render(rsx!(()))
}

pub fn ToOwnedMacro(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    let age = use_state(cx, || 0);
    let name = use_state(cx, || 0);
    let description = use_state(cx, || 0);

    let _ = || {
        // ANCHOR: to_owned_macro
        use dioxus::hooks::to_owned;

        cx.spawn({
            to_owned![count, age, name, description];
            async move {
                // ...
            }
        });
        // ANCHOR_END: to_owned_macro
    };

    cx.render(rsx!(()))
}
