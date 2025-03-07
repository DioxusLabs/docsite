use crate::desktop_dependencies::*;
use crate::doc_examples::*;
use dioxus::prelude::*;
use std::hash::Hash;

pub mod router_03;
pub mod router_04;
pub mod router_05;
pub mod router_06;
pub mod router_blog;

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
fn CodeBlock(contents: String, name: Option<String>) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        div { class: "border overflow-hidden rounded-md border-gray-300 dark:border-gray-700 mb-8",
            div { class: "w-full bg-red flex flex-row justify-between border-b border-gray-300 dark:border-gray-700 py-1 px-2 text-xs items-center bg-gray-100 dark:bg-ideblack",
                div { class: "font-mono",
                    if let Some(path) = name {
                        "src/{path}"
                    }
                }
                button {
                    class: "hover:text-blue-600 flex flex-row items-center gap-1",
                    class: if copied() { "text-green-600" },
                    "onclick": "navigator.clipboard.writeText(this.parentNode.parentNode.lastChild.innerText);",
                    onclick: move |_| copied.set(true),
                    if copied() {
                        "Copied!"
                    }
                    span { Copy {} }
                }
            }
            div { class: "codeblock", dangerous_inner_html: contents }
        }
    }
}

#[component]
fn MermaidBlock(chart: &'static str) -> Element {
    rsx! {
        div {
            document::Link { rel: "stylesheet", href: asset!("assets/mermaid_block.css") }
            div {
                class: "mermaid min-h-[60vh]",
                style: "background-color: #fff; ",
                "value": "{chart}",
            }
            script { r#type: "module",
                r#"
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/+esm';
import Panzoom from 'https://cdn.jsdelivr.net/npm/@panzoom/panzoom@4.6.0/+esm';

mermaid.initialize({{
    startOnLoad: false,
}});

const mermaidElements = document.querySelectorAll('.mermaid');
let elements = [];
mermaidElements.forEach((element, index) => {{
    if (element.getAttribute('data-processed') === 'true') {{
        return;
    }}
    element.textContent = element.value;
    elements.push(element);
}});

mermaid.run().then(() => {{
    elements.forEach((element, index) => {{
        let svg = element.firstElementChild;
        const panzoom = Panzoom(svg, {{
            step: 1,
            maxScale: 10,
            minScale: 0.5,
        }});
        element.addEventListener('wheel', panzoom.zoomWithWheel);
    }})
}});"#
            }
        }
    }
}

pub(crate) static Copy: Component<()> = |_| {
    rsx!(
        svg {
            width: "24",
            height: "24",
            stroke_width: "1.5",
            fill: "none",
            stroke: "currentColor",
            path { d: "M8 16c0 1.886 0 2.828.586 3.414C9.172 20 10.114 20 12 20h4c1.886 0 2.828 0 3.414-.586C20 18.828 20 17.886 20 16v-4c0-1.886 0-2.828-.586-3.414C18.828 8 17.886 8 16 8m-8 8h4c1.886 0 2.828 0 3.414-.586C16 14.828 16 13.886 16 12V8m-8 8c-1.886 0-2.828 0-3.414-.586C4 14.828 4 13.886 4 12V8c0-1.886 0-2.828.586-3.414C5.172 4 6.114 4 8 4h4c1.886 0 2.828 0 3.414.586C16 5.172 16 6.114 16 8" }
        }
    )
};
