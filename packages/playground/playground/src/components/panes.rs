use crate::build::{BuildStage, BuildState};
use dioxus::prelude::*;
use dioxus_document::eval;
// use dioxus_sdk::utils::{timing::use_debounce, window::use_window_size};

use super::Logs;

/// Stores data required for draggable pane resizing to work.
#[derive(Default)]
struct DraggableData {
    /// The client X coordinate of the mouse.
    client_x: i32,

    /// The width of the first (editor) pane.
    first_width: i32,

    /// The width of the second (output) pane.
    second_width: i32,
}

/// Renders the panes that contain the editor and the output.
///
/// The pane widths are passed down as the header requires the widths to resize with the panes.
#[component]
pub fn Panes(
    pane_left_width: Signal<Option<i32>>,
    pane_right_width: Signal<Option<i32>>,
    built_page_url: Memo<Option<String>>,
) -> Element {
    let build = use_context::<BuildState>();
    let mut dragging = use_signal(|| false);
    let mut mouse_data = use_signal(DraggableData::default);

    // // Reset the panes slider on window resize.
    // // TODO: This is annoying for the user, it should instead just recalculate the size from previous data.
    // let window_size = use_window_size();
    // let mut reset_panes_debounce = use_debounce(std::time::Duration::from_millis(200), move |_| {
    //     spawn(async move {
    //         pane_left_width.set(None);
    //         pane_right_width.set(None);
    //     });
    // });

    // use_effect(move || {
    //     window_size();
    //     reset_panes_debounce.action(());
    // });

    // Handle retrieving required data from dom elements and enabling drag.
    let draggable_mousedown = move |e: Event<MouseData>| async move {
        dragging.set(true);

        let mut data = eval(
            r#"
            let leftPane = document.getElementById("dxp-panes-left");
            let rightPane = document.getElementById("dxp-panes-right");

            dioxus.send(leftPane.offsetWidth);
            dioxus.send(rightPane.offsetWidth);
        "#,
        );

        let first_width: i32 = data.recv().await.unwrap();
        let second_width: i32 = data.recv().await.unwrap();
        let coords = e.client_coordinates();

        mouse_data.set(DraggableData {
            client_x: coords.x as i32,
            first_width,
            second_width,
        });
    };

    // Stops dragging when the mouse is released.
    let stop_dragging = move |_| {
        dragging.set(false);
    };

    // Handle resizing panes when mouse moves and mouse is down on the draggable.
    let panes_mousemove = move |e: Event<MouseData>| {
        if !dragging() {
            return;
        }

        let prev_mouse_data = mouse_data.read();
        let coords = e.client_coordinates();

        let mut delta_x = coords.x as i32 - prev_mouse_data.client_x;

        let max = std::cmp::max(delta_x, -prev_mouse_data.first_width);
        delta_x = std::cmp::min(max, prev_mouse_data.second_width);

        pane_left_width.set(Some(prev_mouse_data.first_width + delta_x));
        pane_right_width.set(Some(prev_mouse_data.second_width - delta_x));
    };

    let build_stage = build.stage();

    rsx! {
        // Panes
        div {
            id: "dxp-panes",
            onmousemove: panes_mousemove,
            onmouseup: stop_dragging,
            onmouseleave: stop_dragging,

            // Left Pane
            div {
                id: "dxp-panes-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },
            }
            // Draggable
            div {
                id: "dxp-panes-draggable",
                onmousedown: draggable_mousedown,
                onmouseup: stop_dragging,
            }
            // Right Pane
            div {
                id: "dxp-panes-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" },

                if build_stage.is_running() {
                    Progress {}
                } else {
                    // Viewport
                    if let Some(url) = built_page_url() {
                        div { id: "dxp-viewport",
                            iframe {
                                id: "dxp-iframe",
                                src: "{url}",
                                pointer_events: if dragging() { "none" } else { "all" },
                            }
                        }
                    } else if build_stage.is_err() {
                        Logs {}
                    } else {
                        p { "Click `Rebuild` to start a build!" }
                    }
                }
            }
        }
    }
}

#[component]
fn Progress() -> Element {
    let build = use_context::<BuildState>();

    // Generate the loading message.
    let message = use_memo(move || {
        let compiling = build.stage().get_compiling_stage();
        if let Some((crates_compiled, total_crates, current_crate)) = compiling {
            return format!("[{crates_compiled}/{total_crates}] Compiling {current_crate}");
        }

        match build.stage() {
            BuildStage::NotStarted => "Build has not started.",
            BuildStage::Starting => "Starting build...",
            BuildStage::Building(build_stage) => match build_stage {
                model::BuildStage::RunningBindgen => "Running wasm-bindgen...",
                model::BuildStage::Other => "Computing...",
                model::BuildStage::Compiling { .. } => unreachable!(),
            },
            BuildStage::Finished(_) => "Finished!",
        }
        .to_string()
    });

    // Determine the progress width.
    let progress_width = use_memo(move || {
        let stage = build.stage();
        let compiling = stage.get_compiling_stage();
        if let Some((crates_compiled, total_crates, _)) = compiling {
            return (crates_compiled as f64 / total_crates as f64) * 100.0;
        }

        match stage.is_running() {
            true => 50.0,
            false => 0.0,
        }
    });

    rsx! {
        div { id: "dxp-progress-container",
            p { "{message}" }
            div { id: "dxp-progress",
                div { id: "dxp-bar", width: "{progress_width}%" }
            }
        }
    }
}
