#![allow(non_upper_case_globals)]

use dioxus::{
    prelude::*,
    router::{use_router, Link},
};
use serde::{Deserialize, Serialize};

pub mod icons;
pub mod sitemap;
pub mod components {
    pub mod homepage {
        pub mod call_to_action;
        pub mod featured_examples;
        pub mod hero;
        pub mod snippets;
        pub mod value_add;
    }
    pub mod blog;
    pub mod footer;
    pub mod hero;
    pub mod navbar;
    pub mod snippets;
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum AppRoute {
    Home,
    Blog,
    BlogPost { id: usize },
}
impl Default for AppRoute {
    fn default() -> Self {
        AppRoute::Home
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    dioxus::web::launch(App)
}

pub static App: Component<()> = |cx| {
    let route: &AppRoute = use_router(&cx, |c| {});

    // in deubg mode we want to bring in the dev mode of tailwind, but generate the propduction mode for release
    let remote_css = match cfg!(debug_assertions) {
        true => Some(rsx!(link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        })),
        false => None,
    };

    cx.render(rsx! {
        style { {[include_str!("../tailwind.css")]} }
        script { {[include_str!("./darktheme.js")]} }
        {remote_css}
        link { href: "https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.0.0/github-markdown-light.min.css", rel: "stylesheet" }
        style { {[include_str!("./components/prism/prism.css")]} }
        style { {[r#"
            .markdown-body {
                box-sizing: border-box;
                min-width: 200px;
                max-width: 980px;
                margin: 0 auto;
                padding: 45px;
                list_style: disc;
            }
            @media (max-width: 767px) {
                .markdown-body {
                    padding: 15px;
                }
            }
        "#]} }

        nav_header()
        {match route {
            AppRoute::Home => rsx!(home()),
            AppRoute::Blog => rsx!(components::blog::BlogList {}),
            AppRoute::BlogPost{ id } => rsx!(components::blog::single_blog_post( id: *id )),
        }}



        script { {[include_str!("./components/prism/prism.js")]} }
    })
};
fn home(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "dark:bg-gray-800"
            div { class: "relative max-w-5xl mx-auto pt-20 sm:pt-24 lg:pt-32 text-gray-600",
                h1 { class: "font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white",
                    "Build reliable user interfaces that run "
                    pre {
                        class: "text-transparent text-8xl bg-clip-text pb-3 bg-gradient-to-r from-red-400 via-purple-300 to-blue-500"
                        "anywhere"
                    }
                }
                p { class: "dark:text-white mt-6 text-lg text-center max-w-3xl mx-auto",
                    "Introducing "
                    span { class: "text-red-400", "Dioxus"}
                    ": a React-like library for building fast, portable, and beautiful user interfaces with Rust."
                    " Runs on the web, desktop, mobile, and more."
                }
                div { class: "mt-6 sm:mt-10 flex justify-center space-x-6 text-sm",
                    a { class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                        href: "https://dioxuslabs.com/guide/",
                        "Get started"
                    }

                    button { class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl flex items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200 hidden md:flex",
                        // funny that we can just default to some javascript like this
                        // might want to do the same thing in rust so we can display a selected state
                        "onclick": "navigator.clipboard.writeText(\"cargo add dioxus\")"
                        "type": "button",
                        span { class: "text-gray-900",
                            span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
                            span { class: "text-red-400" "cargo " }
                            "add dioxus"
                        }
                        span { class: "sr-only", "(click to copy to clipboard)" }
                        icons::Copy {}
                    }
                }
            }

            div { class: "container flex flex-col md:flex-row md:px-24 md:py-12 mx-auto",
                div { class: "flex flex-col flex-shrink-0 ml-auto mr-2",
                    div { class: "pt-4"
                        pre {
                            code { class: "language-rust line-numbers", {[include_str!("../snippets/homepage.rs")]} }
                        }
                    }
                }
                InteractiveHeader {}
            }

            components::homepage::value_add::ValueAdd {}
            components::homepage::featured_examples::FeaturedExamples {}
            components::homepage::snippets::Snippets {}
            components::blog::RecentBlogPosts {}
            components::homepage::call_to_action::CallToAction {}
            components::footer::Footer {}

            // ensure Prism is able to highlight all our code elements
            script { "Prism.highlightAll();" }
        }
    ))
}

fn whatever(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "absolute inset-0 bottom-10 bg-bottom bg-no-repeat bg-gray-50 dark:bg-[#0B1120] index_beams__3fKa4",
            div { class: "absolute inset-0 bg-grid-gray-900/[0.04] bg-[bottom_1px_center] dark:bg-grid-gray-400/[0.05] dark:bg-bottom dark:border-b dark:border-gray-100/5",
                style: "mask-image:linear-gradient(to bottom, transparent, black);-webkit-mask-image:linear-gradient(to bottom, transparent, black)",
            }
        }
    })
}

