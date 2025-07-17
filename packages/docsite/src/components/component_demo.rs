use crate::DARK_MODE;
use dioxus::prelude::*;

#[component]
pub(crate) fn Components() -> Element {
    let segments: ReadOnlySignal<Vec<String>> = Default::default();
    let query: ReadOnlySignal<String> = Default::default();

    fn format_segments(segments: &[String], query: &str) -> String {
        let segments = segments.join("/");
        let dark_mode = DARK_MODE()
            .map(|dark_mode| format!("dark_mode={}", dark_mode))
            .unwrap_or_default();
        format!(
            "https://dioxuslabs.github.io/components/{segments}?iframe=true&{dark_mode}&{query}"
        )
    }

    let initial_url = use_hook(|| format_segments(&segments.read(), &query.read()));

    // Handle syncing the parent route/history with the iframe's route on the client.
    // The iframe will send a message to the parent window when its route changes,
    // and the parent will update its route accordingly.
    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        use crate::Route;
        use dioxus::core::use_drop;
        use std::rc::Rc;
        use wasm_bindgen::{prelude::Closure, JsCast};

        let onmessage_callback = use_callback(|event: web_sys::MessageEvent| {
            #[derive(serde::Deserialize)]
            struct MessageData {
                route: String,
            }
            let data = event.data();
            if let Ok(deserialized) = serde_wasm_bindgen::from_value::<MessageData>(data) {
                // Update the frame's url to the new route
                let new_route = deserialized.route;
                let cleaned_route = new_route.trim_start_matches('/');
                let (without_query, query) =
                    cleaned_route.split_once('?').unwrap_or((cleaned_route, ""));
                let new_route = Route::Components {
                    // segments: without_query.split('/').map(String::from).collect(),
                    // query: query.to_string(),
                };
                let router = router();
                if new_route != router.current() {
                    router.push(new_route);
                }
            }
        });

        let onmessage_web_sys_callback = use_hook(move || {
            let onmessage = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                onmessage_callback(event);
            }) as Box<dyn FnMut(_)>);
            Rc::new(onmessage)
        });

        // Listen for route changes from the iframe to update the URL
        use_effect({
            let onmessage_web_sys_callback = onmessage_web_sys_callback.clone();
            move || {
                let window = web_sys::window().expect("No global `window` exists");

                window
                    .add_event_listener_with_callback(
                        "message",
                        &onmessage_web_sys_callback.as_ref().as_ref().unchecked_ref(),
                    )
                    .expect("Failed to add message event listener to iframe");
            }
        });

        use_drop(move || {
            let window = web_sys::window().expect("No global `window` exists");
            window
                .remove_event_listener_with_callback(
                    "message",
                    &onmessage_web_sys_callback.as_ref().as_ref().unchecked_ref(),
                )
                .expect("Failed to remove message event listener from iframe");
        });

        // After the initial load, replace the iframes location instead of changing the src to avoid
        // adding a new history entry.
        use_effect(move || {
            let url = format_segments(&segments.read(), &query.read());

            let window = web_sys::window().expect("No global `window` exists");
            let document = window.document().expect("No global `document` exists");
            let iframe = document
                .get_element_by_id("component-demo-iframe")
                .expect("No element with id 'component-demo-iframe' found")
                .dyn_into::<web_sys::HtmlIFrameElement>()
                .expect("Element with id 'component-demo-iframe' is not an iframe");

            // After the initial load, use the child iframe's history to set the route instead of the src
            if let Some(window) = iframe.content_window() {
                _ = window.location().replace(&url)
            }
        });
    }

    rsx! {
        div {
            div { class: "w-full max-w-screen-xl mx-auto flex flex-col items-center gap-4",
                iframe {
                    style: "width: 100%;",
                    height: "3500px",
                    src: initial_url,
                    id: "component-demo-iframe",
                    "allowfullscreen": true,
                }
            }
        }
    }
}
