#![allow(unused)]

use dioxus::prelude::*;

pub mod doc_examples;
pub use doc_examples::*;

mod desktop_dependencies;
pub use desktop_dependencies::DesktopDependencies;

#[component]
pub fn TwoPanelComponent(left: Element, right: Element) -> Element {
    rsx! {
        div { class: "w-full h-40 overflow-y-hidden flex flex-row justify-between",
            div { class: "w-1/2 h-full", {left} }
            div { class: "w-1/2 h-full text-sm", {right} }
        }
    }
}

#[derive(Default)]
pub struct LogState {
    pub logs: Vec<String>,
}

pub fn use_provide_log_state() -> Signal<LogState> {
    use_context_provider(|| Signal::new(LogState::default()))
}

pub fn log(message: impl ToString) {
    consume_context::<Signal<LogState>>()
        .write()
        .logs
        .insert(0, message.to_string());
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        log(format!($($arg)*))
    }
}

#[component]
pub fn ComponentWithLogs(children: Element) -> Element {
    let logs = use_provide_log_state();

    rsx! {
        TwoPanelComponent {
            left: children,
            right: rsx! {
                div { class: "p-2 text-center border-gray-200 dark:border-gray-800", "Logs" }
                for log in logs.read().logs.iter() {
                    div { class: "p-2 border-b border-gray-200 dark:border-gray-800", "{log}" }
                }
            },
        }
    }
}

#[component]
pub fn FakePage(children: Element) -> Element {
    let mut uuid = use_signal(|| 0);
    rsx! {
        button { onclick: move |_| uuid += 1, "🔄" }
        {std::iter::once(rsx! {
            Fragment { key: "{uuid}", {children} }
        })}
    }
}

#[component]
pub fn SandBoxFrame(url: String) -> Element {
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
pub fn DemoFrame(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "bg-white rounded-md shadow-md p-4 my-4 overflow-auto text-black dioxus-demo mx-4",
            max_height: "50vh",
            ..attributes,
            {children}
        }
    }
}

pub fn LayoutsExplanation() -> Element {
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
pub fn CodeBlock(contents: String, name: Option<String>) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        div { class: "border overflow-hidden rounded-md border-gray-300 dark:border-gray-700 mx-4 mb-4",
            div { class: "w-full bg-red flex flex-row justify-between border-b border-gray-300 dark:border-gray-700 py-1 px-2 text-sm items-center bg-gray-100 dark:bg-ideblack",
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