fn nav_header(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "relative pt-6 lg:pt-8 pb-4 flex items-center justify-between font-semibold text-sm leading-6 dark:text-gray-200 dark:bg-gray-900 px-4 sm:px-6 md:px-8",
            Link {
                class: "flex title-font font-medium items-center text-gray-900"
                to: AppRoute::Home,
                img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
                span { class: "ml-3 text-4xl dark:text-white", "dioxus" }
            }
            div { class: "flex items-center",
                button { class: "text-gray-500 hover:text-gray-600 w-8 h-8 -my-1 flex items-center justify-center md:hidden dark:hover:text-gray-300",
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
                        path { d: "m19 19-3.5-3.5", }
                        circle {
                            cx: "11",
                            cy: "11",
                            r: "6",
                        }
                    }
                }
                div { class: "-my-1 ml-2 -mr-1 md:hidden",
                    button { class: "text-gray-500 w-8 h-8 flex items-center justify-center hover:text-gray-600 dark:text-gray-400 dark:hover:text-gray-300",
                        "type": "button",
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
                                d: "M12 6v.01M12 12v.01M12 18v.01M12 7a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm0 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Zm0 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z",
                            }
                        }
                    }
                }
                div { class: "hidden md:flex items-center",
                    nav {
                        ul { class: "flex items-center space-x-8",
                            li {
                                Link {
                                    class: "hover:text-sky-500 dark:hover:text-sky-400"
                                    href: "/",
                                    to: AppRoute::Home,
                                    "Home"
                                }
                            }
                            li {
                                a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                                    href: "https://github.com/DioxusLabs/awesome-dioxus#community",
                                    "Community"
                                }
                            }
                            li {
                                a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                                    href: "https://dioxuslabs.com/guide/",
                                    "Guide"
                                }
                            }
                            li {
                                a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                                    href: "https://github.com/DioxusLabs/dioxus/tree/master/examples/core_reference",
                                    "Reference"
                                }
                            }
                            li {
                                Link { class: "hover:text-sky-500 dark:hover:text-sky-400"
                                    href: "/blog",
                                    to: AppRoute::Blog,
                                    "Blog"
                                }
                            }
                            li {
                                a { class: "dark:hover:text-sky-400 p-2 rounded bg-gray-600 hover:bg-gray-300 text-white",
                                    href: "https://dioxuslabs.com/guide/",
                                    "Get Started"
                                }
                            }
                        }
                    }
                    div { class: "flex items-center border-l border-gray-200 ml-6 pl-6 dark:border-gray-800",
                        label { class: "sr-only", id: "headlessui-listbox-label-2", "Theme" }
                        a { class: "ml-3 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                            href: "https://github.com/dioxuslabs/dioxus",
                            span { class: "sr-only", "Dioxus on GitHub" }
                            crate::icons::github2()
                        }
                    }
                }
            }
        }
    ))
}

pub static InteractiveHeader: Component<()> = |cx| {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!{
        div { class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 mr-auto hidden lg:block lg:ml-2" background_color: "hsl(220, 13%, 18%)"
            div { class: "pb-3 text-white"
                h1 { "High-Five counter: {count}" }
            }
            div { class: "flex flex-col items-center"
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count += 1, "Up high!"
                }
                img { class: "h-12 mx-4 my-4" src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count -= 1, "Down low!"
                }
            }
        }
    })
};
