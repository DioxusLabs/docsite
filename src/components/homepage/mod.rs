use dioxus::prelude::*;
use dioxus_router::Link;

pub mod call_to_action;
pub mod explainers;
pub mod featured_examples;
pub mod hero;
pub mod snippets;
pub mod value_add;

pub fn Homepage(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "dark:bg-gradient-to-b from-gray-700 to-gray-900",
            hero::Hero {}
            AvailablePlatforms {}
            // ProjectCards {}
            // explainers::Explainers {}
            // snippets::Snippets {}
            featured_examples::FeaturedExamples {}

            // value_add::ValueAdd {}
            crate::components::blog::RecentBlogPosts {}
            call_to_action::CallToAction {}
            // ensure Prism is able to highlight all our code elements
            script { "Prism.highlightAll();" }
        }
    })
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
        "Build faster than ever: your app lives on the server, elminating the need for a dedicated backend API."
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

fn ProjectCards(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "py-12",
            // div { class: "max-w-xl lg:max-w-3xl mx-auto text-center ",
            //     p {
            //         class: "mb-8 text-base leading-relaxed lg:text-xl lg:leading-relaxed text-gray-500",
            //         "Supported Platforms"
            //     }
            // }
            div { class: "container mx-auto px-4 px-6 lg:px-64",
                div { class: "flex flex-wrap -mx-3",
                    CARDS.iter().map(|(title, description)| rsx! {
                        div { class: "w-full md:w-1/2 lg:w-1/3 px-3 mb-6 text-xs dark:text-white", key: "{title}",
                            div { class: "p-6 md:p-8 h-full rounded shadow-white hover:shadow-xl hover:border-transparent cursor-pointer",
                                div {
                                    h3 { class: "mb-4 text-2xl font-semibold font-heading font-mono", "{title}" }
                                    p { class: "text-base text-gray-500 pb-4", "{description}" }
                                    a { class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                                        href: "https://dioxuslabs.com/guide/",
                                        "Get started"
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}

fn AvailablePlatforms(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "pt-36",
            div { class: "container mx-auto max-w-screen-lg",

                div { class: "relative",
                    h1 { class: "text-3xl tracking-tight dark:text-white font-mono text-ghdarkmetal flex flex-row pb-4 mb-4",
                        "One codebase, any platform."
                    }
                    snippets::Snippets {}
                }
                div { class: "ml-24 max-w-screen-lg",
                    div { class: "w-8", div { class: "w-1 mx-auto bg-ghmetal dark:bg-white h-16 relative z-10" } }
                    Platforms {}
                }
            }
        }
    })
}

fn IconSplit(cx: Scope) -> Element {
    cx.render(rsx! {
        svg { class: "mx-auto fill-[#161b22] dark:fill-white",
            version: "1.1",
            view_box: "0 0 24 24",
            width: "24",
            "data-view-component": "true",
            "aria-hidden": "true",
            height: "24",
            path {
                stroke_width: "1.5",
                fill_rule: "evenodd",
                d: "M15.5 11.75a3.5 3.5 0 11-7 0 3.5 3.5 0 017 0zm1.444-.75a5.001 5.001 0 00-9.888 0H2.75a.75.75 0 100 1.5h4.306a5.001 5.001 0 009.888 0h4.306a.75.75 0 100-1.5h-4.306z",
            }
        }
    })
}

// [
//     ("Web with WASM", "Build for the web using Rust and WebAssembly. As fast as SolidJS and more robust than React. Integrated hot reloading for instant iterations."),
//     ("Desktop and Mobile", "Lightweight (<2mb) desktop and mobile apps with zero configuration. Choose between WebView or WGPU-enabled renderers. Runs on macOS, Windows, Linux, iOS, and Android."),
//     ("Commandline Tools", "Quickly convert any commandline tool to a beautiful interactive user interface with just a few lines of code. Runs anywhere with a terminal."),
//     ("Fullstack Apps", "Pre-render on the server, and hydrate on the client. Perfect lighthouse scores and performance over 1000x better than Node and Python. Perfect for static site generation or fullstack apps."),
//     ("LiveView and LiveComponents", "Render your app entirely on the server. Zero backend configuration capable of carring 10s of thousands of actve clients. Integrates with Axum, Actix, Warp, Salvo, and Tokamak." ),
// ]

fn Platforms(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            Platform {
                to: "https://dioxuslabs.com/reference/web",
                name: "Web with WASM",
                content: "Build for the web using Rust and WebAssembly. As fast as SolidJS and more robust than React. Integrated hot reloading for instant iterations."
            }
            Platform {
                to: "https://dioxuslabs.com/reference/desktop",
                name: "Desktop and Mobile",
                content: "Lightweight (<2mb) desktop and mobile apps with zero configuration. Choose between WebView or WGPU-enabled renderers. Runs on macOS, Windows, Linux, iOS, and Android."
            }
            Platform {
                to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/tui",
                name: "Commandline Tools",
                content: "Quickly convert any commandline tool to a beautiful interactive user interface with just a few lines of code. Runs anywhere with a terminal."
            }
            Platform {
                to: "https://dioxuslabs.com/reference/ssr",
                name: "Fullstack Apps",
                content: "Pre-render on the server, and hydrate on the client. Perfect lighthouse scores and performance over 1000x better than Node and Python. Perfect for static site generation or fullstack apps."
            }
            Platform {
                to: "https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview",
                name: "LiveView and LiveComponents",
                content: "Render your app entirely on the server. Zero backend configuration capable of carring 10s of thousands of actve clients. Integrates with Axum, Actix, Warp, Salvo, and Tokamak.",
                last: true
            }
        }
    })
}

// ("Web", "https://dioxuslabs.com/reference/web"),
// ("Desktop", "https://dioxuslabs.com/reference/desktop"),
// ("Mobile", "https://dioxuslabs.com/reference/mobile"),
// ("SSR", "https://dioxuslabs.com/reference/ssr"),
// ("TUI", "https://github.com/dioxusLabs/rink"),

#[inline_props]
fn Platform<'a>(
    cx: Scope<'a>,
    name: &'static str,
    content: &'static str,
    children: Element<'a>,
    to: &'static str,
    last: Option<bool>,
) -> Element {
    let last = last.unwrap_or_default();

    cx.render(rsx! {
        li { class: "text-lg text-gray-600 dark:text-gray-600 flex flex-row",
            div { class: "w-8",
                div { class: "flex flex-col h-full mx-auto",
                    if !last {
                        rsx! { div { class: "bg-ghmetal dark:bg-white w-1 h-12 mx-auto" } }
                    } else {
                        rsx! { div { class: "bg-ghmetal dark:bg-white w-1 h-8 mx-auto" } }
                    }
                    div { class:"mx-auto w-full", IconSplit {} }
                    if !last {
                        Some(rsx!{ div { class: "bg-ghmetal dark:bg-white w-1 h-full mx-auto" } })
                    } else {
                        None
                    }
                }
            }

            Link { class: "min-w-lg mb-12 p-8 rounded max-w-screen-md hover:shadow-pop rounded-lg", to: to,
            // div { class: "min-w-lg p-8 m-8 bg-slate-800 dark:bg-slate-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 rounded shadow-xl",
                h2 { class: "text-2xl text-gray-800 font-semibold font-mono pb-2 dark:text-gray-100 ", *name }
                p { class: "text-md text-gray-500 dark:text-gray-400", *content },
                children
            }
        }
    })
}
