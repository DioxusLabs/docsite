use crate::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use std::ops::Deref;

pub(crate) static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);
pub(crate) static SHOW_DOCS_NAV: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn Nav() -> Element {
    rsx! {
        SearchModal {}
        header { class: "sticky top-0 z-30 bg-white bg-opacity-80 dark:text-gray-200 dark:bg-ideblack dark:bg-opacity-80 border-b dark:border-stone-700 h-16 backdrop-blur-sm",
            div { class: "py-2 px-2 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6 h-16",
                div { class: "flex z-50 md:px-2", LinkList {} }
                div { class: "flex h-full justify-end ml-2 items-center gap-4 py-2",
                    button {
                        class: "
            max-w-[12rem] items-center rounded
            p-1 text-left text-sm font-light leading-none border

            hidden md:flex flex-row
            w-full sm:flex-1 md:w-full xl:max-w-[12rem]

            bg-gray-100 text-gray-400 hover:brightness-95
            dark:bg-ghdarkmetal dark:text-gray-300 dark:border-gray-700 h-full
            ",
                        onclick: move |_| {
                            *SHOW_SEARCH.write() = true;
                        },
                        span { class: "h-4 px-1 dark:hidden",
                            MaterialIcon {
                                name: "search",
                                size: 16,
                                color: MaterialIconColor::Dark,
                            }
                        }
                        span { class: "h-4 px-1 hidden dark:block",
                            MaterialIcon {
                                name: "search",
                                size: 16,
                                color: MaterialIconColor::Light,
                            }
                        }
                        span { class: "hidden content-center text-sm sm:flex flex-row w-60 justify-between",
                            span { "Search..." }
                            span { class: "px-1 min-w-6
                border bg-gray-100 border-gray-300 rounded text-center text-base/[18px] text-[.75rem] align-middle
                dark:bg-ghdarkmetal dark:border-gray-700
                ",
                                "/"
                            }
                        }
                    }
                    div {
                        tabindex: "0",
                        cursor: "pointer",
                        role: "button",
                        onclick: move |_| {
                            let mut sidebar = SHOW_SIDEBAR.write();
                            *sidebar = !*sidebar;
                        },
                        class: "bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-500 rounded-lg p-1 mr-2 lg:hidden my-3 h-8 flex items-center text-lg z-[100]",
                        class: if !SHOW_DOCS_NAV() { "hidden" },
                        MaterialIcon {
                            name: "menu",
                            size: 24,
                            color: MaterialIconColor::Dark,
                        }
                    }

                    div { class: "h-full  gap-4 hidden lg:flex",
                        div { class: "border-l border-gray-200 dark:border-gray-800 h-full" }
                        div { class: "hidden lg:flex items-center gap-4",
                            label {
                                class: "sr-only",
                                id: "headlessui-listbox-label-2",
                                "Theme"
                            }
                            Link {
                                to: "https://discord.gg/XgGxMSkvUM".to_string(),
                                class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                new_tab: true,
                                span { class: "sr-only", "Dioxus on Discord" }
                                crate::icons::DiscordLogo {}
                            }
                            Link {
                                to: "https://github.com/dioxuslabs/dioxus".to_string(),
                                class: "flex flex-row items-center text-gray-400 hover:text-gray-500 dark:hover:text-gray-300 gap-2",
                                new_tab: true,
                                span { class: "sr-only", "Dioxus on GitHub" }
                                crate::icons::Github2 {}
                                span { class: "text-xs", "20.7k" }
                            }
                        }
                        div { class: "border-l border-gray-200 dark:border-gray-800 h-full" }
                        div { class: "hidden lg:flex items-center gap-2 h-full",
                            label {
                                class: "sr-only",
                                id: "headlessui-listbox-label-2",
                                "Theme"
                            }
                            Link {
                                to: Route::Deploy {},
                                class: "h-full flex flex-col justify-center text-center md:px-3 bg-white dark:bg-gray-300 border border-gray-200 dark:border-gray-700 text-sm md:text-sm rounded font-semibold text-gray-700 hover:brightness-95 dark:hover:brightness-105",
                                "Deploy"
                            }
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::Index {},
                                },
                                class: "md:px-3 h-full flex flex-col justify-center bg-blue-500 text-lg md:text-sm text-white rounded font-semibold hover:brightness-95 dark:hover:brightness-105",
                                "Learn"
                            }
                        }
                    }
                }
            }
        }
    }
}

static LINKS: &[(&str, &str)] = &[
    ("Learn", "/learn/0.5/"),
    ("SDK", "/sdk"),
    ("Playground", "/play"),
    ("Components", "/components"),
    ("Blog", "/blog"),
];

#[component]
fn LinkList() -> Element {
    rsx! {
        nav { class: "flex flex-row items-center space-x-2 md:space-x-6 text-md font-light leading-none text-slate-700 dark:text-white whitespace-nowrap",
            Link {
                to: Route::Homepage {},
                class: "title-font font-medium items-center text-gray-900 flex flex-row gap-1",
                img {
                    src: asset!("/assets/static/smalllogo.png"),
                    class: "h-6 w-auto",
                }
                span { class: "text-xl dark:text-white leading-none hidden sm:block font-mono",
                    "DIOXUS"
                }
            }
            for (name , link) in LINKS.iter().cloned() {
                Link {
                    to: link,
                    class: "leading-none hover:text-sky-500 dark:hover:text-sky-400 rounded fill-zinc-700 dark:fill-zinc-100",
                    active_class: "text-sky-500 dark:text-sky-400",
                    position: "relative",
                    "{name}"
                }
            }
        }
    }
}

