use crate::icons;
use dioxus::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "w-full dark:bg-ideblack",
            div { class: "flex flex-wrap items-center pb-12 px-12 max-w-screen-2xl mx-auto",
                div { class: "relative w-full md:w-1/2 mx-4 sm:mx-auto pt-5 sm:pt-24 lg:pt-24 text-gray-600",
                h1 { class: "text-[8em] font-mono dark:text-white text-ghdarkmetal", "Dioxus" }
                    h1 { class: "text-3xl tracking-tight dark:text-white font-mono text-ghdarkmetal flex flex-row",
                        "User interfaces that run anywhere. 🦀"
                    }
                    h3 { class: "text-xl tracking-tight dark:text-white font-thin font-mono text-ghdarkmetal flex flex-row pt-4",
                        "Written in Rust, inspired by React"
                    }
                    div { class: "mt-6 sm:mt-10 flex space-x-6 text-sm ",
                        a { class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                            href: "https://dioxuslabs.com/guide/",
                            "Get started"
                        }
                        SaveClipboard {}
                    }
                }
                div { class: "w-full md:w-1/2 px-4 flex flex-col pt-5 sm:pt-24 lg:pt-24 justify-end", AnimatedIcon {} }
            }
        }
    })
}

fn SaveClipboard(cx: Scope) -> Element {
    let saved = use_state(&cx, || false);

    // funny that we can just default to some javascript like this
    // might want to do the same thing in rust so we can display a selected state
    cx.render(rsx! {
        button {
            class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl flex items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200 hidden md:flex",
            "type": "button",
            "onclick": r#"navigator.clipboard.writeText("cargo add dioxus")"#,
            onclick: move |_| saved.set(true),
            span { class: "text-gray-900",
                span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
                span { class: "text-red-400", "cargo " }
                "add dioxus"
            }
            if **saved {
                rsx! { icons::IconCheckGh {} }
            } else {
                rsx! { icons::Copy {} }
            }
        }
  })
}

pub fn InteractiveHeader(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            "xmlns": "http://www.w3.org/1999/xhtml",
            class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 mr-auto hidden lg:block lg:ml-2",
            background_color: "hsl(220, 13%, 18%)",
            div { class: "flex flex-row items-center",
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
                    onclick: move |_| count += 1,
                    "Up high!"
                }
                div { class: "pb-3 text-white text-center w-44", h1 { "Counter: {count}" } }
                // img { class: "h-12 mx-4 my-4", src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
                    onclick: move |_| count -= 1,
                    "Down low!"
                }
            }
        }
  })
}

fn AnimatedIcon(cx: Scope) -> Element {
    let dark = include_str!("../../../public/static/multiplatform-dark.svg");
    let light = include_str!("../../../public/static/multiplatform-light.svg");

    cx.render(rsx! {
        div {
            div { class: "dark:hidden", dangerous_inner_html: "{dark}" }
            div { class: "hidden dark:block", dangerous_inner_html: "{light}" }
        }
    })
}

fn DioxusIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        svg { fill: "none", height: "187", width: "112", view_box: "0 0 112 187", xmlns: "http://www.w3.org/2000/svg",
            rect { fill: "#D9D9D9", height: "8", width: "85", x: "13", y: "5" }
            rect { fill: "#D9D9D9", height: "8", width: "85", x: "13", y: "31" }
            rect { fill: "#D9D9D9", height: "8", width: "43", x: "34", y: "61" }
            rect { fill: "#D9D9D9", height: "8", transform: "matrix(1 0 0 -1 13 184)", width: "85" }
            rect { fill: "#D9D9D9", height: "8", transform: "matrix(1 0 0 -1 13 158)", width: "85" }
            rect { fill: "#D9D9D9", height: "8", transform: "matrix(1 0 0 -1 34 128)", width: "43" }
            path {
                d: "M9.00001 9C9 62 103.5 124 103.5 178",
                stroke: "#3CC4DC",
                "stroke-linecap": "square",
                "stroke-width": "17"
            }
            path {
                d: "M103.5 9C103.5 62 9.00002 124 9 178",
                stroke: "#FB422D",
                "stroke-linecap": "square",
                "stroke-width": "17"
            }
        }
  })
}
