#![allow(non_upper_case_globals)]

use dioxus::{prelude::*, router::use_router};

pub mod sitemap;
pub mod components {
    pub mod homepage {
        pub mod call_to_action;
        pub mod featured_examples;
        pub mod hero;
        pub mod recent_blog_posts;
        pub mod snippets;
        pub mod value_add;
    }
    pub mod blog;
    pub mod footer;
    pub mod hero;
    pub mod navbar;
    pub mod snippets;
}

pub mod icons;

#[derive(PartialEq, Clone, Debug)]
enum AppRoute {
    Home,
    Blog,
    NotFound,
}

pub static App: Component<()> = |cx| {
    // let route = use_router(&cx, |s| match s {
    //     "blog" => AppRoute::Blog,
    //     _ => AppRoute::Home,
    // });

    cx.render(rsx! {
        style {
            {[include_str!("../tailwind.css")]}
        }
        script {
            {[include_str!("./darktheme.js")]}
        }
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
        // link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
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

        div { class: "dark:bg-gray-800"
            div { class: "px-4 sm:px-6 md:px-8 pb-12",
                // whatever this is
                // whatever()
                nav_header()
                flashy_hero()
            }
    
    
            div { class: "container flex flex-col md:flex-row px-24 mx-auto py-12",
                div { class: "flex flex-col flex-shrink-0 ml-auto mr-2",
                // div { class: "flex flex-col flex-shrink-0 mr-auto ml-2",
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
            components::homepage::recent_blog_posts::RecentBlogPosts {}
            components::homepage::call_to_action::CallToAction {}
            components::footer::Footer {}
        }




        // ensure Prism is able to highlight all our code elements
        script { "Prism.highlightAll();" }

        script { {[include_str!("./components/prism/prism.js")]} }
    })
};

fn whatever(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        div { class: "absolute inset-0 bottom-10 bg-bottom bg-no-repeat bg-gray-50 dark:bg-[#0B1120] index_beams__3fKa4",
            div { class: "absolute inset-0 bg-grid-gray-900/[0.04] bg-[bottom_1px_center] dark:bg-grid-gray-400/[0.05] dark:bg-bottom dark:border-b dark:border-gray-100/5",
                style: "mask-image:linear-gradient(to bottom, transparent, black);-webkit-mask-image:linear-gradient(to bottom, transparent, black)",
            }
        }
    })
}

fn flashy_hero(cx: Scope<()>) -> Element {
    cx.render(rsx!(
        div { class: "relative max-w-5xl mx-auto pt-20 sm:pt-24 lg:pt-32",
            h1 { class: "text-gray-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white",
                // "Rapidly build modern websites without ever leaving your HTML."
                "Build reliable user interfaces."
            }
            p { class: "mt-6 text-lg text-gray-600 text-center max-w-3xl mx-auto dark:text-gray-400",
                span { class: "text-red-400", "Dioxus "}
                "is a React-like library for building fast, portable, and beautiful user interfaces with Rust."
                " Runs on the web, desktop, mobile, and more."

                // "A utility-first CSS framework packed with classes like"
                // code { class: "font-mono font-medium text-sky-500 dark:text-sky-400", "flex" }
                // ","
                // code { class: "font-mono font-medium text-sky-500 dark:text-sky-400", "pt-4" }
                // ","
                // code { class: "font-mono font-medium text-sky-500 dark:text-sky-400", "text-center" }
                // "and"
                // code { class: "font-mono font-medium text-sky-500 dark:text-sky-400", "rotate-90" }
                // "that can be composed to build any design, directly in your markup."
            }
            div { class: "mt-6 sm:mt-10 flex justify-center space-x-6 text-sm",
                a { class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                    href: "/docs/installation",
                    "Get started"
                }
                button { class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl flex items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200",
                    "type": "button",
                    span { class: "text-gray-900",
                        span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
                        span { class: "text-red-400" "cargo " }
                        "add dioxus"
                    }
                    span { class: "sr-only", "(click to copy to clipboard)" }
                    icons::Copy {}
                }

                // todo: add search
                // button { class: "hidden sm:flex items-center w-72 text-left space-x-3 px-4 h-12 bg-white ring-1 ring-gray-900/10 hover:ring-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 shadow-sm rounded-lg text-gray-400 dark:bg-gray-800 dark:ring-0 dark:text-gray-300 dark:highlight-white/5 dark:hover:bg-gray-700",
                //     "type": "button",
                //     svg { class: "flex-none text-gray-300 dark:text-gray-400",
                //         fill: "none",
                //         "stroke-width": "2",
                //         "stroke-linejoin": "round",
                //         stroke: "currentColor",
                //         "stroke-linecap": "round",
                //         "aria-hidden": "true",
                //         width: "24",
                //         height: "24",
                //         path { d: "m19 19-3.5-3.5", }
                //         circle { cy: "11", r: "6", cx: "11", }
                //     }
                //     span { class: "flex-auto", "Quick search..." }
                //     kbd { class: "font-sans font-semibold dark:text-gray-500",
                //         abbr { class: "no-underline text-gray-300 dark:text-gray-500", title: "Command", "⌘" }
                //         "K"
                //     }
                // }
            }
        }
    ))
}

