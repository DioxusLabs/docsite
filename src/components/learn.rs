use crate::*;
use docs::router_05;
use docs::router_06;
use docs::{use_current_docs_version, AnyBookRoute, CurrentDocsVersion};

use mdbook_shared::SummaryItem;

pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);

/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_API_URL: &str = "https://api.github.com/repos/DioxusLabs/docsite/contents/docs-src/";

/// Use this URL while loading the file-specific URL.
const GITHUB_EDIT_PAGE_FALLBACK_URL: &str = "https://github.com/DioxusLabs/docsite";

/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_EDIT_PAGE_EDIT_URL: &str = "https://github.com/DioxusLabs/docsite/edit/main/docs-src/";

#[component]
pub(crate) fn Learn() -> Element {
    let current_version = use_current_docs_version();

    rsx! {
        div { class: "w-full text-sm dark:bg-ideblack border-b dark:border-[#a4a9ac7d]",
            div { class: "flex flex-row justify-center dark:text-[#dee2e6] font-light lg:gap-12",
                match current_version {
                    CurrentDocsVersion::V06(_) => rsx! {
                        GenericDocs::<router_06::BookRoute> {}
                    },
                    CurrentDocsVersion::V05(_) => rsx! {
                        GenericDocs::<router_05::BookRoute> {}
                    },
                    _ => rsx! {  },
                }
            }
        }
    }
}

fn GenericDocs<R: AnyBookRoute>() -> Element {
    rsx! {
        LeftNav::<R> {}
        Content::<R> {}
        RightNav::<R> {}
    }
}

#[component]
fn LeftNav<R: AnyBookRoute>() -> Element {
    let book = R::book();
    let chapters = vec![
        &book.summary.prefix_chapters,
        &book.summary.numbered_chapters,
        &book.summary.suffix_chapters,
    ];

    rsx! {
        div {
            class: if SHOW_SIDEBAR() { "w-full md:w-auto" } else { "hidden" },
            class: "h-full md:block top-28 sticky",
            div { class: "lg:block mb-2 md:text-[14px] leading-5 text-gray-700  dark:text-gray-400 space-y-2 px-4 md:px-2 py-2 md:py-0",
                VersionSwitch {}
                nav { class: "
                styled-scrollbar
                pl-2 pb-2 z-20 text-base sm:block top-28 md:h-[88vh]
                md:w-60 lg:text-[14px] content-start text-gray-600 dark:text-gray-400 overflow-y-scroll pr-2 space-y-1",
                    for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                        SidebarSection { chapter }
                    }
                }
            }
        }
    }
}

fn VersionSwitch() -> Element {
    let mut show_versions = use_signal(|| false);
    let current_version = use_current_docs_version();
    let current_stability = match current_version {
        CurrentDocsVersion::V06(_) => "Alpha",
        CurrentDocsVersion::V05(_) => "Stable",
        CurrentDocsVersion::V04 => "stable",
        CurrentDocsVersion::V03 => "stable",
    };
    let current_version_long = match current_version {
        CurrentDocsVersion::V06(_) => "v0.6.0-alpha.3",
        CurrentDocsVersion::V05(_) => "v0.5.6",
        CurrentDocsVersion::V04 => "v0.4.3",
        CurrentDocsVersion::V03 => "v0.3.2",
    };

    rsx! {
        div {
            tabindex: "0",
            cursor: "pointer",
            role: "button",
            onfocusout: move |_| show_versions.set(false),
            onclick: move |_| show_versions.set(true),
            div { class: "hover:bg-gray-100 dark:hover:bg-ghdarkmetal rounded w-full py-1",
                div { class: "grid grid-cols-[auto,1fr,auto] items-center gap-2 px-1",
                    div { class: "w-8 h-8 rounded-md border flex items-center justify-center bg-gray-50 border-gray-200 text-gray-900 dark:invert",
                        icons::VersionTagIcon {}
                    }
                    div { class: "leading-snug text-xs text-left",
                        p { class: "font-medium text-gray-700 dark:text-gray-100",
                            "Using {current_stability} Version"
                        }
                        p { class: "font-light", {current_version_long} }
                    }
                    icons::DropdownChevrons {}
                }
            }
            // relative then absolute to make sure the width ends up correct
            div {
                class: "relative w-full z-50",
                class: if !show_versions() { "hidden" },
                div { class: "absolute flex flex-col bg-white dark:bg-ghdarkmetal text-left rounded-lg border dark:border-gray-700 mt-2 w-full overflow-hidden text-gray-500 dark:text-gray-100 text-xs shadow-lg",
                    TypedVersionSelectItem::<crate::docs::router_06::BookRoute> {}
                    TypedVersionSelectItem::<crate::docs::router_05::BookRoute> {}
                    UntypedVersionSelectItem {
                        route: "https://dioxuslabs.com/learn/0.4".into(),
                        full_version: "0.4.3",
                        short_version: "0.4",
                    }
                    UntypedVersionSelectItem {
                        route: "https://dioxuslabs.com/learn/0.3".into(),
                        full_version: "0.3.2",
                        short_version: "0.3",
                    }
                }
            }
        }
    }
}

