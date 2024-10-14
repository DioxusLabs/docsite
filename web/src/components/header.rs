use std::time::Duration;

use crate::{bindings::monaco, examples, PlaygroundUrls};
use base64::{prelude::BASE64_URL_SAFE, Engine};
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;

const ARROW_DOWN: &str = asset!("/public/arrow-down.svg");

#[component]
pub fn Header(
    urls: PlaygroundUrls,
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
    on_run: EventHandler,
) -> Element {
    let mut examples_open = use_signal(|| false);
    let mut show_share_copied = use_signal(|| false);

    let mut reset_share_copied = use_debounce(Duration::from_secs(1), move |()| {
        show_share_copied.set(false);
    });

    rsx! {
        div {
            id: "dxp-header",
            // Left pane header
            div {
                id: "dxp-header-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },

                // Run button
                button {
                    id: "dxp-run-btn",
                    class: "dxp-ctrl-btn",
                    onclick: move |_| on_run.call(()),
                    "Run"
                }

                // Examples button/menu
                div {
                    id: "dxp-examples-btn-container",
                    button {
                        id: "dxp-examples-btn",
                        class: "dxp-ctrl-btn",
                        class: if examples_open() { "dxp-open" },
                        onclick: move |_| examples_open.set(!examples_open()),
                        "Examples"
                        img { src: ARROW_DOWN, height: "16px", width: "16px" }
                    }

                    if examples_open() {
                        div {
                            id: "dxp-examples-dropdown",

                            for snippet in examples::SNIPPETS {
                                button {
                                    onclick: move |_| {
                                        examples_open.set(false);
                                        monaco::set_current_model_value(snippet.1);
                                    },
                                    "{snippet.0}"
                                }
                            }
                        }
                    }
                }
                div {
                    id: "dxp-header-left-divider",
                }
                button {
                    class: "dxp-ctrl-btn dxp-file-btn dxp-selected-file",
                    "main.rs"
                }
                // Keeping this for future-multi-file
                // button {
                //     class: "dxp-ctrl-btn dxp-file-btn",
                //     "Cargo.toml"
                // }
            }

            // Right pane header
            div {
                id: "dxp-header-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" } else { "".to_string() },

                // Share button
                button {
                    id: "dxp-share-btn",
                    class: "dxp-ctrl-btn",
                    onclick: move |_| {
                        share_code(urls.location);
                        show_share_copied.set(true);
                        reset_share_copied.action(());
                    },
                    if show_share_copied() { "Copied" } else { "Share" }
                }
            }
        }
    }
}

/// Copy a share link to the clipboard.
/// 
/// This will:
/// 1. Get the current code from the editor.
/// 2. Compress it using `miniz_oxide`.
/// 3. Encodes it in url-safe base64.
/// 4. Formats the code with the provided `location` url prefix.
/// 5. Copies the link to the clipboard.
/// 
/// This allows users to have primitve serverless sharing. 
/// Links will be large and ugly but it works.
fn share_code(location: &str) {
    let code = monaco::get_current_model_value();
    let compressed = miniz_oxide::deflate::compress_to_vec(code.as_bytes(), 10);

    let mut encoded = String::new();
    BASE64_URL_SAFE.encode_string(compressed, &mut encoded);

    let formatted = format!("{}/{}", location, encoded);

    let e = eval(
        r#"
        const data = await dioxus.recv();
        navigator.clipboard.writeText(data);
        "#,
    );

    let _ = e.send(formatted.into());
}