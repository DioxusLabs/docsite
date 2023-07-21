use crate::docs::LAZY_BOOK;

use dioxus::prelude::*;
use crate::*;
use mdbook_shared::Page;
use mdbook_shared::SummaryItem;
use fermi::{use_read, Atom};

pub struct DocsLayoutHighlighted(pub bool);
pub static HIGHLIGHT_DOCS_LAYOUT: Atom<DocsLayoutHighlighted> = Atom(|_| DocsLayoutHighlighted(false));
pub struct DocsContentHighlighted(pub bool);
pub static HIGHLIGHT_DOCS_CONTENT: Atom<DocsContentHighlighted> = Atom(|_| DocsContentHighlighted(false));

#[inline_props]
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

fn LeftNav(cx: Scope) -> Element {
    let show_sidebar = use_state(cx, || false);
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_LAYOUT);
    let extra_class = if highlighted.0 { "border border-green-600 rounded-md" } else { "" };
    let hidden = if **show_sidebar { "" } else { "hidden" };
    let chapters = vec![
        &LAZY_BOOK.summary.prefix_chapters,
        &LAZY_BOOK.summary.numbered_chapters,
        &LAZY_BOOK.summary.suffix_chapters,
    ];

    render! {
        // Now, pin the nav to the left
        button {
            class: "lg:hidden w-8 h-8 mt-[58px] sm:mt-20 fixed top-0 left-0 p-1 text-lg z-[100]",
            onclick: move |_| show_sidebar.modify(|f| !f),
            "☰"
        }
        nav { class: "bg-white dark:bg-ideblack lg:bg-inherit pl-6 z-20 text-base lg:block fixed top-0 pt-36 pb-16 md:-ml-3.5 w-[calc(100%-1rem)] md:w-60 h-full max-h-screen md:text-[13px] text-navy content-start overflow-y-auto leading-5 {extra_class} {hidden}",
            // I like the idea of breadcrumbs, but they add a lot of visual noise, and like, who cares?
            // BreadCrumbs {}
            for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
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
fn SidebarSection(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link()?;

    let sections = link
        .nested_items
        .iter()
        .filter_map(|chapter| render! { SidebarChapter { chapter: chapter } });

    render! {
        div { class: "pb-4",
            if let Some(url) = &link.location {
                rsx! {
                    Link { target: Route::Docs { child: url.clone() }, h2 { class: "font-semibold", "{link.name}" } }
                }
            }
            ul { class: "pl-2", sections }
        }
    }
}

#[inline_props]
fn SidebarChapter(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();
    let list_toggle = use_state(cx, || false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book(cx).to_string();

    // for instance, if the current page is /docs/0.4/en/learn/overview
    // then we want to show the dropdown for /docs/0.4/en/learn
    let show_dropdown = *list_toggle.get() || book_url.starts_with(&*url.to_string());
    let show_chevron = link.nested_items.len() > 0;

    if link.nested_items.is_empty() {
        render! {
            LocationLink {
                chapter: chapter
            }
        }
    }
    else {
        render! {
            li {
                class: "pt-1",
                if show_chevron {
                    rsx! {
                        button { onclick: move |_| list_toggle.set(!list_toggle.get()),
                            dioxus_material_icons::MaterialIcon {
                                name: "chevron_right",
                                color: "gray",
                            }
                        }
                    }
                }
                Link {
                    target: Route::Docs { child: url.clone() },
                    "{link.name}"
                }
            }
            if show_chevron && show_dropdown {
                rsx! {
                    ul { class: "ml-6 border-l border-gray-300 py-1",
                        for chapter in link.nested_items.iter() {
                            SidebarChapter { chapter: chapter }
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn LocationLink(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let book_url = use_book(cx).to_string();

    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url.starts_with(&*url.to_string()) {
        true => "bg-gray-200 dark:bg-gray-800",
        false => "",
    };

    render! {
        Link { target: Route::Docs { child: url.clone() },
            li { class: "m-1 dark:hover:bg-gray-800 rounded-md pl-2 {current_class}", "{link.name}" }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav(cx: Scope) -> Element {
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_LAYOUT);
    let extra_class = if highlighted.0 { "border border-green-600 rounded-md" } else { "" };
    let page = use_book(cx);

    render! {
        div {
            class: "overflow-y-auto hidden xl:block fixed top-0 pt-36 pb-16 pl-3.5 -ml-3.5 w-60 h-full md:text-[13px] leading-5 text-navy docs-right-sidebar {extra_class}",
            right: "calc(40vw - 40.875rem)",
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul { class: "",
                for section in page.sections().iter().filter(|s| s.level <= 2) {
                    li { class: "pb-2",
                        a {
                            href: "?phantom={section.id}#{section.id}",
                            "{section.title}"
                        }
                    }
                }
            }
        }
    }
}

fn Content(cx: Scope) -> Element {
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_CONTENT);
    let extra_class = if highlighted.0 { "border border-blue-600 rounded-md" } else { "" };

    render! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack mx-auto container pt-12 pb-12 max-w-screen-md",
            div { class: "-py-8 {extra_class}",
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body ol {{ list-style: decimal; }}"
                        ".markdown-body li {{ display: list-item; }}"
                        ".markdown-body button {{ display: inline-block; background-color: rgba(209, 213, 219, 0.3); border-radius: 0.25rem; padding: 0.25rem 0.5rem; border: 1px solid; margin: 0.25rem; }}"
                    }
                    article { class: "markdown-body pt-1",
                        Outlet {}
                    }
                }
            }
        }
    }
}

fn BreadCrumbs(cx: Scope) -> Element {
    // parse out the route after the version and language
    let route = use_route(cx)?;

    render! {
        h2 { class: "font-semibold pb-4",
            for segment in route.to_string().split("/").skip(3).filter(|f| !f.is_empty()) {
                rsx! {
                    if segment != "index" {
                        rsx! {
                            Link { target: Route::Homepage {}, class: "text-blue-600", "{segment}" }
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
fn use_book(cx: &ScopeState) -> BookRoute {
    let route = use_route(cx).unwrap();
    match route {
        Route::Docs { child } => child,
        _ => unreachable!(),
    }
}

fn default_page() -> &'static Page<BookRoute> {
    let id = LAZY_BOOK.page_id_mapping.get(&BookRoute::default()).unwrap();
    LAZY_BOOK.pages.get(id.0).unwrap()
}
