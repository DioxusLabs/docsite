use crate::docs::LAZY_BOOK;
use crate::*;
use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;
use dioxus_material_icons::MaterialIconColor;
use fermi::use_atom_state;
use fermi::{use_read, Atom};
use mdbook_shared::Page;
use mdbook_shared::SummaryItem;

pub struct DocsLayoutHighlighted(pub bool);
pub static HIGHLIGHT_DOCS_LAYOUT: Atom<DocsLayoutHighlighted> =
    Atom(|_| DocsLayoutHighlighted(false));
pub static SHOW_SIDEBAR: Atom<bool> = Atom(|_| false);
pub struct DocsContentHighlighted(pub bool);
pub static HIGHLIGHT_DOCS_CONTENT: Atom<DocsContentHighlighted> =
    Atom(|_| DocsContentHighlighted(false));

/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_API_URL: &str = "https://api.github.com/repos/DioxusLabs/docsite/contents/docs-src/0.4/en";
/// Use this URL while loading the file-specific URL.
const GITHUB_EDIT_PAGE_FALLBACK_URL: &str = "https://github.com/DioxusLabs/docsite";
/// The Markdown file path needs to be appended to this, including the first slash!
const GITHUB_EDIT_PAGE_EDIT_URL: &str = "https://github.com/DioxusLabs/docsite/edit/master/docs-src/0.4/en";

#[component]
pub fn Learn(cx: Scope) -> Element {
    let show_sidebar_button = use_atom_state(cx, &SHOW_DOCS_NAV);
    cx.use_hook(|| show_sidebar_button.set(true));
    use_on_destroy(cx, {
        to_owned![show_sidebar_button];
        move || show_sidebar_button.set(false)
    });

    cx.render(rsx! {
        div { class: "w-full pt-12 text-sm dark:bg-ideblack", min_height: "100vh",
            // do a typical three-column flex layout with a single centered then pin the nav items on top
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto dark:text-white",
                LeftNav {}
                Content {}
                RightNav {}
            }
        }
    })
}

fn LeftNav(cx: Scope) -> Element {
    let show_sidebar = use_atom_state(cx, &SHOW_SIDEBAR);
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_LAYOUT);
    let extra_class = if highlighted.0 {
        "border border-green-600 rounded-md"
    } else {
        ""
    };
    let hidden = if **show_sidebar { "" } else { "hidden" };
    let full_width = if **show_sidebar { "min-w-full" } else { "" };
    let chapters = vec![
        &LAZY_BOOK.summary.prefix_chapters,
        &LAZY_BOOK.summary.numbered_chapters,
        &LAZY_BOOK.summary.suffix_chapters,
    ];

    render! {
        // Now, pin the nav to the left
        nav { class: "bg-white dark:bg-ideblack lg:bg-inherit pl-6 pb-32 z-20 text-base lg:block sticky top-28 lg:-ml-3.5 w-[calc(100%-1rem)] md:w-60 h-screen max-h-screen lg:text-[13px] text-navy content-start overflow-y-auto leading-5 {extra_class} {full_width} {hidden}",
            // I like the idea of breadcrumbs, but they add a lot of visual noise, and like, who cares?
            // BreadCrumbs {}

            for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                SidebarSection { chapter: chapter }
            }
        }
    }
}

