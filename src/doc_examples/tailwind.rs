use dioxus::prelude::*;

#[component]
fn app() -> Element {
    rsx! {
        // The style component inserts a style link into the head of the document
        document::Style {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css")
        }
    }
}
