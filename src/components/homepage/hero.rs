use crate::icons;
use dioxus::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx!{
        section { class: "text-gray-400 bg-gray-800",
            div { class: "container flex flex-col md:flex-row w:2/3 px-24 py-20 mx-auto",
                div { class: "flex flex-col md:pr-10 md:mb-0 mb-6 pr-0 w-full md:w-auto md:text-left text-center w:1/2",
                    h1 { class: "sm:text-6xl text-16xl font-medium title-font mb-2 text-white", "Dioxus" }
                    p { class: "leading-relaxed text-white text-4xl", "A Rust library for building user interfaces." }
                    p { class: "leading-relaxed text-opacity-90 text-xl py-5",
                        "Build reliable, fast, and scalable user interfaces that run on the web, desktop, mobile, server, and more."
                    }
                    div { class: "container flex flex-wrap p-5 flex-col md:flex-row mx-0 px-0",
                        a {
                            href: "book"
                            class: "inline-flex items-center text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600",
                            "Read the book"
                            icons::ArrowRight {}
                        }
                    }
                }
                div { class: "flex flex-col mx-auto w:1/2 flex-shrink-0",
                    div { class: "pt-4"
                        pre {
                            padding_bottom: "0px"
                            margin_bottom: "0px"
                            code { class: "language-rust", {[include_str!("../../../snippets/homepage.rs")]} }
                        }
                    }
                    InteractiveHeader {}
                }
            }
        }
    })
}

pub static InteractiveHeader: Component<()> = |cx| {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!{
        div { class: "flex flex-col items-center py-3" background_color: "hsl(220, 13%, 18%)"
            div { class: "pb-3"
                h1 { "High-Five counter: {count}" }
            }
            div { class: "flex flex-row items-center"
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count += 1, "Up high!"
                }
                img { class: "h-12 mx-4" src: "https://i.imgur.com/aK3dWXs.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count -= 1, "Down low!"
                }
            }
        }
    })
};
