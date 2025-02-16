use crate::build::BuildState;
use crate::components::icons::LoadingSpinner;
use crate::share_code::copy_share_link;
use crate::PlaygroundUrls;
use dioxus::prelude::*;
use dioxus_sdk::utils::timing::use_debounce;
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

    let mut show_share_copied = use_signal(|| false);

    let mut reset_share_copied = use_debounce(Duration::from_secs(1), move |()| {
        spawn(async move {
            show_share_copied.set(false);
        });
    });

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
                    onclick: move |_| {
                        copy_share_link(urls.location);
                        show_share_copied.set(true);
                        reset_share_copied.action(());
                    },
                    if show_share_copied() {
                        "Copied!"
                    } else {
                        "Share"
                    }
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
