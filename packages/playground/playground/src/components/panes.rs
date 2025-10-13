use crate::build::BuildState;
use dioxus::prelude::*;
use dioxus_document::eval;

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
                width: if let Some(val) = pane_left_width() { "{val}px;" },
            }
            // Draggable
            div {
                id: "dxp-panes-draggable",
                onmousedown: draggable_mousedown,
                onmouseup: stop_dragging,
                // Two vertical lines to indicate draggable
                svg {
                    width: "12",
                    height: "48",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 34 48",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "6",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M10 8v48" }
                    path { d: "M24 8v48" }
                }
            }
            // Right Pane
            div {
                id: "dxp-panes-right",
                width: if let Some(val) = pane_right_width() { "{val}px;" },

                // Viewport
                div { id: "dxp-viewport",
                    if build_stage.is_err() {
                        Logs {}
                    } else if let Some(url) = built_page_url() {
                        iframe {
                            id: "dxp-iframe",
                            src: "{url}",
                            pointer_events: if dragging() { "none" } else { "all" },
                        }
                    } else {
                        p { "Click `Rebuild` to start a build!" }
                    }
                }
            }
        }
    }
}
