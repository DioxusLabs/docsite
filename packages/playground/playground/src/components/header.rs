use crate::build::BuildState;
use crate::components::icons::LoadingSpinner;
use crate::share_code::copy_share_link;
use crate::{Errors, PlaygroundUrls};
use dioxus::prelude::*;
// use dioxus_sdk::utils::timing::use_debounce;
use model::api::ApiClient;
use model::Project;
use std::time::Duration;

#[component]
pub fn Header(
    urls: PlaygroundUrls,
    on_rebuild: EventHandler,
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
    mut show_examples: Signal<bool>,
    file_name: ReadOnlySignal<String>,
) -> Element {
    let build = use_context::<BuildState>();
    let api_client = use_context::<Signal<ApiClient>>();
    let project = use_context::<Signal<Project>>();
    let mut errors = use_context::<Errors>();

    let mut share_btn_text = use_signal(|| "Share");
    // let mut reset_share_btn = use_debounce(Duration::from_secs(1), move |()| {
    //     share_btn_text.set("Share")
    // });
    // reset_share_btn.action(());

    rsx! {
        div { id: "dxp-header",
            // Left pane header
            div {
                id: "dxp-header-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },

                // Examples button/menu
                button {
                    id: "dxp-menu-btn",
                    class: "dxp-ctrl-btn",
                    class: if show_examples() { "dxp-open" },
                    onclick: move |_| show_examples.toggle(),
                    crate::components::icons::MenuIcon {}
                }
                button { class: "dxp-ctrl-btn dxp-file-btn dxp-selected-file", {file_name} }
            }

            // Right pane header
            div {
                id: "dxp-header-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" } else { "".to_string() },

                // Share button
                button {
                    id: "dxp-share-btn",
                    class: "dxp-ctrl-btn",
                    onclick: move |_| async move {
                        share_btn_text.set("Sharing...");
                        match copy_share_link(&api_client(), project, urls.location).await {
                            Ok(()) => share_btn_text.set("Link Copied!"),
                            Err(error) => {
                                share_btn_text.set("Error!");
                                errors
                                    .push_error((
                                        "Share Failed",
                                        format!(
                                            "An error occured while generating a share link: {error:?}",
                                        ),
                                    ));
                            }
                        }
                    },
                    "{share_btn_text}"
                }


                // Run button
                button {
                    id: "dxp-run-btn",
                    class: "dxp-ctrl-btn",
                    class: if build.stage().is_running() { "disabled" },
                    onclick: move |_| {
                        on_rebuild.call(());
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
                        "Rebuild"
                    }

                }
            }
        }
    }
}
