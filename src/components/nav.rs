use crate::docs::AnyBookRoute;
use crate::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use std::ops::Deref;

pub(crate) static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn Nav() -> Element {
    let route: Route = use_route();

    rsx! {
        SearchModal {}
        header { class: "sticky top-0 z-30 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-b dark:border-stone-700 h-16 backdrop-blur-sm",
            div { class: "py-2 px-2 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6 h-16",
                div { class: "flex z-50 md:px-2 flex-1", LinkList {} }
                div { class: "flex h-full justify-end ml-2 items-center gap-3 py-2",
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
                        class: "bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-500 md:hidden rounded-lg p-1 mr-2 my-3 h-8 flex items-center text-lg z-50",
                        class: if !route.is_docs() { "hidden" },
                        MaterialIcon {
                            name: "menu",
                            size: 24,
                            color: MaterialIconColor::Dark,
                        }
                    }

                    div { class: "h-full  gap-3 hidden lg:flex",
                        div { class: "border-l border-gray-200 dark:border-gray-800 h-full" }
                        div { class: "hidden lg:flex items-center gap-3",
                            label {
                                class: "sr-only",
                                id: "headlessui-listbox-label-2",
                                "Theme"
                            }
                            Link {
                                to: "https://crates.io/crates/dioxus",
                                new_tab: true,
                                class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                span { class: "sr-only", "Dioxus on docs.rs" }
                                svg {
                                    "viewBox": "0 0 576 512",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    path {
                                        d: "M290.8 48.6l78.4 29.7L288 109.5 206.8 78.3l78.4-29.7c1.8-.7 3.8-.7 5.7 0zM136 92.5l0 112.2c-1.3 .4-2.6 .8-3.9 1.3l-96 36.4C14.4 250.6 0 271.5 0 294.7L0 413.9c0 22.2 13.1 42.3 33.5 51.3l96 42.2c14.4 6.3 30.7 6.3 45.1 0L288 457.5l113.5 49.9c14.4 6.3 30.7 6.3 45.1 0l96-42.2c20.3-8.9 33.5-29.1 33.5-51.3l0-119.1c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-1.3-.5-2.6-.9-3.9-1.3l0-112.2c0-23.3-14.4-44.1-36.1-52.4l-96-36.4c-12.8-4.8-26.9-4.8-39.7 0l-96 36.4C150.4 48.4 136 69.3 136 92.5zM392 210.6l-82.4 31.2 0-89.2L392 121l0 89.6zM154.8 250.9l78.4 29.7L152 311.7 70.8 280.6l78.4-29.7c1.8-.7 3.8-.7 5.7 0zm18.8 204.4l0-100.5L256 323.2l0 95.9-82.4 36.2zM421.2 250.9c1.8-.7 3.8-.7 5.7 0l78.4 29.7L424 311.7l-81.2-31.1 78.4-29.7zM523.2 421.2l-77.6 34.1 0-100.5L528 323.2l0 90.7c0 3.2-1.9 6-4.8 7.3z",
                                        fill: "currentColor",
                                        fill_rule: "nonzero",
                                    }
                                }
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
                                span { class: "text-xs text", "21.7k" }
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
                                to: crate::docs::router_06::BookRoute::Index {
                                }
                                    .global_route(),
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
    ("Learn", "/learn/0.6/"),
    // ("SDK", "/sdk"),
    // ("Playground", "/play"),
    // ("Components", "/components"),
    ("Awesome", "/awesome"),
    ("Blog", "/blog"),
];

#[component]
fn LinkList() -> Element {
    rsx! {
        nav { class: "flex-grow md:flex-grow-0 flex flex-row items-center  text-md font-light leading-none text-slate-700 dark:text-white whitespace-nowrap md:gap-6",
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
            div { class: "flex-1 flex flex-row items-center md:space-x-6 justify-evenly",
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
}

type Results = Result<Vec<dioxus_search::SearchResult<Route>>, stork_lib::SearchError>;

fn SearchModal() -> Element {
    let mut search_text = use_signal(String::new);

    let search_index = use_resource(|| async move {
        #[cfg(debug_assertions)]
        let url = "http://localhost:8080/assets/dioxus_search/index_searchable.bin";

        #[cfg(not(debug_assertions))]
        let url = "https://dioxuslabs.com/assets/dioxus_search/index_searchable.bin";

        let data = reqwest::get(url).await.ok()?.bytes().await.ok()?;

        let (bytes, _) =
            dioxus_search::yazi::decompress(&data, dioxus_search::yazi::Format::Zlib).ok()?;

        let index = dioxus_search::SearchIndex::from_bytes("search", bytes);

        Some(index)
    });

    let search = move || {
        let query = &search_text.read();
        search_index
            .value()
            .as_ref()
            .map(|search| search.as_ref().map(|s| s.search(query)))
            .flatten()
            .unwrap_or_else(|| Ok(vec![]))
    };

    let mut results = use_signal(|| search());

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
                results.set(search());
                last_key_press.set(js_sys::Date::now());
            } else {
                gloo_timers::future::TimeoutFuture::new(100).await;
                results.set(search());
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
            class: "fixed top-0 left-0 z-50 block bg-gray-100 dark:bg-opacity-70 bg-opacity-70 overflow-y-hidden search-modal-animated  ",
            class: if *SHOW_SEARCH.read() { "dioxus-show" } else { "dioxus-hide" },
            onclick: move |_| *SHOW_SEARCH.write() = false,

            // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
            div { class: "max-w-screen-sm mx-auto h-full flex flex-col",
                div { class: "h-40" }

                // The actual modal
                div { class: "bg-white dark:bg-ideblack rounded-xl max-h-[calc(100%-8rem)] overflow-y-auto text-gray-800 dark:text-gray-100 border dark:border-[#a4a9ac7d]",
                    // Search input
                    div { class: "flex flex-col flex-grow border-b p-2 gap-2 border-inherit",
                        div { class: "my-auto flex flex-row items-center pl-2",
                            div { class: "dark:invert h-5",
                                MaterialIcon {
                                    name: "search",
                                    size: 20,
                                    color: MaterialIconColor::Dark,
                                }
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
                                    class: "flex-grow bg-transparent border-none outline-none pl-2 text-gray-800 dark:text-gray-100 py-2 placeholder-gray-200",
                                    placeholder: "Search the docs...",
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

    use crate::docs::router_06::BookRoute;

    let default_searches = [
        ("Tutorial", BookRoute::GuideIndex {}),
        ("Web", BookRoute::GuidesWebIndex {}),
        ("Desktop", BookRoute::GuidesDesktopIndex {}),
        ("Mobile", BookRoute::GuidesMobileIndex {}),
        ("Fullstack", BookRoute::GuidesFullstackIndex {}),
        ("Typesafe Routing", BookRoute::RouterReferenceIndex {}),
    ];

    rsx! {
        ul { class: "p-2 flex flex-col",
            if search_text.read().is_empty() {
                for (search , child) in default_searches {
                    SearchResultItem {
                        title: search.to_string(),
                        route: child.global_route(),
                    }
                }
            } else if results.is_empty() {
                div { class: "text-center text-xlg p-4", "No results found for: {search_text}" }
            } else {
                for result in results {
                    SearchResultItem {
                        title: result.title.clone(),
                        route: result.route.clone(),
                        span { class: "mt-1",
                            for segment in result.excerpts.first().unwrap().text.iter() {
                                if segment.highlighted {
                                    span { class: "text-blue-500", "{segment.text}" }
                                } else {
                                    span { class: "text-gray-400", "{segment.text}" }
                                }
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
        li { class: "w-full rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors duration-200 ease-in-out",
            Link {
                to: route,
                onclick: move |_| {
                    *SHOW_SEARCH.write() = false;
                },
                class: "flex flex-row items-center gap-x-2 p-2",
                div { class: "flex flex-col mt-1 mb-1",
                    span { class: "flex flex-row items-center gap-x-1",
                        icons::DocumentIcon {}
                        h2 { class: "dark:text-white ml-1", "{title}" }
                    }
                    {children}
                }
            }
        }
    }
}
