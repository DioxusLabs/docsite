use dioxus::prelude::*;
use dioxus_sdk::utils::{timing::use_debounce, window::use_window_size};

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
) -> Element {
    let mut draggable_mouse_down = use_signal(|| false);
    let mut mouse_data = use_signal(DraggableData::default);

    // Reset the panes slider on window resize.
    // TODO: This is annoying for the user, it should instead just recalculate the size from previous data.
    let window_size = use_window_size();
    let mut reset_panes_debounce = use_debounce(std::time::Duration::from_millis(200), move |_| {
        pane_left_width.set(None);
        pane_right_width.set(None);
    });

    use_effect(move || {
        window_size();
        reset_panes_debounce.action(());
    });

    // Handle retrieving required data from dom elements and enabling drag.
    let draggable_mousedown = move |e: Event<MouseData>| async move {
        draggable_mouse_down.set(true);

        let mut data = eval(
            r#"
            let leftPane = document.getElementById("dxp-panes-left");
            let rightPane = document.getElementById("dxp-panes-right");

            dioxus.send(leftPane.offsetWidth);
            dioxus.send(rightPane.offsetWidth);
        "#,
        );

        let first_width = data.recv().await.unwrap();
        let first_width = first_width.as_i64().unwrap();

        let second_width = data.recv().await.unwrap();
        let second_width = second_width.as_i64().unwrap();

        let coords = e.client_coordinates();

        mouse_data.set(DraggableData {
            client_x: coords.x as i32,
            first_width: first_width as i32,
            second_width: second_width as i32,
        });
    };

    // Stops dragging when the mouse is released.
    let draggable_mouseup = move |_| {
        draggable_mouse_down.set(false);
    };

    // Handle resizing panes when mouse moves and mouse is down on the draggable.
    let panes_mousemove = move |e: Event<MouseData>| {
        if !draggable_mouse_down() {
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

    rsx! {
        // Panes
        div {
            id: "dxp-panes",
            onmousemove: panes_mousemove,

            // Left Pane
            div {
                id: "dxp-panes-left",
                style: if let Some(val) = pane_left_width() { "width:{val}px;" },
            }
            // Draggable
            div {
                id: "dxp-panes-draggable",
                onmousedown: draggable_mousedown,
                onmouseup: draggable_mouseup,
            }
            // Right Pane
            div {
                id: "dxp-panes-right",
                style: if let Some(val) = pane_right_width() { "width:{val}px;" },
            }
        }
    }
}
