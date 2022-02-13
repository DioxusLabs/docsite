use crate::icons;
use dioxus::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx!{
        div { class: "relative max-w-5xl mx-4 sm:mx-auto pt-20 sm:pt-24 lg:pt-32 text-gray-600",
            h1 { class: "font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white",
                "Build reliable user interfaces that run "
                pre {
                    class: "text-transparent text-6xl lg:text-8xl bg-clip-text pb-3 bg-gradient-to-r from-red-400 via-purple-300 to-blue-500",
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
                    "onclick": "navigator.clipboard.writeText(\"cargo add dioxus\")",
                    "type": "button",
                    span { class: "text-gray-900",
                        span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
                        span { class: "text-red-400", "cargo " }
                        "add dioxus"
                    }
                    span { class: "sr-only", "(click to copy to clipboard)" }
                    icons::Copy {}
                }
            }
        }

        div { class: "container flex flex-col md:flex-row md:px-24 md:py-12 mx-auto",
            div { class: "lg:ml-auto lg:mr-2",
                div { class: "pt-4",
                    pre {
                        code { class: "language-rust line-numbers", [include_str!("../../../snippets/homepage.rs")] }
                    }
                }
            }
            InteractiveHeader {}
        }
    })
}

pub fn InteractiveHeader(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx!{
        div {
            class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 mr-auto hidden lg:block lg:ml-2" ,
            background_color: "hsl(220, 13%, 18%)",

            div { class: "pb-3 text-white",
                h1 { "High-Five counter: {count}" }
            }
            div { class: "flex flex-col items-center",
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
                    onclick: move |_| set_count(count + 1),
                    "Up high!"
                }
                img { class: "h-12 mx-4 my-4", src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
                    onclick: move |_| set_count(count - 1),
                    "Down low!"
                }
            }
        }
    })
}