fn nav_header(cx: Scope<()>) -> Element {
    cx.render(rsx!(
        div { class: "relative pt-6 lg:pt-8 flex items-center justify-between text-gray-700 font-semibold text-sm leading-6 dark:text-gray-200 dark:bg-gray-900",
            a { class: "flex title-font font-medium items-center",
                img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
                span { class: "ml-3 text-4xl text-gray-900", "dioxus" }
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
                    nav_words()
                    div { class: "flex items-center border-l border-gray-200 ml-6 pl-6 dark:border-gray-800",
                        label { class: "sr-only", id: "headlessui-listbox-label-2", "Theme" }
                        button {
                            id: "headlessui-listbox-button-3",
                            "aria-expanded": "false",
                            "type": "button",
                            "aria-labelledby": "headlessui-listbox-label-2 headlessui-listbox-button-3",
                            "aria-haspopup": "true",
                            span { class: "dark:hidden",
                                svg { class: "w-6 h-6",
                                    "stroke-width": "2",
                                    "viewBox": "0 0 24 24",
                                    fill: "none",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    path { class: "stroke-gray-400 dark:stroke-gray-500",
                                        d: "M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z",
                                    }
                                    path { class: "stroke-gray-400 dark:stroke-gray-500",
                                        d: "M12 4v1M17.66 6.344l-.828.828M20.005 12.004h-1M17.66 17.664l-.828-.828M12 20.01V19M6.34 17.664l.835-.836M3.995 12.004h1.01M6 6l.835.836",
                                    }
                                }
                            }
                            span { class: "hidden dark:inline",
                                svg { class: "w-6 h-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    path { class: "fill-transparent",
                                        "clip-rule": "evenodd",
                                        d: "M17.715 15.15A6.5 6.5 0 0 1 9 6.035C6.106 6.922 4 9.645 4 12.867c0 3.94 3.153 7.136 7.042 7.136 3.101 0 5.734-2.032 6.673-4.853Z",
                                        "fill-rule": "evenodd",
                                    }
                                    path { class: "fill-gray-400 dark:fill-gray-500",
                                        d: "m17.715 15.15.95.316a1 1 0 0 0-1.445-1.185l.495.869ZM9 6.035l.846.534a1 1 0 0 0-1.14-1.49L9 6.035Zm8.221 8.246a5.47 5.47 0 0 1-2.72.718v2a7.47 7.47 0 0 0 3.71-.98l-.99-1.738Zm-2.72.718A5.5 5.5 0 0 1 9 9.5H7a7.5 7.5 0 0 0 7.5 7.5v-2ZM9 9.5c0-1.079.31-2.082.845-2.93L8.153 5.5A7.47 7.47 0 0 0 7 9.5h2Zm-4 3.368C5 10.089 6.815 7.75 9.292 6.99L8.706 5.08C5.397 6.094 3 9.201 3 12.867h2Zm6.042 6.136C7.718 19.003 5 16.268 5 12.867H3c0 4.48 3.588 8.136 8.042 8.136v-2Zm5.725-4.17c-.81 2.433-3.074 4.17-5.725 4.17v2c3.552 0 6.553-2.327 7.622-5.537l-1.897-.632Z",
                                    }
                                    path { class: "fill-gray-400 dark:fill-gray-500",
                                        "fill-rule": "evenodd",
                                        d: "M17 3a1 1 0 0 1 1 1 2 2 0 0 0 2 2 1 1 0 1 1 0 2 2 2 0 0 0-2 2 1 1 0 1 1-2 0 2 2 0 0 0-2-2 1 1 0 1 1 0-2 2 2 0 0 0 2-2 1 1 0 0 1 1-1Z",
                                        "clip-rule": "evenodd",
                                    }
                                }
                            }
                        }
                        a { class: "ml-6 block text-gray-400 hover:text-gray-500 dark:hover:text-gray-300",
                            href: "https://github.com/tailwindlabs/tailwindcss",
                            span { class: "sr-only", "Tailwind CSS on GitHub" }
                            svg { class: "w-5 h-5",
                                "viewBox": "0 0 16 16",
                                "aria-hidden": "true",
                                fill: "currentColor",
                                path {
                                    d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

fn nav_words(cx: Scope<()>) -> Element {
    cx.render(rsx!(
        nav {
            ul { class: "flex items-center space-x-8",
                li {
                    a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                        href: "/docs/installation",
                        "Docs"
                    }
                }
                li {
                    a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                        href: "https://tailwindui.com",
                        "Components"
                    }
                }
                li {
                    a { class: "hover:text-sky-500 dark:hover:text-sky-400",
                        href: "/blog",
                        "Blog"
                    }
                }
            }
        }
    ))
}

pub static InteractiveHeader: Component<()> = |cx| {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!{
        div { class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 mr-auto ml-2" background_color: "hsl(220, 13%, 18%)"
            div { class: "pb-3 text-white"
                h1 { "High-Five counter: {count}" }
            }
            div { class: "flex flex-col items-center"
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count += 1, "Up high!"
                }
                img { class: "h-12 mx-4 my-4" src: "https://i.imgur.com/aK3dWXs.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count -= 1, "Down low!"
                }
            }
        }
    })
};

// div { class: "text-gray-500 antialiased bg-white js-focus-visible"
//     div { class: "space-y-20 sm:space-y-32 md:space-y-40 lg:space-y-44 overflow-hidden"
//         header { class: "relative z-10 max-w-screen-lg xl:max-w-screen-xl mx-auto"
//             div { class: "px-4 sm:px-6 md:px-8 mb-14 sm:mb-20 xl:mb-8",

//                 div { class: "border-b border-gray-200 py-4 flex items-center justify-between mb-4 -mx-4 px-4 sm:mx-0 sm:px-0",
//                     button { class: "group leading-6 font-medium flex items-center space-x-3 sm:space-x-4 hover:text-gray-600 transition-colors duration-200 w-full py-2",
//                         r#"type": "button",
//                         icons::Search {}
//                         span {  "Quick search" span { class: "hidden sm:inline", " for anything" } }
//                         span { class: "hidden sm:block text-gray-400 text-sm leading-5 py-0.5 px-1.5 border border-gray-300 rounded-md",
//                             style: "opacity: 1;",
//                             span { class: "sr-only", "Press" }
//                             kbd { class: "font-sans", abbr { class: "no-underline", title: "Command", "⌘" } }
//                             span { class: "sr-only", "and" }
//                             kbd { class: "font-sans", "K" }
//                             span { class: "sr-only", "to search" }
//                         }
//                     }
//                     div { class: "flex items-center space-x-6 sm:space-x-10 ml-6 sm:ml-10",
//                         a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
//                             href: "/docs",
//                             span { class: "sm:hidden", "Docs" }
//                             span { class: "hidden sm:inline", "Book" }
//                         }
//                         a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
//                             href: "/docs",
//                             span { class: "sm:hidden", "Docs" }
//                             span { class: "hidden sm:inline", "Reference" }
//                         }
//                         a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
//                             href: "/docs",
//                             span { class: "sm:hidden", "Docs" }
//                             span { class: "hidden sm:inline", "docs.rs" }
//                         }
//                         a { class: "text-gray-400 hover:text-gray-500 transition-colors duration-200",
//                             href: "https://github.com/dioxuslabs/dioxus",
//                             span { class: "sr-only", "Tailwind CSS on GitHub" }
//                             icons::GithubLogo {}
//                         }
//                     }
//                 }
//                 // icons::TailwindLogo {}
//                 a { class: "flex title-font font-medium items-center mt-24",
//                     img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
//                     span { class: "ml-3 text-4xl text-gray-900", "dioxus" }
//                 }
//                 h1 { class: "text-4xl sm:text-6xl lg:text-7xl leading-none font-extrabold text-gray-900 mt-8 mb-8 sm:mt-14 sm:mb-10",
//                     "Build reliable user interfaces."
//                     // span { class: "text-red-400", "Rust"}
//                 }
//                 p { class: "max-w-screen-lg text-lg sm:text-2xl sm:leading-10 font-medium mb-10 sm:mb-11",
//                     // "Feels like React."
//                     span { class: "text-red-400", "Dioxus "}
//                     "is a React-like library for building fast, portable, and beautiful user interfaces with Rust."

//                     " Runs on the web, desktop, mobile, and more."
//                     // "A React-like library for building fast, portable, and beautiful user interfaces with Rust that run on the web, desktop, mobile, and more."
//                     // "If it compiles, it works."
//                     // " Runs blazingly fast on the web, desktop, mobile, and more."
//                 }
//                 // p { class: "max-w-screen-lg text-lg sm:text-2xl sm:leading-10 font-medium mb-10 sm:mb-11",
//                 //     "If it compiles, it works."
//                 // }
//                 div { class: "flex flex-wrap space-y-4 sm:space-y-0 sm:space-x-4 text-center",
//                     a { class: "w-full sm:w-auto flex-none bg-gray-900 hover:bg-gray-700 text-white text-lg leading-6 font-semibold py-3 px-6 border border-transparent rounded-xl focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-900 focus:outline-none transition-colors duration-200",
//                         href: "/docs",
//                         "Get started"
//                     }
//                     button { class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl flex items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200",
//                         r#"type": "button",
//                         span { class: "text-gray-900",
//                             span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
//                             span { class: "text-red-400" "cargo " }
//                             "add dioxus"
//                         }
//                         span { class: "sr-only", "(click to copy to clipboard)" }
//                         icons::Copy {}
//                     }
//                 }
//             }
//         }
//     }
// }
