#![allow(unused)]

use dioxus::prelude::*;

#[component]
pub fn App(cx: Scope) -> Element {
    // ANCHOR: spawn
    let logged_in = use_state(cx, || false);

    let log_in = move |_| {
        cx.spawn({
            let logged_in = logged_in.to_owned();

            async move {
                let resp = reqwest::Client::new()
                    .get("http://example.com")
                    .send()
                    .await;

                match resp {
                    Ok(_data) => {
                        log::info!("Login successful!");
                        logged_in.set(true);
                    }
                    Err(_err) => {
                        log::info!(
                            "Login failed - you need a login server running on https://example.com."
                        )
                    }
                }
            }
        });
    };

    cx.render(rsx! {
        button {
            onclick: log_in,
            "Logined in? {logged_in}",
        }
    })
    // ANCHOR_END: spawn
}

#[cfg(feature = "doc_test")]
#[component]
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

#[component]
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