/// Navigate between doc versions
fn DocVersionNav(cx: Scope) -> Element {
    let navigator = use_navigator(cx);

    render! {
        div { class: "pb-4",
            ul { class: "pl-2",
                li { class: "m-1 rounded-md pl-2 hover:bg-gray-200 hover:dark:bg-gray-800",
                    a { href: "/learn/0.3",
                        dioxus_material_icons::MaterialIcon { name: "chevron_left", color: "gray" }
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
fn SidebarSection(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link()?;

    let sections = link
        .nested_items
        .iter()
        .map(|chapter| render! { SidebarChapter { chapter: chapter } });

    render! {
        div { class: "pb-4",
            if let Some(url) = &link.location {
                rsx! {
                    Link { to: Route::Docs { child: *url }, h2 { class: "font-semibold", "{link.name}" } }
                }
            }
            ul { class: "pl-2", sections }
        }
    }
}

#[component]
fn SidebarChapter(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();
    let list_toggle = use_state(cx, || false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book(cx).to_string();

    // for instance, if the current page is /docs/0.4/en/learn/overview
    // then we want to show the dropdown for /docs/0.4/en/learn
    let show_dropdown = *list_toggle.get() || book_url.starts_with(&*url.to_string());
    let show_chevron = !link.nested_items.is_empty();

    if show_chevron {
        render! {
            li { class: "m-1 rounded-md ml-[-1px] hover:bg-gray-200 hover:dark:bg-gray-800",
                button { onclick: move |_| list_toggle.set(!list_toggle.get()),
                    dioxus_material_icons::MaterialIcon { name: "chevron_right", color: "gray" }
                }
                Link { to: Route::Docs { child: *url }, "{link.name}" }
            }
            if show_dropdown {
                rsx! {
                    ul { class: "ml-6 border-l border-gray-300 py-1",
                        for chapter in link.nested_items.iter() {
                            SidebarChapter { chapter: chapter }
                        }
                    }
                }
            }
        }
    } else {
        render! { LocationLink { chapter: chapter } }
    }
}

#[component]
fn LocationLink(cx: Scope, chapter: &'static SummaryItem<BookRoute>) -> Element {
    let book_url = use_book(cx).to_string();

    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url.starts_with(&*url.to_string()) {
        true => "bg-gray-200 dark:bg-gray-800",
        false => "",
    };

    render! {
        Link { to: Route::Docs { child: *url },
            li { class: "m-1 rounded-md pl-2 hover:bg-gray-200 hover:dark:bg-gray-800 {current_class}",
                "{link.name}"
            }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav(cx: Scope) -> Element {
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_LAYOUT);
    let extra_class = if highlighted.0 {
        "border border-green-600 rounded-md"
    } else {
        ""
    };
    let page = use_book(cx);
    let padding_map = ["pl-2", "pl-4", "pl-6", "pl-8", "pl-10"];
    let page_url = page.to_string();

    // This is the URL for the file if that file is not a directory that uses /index.md
    // page_url starts with '/', so we don't need to worry about that
    let github_api_url = format!("{GITHUB_API_URL}{page_url}.md");

    let edit_github_url = use_future(cx, &page_url, |page_url| async move {
        // If the file is not found, that means that we have to use /index.md
        if reqwest::get(github_api_url).await.unwrap().status() == reqwest::StatusCode::NOT_FOUND {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}/index.md")
        } else {
            format!("{GITHUB_EDIT_PAGE_EDIT_URL}{page_url}.md")
        }
    });
    // That might be a naive approach, but it's the easiest

    render! {
        div {
            class: "overflow-y-auto hidden xl:block sticky top-28 pl-3.5 -ml-3.5 w-60 h-full md:text-[13px] leading-5 text-navy docs-right-sidebar {extra_class}",
            left: "calc(100vw - 15rem)",
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul { class: "",
                for section in page.sections() {
                    li { class: "pb-2 {padding_map[section.level-1]}",
                        a { href: "?phantom={section.id}#{section.id}", "{section.title}" }
                    }
                }
            }
            h2 { class: "py-4 font-semibold",
                match edit_github_url.value() {
                    Some(url) => rsx!(a { href: "{url}", "Edit this page!" }),
                    None => rsx!(a { href: "{GITHUB_EDIT_PAGE_FALLBACK_URL}", "Edit this page!" })
                }
            }
            h2 { class: "py-4 font-semibold", "Go to version" }
            DocVersionNav {}
        }
    }
}

fn Content(cx: Scope) -> Element {
    let highlighted = use_read(cx, &HIGHLIGHT_DOCS_CONTENT);
    let extra_class = if highlighted.0 {
        "border border-blue-600 rounded-md"
    } else {
        ""
    };

    render! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack container pt-6 pb-12",
            div { class: "-py-8 {extra_class}",
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body ol {{ list-style: decimal; }}"
                        ".markdown-body li {{ display: list-item; }}"
                        ".markdown-body button {{ display: inline-block; background-color: rgba(209, 213, 219, 0.3); border-radius: 0.25rem; padding: 0.25rem 0.5rem; border: 1px solid; margin: 0.25rem; }}"
                        ".markdown-body .header {{ color: inherit }}"
                    }
                    article { class: "markdown-body pt-1", Outlet::<Route> {} }
                }
            }
        }
    }
}

fn BreadCrumbs(cx: Scope) -> Element {
    // parse out the route after the version and language
    let route: Route = use_route(cx)?;

    render! {
        h2 { class: "font-semibold pb-4",
            for segment in route.to_string().split('/').skip(3).filter(|f| !f.is_empty()) {
                rsx! {
                    if segment != "index" {
                        rsx! {
                            Link { to: Route::Homepage {}, class: "text-blue-600", "{segment}" }
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
    let id = LAZY_BOOK
        .page_id_mapping
        .get(&BookRoute::default())
        .unwrap();
    LAZY_BOOK.pages.get(id.0).unwrap()
}

#[component]
pub fn DocsO3(cx: Scope, segments: Vec<String>) -> Element {
    let navigator = use_navigator(cx);
    let route: Route = use_route(cx).unwrap();
    navigator.push(route);
    None
}
