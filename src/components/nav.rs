use crate::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use std::ops::Deref;

pub(crate) static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);
pub(crate) static LOGGED_IN: GlobalSignal<bool> = Signal::global(|| false);
pub(crate) static SHOW_DOCS_NAV: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn Nav() -> Element {
    rsx! {
        SearchModal {}
        header { class: "sticky top-0 z-30 bg-white bg-opacity-80 dark:text-gray-200 dark:bg-ideblack dark:bg-opacity-80 border-b dark:border-stone-700 h-16 backdrop-blur-sm",
            div { class: "lg:py-2 px-2 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6 h-16",
                button {
                    class: "bg-zinc-300 rounded-lg p-1 mr-4 lg:hidden my-3 h-10 flex items-center text-lg z-[100]",
                    class: if !SHOW_DOCS_NAV() { "hidden" },
                    onclick: move |_| {
                        let mut sidebar = SHOW_SIDEBAR.write();
                        *sidebar = !*sidebar;
                    },
                    MaterialIcon {
                        name: "menu",
                        size: 24,
                        color: MaterialIconColor::Dark,
                    }
                }
                div { class: "flex z-50 md:flex-1 px-2", LinkList {} }
                div { class: "hidden md:flex h-full justify-end ml-2 flex-1",
                    div { class: "hidden md:flex items-center",
                        Search {}
                        div { class: "hidden lg:flex items-center border-l border-gray-200 ml-4 pl-4 dark:border-gray-800",
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
                                class: "ml-4 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                new_tab: true,
                                span { class: "sr-only", "Dioxus on GitHub" }
                                crate::icons::Github2 {}
                            }
                        }
                        div { class: "hidden lg:flex items-center border-l border-gray-200 ml-4 pl-6 dark:border-gray-800",
                            label {
                                class: "sr-only",
                                id: "headlessui-listbox-label-2",
                                "Theme"
                            }
                            Link {
                                to: Route::Deploy {},
                                class: "md:ml-0 md:py-2 md:px-3 bg-blue-500 ml-4 text-lg md:text-sm text-white rounded font-semibold",
                                "DEPLOY"
                            }
                            if LOGGED_IN() {
                                Link { to: Route::Homepage {},
                                    img {
                                        src: "https://avatars.githubusercontent.com/u/10237910?s=40&v=4",
                                        class: "ml-4 h-10 rounded-full w-auto",
                                    }
                                }
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
    ("Playground", "/play"),
    ("SDK", "/sdk"),
    ("Blog", "/blog"),
    // ("Awesome", "/awesome"),
    // ("docs.rs", "https://docs.rs/dioxus/latest/dioxus/"),
];

#[component]
fn LinkList() -> Element {
    rsx! {
        nav { class: "flex items-center space-x-2 text-md font-light leading-none text-slate-700 dark:text-white whitespace-nowrap",
            Link {
                to: Route::Homepage {},
                class: "flex title-font font-medium items-center text-gray-900",
                img {
                    src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                    class: "h-8 w-auto",
                }
                span { class: "text-xl dark:text-white leading-none font-bold hidden sm:block px-4",
                    "Dioxus"
                }
            }
            for (name , link) in LINKS.iter().cloned() {
                Link {
                    to: link,
                    class: "p-2 leading-none hover:text-sky-500 dark:hover:text-sky-400 rounded fill-zinc-700 dark:fill-zinc-100",
                    active_class: "text-sky-500 dark:text-sky-400",
                    position: "relative",
                    "{name}"
                }
            }
        }
    }
}

fn Search() -> Element {
    rsx! {
        div { class: "relative md:w-full max-w-[20rem] xl:max-w-[20rem] 2xl:max-w-[20rem] sm:mx-auto sm:flex-1 text-sm font-light leading-none",
            // Pop up a modal
            button {
                // Pop up a modal
                class: "bg-gray-100 rounded-lg p-1 sm:w-full text-left text-gray-400 my-auto sm:flex sm:flex-row sm:align-middle sm:justify-between",
                onclick: move |_| {
                    *SHOW_SEARCH.write() = true;
                },
                div { class: "h-full my-auto flex flex-row content-center justify-between",
                    MaterialIcon {
                        name: "search",
                        size: 20,
                        color: MaterialIconColor::Dark,
                    }
                    span { class: "hidden sm:block pl-2 pr-4 content-center", "Search the docs" }
                }
                div { class: "hidden md:block border border-gray-300 rounded-lg p-1 text-xs text-gray-400",
                    "CTRL + /"
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
            class: "fixed top-0 left-0 z-50 block bg-gray-500 bg-opacity-50 overflow-y-hidden search-modal-animated",
            class: if *SHOW_SEARCH.read() { "dioxus-show" } else { "dioxus-hide" },
            onclick: move |_| *SHOW_SEARCH.write() = false,

            // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
            div { class: "max-w-screen-md mx-auto h-full flex flex-col",
                div { class: "h-30" }

                // The actual modal
                div { class: "bg-white dark:bg-ideblack p-2 md:p-6 rounded-2xl m-2 md:m-8 max-h-[calc(100%-8rem)] overflow-y-auto text-gray-800 dark:text-gray-100",
                    // Search input
                    div { class: "flex flex-row flex-grow border-b border-gray-300 pb-4",
                        div { class: "my-auto flex flex-row",
                            MaterialIcon {
                                name: "search",
                                size: 40,
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
                                    class: "flex-grow bg-transparent border-none outline-none text-xl pl-2 text-gray-800 dark:text-gray-100",
                                    placeholder: "Search the docs",
                                    value: "{search_text}",
                                }
                            }
                        }
                    }

                    div { class: "overflow-y-auto",
                        ul {
                            SearchResults { results, search_text }
                        }
                    }
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

    if !results.is_empty() {
        return rsx! {
            for result in results {
                SearchResult { result: result.clone() }
            }
        };
    }

    rsx! {
        div { class: "text-center text-xlg p-4",
            "No results found"
            div { class: "dark:text-white text-left text-lg p-4",
                div {
                    "Try searching for:"
                    ul {
                        for search in ["Fullstack", "Typesafe Routing", "Authentication"] {
                            li {
                                button {
                                    class: "underline p-1 md:p-2",
                                    onclick: move |_| {
                                        search_text.set(search.to_string());
                                    },
                                    "{search}"
                                }
                            }
                        }
                    }
                }

                div { class: "mt-4",
                    "Or go to:"
                    ul {
                        for (name , link) in LINKS.iter().cloned() {
                            li { class: "p-1 md:p-2",
                                Link { to: link, class: "underline ", "{name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SearchResult(result: dioxus_search::SearchResult<Route>) -> Element {
    let title = &result.title;
    let route = &result.route;
    let top_excerpt_segments = &result.excerpts.first().unwrap().text;

    rsx! {
        li { class: "w-full mt-4 p-2 rounded hover:bg-gray-100 dark:hover:bg-ideblack transition-colors duration-200 ease-in-out",
            Link {
                to: route.clone(),
                onclick: move |_| {
                    *SHOW_SEARCH.write() = false;
                },
                div { class: "flex flex-col justify-between pb-1",
                    h2 { class: "font-semibold dark:text-white", "{title}" }
                }
                p { class: "text-sm pr-8 text-gray-500 dark:text-gray-300",
                    for segment in top_excerpt_segments {
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