#[component]
fn TypedVersionSelectItem<R: AnyBookRoute>() -> Element {
    rsx! {
        UntypedVersionSelectItem {
            route: R::index().global_route().into(),
            full_version: R::full_version(),
            short_version: R::short_version(),
        }
    }
}
#[component]
fn UntypedVersionSelectItem(
    route: IntoRoutable,
    full_version: &'static str,
    short_version: &'static str,
) -> Element {
    rsx! {
        Link { to: route, active_class: "bg-gray-100 dark:bg-gray-900",
            div { class: "flex flex-row items-center hover:bg-gray-100 dark:hover:bg-gray-900 py-2 gap-2 px-2",
                span { class: "row-span-3", " " }
                div { class: "flex flex-col",
                    span { class: "text-gray-700 dark:text-gray-100 font-semibold",
                        "Version {short_version}"
                    }
                    span { class: "row-span-2 col-span-2", "{full_version}" }
                }
                div { class: "flex-1" }
                span {}
            }
        }
    }
}

/// Render a single section of the sidebar
///
/// This is a recursive function that will render the section and all of its nested sections
///
/// The typical nesting for books is
/// - sections
/// - chapters
/// - page
///
/// This renders a single section
#[component]
fn SidebarSection<R: AnyBookRoute>(chapter: &'static SummaryItem<R>) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;

    rsx! {
        div { class: "full-chapter",
            if let Some(url) = &link.location {
                Link {
                    onclick: move |_| *SHOW_SIDEBAR.write() = false,
                    to: url.global_route(),
                    class: "font-semibold hover:text-sky-500 dark:hover:text-sky-400 dark:text-gray-100 text-gray-700",
                    active_class: "text-sky-600 dark:text-sky-400",
                    h3 { class: "pb-2", "{link.name}" }
                }
            }
            ul { class: "ml-1 space-y-1",
                for chapter in link.nested_items.iter() {
                    SidebarChapter { chapter }
                }
            }
        }
    }
}