type Results = Result<Vec<dioxus_search::SearchResult<Route>>, stork_lib::SearchError>;

fn SearchModal() -> Element {
    let mut search_text = use_signal(String::new);
    let mut results = use_signal(|| SEARCH_INDEX.search(&search_text.read()));

    let mut last_key_press = use_signal(|| {
        if cfg!(target_arch = "wasm32") {
            js_sys::Date::now()
        } else {
            0.
        }
    });

    _ = use_resource(move || {
        async move {
            _ = search_text();

            // debounce the search
            if *last_key_press.read() - js_sys::Date::now() > 100. {
                results.set(SEARCH_INDEX.search(&search_text.read()));
                last_key_press.set(js_sys::Date::now());
            } else {
                gloo_timers::future::TimeoutFuture::new(100).await;
                results.set(SEARCH_INDEX.search(&search_text.read()));
            }
        }
    });

    // when we search, we do a similar search to mdbook
    // This will bring up individual sections that reference the search term with the breadcrumb
    // entries are sorted by breadcrumb

    rsx! {
        div {
            height: "100vh",
            width: "100vw",
            class: "fixed top-0 left-0 z-50 block bg-gray-100 bg-opacity-70 overflow-y-hidden search-modal-animated",
            class: if *SHOW_SEARCH.read() { "dioxus-show" } else { "dioxus-hide" },
            onclick: move |_| *SHOW_SEARCH.write() = false,

            // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
            div { class: "max-w-screen-sm mx-auto h-full flex flex-col",
                div { class: "h-40" }

                // The actual modal
                div { class: "bg-white dark:bg-ideblack rounded-xl max-h-[calc(100%-8rem)] overflow-y-auto text-gray-800 dark:text-gray-100 border",
                    // Search input
                    div { class: "flex flex-col flex-grow border-b border-gray-300 p-2 gap-2",
                        div { class: "my-auto flex flex-row items-center",
                            MaterialIcon {
                                name: "search",
                                size: 20,
                                color: MaterialIconColor::Dark,
                            }

                            // hide the input until show search so the onmounted fires
                            if SHOW_SEARCH() {
                                input {
                                    onclick: move |evt| evt.stop_propagation(),
                                    onkeydown: move |evt| {
                                        if evt.key() == Key::Escape {
                                            *SHOW_SEARCH.write() = false;
                                        }
                                    },
                                    oninput: move |evt| {
                                        search_text.set(evt.value());
                                    },
                                    onmounted: move |evt| async move {
                                        _ = evt.set_focus(true).await;
                                    },
                                    class: "flex-grow bg-transparent border-none outline-none pl-2 text-gray-800 dark:text-gray-100 py-2",
                                    placeholder: "Search the docs",
                                    value: "{search_text}",
                                }
                            }
                        }
                    }

                    SearchResults { results, search_text }
                }
            }
        }
    }
}

#[component]
fn SearchResults(results: Signal<Results>, search_text: Signal<String>) -> Element {
    if let Err(err) = results.read().as_ref() {
        return rsx! {
            div { class: "text-red-500", "{err}" }
        };
    }

    let _results = results.read();
    let results = _results.deref().as_ref().unwrap();

    rsx! {
        ul { class: "p-2",
            if search_text.read().is_empty() {
                for (search , route) in [
                    (
                        "Tutorial",
                        Route::Docs {
                            child: BookRoute::GuideIndex {},
                        },
                    ),
                    (
                        "Web",
                        Route::Docs {
                            child: BookRoute::ReferenceWebIndex {},
                        },
                    ),
                    (
                        "Desktop",
                        Route::Docs {
                            child: BookRoute::ReferenceDesktopIndex {
                            },
                        },
                    ),
                    (
                        "Mobile",
                        Route::Docs {
                            child: BookRoute::ReferenceMobileIndex {},
                        },
                    ),
                    (
                        "Fullstack",
                        Route::Docs {
                            child: BookRoute::ReferenceFullstackIndex {
                            },
                        },
                    ),
                    (
                        "Typesafe Routing",
                        Route::Docs {
                            child: BookRoute::RouterReferenceRoutesIndex {
                            },
                        },
                    ),
                ]
                {
                    SearchResultItem { title: search.to_string(), route }
                }
            } else if results.is_empty() {
                div { class: "text-center text-xlg p-4", "No results found for: {search_text}" }
            } else {
                for result in results {
                    SearchResultItem {
                        title: result.title.clone(),
                        route: result.route.clone(),
                        for segment in result.excerpts.first().unwrap().text.iter() {
                            if segment.highlighted {
                                span { class: "text-blue-500", "{segment.text}" }
                            } else {
                                span { "{segment.text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SearchResultItem(title: String, route: Route, children: Element) -> Element {
    rsx! {
        li { class: "w-full p-2 rounded hover:bg-gray-100 dark:hover:bg-ideblack transition-colors duration-200 ease-in-out",
            Link {
                to: route,
                onclick: move |_| {
                    *SHOW_SEARCH.write() = false;
                },
                class: "flex flex-row items-center gap-x-2",
                icons::DocumentIcon {}
                div { class: "flex flex-col justify-between",
                    h2 { class: "dark:text-white", "{title}" }
                    {children}
                }
            }
        }
    }
}
