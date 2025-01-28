use crate::build::BuildState;
use crate::components::icons::{ArrowDownIcon, LoadingSpinner};
use crate::share_code::copy_share_link;
use crate::snippets::{SelectedExample, EXAMPLES};
use crate::{editor::monaco, snippets, PlaygroundUrls};
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
use std::time::Duration;

#[component]
pub fn Header(
    urls: PlaygroundUrls,
    on_run: EventHandler,
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
) -> Element {
    let build = use_context::<BuildState>();
    let mut selected_example = use_context::<Signal<SelectedExample>>();

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
                    class: if build.stage().is_running() { "disabled" },
                    onclick: move |_| {
                        examples_open.set(false);
                        on_run.call(());
                    },

                    if build.stage().is_running() {
                        LoadingSpinner {}
                        if let Some(pos) = build.queue_position() {
                            if pos == 0 {
                                "Building"
                            } else {
                                "#{pos}"
                            }
                        } else {
                            "Starting"
                        }
                    } else {
                       "Run"
                    }

                }

                // Examples button/menu
                div {
                    id: "dxp-examples-btn-container",
                    button {
                        id: "dxp-examples-btn",
                        class: "dxp-ctrl-btn",
                        class: if build.stage().is_running() { "disabled" },
                        class: if examples_open() { "dxp-open" },
                        onclick: move |_| {
                            if !build.stage().is_running() {
                                examples_open.set(!examples_open());
                            }
                        },
                        "Examples"
                        ArrowDownIcon {}
                    }

                    if examples_open() {
                        div {
                            id: "dxp-examples-dropdown",

                            for (i, snippet) in snippets::EXAMPLES.into_iter().enumerate() {
                                button {
                                    onclick: move |_| {
                                        if !build.stage().is_running() {
                                            examples_open.set(false);
                                            selected_example.set(EXAMPLES[i].2);
                                            monaco::set_current_model_value(snippet.1);
                                        }
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
                        copy_share_link(urls.location);
                        show_share_copied.set(true);
                        reset_share_copied.action(());
                    },
                    if show_share_copied() { "Copied" } else { "Share" }
                }
            }
        }
    }
}
