use crate::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use std::ops::Deref;

pub static HIGHLIGHT_NAV_LAYOUT: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_NAV: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_SEARCH: GlobalSignal<bool> = Signal::global(|| false);
pub static LOGGED_IN: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_DOCS_NAV: GlobalSignal<bool> = Signal::global(|| false);

pub fn Nav() -> Element {
    rsx! {
        SearchModal {}
        header {
            class: "sticky top-0 z-30 bg-white dark:text-gray-200 dark:bg-ideblack border-b dark:border-stone-600",
            class: if HIGHLIGHT_NAV_LAYOUT() { "border border-orange-600 rounded-md" },
            div { class: "py-3 px-12 max-w-screen-2xl mx-auto flex items-center justify-between text-sm leading-6",
                button {
                    class: "bg-gray-100 rounded-lg p-2 mr-4 lg:hidden my-3 h-10 flex items-center text-lg z-[100]",
                    class: if !SHOW_DOCS_NAV() { "hidden" },
                    onclick: move |_| {
                        let mut sidebar = SHOW_SIDEBAR.write();
                        *sidebar = !*sidebar;
                    },
                    MaterialIcon { name: "menu", size: 24, color: MaterialIconColor::Dark }
                }
                div { class: "flex z-50 md:flex-1 px-2",
                    Link {
                        to: Route::Homepage {},
                        class: "flex title-font font-medium items-center text-gray-900",
                        img {
                            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            class: "h-5 w-auto"
                        }
                        span { class: "ml-3 text-xl dark:text-white font-mono", "Dioxus" }
                    }
                }

                Search {}

                div { class: "hidden md:flex h-full justify-end ml-2 flex-1",
                    div { class: "hidden md:flex items-center font-semibold",
                        nav {
                            ul { class: "flex items-center space-x-2", LinkList {} }
                        }
                        div { class: "hidden lg:flex items-center border-l border-gray-200 ml-4 pl-4 dark:border-gray-800",
                            label {
                                class: "sr-only",
                                id: "headlessui-listbox-label-2",
                                "Theme"
                            }
                            Link {
                                to: "https://discord.gg/XgGxMSkvUM",
                                class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                span { class: "sr-only", "Dioxus on Discord" }
                                crate::icons::DiscordLogo {}
                            }
                            Link {
                                to: "https://github.com/dioxuslabs/dioxus",
                                class: "ml-4 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
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
                                        class: "ml-4 h-10 rounded-full w-auto"
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

fn FullNav() -> Element {
    rsx! {
        div { class: "hidden md:flex items-center",
            nav {
                ul { class: "flex items-center space-x-2", LinkList {} }
            }
            div { class: "flex items-center border-l border-gray-200 ml-2 pl-3 dark:border-gray-800",
                label { class: "sr-only", id: "headlessui-listbox-label-2", "Theme" }
                a {
                    class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                    target: "https://discord.gg/XgGxMSkvUM",
                    span { class: "sr-only", "Dioxus on Discord" }
                    crate::icons::DiscordLogo {}
                }
                a {
                    class: "ml-6 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                    target: "https://github.com/dioxuslabs/dioxus",
                    span { class: "sr-only", "Dioxus on GitHub" }
                    crate::icons::Github2 {}
                }
            }
        }
    }
}

fn MobileNav() -> Element {
    rsx! {
        div { class: "flex items-center",
            button {
                class: "text-gray-500 hover:text-gray-600 w-8 h-8 -my-1 flex items-center justify-center md:hidden dark:hover:text-gray-300",
                "type": "button",
                span { class: "sr-only", "Search" }
                svg {
                    stroke: "currentColor",
                    "stroke-linecap": "round",
                    fill: "none",
                    "stroke-width": "2",
                    "aria-hidden": "true",
                    height: "24",
                    width: "24",
                    "stroke-linejoin": "round",
                    path { d: "m19 19-3.5-3.5" }
                    circle { cx: "11", cy: "11", r: "6" }
                }
            }
            div { class: "-my-1 ml-2 -mr-1 md:hidden",
                button {
                    class: "text-gray-500 w-8 h-8 flex items-center justify-center hover:text-gray-600 dark:text-gray-400 dark:hover:text-gray-300",
                    "type": "button",
                    onclick: move |_| {
                        let mut nav = SHOW_NAV.write();
                        *nav = !*nav;
                    },
                    span { class: "sr-only", "Navigation" }
                    svg {
                        width: "24",
                        height: "24",
                        "aria-hidden": "true",
                        fill: "none",
                        path {
                            stroke: "currentColor",
                            "stroke-width": "1.5",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            d: "M12 6v.01M12 12v.01M12 18v.01M12 7a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm0 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm0 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"
                        }
                    }
                }
            }
        }
    }
}

type LinkPairs<'a> = &'a [(&'a str, &'a str)];
static LINKS: &[(&str, &str, LinkPairs)] = &[
    // ("Tutorials", "/tutorials/", &[]),
    ("Awesome", "/awesome", &[]),
    (
        "API",
        "https://docs.rs/dioxus/0.5.0-alpha.2/dioxus/index.html",
        &[],
    ),
    ("Docs", "/learn/0.5/", &[]),
    ("Blog", "/blog", &[]),
];

#[component]
fn LinkList() -> Element {
    let hover = "hover:text-sky-500 dark:hover:text-sky-400";
    let hover_bg = "dark:hover:bg-gray-500 hover:bg-gray-200 rounded";

    rsx! {
        for (name , link , links) in LINKS.iter().cloned() {
            if links.is_empty() {
                li { key: "{link}",
                    Link {
                        class: "ml-[-3.8em] md:ml-0 md:py-2 md:px-2 {hover} {hover_bg} text-lg md:text-sm",
                        to: link,
                        "{name}"
                    }
                }
            } else {
                li { key: "{link}", class: "group relative dropdown",
                    span { class: "py-1 px-[0.25rem] md:px-2 text-lg md:text-sm {hover} {hover_bg} cursor-default",
                        "{name}"
                    }
                    nav { class: "md:dropdown-menu md:absolute h-auto md:-mt-64 md:group-hover:mt-0 md:opacity-0 md:group-hover:opacity-100 md:transition-opacity md:duration-250",
                        ul { class: "top-0 w-36 md:bg-white dark:md:bg-gray-800 md:shadow md:px-4 md:py-4 rounded",
                            for (name , link) in links.iter() {
                                Link { to: *link,
                                    li { class: "rounded px-1 py-1 {hover} {hover_bg} text-base md:text-sm",
                                        "{name}"
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

fn Search() -> Element {
    rsx! {
        div { class: "relative md:w-full max-w-[20rem] xl:max-w-[20rem] 2xl:max-w-[20rem] sm:mx-auto sm:flex-1",
            // Pop up a modal
            button {
                // Pop up a modal
                class: "bg-gray-100 rounded-lg p-2 sm:w-full text-left text-gray-400 my-auto sm:flex sm:flex-row sm:align-middle sm:justify-between",
                onclick: move |_| {
                    *SHOW_SEARCH.write() = true;
                },
                div { class: "h-full my-auto flex flex-row align-middle justify-between",
                    MaterialIcon { name: "search", size: 24, color: MaterialIconColor::Dark }
                    span { class: "hidden sm:block pl-2", "Search the docs" }
                }
                div { class: "hidden md:block border border-gray-300 rounded-lg p-1 text-xs text-gray-400",
                    "CTRL + /"
                }
            }
        }
    }
}

fn SearchModal() -> Element {
    let mut search_text = use_signal(String::new);
    let mut results = use_signal(|| SEARCH_INDEX.search(&search_text.read()));

    let mut last_key_press = use_signal(|| {
        #[cfg(not(target_arch = "wasm32"))]
        return 0.;
        js_sys::Date::now()
    });
    use_resource(move || {
        async move {
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
        if SHOW_SEARCH() {
            div {
                height: "100vh",
                width: "100vw",
                class: "fixed top-0 left-0 z-50 block bg-gray-500 bg-opacity-50 overflow-y-hidden",
                onclick: move |_| *SHOW_SEARCH.write() = false,

                // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
                div { class: "max-w-screen-md mx-auto h-full flex flex-col",
                    div { class: "h-30" }

                    // The actual modal
                    div { class: "bg-white dark:bg-ideblack p-2 md:p-6 rounded-2xl m-2 md:m-8 max-h-[calc(100%-8rem)] overflow-y-auto text-gray-800 dark:text-gray-100",
                        // Search input
                        div { class: "flex flex-row flex-grow border-b border-gray-300 pb-4",
                            div { class: "my-auto flex flex-row",
                                MaterialIcon { name: "search", size: 40, color: MaterialIconColor::Dark }
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
                                    onmounted: move |evt| {
                                        evt.set_focus(true);
                                    },
                                    class: "flex-grow bg-transparent border-none outline-none text-xl pl-2 text-gray-800 dark:text-gray-100",
                                    placeholder: "Search the docs",
                                    value: "{search_text}"
                                }
                            }
                            div {}
                        }

                        // Results
                        div { class: "overflow-y-auto",
                            ul {
                                match results.read().deref() {
                                    Ok(results) => {
                                        if results.is_empty() {
                                            rsx! {
                                                div {
                                                    class: "text-center text-xlg p-4",
                                                    "No results found"
                                                    div {
                                                        class: "dark:text-white text-left text-lg p-4",
                                                        "Try searching for:"
                                                        ul {
                                                            for search in ["Fullstack", "Typesafe Routing", "Authentication"] {
                                                                li {
                                                                    button {
                                                                        class: "underline p-4",
                                                                        onclick: move |_| {
                                                                            search_text.set(search.to_string());
                                                                        },
                                                                        "{search}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        else {
                                            rsx! {
                                                for result in results {
                                                    SearchResult { result: result.clone() }
                                                }
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        rsx! {
                                            div { class: "text-red-500", "{err}" }
                                        }
                                    }
                                }
                            }
                        }

                        //
                        div {
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
