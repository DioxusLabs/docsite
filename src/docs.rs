use crate::components::*;
use crate::doc_examples::*;
use dioxus::prelude::*;

#[component]
fn SandBoxFrame(url: String) -> Element {
    rsx! {
        iframe {
            style: "border: 1px solid rgba(0, 0, 0, 0.1);border-radius:2px;",
            width: "800",
            height: "450",
            src: "{url}?embed=1",
            "allowfullscreen": true,
        }
    }
}

#[component]
fn DemoFrame(children: Element) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-md shadow-md p-4 my-4 overflow-auto text-black dioxus-demo",
            max_height: "50vh",
            {children}
        }
    }
}

fn LayoutsExplanation() -> Element {
    rsx! {
        pre { onmouseenter: move |_| {}, onmouseleave: move |_| {},
            span {
                "#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {{\n\t"
            }
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-orange-600 rounded-md",
                "#[layout(HeaderFooter)]"
            }
            span { "\n\t\t// ... other routes\n\t\t" }
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-green-600 rounded-md",
                r##"#[layout(DocsSidebars)]"##
            }
            "\n\t\t\t"
            span {
                onmouseenter: move |_| {},
                onmouseleave: move |_| {},
                class: "border border-blue-600 rounded-md",
                r##"#[route("/learn")]"##
            }
            span { "\n\t\t\tDocs {{}},\n}}" }
        }
    }
}

#[component]
fn CodeBlock(contents: String) -> Element {
    rsx! {
        div { style: "position: relative;",
            // button {
            //     style: "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.15); color: white; border: 1px solid white; padding: 0.25em;",
            //     background_color: "red",
            //     "onclick": "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
            //     "Copy"
            // }
            div { dangerous_inner_html: contents }
        }
    }
}

mod router_06;
pub use router_06::*;
