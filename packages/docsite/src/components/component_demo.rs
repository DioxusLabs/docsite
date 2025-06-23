use dioxus::prelude::*;
use crate::DARK_MODE;

#[component]
pub(crate) fn Components(segments: Vec<String>) -> Element {
    let segments = format!("/{}", segments.join("/"));
    rsx! {
        iframe {
            style: "border: 1px solid rgba(0, 0, 0, 0.1);border-radius:2px;height: calc(100vh - var(--spacing) * 16);width: 100%;",
            src: "https://dioxuslabs.github.io/components{segments}?iframe=true&dark_mode={DARK_MODE}",
            "allowfullscreen": true,
        }
    }
}
