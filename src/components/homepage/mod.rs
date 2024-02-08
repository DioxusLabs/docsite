use crate::*;
use dioxus::prelude::*;

pub mod call_to_action;
pub mod explainers;
pub mod featured_examples;
pub mod hero;
pub mod snippets;
pub mod value_add;

#[component]
pub fn Homepage() -> Element {
    rsx! {
        div {
            section { class: "w-full dark:bg-ideblack",
                hero::Hero {}
                AvailablePlatforms {}
            }
            featured_examples::FeaturedExamples {}
            crate::components::blog::BlogList {}
            Stats {}
        }
        call_to_action::CallToAction {}
    }
}

const CARDS: &[(&str, &str)] = &[
    (
        "Web",
        "Progressively render directly to the DOM, enhanced with SSR, rehydration, and streaming.",
    ),
    (
        "Desktop",
        "Render your apps natively with WGPU or through WebView with full access to system resources.",
    ),
    (
        "Mobile",
        "Compile natively to mobile architectures. Apps stay fast even on low-end devices.",
    ),
    (
        "LiveView",
        "Build faster than ever: your app lives on the server, eliminating the need for a dedicated backend API."
    ),
    (
        "TUI",
        "Quickly convert scripts to interactive tools with a full fledged renderer for the terminal.",
    ),
    (
        "3D Scenes",
        "Dioxus is extensible. Quickly build highly interactive 3D scenes, inspired by React-Three-Fiber.",
    ),
];