#[component]
fn SidebarChapter<R: AnyBookRoute>(chapter: &'static SummaryItem<R>) -> Element {
    // current route of the browser, trimmed to the book url
    let mut list_toggle = use_signal(|| false);
    let book_url = R::use_route().to_string();
    let link = chapter.maybe_link().context("Could not get link")?;

    // for instance, if the current page is /docs/0.5/en/learn/overview
    // then we want to show the dropdown for /docs/0.5/en/learn
    // but the toggle should still work if the url is exactly the same
    let url = link.location.as_ref().unwrap();
    let show_dropdown = list_toggle() || {
        book_url.starts_with(&*url.to_string()) && !book_url.ends_with(&*url.to_string())
    };
    let show_chevron = !link.nested_items.is_empty();

    rsx! {
        li { class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400 flex flex-row",
            Link {
                onclick: move |_| {
                    list_toggle.toggle();
                    *SHOW_SIDEBAR.write() = false;
                },
                to: url.global_route(),
                class: "flex-grow flex flex-row items-center",
                active_class: "text-sky-600 dark:text-sky-400",
                "{link.name}"
                if show_chevron {
                    div { class: "flex-grow" }
                    if list_toggle() {
                        icons::ChevronDownIcon {}
                    } else {
                        icons::ChevronRightIconSmall {}
                    }
                }
            }
        }
        if !link.nested_items.is_empty() {
            ul {
                class: "border-l border-gray-300 px-4 ml-2 space-y-1 py-2 transition-[transform] duration-500 ease-in-out ",
                class: if show_dropdown { "block transform translate-y-0" } else { "hidden transform translate-y-12" },
                for chapter in link.nested_items.iter() {
                    SidebarChapter { chapter }
                }
            }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav<R: AnyBookRoute>() -> Element {
    let page = R::use_route();
    let short_version = R::short_version();

    let page_url = use_memo(move || page.to_string());

    let edit_github_url = use_resource(move || async move {
        // This is the URL for the file if that file is not a directory that uses /index.md
        // page_url starts with '/', so we don't need to worry about that
        let github_api_url = format!("{GITHUB_API_URL}{short_version}/en/{page_url}.md");

        // If the file is not found, that means that we have to use /index.md
        if reqwest::get(github_api_url).await.ok().map(|f| f.status())
            == Some(reqwest::StatusCode::NOT_FOUND)
        {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{short_version}/en/{page_url}/index.md")
        } else {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{short_version}/en/{page_url}.md")
        }
    });

    // That might be a naive approach, but it's the easiest
    rsx! {
        div { class: "overflow-y-auto hidden xl:block top-28 px-2 h-full md:text-[14px] leading-5 text-gray-600  w-48 sticky  dark:text-gray-400 pt-1",
            div { class: "border-b pb-2 dark:border-[#a4a9ac7d]",
                h2 { class: "pb-2 font-semibold text-gray-600 dark:text-gray-100",
                    "On this page"
                }
                ul {
                    for section in page.sections().iter().skip(1) {
                        li {
                            class: "pb-2",
                            class: if section.level == 0 { "" },
                            class: if section.level == 1 { "" },
                            class: if section.level == 2 { "" },
                            class: if section.level == 3 { "pl-2" },
                            class: if section.level == 4 { "pl-4" },
                            class: if section.level == 5 { "pl-6" },
                            a {
                                class: "hover:text-sky-500 dark:hover:text-sky-400",
                                href: "#{section.id}",
                                "{section.title}"
                            }
                        }
                    }
                }
            }
            h2 { class: "py-4 ",
                if let Some(url) = edit_github_url.cloned() {
                    a {
                        class: "hover:text-sky-500 dark:hover:text-sky-400 flex flex-row items-center gap-x-1",
                        href: "{url}",
                        "Edit this page"
                        icons::ExternalLinkIcon2 {}
                    }
                } else {
                    a {
                        href: "{GITHUB_EDIT_PAGE_FALLBACK_URL}",
                        class: "flex flex-row items-center gap-x-1",
                        "Edit this page"
                        icons::ExternalLinkIcon2 {}
                    }
                }
            }
        }
    }
}

#[component]
fn Content<R: AnyBookRoute>() -> Element {
    rsx! {
        section {
            class: "text-gray-600 dark:text-gray-300 body-font overflow-hidden dark:bg-ideblack container pb-12 max-w-screen-sm px-4 pt-4 md:pt-[3.125rem] grow min-h-[100vh] ",
            class: if SHOW_SIDEBAR() { "hidden md:block" },
            div { class: "",
                Breadcrumbs::<R> {}
                VersionWarning {}
                div { class: "flex w-full flex-wrap list-none",
                    article { class: "markdown-body", Outlet::<Route> {} }
                }
                NextPrev::<R> {}
            }
        }
    }
}

fn VersionWarning() -> Element {
    let current_version = use_current_docs_version();

    match current_version {
        CurrentDocsVersion::V06(_) => rsx! {
            div { class: "flex flex-row items-center justify-start w-full bg-yellow-200 opacity-80 text-yellow-800 text-sm font-normal py-2 px-2 rounded-md mb-4 gap-2",
                crate::icons::IconWarning {}
                "You are currently viewing the docs for Dioxus 0.6.0 which is under construction."
            }
        },
        CurrentDocsVersion::V05(_) => rsx! {},
        CurrentDocsVersion::V04 => rsx! {},
        CurrentDocsVersion::V03 => rsx! {},
    }
}

fn Breadcrumbs<R: AnyBookRoute>() -> Element {
    let route = R::use_route();
    let long_version = R::full_version();
    let is_index = route.to_string() == "/";

    let mut routes = vec![route.clone()];
    let mut cur = route.clone();
    while let Some(parent) = cur.parent() {
        routes.push(parent.clone());
        cur = parent.clone();
    }

    rsx! {
        div {
            class: "flex flex-row items-center space-x-2 font-extralight pb-9",
            class: if is_index { "hidden" },
            Link { to: route.global_route(), "Dioxus {long_version}" }
            for (idx , route) in routes.iter().rev().enumerate() {
                icons::ChevronRightIcon {}
                Link {
                    to: route.global_route(),
                    class: if idx == routes.len() - 1 { "font-semibold" },
                    "{route.page().title}"
                }
            }
        }
    }
}

#[component]
fn NextPrev<R: AnyBookRoute>() -> Element {
    let book = R::book();
    let route = R::use_route();

    let id = route.page_id();
    let prev_id = id.0.saturating_sub(1);
    let next_id = id.0.saturating_add(1);
    let prev_page = book.pages.get(prev_id);
    let next_page = book.pages.get(next_id);

    rsx! {
        div { class: "flex flex-row w-full pt-8",
            if let Some(prev_page) = prev_page {
                Link {
                    class: "text-gray-700 dark:text-gray-100 p-4 rounded text-left flex-1 ",
                    to: prev_page.url.global_route(),
                    div { class: "flex flex-row items-center gap-x-2 hover:text-sky-500 dark:hover:text-sky-400",
                        svg {
                            "viewBox": "0 0 16 16",
                            width: "16",
                            style: "width: 20px; height: 20px; color: currentcolor;",
                            "data-testid": "geist-icon",
                            height: "16",
                            "stroke-linejoin": "round",
                            path {
                                d: "M10.5 14.0607L9.96966 13.5303L5.14644 8.7071C4.75592 8.31658 4.75592 7.68341 5.14644 7.29289L9.96966 2.46966L10.5 1.93933L11.5607 2.99999L11.0303 3.53032L6.56065 7.99999L11.0303 12.4697L11.5607 13L10.5 14.0607Z",
                                "clip-rule": "evenodd",
                                fill: "currentColor",
                                "fill-rule": "evenodd",
                            }
                        }
                        div { class: "flex flex-col",
                            span { class: "text-xs", "PREVIOUS" }
                            span { class: "font-semibold", "{prev_page.title}" }
                        }
                    }
                }
            }


            if let Some(next_page) = next_page {
                Link {
                    class: "text-gray-700 dark:text-gray-100 p-4 rounded text-right flex-1",
                    to: next_page.url.global_route(),
                    div { class: "flex flex-row items-center gap-x-2 justify-end hover:text-sky-500 dark:hover:text-sky-400",
                        div { class: "flex flex-col",
                            span { class: "text-xs", "NEXT" }
                            span { class: "font-semibold", "{next_page.title}" }
                        }
                        svg {
                            height: "16",
                            width: "16",
                            "stroke-linejoin": "round",
                            "viewBox": "0 0 16 16",
                            style: "width: 20px; height: 20px; color: currentcolor;",
                            "data-testid": "geist-icon",
                            path {
                                d: "M5.50001 1.93933L6.03034 2.46966L10.8536 7.29288C11.2441 7.68341 11.2441 8.31657 10.8536 8.7071L6.03034 13.5303L5.50001 14.0607L4.43935 13L4.96968 12.4697L9.43935 7.99999L4.96968 3.53032L4.43935 2.99999L5.50001 1.93933Z",
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                fill: "currentColor",
                            }
                        }
                    }
                }
            }
        }
    }
}
