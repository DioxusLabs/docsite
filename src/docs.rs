use crate::doc_examples::*;
use crate::{components::*, Route};
use dioxus::prelude::*;
use mdbook_shared::MdBook;
use std::hash::Hash;

pub mod router_03;
pub mod router_04;
pub mod router_05;
pub mod router_06;
pub mod router_blog;

pub enum CurrentDocsVersion {
    V06(router_06::BookRoute),
    V05(router_05::BookRoute),
    V04(router_04::BookRoute),
    V03(router_03::BookRoute),
}
pub fn use_current_docs_version() -> CurrentDocsVersion {
    let route = use_route();
    match route {
        Route::Docs06 { child } => CurrentDocsVersion::V06(child),
        Route::Docs05 { child } => CurrentDocsVersion::V05(child),
        Route::Docs04 { child } => CurrentDocsVersion::V04(child),
        Route::Docs03 { child } => CurrentDocsVersion::V03(child),
        _ => panic!("current docs version should be set"),
    }
}

pub trait AnyBookRoute: Routable + PartialEq + Hash + Eq + Clone + Copy {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section];
    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self>;
    fn global_route(&self) -> crate::Route;
    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId;
    fn book() -> &'static MdBook<Self>;
    fn use_current() -> Option<Self>;
    fn use_route() -> Self {
        Self::use_current().expect("current route to be the same as the route")
    }
    fn short_version() -> &'static str;
    fn full_version() -> &'static str;
    fn index() -> Self;
}

impl AnyBookRoute for router_03::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs03 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_03::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs03 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.3"
    }

    fn full_version() -> &'static str {
        "0.3.2"
    }

    fn index() -> Self {
        Self::Index {}
    }
}
impl AnyBookRoute for router_04::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs04 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_04::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs04 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.4"
    }

    fn full_version() -> &'static str {
        "0.4.3"
    }

    fn index() -> Self {
        Self::Index {}
    }
}
impl AnyBookRoute for router_05::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs05 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_05::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs05 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.5"
    }

    fn full_version() -> &'static str {
        "0.5.5"
    }

    fn index() -> Self {
        Self::Index {}
    }
}

impl AnyBookRoute for router_06::BookRoute {
    fn sections(&self) -> &[use_mdbook::mdbook_shared::Section] {
        self.sections()
    }

    fn page(&self) -> &use_mdbook::mdbook_shared::Page<Self> {
        self.page()
    }

    fn global_route(&self) -> crate::Route {
        crate::Route::Docs06 { child: *self }
    }

    fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        self.page_id()
    }
    fn book() -> &'static MdBook<Self> {
        &*router_06::LAZY_BOOK
    }

    fn use_current() -> Option<Self> {
        let route = use_route();
        match route {
            Route::Docs06 { child } => Some(child),
            _ => None,
        }
    }
    fn short_version() -> &'static str {
        "0.6"
    }
    fn full_version() -> &'static str {
        "0.6.0"
    }
    fn index() -> Self {
        Self::Index {}
    }
}

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
                    span { crate::icons::Copy {} }
                }
            }
            div { class: "codeblock", dangerous_inner_html: contents }
        }
    }
}