fn ProjectCards() -> Element {
    rsx! {
        section { class: "py-12",
            div { class: "container mx-auto px-6 lg:px-64",
                div { class: "flex flex-wrap -mx-3",
                    for (title , description) in CARDS.iter() {
                        div { key: "{title}", class: "w-full md:w-1/2 lg:w-1/3 px-3 mb-6 text-xs dark:text-white",
                            div { class: "p-6 md:p-8 h-full rounded shadow-white hover:shadow-xl hover:border-transparent cursor-pointer",
                                div {
                                    h3 { class: "mb-4 text-2xl font-semibold font-heading font-sans",
                                        "{title}"
                                    }
                                    p { class: "text-base text-gray-500 pb-4", "{description}" }
                                    Link {
                                        class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                                        to: Route::Docs {
                                            child: BookRoute::GettingStartedIndex {},
                                        },
                                        "Get started"
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

fn AvailablePlatforms() -> Element {
    rsx! {
        section { class: "w-full dark:bg-ideblack",
            div { class: "container mx-auto max-w-screen-lg",
                div { class: "relative overflow-x-hidden",
                    div { class: "flex flex-col items-center justify-center text-center max-w-screen-lg mx-auto pb-4",
                        h1 { class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal pb-4 mb-4 ",
                            "One codebase, every platform."
                        }
                        p { class: "text-xl text-gray-600 dark:text-gray-400 pb-4 max-w-screen-sm",
                            "Dioxus is a React-inspired library for Rust focused on developer experience. Build fast, beautiful, and fully-featured apps for every platform in less time."
                        }
                    }
                    snippets::Snippets {}
                }
            }
            div { class: "max-w-screen-lg mx-auto pb-8 px-2 md:px-16 dark:text-white",
                // div { class: "max-w-screen-xl mx-auto pb-64 px-16 dark:text-white",
                TriShow {
                    left: None,
                    center: None,
                    right: rsx!(
                        "Build for the web using Rust and WebAssembly. As fast as SolidJS and more robust than React. Integrated hot reloading for instant iterations."
                    ),
                    to: Route::Docs {
                        child: BookRoute::GettingStartedWasm {},
                    },
                    title: "Web with WASM"
                }
                TriShow {
                    left: None,
                    center: None,
                    right: rsx!(
                        "Lightweight (<2mb) desktop and mobile apps with zero configuration. Choose between WebView or WGPU-enabled renderers. Runs on macOS, Windows, Linux, iOS, and Android."
                    ),
                    to: Route::Docs {
                        child: BookRoute::GettingStartedDesktop {
                        },
                    },
                    title: "Desktop and Mobile"
                }
                TriShow {
                    to: Route::Docs {
                        child: BookRoute::GettingStartedTui {},
                    },
                    title: "Terminal User Interfaces",
                    right: rsx!(
                        "Quickly convert any CLI tool to a beautiful interactive user interface with just a few lines of code. Runs anywhere with a terminal."
                    ),
                    left: None,
                    center: None
                }
                TriShow {
                    to: Route::Docs {
                        child: BookRoute::GettingStartedFullstack {
                        },
                    },
                    title: "Fullstack Apps",
                    right: rsx!(
                        "Pre-render on the server, and hydrate on the client. Perfect lighthouse scores and performance over 1000x better than Node and Python. Perfect for static site generation or fullstack apps."
                    ),
                    left: None,
                    center: None
                }
                TriShow {
                    to: Route::Docs {
                        child: BookRoute::GettingStartedLiveview {
                        },
                    },
                    title: "LiveView and LiveComponents",
                    right: rsx!(
                        "Render your app entirely on the server. Zero backend configuration capable of handling thousands of active clients. Integrates with Axum, Warp, Salvo, and Tokamak.",
                    ),
                    left: None,
                    center: None,
                    last: true
                }
            }
        }
    }
}

#[component]
fn TriShow(
    left: Element,
    center: Element,
    right: Element,
    title: &'static str,
    to: Route,
    last: Option<bool>,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-row justify-center max-w-screen-lg",
            // div { class: "grow basis-0", left }
            TriPadding { last: last.unwrap_or_default(), {center} }
            div { class: "grow basis-0",
                Link { to: to.clone(),
                    div { class: "min-w-lg max-w-screen-md hover:shadow-pop rounded-lg p-8",
                        h2 { class: "text-2xl text-gray-800 font-semibold pb-2 dark:text-gray-100 ",
                            "{title}"
                        }
                        {right}
                    }
                }
            }
        }
    }
}

#[component]
fn TriPadding(children: Element, last: bool) -> Element {
    rsx!(
        div { class: "flex flex-col items-center",
            div { class: "w-0 h-10 border-dashed border border-[#444]" }
            IconSplit {}

            if !last {
                div { class: "w-0 h-full border-dashed border border-[#444]", {children} }
            }
        }
    )
}

#[component]
fn DeveloperExperience() -> Element {
    rsx! (
        section { class: "pt-36 w-full dark:bg-ideblack dark:text-white",
            div { class: "container mx-auto max-w-screen-2xl",
                div { class: "relative",
                    div { class: "flex flex-col max-w-screen-lg mx-auto pb-20",
                        h1 { class: "text-[3.3em] font-bold tracking-tight items-center justify-center text-center dark:text-white text-ghdarkmetal pb-4 mb-4 ",
                            "Redefining developer experience."
                        }
                        div { class: "flex flex-row",
                            p { class: "text-xl text-gray-600 dark:text-gray-400 pb-4 max-w-screen-sm w-1/2",
                                "Dioxus is a React-inspired library for Rust that empowers you to quickly build fast, beautiful, and fully-featured apps for every platform."
                            }
                            p { class: "text-xl text-gray-600 dark:text-gray-400 pb-4 max-w-screen-sm w-1/2",
                                "Dioxus is a React-inspired library for Rust that empowers you to quickly build fast, beautiful, and fully-featured apps for every platform."
                            }
                        }
                    }
                    div { class: "max-w-screen-2xl mx-auto flex flex-row",
                        div { class: "w-1/2" }
                        div { class: "w-1/2",
                            ExperienceText {
                                title: "Integrated Devtools",
                                content: "Hot reloading for instant iteration, automatic code formatting, convert HTML to RSX, and more."
                            }
                            ExperienceText {
                                title: "Minimal configuration",
                                content: "Start projects with `cargo new`. No build scripts or configuration required for development."
                            }
                            ExperienceText {
                                title: "",
                                content: "Strong typing with no runtime overhead. Automatically derive props, forms, API clients, and more."
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ExperienceText(title: &'static str, content: &'static str) -> Element {
    rsx!(
        div { class: "pb-12",
            h3 { class: "text-2xl text-gray-800 font-semibold pb-2 dark:text-gray-100 ",
                "{title}"
            }
            p { "{content}" }
        }
    )
}

fn IconSplit() -> Element {
    rsx! {
        svg {
            class: "mx-auto fill-[#444] dark:fill-white",
            version: "1.1",
            view_box: "0 0 24 24",
            width: "24",
            "data-view-component": "true",
            "aria-hidden": "true",
            height: "24",
            path {
                stroke_width: "1.5",
                fill_rule: "evenodd",
                d: "M15.5 11.75a3.5 3.5 0 11-7 0 3.5 3.5 0 017 0zm1.444-.75a5.001 5.001 0 00-9.888 0H2.75a.75.75 0 100 1.5h4.306a5.001 5.001 0 009.888 0h4.306a.75.75 0 100-1.5h-4.306z"
            }
        }
    }
}

fn Stats() -> Element {
    rsx! {
        section { class: "pb-24 w-full dark:bg-ideblack",
            div { class: "container mx-auto max-w-screen-lg",
                div { class: "relative ",
                    div { class: "flex flex-col items-center justify-center text-center max-w-screen-lg mx-auto pb-4",
                        // span { class: "text-xl text-blue-300", "Portable" }
                        h1 { class: "text-[3.3em] font-bold tracking-tight dark:text-white text-ghdarkmetal pb-4 mb-4 ",
                            "A vibrant, active community."
                        }
                        p { class: "text-xl text-gray-600 dark:text-gray-400 pb-4 max-w-screen-sm",
                            "Driven by a large, active, and welcoming community."
                        }
                    }
                }
            }
            div { class: "w-full mx-auto dark:bg-[#111111] border-t border-b mb-12",
                div { class: "grid grid-cols-2 grid-rows-2 sm:grid-cols-4 sm:grid-rows-1",
                    StatsItem { major: "10k", minor: "Stars" }
                    StatsItem { major: "63k", minor: "Downloads" }
                    StatsItem { major: "136", minor: "Contributors" }
                    StatsItem { major: "873", minor: "Community Projects" }
                }
            }

            a { href: "https://github.com/dioxuslabs/dioxus/graphs/contributors",
                img {
                    src: "https://contrib.rocks/image?repo=dioxuslabs/dioxus&max=52&columns=13",
                    class: "mx-auto pb-12",
                    alt: "Dioxus Contributors"
                }
            }
        }
    }
}

#[component]
fn StatsItem(major: &'static str, minor: &'static str) -> Element {
    rsx! {
        div { class: "text-center py-6 border border-[#444]",
            div { class: "text-5xl font-bold text-gray-800 dark:text-gray-100", {major} }
            div { class: "text-xl text-gray-600 dark:text-gray-400", {minor} }
        }
    }
}

#[component]
fn Platform(
    name: &'static str,
    content: &'static str,
    children: Element,
    to: Route,
    last: Option<bool>,
) -> Element {
    let last = last.unwrap_or_default();

    rsx! {
        li { class: "text-lg text-gray-600 dark:text-gray-600 flex flex-row",
            div { class: "w-8",
                div { class: "flex flex-col h-full mx-auto",
                    if !last {
                        div { class: "bg-ghmetal dark:bg-white w-1 h-12 mx-auto" }
                    } else {
                        div { class: "bg-ghmetal dark:bg-white w-1 h-8 mx-auto" }
                    }
                    div { class: "mx-auto w-full", IconSplit {} }
                    if !last {
                        div { class: "bg-ghmetal dark:bg-white w-1 h-full mx-auto" }
                    }
                }
            }

            Link {
                class: "min-w-lg mb-12 p-8 max-w-screen-md hover:shadow-pop rounded-lg",
                to: to.clone(),
                // div { class: "min-w-lg p-8 m-8 bg-slate-800 dark:bg-slate-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 rounded shadow-xl",
                h2 { class: "text-2xl text-gray-800 font-semibold font-mono pb-2 dark:text-gray-100 ",
                    {name}
                }
                p { class: "text-md text-gray-500 dark:text-gray-400", {content} }
                {children}
            }
        }
    }
}

fn JumpStart() -> Element {
    rsx! {
        section { class: "pt-36 w-full dark:bg-ideblack",
            div { class: "container mx-auto max-w-screen-lg",
                div { class: "relative ",
                    div { class: "flex flex-col items-center justify-center text-center max-w-screen-lg mx-auto pb-20",
                        // span { class: "text-xl text-blue-300", "Portable" }
                        h1 { class: "text-[3.3em] font-bold tracking-tight dark:text-white font-mono text-ghdarkmetal pb-4 mb-4 ",
                            "Get Started in Seconds"
                        }
                        p { class: "text-xl text-gray-600 dark:text-gray-400 pb-4 max-w-screen-sm",
                            "Driven by a large, active, and welcoming community, Dioxus is just getting started."
                        }
                    }
                }
            }
            div { class: "w-full mx-auto dark:bg-[#111111] border-t border-b mb-12",
                div { class: "grid grid-cols-2 grid-rows-2 sm:grid-cols-4 sm:grid-rows-1",
                    StatsItem { major: "10k", minor: "Stars" }
                    StatsItem { major: "63k", minor: "Downloads" }
                    StatsItem { major: "136", minor: "Contributors" }
                    StatsItem { major: "873", minor: "Community Projects" }
                }
            }
        }
    }
}
