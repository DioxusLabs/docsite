use crate::docs::LAZY_BOOK;
use crate::*;
use mdbook_shared::SummaryItem;

pub(crate) static SHOW_SIDEBAR: GlobalSignal<bool> = Signal::global(|| false);

/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/DioxusLabs/docsite/contents/docs-src/0.5/en";

/// Use this URL while loading the file-specific URL.
const GITHUB_EDIT_PAGE_FALLBACK_URL: &str = "https://github.com/DioxusLabs/docsite";

/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_EDIT_PAGE_EDIT_URL: &str =
    "https://github.com/DioxusLabs/docsite/edit/main/docs-src/0.5/en";

#[component]
pub(crate) fn Learn() -> Element {
    rsx! {
        div { class: "w-full text-sm dark:bg-ideblack border-b dark:border-[#a4a9ac7d]",
            div { class: "flex flex-row justify-center dark:text-[#dee2e6] font-light lg:gap-12",
                LeftNav {}
                Content {}
                RightNav {}
            }
        }
    }
}

fn LeftNav() -> Element {
    let chapters = vec![
        &LAZY_BOOK.summary.prefix_chapters,
        &LAZY_BOOK.summary.numbered_chapters,
        &LAZY_BOOK.summary.suffix_chapters,
    ];

    rsx! {

        div {
            class: if SHOW_SIDEBAR() { "w-full md:w-auto" } else { "hidden" },
            class: "h-full md:block top-28 sticky",
            // class: "absolute md:block md:sticky md:top-28 overflow-y-hidden bg-white dark:bg-ideblack lg:bg-inherit z-20",
            div { class: "
                lg:block mb-2 md:text-[14px] leading-5 text-gray-700  dark:text-gray-400 space-y-2 px-4 md:px-2 py-2 md:py-0
                ",
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
    let versions = [
        ("Version 0.6 (alpha)", "v0.6.0-alpha.3", "0.6"),
        ("Version 0.5 (stable)", "v0.5.4", "0.5"),
        ("Version 0.4", "v0.4.3", "0.4"),
        ("Version 0.3", "v0.3.0", "0.3"),
    ];
    let mut show_versions = use_signal(|| false);

    rsx! {
        div {
            tabindex: "0",
            cursor: "pointer",
            role: "button",
            onfocusout: move |_| show_versions.set(false),
            onclick: move |_| show_versions.set(true),
            div { class: " hover:bg-gray-100 dark:hover:bg-ghdarkmetal rounded w-full py-1",
                div { class: "grid grid-cols-[auto,1fr,auto] items-center gap-2 px-1",
                    div {
                        div { class: "w-8 h-8 rounded-md border flex items-center justify-center bg-gray-50 border-gray-200 text-gray-900 dark:invert",
                            svg {
                                style: "width: 16px; height: 16px; color: currentcolor;",
                                "stroke-linejoin": "round",
                                "viewBox": "0 0 16 16",
                                width: "16",
                                height: "16",
                                class: "translate-x-px translate-y-px",
                                path {
                                    "fill-rule": "evenodd",
                                    fill: "currentColor",
                                    d: "M1.5 1.5H6.34315C7.00619 1.5 7.64207 1.76339 8.11091 2.23223L13.8787 8L8 13.8787L2.23223 8.11091C1.76339 7.64207 1.5 7.00619 1.5 6.34315V1.5ZM16 8L14.9393 6.93934L9.17157 1.17157C8.42143 0.421427 7.40401 0 6.34315 0H1.5H0V1.5V6.34315C0 7.40401 0.421426 8.42143 1.17157 9.17157L6.93934 14.9393L8 16L9.06066 14.9393L14.9393 9.06066L16 8ZM4.5 5.25C4.91421 5.25 5.25 4.91421 5.25 4.5C5.25 4.08579 4.91421 3.75 4.5 3.75C4.08579 3.75 3.75 4.08579 3.75 4.5C3.75 4.91421 4.08579 5.25 4.5 5.25Z",
                                    "clip-rule": "evenodd",
                                }
                            }
                        }
                    }
                    div { class: "leading-snug text-xs text-left",
                        p { class: "font-medium text-gray-700 dark:text-gray-100",
                            "Using Nightly Version"
                        }
                        p { class: "font-light", "v0.6.0-alpha.3" }
                    }
                    div {
                        svg {
                            "stroke-linecap": "round",
                            width: "24",
                            "shape-rendering": "geometricPrecision",
                            height: "24",
                            "data-testid": "geist-icon",
                            stroke: "currentColor",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            style: "color: currentcolor; width: 20px; height: 20px;",
                            fill: "none",
                            "aria-hidden": "true",
                            "stroke-linejoin": "round",
                            class: "with-icon_icon__MHUeb",
                            path { d: "M17 8.517L12 3 7 8.517M7 15.48l5 5.517 5-5.517" }
                        }
                    }
                }
            }
            // relative then absolute to make sure the width ends up correct
            div {
                class: "relative w-full z-50",
                class: if !show_versions() { "hidden" },
                ul { class: "absolute flex flex-col bg-white dark:bg-ghdarkmetal text-left rounded-lg border dark:border-gray-700 mt-2 w-full overflow-hidden text-gray-500 dark:text-gray-100 text-xs shadow-lg",
                    for (name , version , at) in versions.iter() {
                        li {
                            Link { to: "https://dioxuslabs.com/learn/{at}/",
                                div { class: "flex flex-row items-center hover:bg-gray-100 dark:hover:bg-ghdarkmetal py-2 gap-2 px-2",
                                    span { class: "row-span-3", "⭐️" }
                                    div { class: "flex flex-col",
                                        span { class: "text-gray-700 dark:text-gray-100 font-semibold",
                                            "{name}"
                                        }
                                        span { class: "row-span-2 col-span-2", "{version}" }
                                    }
                                    div { class: "flex-1" }
                                    span { "✅" }
                                }
                            }
                        }
                    }
                }
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
fn SidebarSection(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;

    rsx! {
        div { class: "full-chapter",
            if let Some(url) = &link.location {
                Link {
                    onclick: move |_| *SHOW_SIDEBAR.write() = false,
                    to: Route::Docs { child: *url },
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
fn SidebarChapter(chapter: &'static SummaryItem<BookRoute>) -> Element {
    // current route of the browser, trimmed to the book url
    let mut list_toggle = use_signal(|| false);
    let book_url = use_book().to_string();
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
                to: Route::Docs { child: *url },
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
fn RightNav() -> Element {
    let page = use_book();

    let page_url = use_memo(move || page.to_string());

    let edit_github_url = use_resource(move || async move {
        // This is the URL for the file if that file is not a directory that uses /index.md
        // page_url starts with '/', so we don't need to worry about that
        let github_api_url = format!("{GITHUB_API_URL}{page_url}.md");

        // If the file is not found, that means that we have to use /index.md
        if reqwest::get(github_api_url).await.ok().map(|f| f.status())
            == Some(reqwest::StatusCode::NOT_FOUND)
        {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}/index.md")
        } else {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}.md")
        }
    });

    // That might be a naive approach, but it's the easiest
    rsx! {
        div { class: "overflow-y-auto hidden xl:block top-28 ml-12 h-full md:text-[14px] leading-5 text-gray-600  w-48 sticky  dark:text-gray-400 pt-1",
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

fn Content() -> Element {
    rsx! {
        section {
            class: "text-gray-600 dark:text-gray-300 body-font overflow-hidden dark:bg-ideblack container pb-12 max-w-screen-sm px-4 pt-4 md:pt-[3.125rem] grow min-h-[100vh] ",
            class: if SHOW_SIDEBAR() { "hidden md:block" },
            div { class: "",
                Breadcrumbs {}
                div { class: "flex w-full flex-wrap list-none",
                    article { class: "markdown-body", Outlet::<Route> {} }
                }
                NextPrev {}
            }
        }
    }
}

fn Breadcrumbs() -> Element {
    let route: BookRoute = use_book();
    let is_index = route == BookRoute::Index {};

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
            Link {
                to: Route::Docs {
                    child: BookRoute::Index {},
                },
                "Dioxus v0.6.0-alpha.3"
            }
            for (idx , route) in routes.iter().rev().enumerate() {
                icons::ChevronRightIcon {}
                Link {
                    to: Route::Docs {
                        child: route.clone(),
                    },
                    class: if idx == routes.len() - 1 { "font-semibold" },
                    "{route.page().title}"
                }
            }
        }
    }
}

fn NextPrev() -> Element {
    let route: BookRoute = use_book();

    let id = route.page_id();
    let prev_id = id.0.saturating_sub(1);
    let next_id = id.0.saturating_add(1);
    let prev_page = LAZY_BOOK.pages.get(prev_id);
    let next_page = LAZY_BOOK.pages.get(next_id);

    rsx! {
        div { class: "flex flex-row w-full pt-8",
            if let Some(prev_page) = prev_page {
                Link {
                    class: "text-gray-700 dark:text-gray-100 p-4 rounded text-left flex-1 ",
                    to: Route::Docs {
                        child: prev_page.url,
                    },
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
                    to: Route::Docs {
                        child: next_page.url,
                    },
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

/// Get the book URL from the current URL
/// Ignores language and version (for now)
fn use_book() -> BookRoute {
    let route = use_route();
    match route {
        Route::Docs { child } => child,
        _ => unreachable!(),
    }
}

#[component]
pub fn DocsO3(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    navigator.push(route);
    rsx!()
}

#[component]
pub fn DocsO4(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    navigator.push(route);
    rsx!()
}
