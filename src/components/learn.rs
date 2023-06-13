use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_router::use_route;
use dioxus_router::Link;
use mdbook_shared::Page;
use mdbook_shared::SummaryItem;
use use_mdbook::{include_mdbook, LazyMdbook};

pub fn Learn(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full pt-12 text-sm dark:bg-ideblack", min_height: "100vh",
            // do a typical three-column flex layout with a single centered then pin the nav items on top
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto dark:text-white",
                Content {}
                LeftNav {}
                RightNav {}
            }
        }
    })
}

static DOCS: LazyMdbook = include_mdbook!("docs");

fn LeftNav(cx: Scope) -> Element {
    let chapters = vec![
        &DOCS.summary.prefix_chapters,
        &DOCS.summary.numbered_chapters,
        &DOCS.summary.suffix_chapters,
    ];

    render! {
        // Now, pin the nav to the left
        nav { class: "pl-6 z-20 text-base hidden md:block fixed top-0 pt-36 pb-16 pl-3.5 md:-ml-3.5 w-[calc(100%-1rem)] md:w-60 h-full max-h-screen md:text-[13px] text-navy content-start overflow-y-auto leading-5",
            // I like the idea of breadcrumbs, but they add a lot of visual noise, and like, who cares?
            // BreadCrumbs {}
            for chapter in chapters.into_iter().flatten() {
                SidebarSection { chapter: chapter }
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
#[inline_props]
fn SidebarSection(cx: Scope, chapter: &'static SummaryItem) -> Element {
    let link = chapter.maybe_link()?;

    let sections = link
        .nested_items
        .iter()
        .filter_map(|link| render! { SidebarChapter { link: link } });

    render! {
        div {  class: "pb-4",
            Link { to: "/docs/0.4/en/{link.location.as_ref().unwrap().to_string_lossy()}",
                h2 { class: "font-semibold", "{link.name}" }
            }
            ul { class: "pl-2", sections }
        }
    }
}

#[inline_props]
fn SidebarChapter(cx: Scope, link: &'static SummaryItem) -> Element {
    let link = link.maybe_link()?;
    let show_chevron = link.nested_items.len() > 0;
    let url = link.location.as_ref().unwrap();

    let list_toggle = use_state(cx, || false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book_url(cx);

    // for instance, if the current page is /docs/0.4/en/learn/overview
    // then we want to show the dropdown for /docs/0.4/en/learn
    let url_without_index = url.with_file_name("");
    let show_dropdown = *list_toggle.get() || book_url.starts_with(&url_without_index);

    render! {
        li { class: "pt-1",
            // h1 {
            //     "{book_url.to_string_lossy()}"
            // }
            // h1 {
            //     "{url.to_string_lossy()}"
            // }
            if show_chevron {
                rsx! {
                    button {
                        onclick: move |_| {
                            list_toggle.set(!list_toggle.get());
                        },
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_right",
                            color: "gray",
                        }
                    }
                }
            }
            Link {
                to: "/docs/0.4/en/{url.to_string_lossy()}",
                "{link.name}"
            }
            if show_chevron && show_dropdown {
                rsx! {
                    ul { class: "ml-6 border-l border-gray-300",
                        for nest in link.nested_items.iter().filter_map(|link| link.maybe_link()) {
                            Link { to: "/docs/0.4/en/{nest.location.as_ref().unwrap().to_string_lossy()}",
                                li { class: "py-1 dark:hover:bg-gray-800 rounded-md pl-2 ",
                                    "{nest.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav(cx: Scope) -> Element {
    let page = use_book_page(cx);

    render! {
        div { class: "overflow-y-auto hidden xl:block fixed top-0 pt-36 pb-16 pl-3.5 -ml-3.5 w-60 h-full md:text-[13px] leading-5 text-navy docs-right-sidebar",
            right: "calc(40vw - 40.875rem)",
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul { class: "",
                for section in &page.sections {
                    li { class: "pb-2", Link { to: "/docs/0.4/en/{section.id}", "{section.title}" } }
                }
            }
        }
    }
}

fn Content(cx: Scope) -> Element {
    let page = use_book_page(cx);

    render! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack mx-auto container pt-12 pb-12 max-w-screen-md",
            div { class: "-my-8",
                script { "Prism.highlightAll()" }
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body li {{ display: list-item; }}"
                    }
                    article { class: "markdown-body pt-1", dangerous_inner_html: format_args!("{}", page.content) }
                    script { "Prism.highlightAll()" }
                }
            }
        }
    }
}

fn BreadCrumbs(cx: Scope) -> Element {
    // parse out the route after the version and language
    let route = use_route(cx);

    render! {
        h2 { class: "font-semibold pb-4",
            // dioxus_router::Link { to: "https://google.com", class: "text-blue-600", "Learn" }
            for segment in route.url().path_segments().unwrap().skip(3).filter(|f| !f.is_empty()).map(|f| f.trim_end_matches(".md")) {
                rsx! {
                    if segment != "index" {
                        rsx! {
                            dioxus_router::Link { to: "https://google.com", class: "text-blue-600", "{segment}" }
                            " / "
                        }
                    }
                }
            }
        }
    }
}

/// Get the book URL from the current URL
/// Ignores language and version (for now)
fn use_book_url(cx: &ScopeState) -> PathBuf {
    // Get the URL segments from the URL
    let segments = use_route(cx)
        .url()
        .path_segments()
        .unwrap()
        .skip(3)
        .collect::<Vec<&str>>();

    // Join the segments back together and parse it as a PathBuf
    segments.join("/").parse().unwrap()
}

fn default_page() -> &'static Page {
    let index: PathBuf = "index.md".parse().unwrap();
    DOCS.pages.get(&index).unwrap()
}

fn use_book_page(cx: &ScopeState) -> &'static Page {
    let url = use_book_url(cx);

    DOCS.pages
        .get(&url)
        // Return the index page if the page doesn't exist instead of 404ing
        // Just a slightly better DX, I guess
        .unwrap_or_else(default_page)
}
