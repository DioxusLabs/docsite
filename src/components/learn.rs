use crate::docs::LAZY_BOOK;
use crate::*;
use dioxus_material_icons::MaterialIconColor;
use mdbook_shared::SummaryItem;

pub(crate) static HIGHLIGHT_DOCS_LAYOUT: GlobalSignal<bool> = Signal::global(|| false);
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
    use_hook(|| *SHOW_DOCS_NAV.write() = true);
    use_drop(|| *SHOW_DOCS_NAV.write() = false);

    rsx! {
        div { class: "w-full text-sm dark:bg-ideblack", min_height: "100vh",

            // Flex centered, every column grows to split into 3
            div { class: "flex flex-row justify-center dark:text-[#dee2e6] font-light",
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

    // We use this to remove the spacing between "Introduction" and "Getting Started"
    // TODO: Make this depend on if the chapter has any links.
    let mut keep_bottom_spacing = false;

    rsx! {
        // Create a flex grow container, and then right-align its contents so it's squahed against the center
        div { class: "overflow-y-auto sticky docs-links pt-12 flex flex-row justify-end",
            nav {
                class: "bg-white dark:bg-ideblack lg:bg-inherit pl-6 pb-32 z-20 text-base lg:block top-28 lg:-ml-3.5 pr-2 w-[calc(100%-1rem)] md:w-60 lg:text-[14px] text-navy content-startleading-5 ",
                class: if HIGHLIGHT_DOCS_LAYOUT() { "border border-green-600 rounded-md" },
                class: if SHOW_SIDEBAR() { "min-w-full" } else { "hidden" },
                for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                    SidebarSection { chapter, keep_bottom_spacing }
                    {keep_bottom_spacing = true}
                }
            }
        }
    }
}

/// Navigate between doc versions
fn DocVersionNav() -> Element {
    rsx! {
        div { class: "pb-4",
            ul { class: "pl-2",
                li { class: "m-1 rounded-md pl-2",
                    span { class: "hover:text-sky-500 dark:hover:text-sky-400",
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_left",
                            color: MaterialIconColor::Custom("gray".to_string()),
                        }
                        "0.5"
                    }
                }
                li { class: "m-1 rounded-md pl-2",
                    a {
                        href: "/learn/0.4",
                        class: "hover:text-sky-500 dark:hover:text-sky-400",
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_left",
                            color: MaterialIconColor::Custom("gray".to_string()),
                        }
                        "0.4"
                    }
                }
                li { class: "m-1 rounded-md pl-2",
                    a {
                        href: "/learn/0.3",
                        class: "hover:text-sky-500 dark:hover:text-sky-400",
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_left",
                            color: MaterialIconColor::Custom("gray".to_string()),
                        }
                        "0.3"
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
fn SidebarSection(chapter: &'static SummaryItem<BookRoute>, keep_bottom_spacing: bool) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;

    let sections = link.nested_items.iter().map(|chapter| {
        rsx! {
            SidebarChapter { chapter }
        }
    });

    rsx! {
        div {
            class: "full-chapter",
            class: if keep_bottom_spacing { "pb-4 mb-6" },
            if let Some(url) = &link.location {
                Link {
                    onclick: move |_| *SHOW_SIDEBAR.write() = false,
                    to: Route::Docs { child: *url },
                    h3 { class: "font-semibold mb-2 hover:text-sky-500 dark:hover:text-sky-400",
                        "{link.name}"
                    }
                }
            }
            ul { class: "ml-1", {sections} }
        }
    }
}

#[component]
fn SidebarChapter(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link().context("Could not get link")?;
    let url = link.location.as_ref().unwrap();
    let mut list_toggle = use_signal(|| false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book().to_string();

    // for instance, if the current page is /docs/0.5/en/learn/overview
    // then we want to show the dropdown for /docs/0.5/en/learn
    let show_dropdown = list_toggle() || book_url.starts_with(&*url.to_string());
    let show_chevron = !link.nested_items.is_empty();

    if show_chevron {
        rsx! {
            li { class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400",
                Link {
                    onclick: move |_| *SHOW_SIDEBAR.write() = false,
                    to: Route::Docs { child: *url },
                    "{link.name}"
                }
                button {
                    onclick: move |_| list_toggle.toggle(),
                    class: "align-middle",
                    dioxus_material_icons::MaterialIcon {
                        name: "chevron_right",
                        color: MaterialIconColor::Custom("gray".to_string()),
                    }
                }
            }
            if show_dropdown {
                ul { class: "border-l border-gray-300 m-2 px-2 space-y-1",
                    for chapter in link.nested_items.iter() {
                        SidebarChapter { chapter }
                    }
                }
            }
        }
    } else {
        rsx! {
            LocationLink { chapter }
        }
    }
}

#[component]
fn LocationLink(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let book_url = use_book().to_string();

    let link = chapter.maybe_link().context("Could not get link")?;
    let url = link.location.as_ref().unwrap();

    rsx! {
        Link {
            onclick: move |_| *SHOW_SIDEBAR.write() = false,
            to: Route::Docs { child: *url },
            li {
                class: "rounded-md hover:text-sky-500 dark:hover:text-sky-400",
                class: if book_url.starts_with(&*url.to_string()) { "text-sky-500 dark:text-sky-400" },
                "{link.name}"
            }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav() -> Element {
    let page = use_book();

    let padding_map = ["", "", "pl-2", "pl-4", "pl-6", "pl-8"];
    let page_url = use_memo(move || page.to_string());

    let edit_github_url = use_resource(move || async move {
        // This is the URL for the file if that file is not a directory that uses /index.md
        // page_url starts with '/', so we don't need to worry about that
        let github_api_url = format!("{GITHUB_API_URL}{page_url}.md");
        // If the file is not found, that means that we have to use /index.md
        if reqwest::get(github_api_url).await.unwrap().status() == reqwest::StatusCode::NOT_FOUND {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}/index.md")
        } else {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}.md")
        }
    });

    // That might be a naive approach, but it's the easiest
    rsx! {
        div {
            class: "overflow-y-auto hidden xl:block top-28 ml-12 h-full md:text-[14px] leading-5 text-navy dark:text-[#dee2e6] docs-right-sidebar w-48 sticky",
            class: if HIGHLIGHT_DOCS_LAYOUT() { "border border-green-600 rounded-md" },
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul {
                for section in page.sections().iter().skip(1) {
                    li { class: "pb-2 {padding_map[section.level-1]}",
                        a {
                            class: "hover:text-sky-500 dark:hover:text-sky-400",
                            href: "?phantom={section.id}#{section.id}",
                            "{section.title}"
                        }
                    }
                }
            }
            h2 { class: "py-4 font-semibold",
                match edit_github_url.cloned() {
                    Some(url) => rsx! {
                        a { class: "hover:text-sky-500 dark:hover:text-sky-400", href: "{url}", "Edit this page!" }
                    },
                    None => rsx! {
                        a { href: "{GITHUB_EDIT_PAGE_FALLBACK_URL}", "Edit this page!" }
                    },
                }
            }
            h2 { class: "py-4 font-semibold", "Go to version" }
            DocVersionNav {}
        }
    }
}

fn Content() -> Element {
    rsx! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack container pb-12 max-w-screen-sm mx-2 lg:mx-24 pt-12 grow",
            div {
                class: "-py-8",
                class: if HIGHLIGHT_DOCS_LAYOUT() { "border border-green-600 rounded-md" },
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body ol {{ list-style: decimal; }}"
                        ".markdown-body li {{ display: list-item; }}"
                        ".markdown-body button {{ display: inline-block; background-color: rgba(209, 213, 219, 0.3); border-radius: 0.25rem; padding: 0.25rem 0.5rem; border: 1px solid; margin: 0.25rem; }}"
                        ".markdown-body .header {{ color: inherit }}"
                    }
                    article { class: "markdown-body", Outlet::<Route> {} }

                // todo: we want left-right buttons to go between pages in the docs
                // ContentFooter {}
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
pub(crate) fn DocsO3(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    navigator.push(route);
    rsx!()
}

#[component]
pub(crate) fn DocsO4(segments: Vec<String>) -> Element {
    let navigator = use_navigator();
    let route: Route = use_route();
    navigator.push(route);
    rsx!()
}
