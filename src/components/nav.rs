use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};
use crate::*;
use fermi::{use_atom_state, use_read, Atom};
use dioxus::html::input_data::keyboard_types::{Key};

pub static SHOW_NAV: Atom<bool> = |_| false;
pub static SHOW_SEARCH: Atom<bool> = |_| false;
pub static LOGGED_IN: Atom<bool> = |_| false;

pub fn Nav(cx: Scope) -> Element {
    let show = use_read(cx, SHOW_NAV);
    let show_modal = use_read(cx, SHOW_SEARCH);
    let logged_in = use_read(cx, LOGGED_IN);

    cx.render(rsx! {
        SearchModal {}
        header { class: "sticky top-0 z-30 bg-white shadow dark:text-gray-200 dark:bg-ideblack dark:border-b border-stone-600",
            div { class: "py-3 px-12 max-w-screen-3xl mx-auto flex items-center justify-between text-sm leading-6",
                div { class: "flex z-50 flex-1",
                    Link {
                        target: Route::Homepage {},
                        class: "flex title-font font-medium items-center text-gray-900",
                        img {
                            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            class: "h-5 w-auto"
                        }
                        span { class: "ml-3 text-xl dark:text-white font-mono", "Dioxus Labs" }
                    }
                }

                Search {}

                div { class: "hidden xl:flex flex-1 h-full justify-end",
                    div { class: "hidden md:flex items-center  font-semibold",
                        nav {
                            ul { class: "flex items-center space-x-2", LinkList {} }
                        }
                        div { class: "flex items-center border-l border-gray-200 ml-4 pl-4 dark:border-gray-800",
                            label { class: "sr-only", id: "headlessui-listbox-label-2", "Theme" }
                            Link {
                                target: NavigationTarget::External("https://discord.gg/XgGxMSkvUM".into()),
                                class: "block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                span { class: "sr-only", "Dioxus on Discord" }
                                crate::icons::DiscordLogo {}
                            }
                            Link {
                                target: NavigationTarget::External("https://github.com/dioxuslabs/dioxus".into()),
                                class: "ml-4 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                                span { class: "sr-only", "Dioxus on GitHub" }
                                crate::icons::Github2 {}
                            }
                        }
                        div { class: "flex items-center border-l border-gray-200 ml-4 pl-6 dark:border-gray-800",
                            label { class: "sr-only", id: "headlessui-listbox-label-2", "Theme" }
                            Link {
                                target: Route::Homepage {},
                                class: "ml-[-3.8em] md:ml-0 md:py-2 md:px-3 bg-blue-500 ml-4 text-lg md:text-sm text-white rounded font-semibold",
                                "DEPLOY"
                            }
                            if *logged_in {
                                rsx! {
                                    Link { target: Route::Homepage {},
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
    })
}

fn FullNav(cx: Scope) -> Element {
    cx.render(rsx! {
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
    })
}

fn MobileNav(cx: Scope) -> Element {
    let show = use_atom_state(cx, SHOW_NAV);

    cx.render(rsx! {
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
                    onclick: move |_| show.modify(|f| !f),
                    span { class: "sr-only", "Navigation" }
                    svg { width: "24", height: "24", "aria-hidden": "true", fill: "none",
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
    })
}

type LinkPairs<'a> = &'a [(Route, Route)];
static LINKS: &[(&str, Route, LinkPairs)] = &[
    ("Blog", Route::BlogList {}, &[]),
    ("Docs", Route::Docs { child: BookRoute::GettingStartedIndex {} }, &[]),
    // ("Deploy", "/deploy", &[]),
];

#[inline_props]
fn LinkList(cx: Scope) -> Element {
    let hover = "hover:text-sky-500 dark:hover:text-sky-400";
    let hover_bg = "dark:hover:bg-gray-500 hover:bg-gray-200 rounded";

    let links = LINKS.iter().cloned().map(|(name, link, links)| {
        if links.is_empty() {
            rsx! {
                li { key: "{link}",
                    Link {
                        class: "ml-[-3.8em] md:ml-0 md:py-2 md:px-2 {hover} {hover_bg} text-lg md:text-sm",
                        target: link,
                        "{name}"
                    }
                }
            }
        } else {
            rsx! {
                li { key: "{link}", class: "group relative dropdown",
                    span { class: "py-1 px-[0.25rem] md:px-2 text-lg md:text-sm {hover} {hover_bg} cursor-default",
                        "{name}"
                    }
                    // Link { target: "{link}", class: "py-1 px-2 {hover} {hover_bg}", "{name}" }
                    // Link { target: "{link}", class: "py-1 px-2 {hover} {hover_bg}", "{name}" }
                    nav { class: "md:dropdown-menu md:absolute h-auto md:-mt-64 md:group-hover:mt-0 md:opacity-0 md:group-hover:opacity-100 md:transition-opacity md:duration-250",
                        ul { class: "top-0 w-36 md:bg-white dark:md:bg-gray-800 md:shadow md:px-4 md:py-4 rounded",
                            for (name , link) in links.iter() {
                                Link { target: link.clone(), key: "{name}", li { class: "rounded px-1 py-1 {hover} {hover_bg} text-base md:text-sm",
                                    "{name}"
                                } }
                            }
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! {links})
}

fn Search(cx: Scope) -> Element {
    let show_modal = use_atom_state(cx, SHOW_SEARCH);

    render! {
        div { class: "relative hidden sm:block md:w-full max-w-[40rem] xl:max-w-[40rem] 2xl:max-w-[40rem] mx-auto",
            // Pop up a modal
            button {
                // Pop up a modal
                class: "bg-gray-100 rounded-lg px-3 py-3 w-full text-left text-gray-400 my-auto flex flex-row align-middle justify-between",
                onclick: move |_| {
                    show_modal.set(true);
                },
                div { class: "h-full my-auto flex flex-row align-middle justify-between",
                    MaterialIcon { name: "search", size: 24, color: MaterialIconColor::Dark }
                    span { class: "pl-2", "Search the docs" }
                }
                div { class: "border border-gray-300 rounded-lg p-1 text-xs text-gray-400",
                    "⌘K"
                }
            }
        }
    }
}

fn SearchModal(cx: Scope) -> Element {
    let show_modal = use_atom_state(cx, SHOW_SEARCH);
    let search_text = use_state(cx, String::new);
    let results = crate::docs::LAZY_BOOK.search_index.as_ref().unwrap().search(&search_text.get());

    // when we search, we do a similar search to mdbook
    // This will bring up individual sections that reference the search term with the breadcrumb
    // entries are sorted by breadcrumb

    render! {
        if *show_modal.get() {
            rsx! {
                div {
                    height: "100vh",
                    width: "100vw",
                    class: "fixed top-0 left-0 z-50 hidden md:block bg-gray-500 bg-opacity-50 overflow-y-hidden",
                    onclick: move |_| show_modal.set(false),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            show_modal.set(false);
                        }
                    },

                    // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
                    div { class: "max-w-screen-md mx-auto h-full flex flex-col",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "h-30" }

                        // The actual modal
                        div { class: "bg-white dark:bg-ideblack p-6 rounded-2xl m-8 max-h-[calc(100%-8rem)] overflow-y-auto",
                            // Search input
                            div { class: "flex flex-row flex-grow border-b border-gray-300 pb-4",
                                div { class: "my-auto flex flex-row",
                                    MaterialIcon {
                                        name: "search",
                                        size: 40,
                                        color: MaterialIconColor::Dark,
                                    }
                                    input {
                                        oninput: move |evt| {
                                            search_text.set(evt.value.clone());
                                        },
                                        onmounted: move |evt| {
                                            evt.inner().set_focus(true);
                                        },
                                        class: "flex-grow bg-transparent border-none outline-none text-xl pl-2 text-white",
                                        placeholder: "Search the docs",
                                        value: "{search_text}",
                                    }
                                }
                                div {}
                            }

                            // Results
                            div { class: "overflow-y-auto",
                                ul {
                                    color: "white",
                                    match results {
                                        Ok(results) => {
                                            rsx! {
                                                for result in results {
                                                    SearchResult { result: result }
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
}

#[inline_props]
fn SearchResult(cx: Scope, result: mdbook_shared::search_index::SearchResult) -> Element {
    let set_show_modal = fermi::use_set(cx, SHOW_SEARCH);
    let title = &result.title;
    let page = crate::docs::LAZY_BOOK.get_page(result.id);
    let top_excerpt_segments = &result.excerpts.first().unwrap().text;

    render! {
        li { class: "w-full mt-4 p-2 rounded hover:bg-gray-100 dark:hover:bg-ideblack transition-colors duration-200 ease-in-out",
            Link {
                target: Route::Docs { child: page.url.clone() },
                onclick: move |_| {
                    set_show_modal(false);
                },
                div { class: "flex flex-col justify-between pb-1",
                    h2 { class: "font-semibold dark:text-white", "{title}" }
                }
                p { class: "text-sm text-gray-500 dark:text-gray-300 pr-8",
                    for segment in top_excerpt_segments {
                        if segment.highlighted {
                            rsx! {
                                span { class: "text-blue-500", "{segment.text}" }
                            }
                        }
                        else {
                            rsx! {
                                span { "{segment.text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// div { class: "py-4 px-12 max-w-screen-2xl mx-auto flex items-center justify-between font-semibold text-sm leading-6",
//     // div { class: "py-4 flex items-center justify-between font-semibold text-sm leading-6 bg-white shadow dark:text-gray-200 dark:bg-black px-48",
//     // div { class: "py-4 flex items-center justify-between font-semibold text-sm leading-6 bg-white shadow dark:text-gray-200 dark:bg-black px-4 sm:px-6 md:px-8",
//     Link { class: "flex title-font font-medium items-center text-gray-900", target: "/",
//         img {
//             src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
//             class: "h-5 w-auto"
//         }
//         span { class: "ml-3 text-xl dark:text-white font-mono", "Dioxus Labs" }
//     }
//     Search {}
//     div { class: "flex items-center font-mono",
//         MobileNav {}
//         FullNav {}
//     }
// }
// if *show {rsx! {
//     ul { class: "flex items-center flex-col py-4", gap: "10px", LinkList { } }
// }}

// (
//     "Platforms",
//     "/platforms",
//     &[
//         (
//             "Web",
//             "https://dioxuslabs.com/docs/0.3/guide/en/getting_started/web",
//         ),
//         (
//             "Desktop",
//             "https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop",
//         ),
//         (
//             "Mobile",
//             "https://dioxuslabs.com/docs/0.3/guide/en/getting_started/mobile",
//         ),
//         (
//             "SSR",
//             "https://dioxuslabs.com/docs/0.3/guide/en/getting_started/ssr",
//         ),
//         (
//             "TUI",
//             "https://github.com/DioxusLabs/dioxus/tree/master/packages/dioxus-tui",
//         ),
//     ],
// ),
// (
//     "Projects",
//     "https://github.com/dioxuslabs",
//     &[
//         (
//             "Fermi",
//             "https://github.com/DioxusLabs/dioxus/tree/master/packages/fermi",
//         ),
//         (
//             "Router",
//             "https://github.com/DioxusLabs/dioxus/tree/master/packages/router",
//         ),
//         ("Taffy", "https://github.com/DioxusLabs/taffy"),
//         ("CLI", "https://github.com/DioxusLabs/cli"),
//     ],
// ),
// ("Tutorials", "/tutorials/", &[]),

// &[
//     ("Guide", "https://dioxuslabs.com/docs/0.3/guide/en/"),
//     // ("Advanced", "https://dioxuslabs.com/docs/0.3/reference/"),
//     // ("Reference", "https://dioxuslabs.com/docs/0.3/reference/"),
//     ("Router", "https://dioxuslabs.com/docs/0.3/router/"),
// ],
