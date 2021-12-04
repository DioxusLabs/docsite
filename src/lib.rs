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

pub static App: FC<()> = |cx, _| {
    let route = use_router(cx, |s| match s {
        "blog" => AppRoute::Blog,
        _ => AppRoute::Home,
    });

    cx.render(rsx! {
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
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

        div { class: "text-gray-500 antialiased bg-white js-focus-visible"
            div { class: "space-y-20 sm:space-y-32 md:space-y-40 lg:space-y-44 overflow-hidden"
                header { class: "relative z-10 max-w-screen-lg xl:max-w-screen-xl mx-auto"
                    div { class: "px-4 sm:px-6 md:px-8 mb-14 sm:mb-20 xl:mb-8",

                        div { class: "border-b border-gray-200 py-4 flex items-center justify-between mb-4 -mx-4 px-4 sm:mx-0 sm:px-0",
                            button { class: "group leading-6 font-medium flex items-center space-x-3 sm:space-x-4 hover:text-gray-600 transition-colors duration-200 w-full py-2",
                                r#type: "button",
                                icons::Search {}
                                span {  "Quick search" span { class: "hidden sm:inline", " for anything" } }
                                span { class: "hidden sm:block text-gray-400 text-sm leading-5 py-0.5 px-1.5 border border-gray-300 rounded-md",
                                    style: "opacity: 1;",
                                    span { class: "sr-only", "Press" }
                                    kbd { class: "font-sans", abbr { class: "no-underline", title: "Command", "⌘" } }
                                    span { class: "sr-only", "and" }
                                    kbd { class: "font-sans", "K" }
                                    span { class: "sr-only", "to search" }
                                }
                            }
                            div { class: "flex items-center space-x-6 sm:space-x-10 ml-6 sm:ml-10",
                                a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
                                    href: "/docs",
                                    span { class: "sm:hidden", "Docs" }
                                    span { class: "hidden sm:inline", "Book" }
                                }
                                a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
                                    href: "/docs",
                                    span { class: "sm:hidden", "Docs" }
                                    span { class: "hidden sm:inline", "Reference" }
                                }
                                a { class: "text-base leading-6 font-medium hover:text-gray-600 transition-colors duration-200 py-2",
                                    href: "/docs",
                                    span { class: "sm:hidden", "Docs" }
                                    span { class: "hidden sm:inline", "docs.rs" }
                                }
                                a { class: "text-gray-400 hover:text-gray-500 transition-colors duration-200",
                                    href: "https://github.com/dioxuslabs/dioxus",
                                    span { class: "sr-only", "Tailwind CSS on GitHub" }
                                    icons::GithubLogo {}
                                }
                            }
                        }
                        // icons::TailwindLogo {}
                        a { class: "flex title-font font-medium items-center mt-24",
                            img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
                            span { class: "ml-3 text-4xl text-gray-900", "dioxus" }
                        }
                        h1 { class: "text-4xl sm:text-6xl lg:text-7xl leading-none font-extrabold text-gray-900 mt-8 mb-8 sm:mt-14 sm:mb-10",
                            "Build reliable user interfaces."
                            // span { class: "text-red-400", "Rust"}
                        }
                        p { class: "max-w-screen-lg text-lg sm:text-2xl sm:leading-10 font-medium mb-10 sm:mb-11",
                            // "Feels like React."
                            span { class: "text-red-400", "Dioxus "}
                            "is a React-like library for building fast, portable, and beautiful user interfaces with Rust."

                            " Runs on the web, desktop, mobile, and more."
                            // "A React-like library for building fast, portable, and beautiful user interfaces with Rust that run on the web, desktop, mobile, and more."
                            // "If it compiles, it works."
                            // " Runs blazingly fast on the web, desktop, mobile, and more."
                        }
                        // p { class: "max-w-screen-lg text-lg sm:text-2xl sm:leading-10 font-medium mb-10 sm:mb-11",
                        //     "If it compiles, it works."
                        // }
                        div { class: "flex flex-wrap space-y-4 sm:space-y-0 sm:space-x-4 text-center",
                            a { class: "w-full sm:w-auto flex-none bg-gray-900 hover:bg-gray-700 text-white text-lg leading-6 font-semibold py-3 px-6 border border-transparent rounded-xl focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-900 focus:outline-none transition-colors duration-200",
                                href: "/docs",
                                "Get started"
                            }
                            button { class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl flex items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200",
                                r#type: "button",
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
                }
            }
        }




        div { class: "container flex flex-col md:flex-row px-24 mx-auto py-12",
            InteractiveHeader {}
            div { class: "flex flex-col flex-shrink-0 mr-auto ml-2",
                div { class: "pt-4"
                    pre {
                        code { class: "language-rust line-numbers", {[include_str!("../snippets/homepage.rs")]} }
                    }
                }
            }
        }




    // let mut count = use_state(cx, || 0);

    // cx.render(rsx!{
    //     div { class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 ml-auto mr-2" background_color: "hsl(220, 13%, 18%)"
    //         div { class: "pb-3 text-white"
    //             h1 { "High-Five counter: {count}" }
    //         }
    //         div { class: "flex flex-col items-center"
    //             button {
    //                 class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
    //                 onclick: move |_| count += 1, "Up high!"
    //             }
    //             img { class: "h-12 mx-4 my-4" src: "https://i.imgur.com/aK3dWXs.png" }
    //             button {
    //                 class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
    //                 onclick: move |_| count -= 1, "Down low!"
    //             }
    //         }
    //     }
    // })        

        components::homepage::value_add::ValueAdd {}
        components::homepage::featured_examples::FeaturedExamples {}
        components::homepage::snippets::Snippets {}
        components::homepage::recent_blog_posts::RecentBlogPosts {}
        components::homepage::call_to_action::CallToAction {}
        components::footer::Footer {}

        // ensure Prism is able to highlight all our code elements
        script { "Prism.highlightAll();" }

        script { {[include_str!("./components/prism/prism.js")]} }
    })
};

pub static InteractiveHeader: FC<()> = |cx, _props| {
    let mut count = use_state(cx, || 0);

    cx.render(rsx!{
        div { class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 ml-auto mr-2" background_color: "hsl(220, 13%, 18%)"
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
