use crate::docs::AnyBookRoute;
use crate::*;
use dioxus::html::input_data::keyboard_types::Key;
// use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use std::ops::Deref;

pub(crate) static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn Nav() -> Element {
    let route: Route = use_route();

    rsx! {
        SearchModal {}
        header { class: "sticky top-0 z-30 bg-opacity-80 dark:text-gray-200 dark:bg-opacity-80 border-b border-stone-300 dark:border-stone-700 h-16 backdrop-blur-sm",
            div { class: "py-2 px-4 sm:px-6 md:px-8 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6 h-16",
                div { class: "flex z-50 flex-1", LinkList {} }
                div { class: "flex h-full justify-end ml-2 items-center gap-3 py-2",
                    button {
                        class: "
            max-w-[12rem] items-center rounded
            p-1 text-left text-sm font-light leading-none border  border-gray-300

            hidden md:flex flex-row
            w-full sm:flex-1 md:w-full xl:max-w-[12rem]

            bg-gray-100 text-gray-400 hover:brightness-95
            dark:bg-ghdarkmetal dark:text-gray-300 dark:border-gray-700 h-full
            ",
                        onclick: move |_| {
                            *SHOW_SEARCH.write() = true;
                        },
                        // span { class: "h-4 px-1 dark:hidden",
                        //     MaterialIcon {
                        //         name: "search",
                        //         size: 16,
                        //         color: MaterialIconColor::Dark,
                        //     }
                        // }
                        // span { class: "h-4 px-1 hidden dark:block",
                        //     MaterialIcon {
                        //         name: "search",
                        //         size: 16,
                        //         color: MaterialIconColor::Light,
                        //     }
                        // }
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
                                        // MaterialIcon {
                    //     name: "menu",
                    //     size: 24,
                    //     color: MaterialIconColor::Dark,
                    // }
                    }

                    div { class: "h-full  gap-3 hidden lg:flex",
                        div { class: "border-l border-gray-200 dark:border-gray-800 h-full" }
                        div { class: "hidden lg:flex items-center gap-3",
                            DarkModeToggle { class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300" }
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
                                span { class: "text-xs text", CurrentStarCount {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn CurrentStarCount() -> Element {
    let num_stars = use_resource(move || async move {
        use crate::awesome::StarsResponse;
        let username = "DioxusLabs";
        let repo = "dioxus";
        let res = reqwest::get(format!("https://api.github.com/repos/{username}/{repo}")).await;
        let res = res.ok()?.json::<StarsResponse>().await.ok()?;
        Some(res.stargazers_count as usize)
    });

    let mut rendered_stars = 24.5;

    if let Some(Some(loaded)) = num_stars.value()() {
        let as_float = loaded as f64;
        rendered_stars = as_float.round() / 1000.0;
    }

    rsx! { "{rendered_stars:.1}k" }
}

static LINKS: &[(&str, &str)] = &[
    // while developing, the `learn` link should point to the latest version of the docs. makes our lives a bit easier
    ("Learn", if cfg!(debug_assertions) { "/learn/0.7/" } else { "/learn/0.6/" }),
    // ("SDK", "/sdk"),
    // ("Playground", "/playground"),
    ("Components", "/components"),
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
        #[cfg(not(feature = "production"))]
        let url_base = "http://localhost:8080/assets/dioxus_search";

        #[cfg(feature = "production")]
        let url_base = "https://dioxuslabs.com/assets/dioxus_search";

        let current_route: Route = router().current();
        // Only show search results from the version of the docs the user is currently on (or the latest if they
        // are not on a doc page)
        let docs_index_version = match &current_route {
            Route::Docs07 { .. } => "0_7",
            Route::Docs06 { .. } => "0_6",
            Route::Docs05 { .. } => "0_5",
            Route::Docs04 { .. } => "0_4",
            Route::Docs03 { .. } => "0_3",
            _ => "0_6",
        };

        let url = format!("{url_base}/index_searchable_{docs_index_version}.bin");

        let data = reqwest::get(url).await.ok()?.bytes().await.ok()?;

        let (bytes, _) =
            dioxus_search::yazi::decompress(&data, dioxus_search::yazi::Format::Zlib).ok()?;

        let index: dioxus_search::SearchIndex<Route> =
            dioxus_search::SearchIndex::from_bytes("search", bytes);

        Some(index)
    });

    let search = move || {
        let query = &search_text.read();
        let mut results = search_index
            .value()
            .as_ref()
            .and_then(|search| search.as_ref().map(|s| s.search(query)))
            .unwrap_or_else(|| Ok(vec![]));

        results
    };

    let mut results = use_signal(search);

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
            class: "fixed top-0 left-0 z-50 block bg-gray-100/70 dark:bg-gray-950/70 backdrop-blur-sm overflow-y-hidden search-modal-animated  ",
            class: if *SHOW_SEARCH.read() { "dioxus-show" } else { "dioxus-hide" },
            onclick: move |_| *SHOW_SEARCH.write() = false,

            // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
            div { class: "max-w-screen-sm mx-auto h-full flex flex-col",
                div { class: "h-40" }

                // The actual modal
                div { class: "bg-white dark:bg-ideblack rounded-xl max-h-[calc(100%-8rem)] overflow-y-auto text-gray-800 dark:text-gray-100 border border-gray-300 dark:border-[#a4a9ac7d]",
                    // Search input
                    div { class: "flex flex-col flex-grow border-b p-2 gap-2 border-inherit",
                        div { class: "my-auto flex flex-row items-center pl-2",
                            div { class: "dark:invert h-5" }

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
                                    class: "flex-grow bg-transparent border-none outline-none pl-2 text-gray-500 dark:text-gray-100 py-2 placeholder-gray-400",
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
    let cur_route = use_route::<Route>();
    let results = results
        .iter()
        .filter(|route| match route.route {
            Route::Docs03 { .. } => matches!(cur_route, Route::Docs03 { .. }),
            Route::Docs04 { .. } => matches!(cur_route, Route::Docs04 { .. }),
            Route::Docs05 { .. } => matches!(cur_route, Route::Docs05 { .. }),
            Route::Docs06 { .. } => {
                !matches!(cur_route, Route::Docs03 { .. })
                    && !matches!(cur_route, Route::Docs04 { .. })
                    && !matches!(cur_route, Route::Docs05 { .. })
            }
            _ => true,
        })
        .collect::<Vec<_>>();

    use crate::docs::router_06::BookRoute;

    let default_searches = [
        (
            "Tutorial",
            BookRoute::GuideIndex {
                section: Default::default(),
            },
        ),
        (
            "Web",
            BookRoute::GuidesWebIndex {
                section: Default::default(),
            },
        ),
        (
            "Desktop",
            BookRoute::GuidesDesktopIndex {
                section: Default::default(),
            },
        ),
        (
            "Mobile",
            BookRoute::GuidesMobileIndex {
                section: Default::default(),
            },
        ),
        (
            "Fullstack",
            BookRoute::GuidesFullstackIndex {
                section: Default::default(),
            },
        ),
        (
            "Typesafe Routing",
            BookRoute::RouterReferenceIndex {
                section: Default::default(),
            },
        ),
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
