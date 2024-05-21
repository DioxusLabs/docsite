use dioxus::prelude::*;

#[component]
pub fn RightPane(show_page_uri: Option<String>) -> Element {
    rsx! {
        div {
            id: "right-pane",

            if let Some(uri) = show_page_uri {
                iframe {
                    src: uri,
                }
            }
        }
    }
}