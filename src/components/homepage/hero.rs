use crate::icons;
use dioxus::prelude::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-wrap items-center pb-12 px-12 max-w-screen-2xl mx-auto",
            div { class: "relative w-1/2 mx-4 sm:mx-auto pt-5 sm:pt-24 lg:pt-24 text-gray-600",
            // div { class: "relative max-w-5xl mx-4 sm:mx-auto pt-5 sm:pt-24 lg:pt-32 text-gray-600",
                // div { class: "flex flex-row",
                    // img { src: "https://avatars.githubusercontent.com/u/79236386?s=400", class: "w-auto aspect-square" },
                h1 { class: "text-[8em] font-mono dark:text-white text-ghdarkmetal",
                // h1 { class: "text-[8em] font-mono dark:text-white ml-8 text-transparent bg-clip-text pb-3 bg-gradient-to-b from-sky-400 to-sky-200",
                // h1 { class: "text-[8em] font-mono dark:text-white ml-8 text-transparent bg-clip-text pb-3 bg-gradient-to-r from-rose-500 via-red-400 to-red-500",
                // h1 { class: "text-[8em] font-mono dark:text-white ml-8 text-transparent bg-clip-text pb-3 bg-gradient-to-r from-red-400 via-purple-300 to-blue-500",
                // h1 { class: "text-[8em] font-mono dark:text-white ml-8 text-transparent bg-clip-text pb-3 bg-gradient-to-r from-red-400 via-purple-300 to-blue-500",
                    "Dioxus"
                }
                h1 { class: "text-3xl tracking-tight dark:text-white font-mono text-ghdarkmetal flex flex-row",
                    "User interfaces that run anywhere. 🦀"
                    // "User interfaces that run anywhere."
                    // pre {
                    //     class: "text-transparent text-3xl lg:text-3xl bg-clip-text pb-3 font-mono bg-gradient-to-r from-red-400 via-purple-300 to-blue-500",
                    //     " anywhere"
                    // }
                }
                // DioxusIcon {}

                // }
                // h1 { class: "font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center dark:text-white",
                //     "Build reliable user interfaces that run "
                //     pre {
                //         class: "text-transparent text-6xl lg:text-8xl bg-clip-text pb-3 bg-gradient-to-r from-red-400 via-purple-300 to-blue-500",
                //         "anywhere"
                //     }
                // }
                // p { class: "dark:text-white mt-6 text-lg max-w-3xl leading-8",
                //     "Inspired by React and written in Rust."
                    // "This is the Century of Biology. Breakthroughs today and in the coming decades will transform the world. We accelerate your progress by powering research across the life sciences with single cell, spatial, and in situ products."
                    // " Inspired by React, written for Rust, and supports web, desktop, mobile, liveview, and more."
                    // "Introducing "
                    // span { class: "text-red-400", "Dioxus"}
                    // ": a React-like library for building fast, portable, and beautiful user interfaces with Rust."
                    // " Runs on the web, desktop, mobile, and more."
                // }
                div { class: "mt-6 sm:mt-10 flex space-x-6 text-sm ",
                    a { class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center sm:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                        href: "https://dioxuslabs.com/guide/",
                        "Get started"
                    }

                    SaveClipboard {}
                }
            }
            div { class: "w-1/2 px-4 flex flex-col pt-5 sm:pt-24 lg:pt-24 justify-end",
                // // img { class: "h-96",
                // //     src: "https://www.pngkit.com/png/full/256-2560512_web-design-cross-platform-icon.png"
                // // }
                // InteractiveHeader {}
                // div { class: "lg:ml-auto lg:mr-2",
                //     div { class: "pt-4",
                //         pre {
                //             code { class: "language-rust line-numbers", [include_str!("../../../snippets/homepage.rs")] }
                //         }
                //     }
                // }
                AnimatedIcon {}
            }
        }

        // div { class: "container flex flex-col md:flex-row md:px-24 md:py-12 mx-auto",
        //     div { class: "lg:ml-auto lg:mr-2",
        //         div { class: "pt-4",
        //             pre {
        //                 code { class: "language-rust line-numbers", [include_str!("../../../snippets/homepage.rs")] }
        //             }
        //         }
        //     }
        //     InteractiveHeader {}
        // }
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
    // cx.render(rsx!{
    //     div {
    //         class: "flex flex-col items-center px-10 rounded mt-6 mb-2 pt-4 mr-auto hidden lg:block lg:ml-2" ,
    //         background_color: "hsl(220, 13%, 18%)",

    //         div { class: "pb-3 text-white text-center w-44",
    //             h1 { "Counter: {count}" }
    //         }
    //         div { class: "flex flex-col items-center",
    //             button {
    //                 class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
    //                 onclick: move |_| count += 1,
    //                 "Up high!"
    //             }
    //             // img { class: "h-12 mx-4 my-4", src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
    //             button {
    //                 class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600",
    //                 onclick: move |_| count -= 1,
    //                 "Down low!"
    //             }
    //         }
    //     }
    // })
}

fn AnimatedIcon(cx: Scope) -> Element {
    let s = include_str!("../../../public/static/multiplatform.svg");
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! {
          // svg {
          //     xmlns: "http://www.w3.org/2000/svg", shape_rendering: "geometricPrecision", text_rendering: "geometricPrecision", view_box: "0 0 640 480", style: "white-space: pre;",
          //     style {
          //         "{ANIM}"
          //     }
          //     rect {
          //         onclick: move |_| {
          //             //
          //             count += 1;
          //         },
          //         width: "166", height: "255", fill: "#ca3939", stroke: "none", transform: "translate(297,224.5) translate(-83,-127.5)", style: "animation: 2s linear infinite both a0_f, 2s linear infinite both a0_w, 2s linear infinite both a0_h;",
          //         // foreignObject {
          //         //     x:"20", y:"20", width:"160", height:"100",
          //         //     InteractiveHeader {}
          //         // }
          //     }
          //     text {
          //         x: "20", y:"35",
          //         "{count}"
          //     }
          // }
          div { dangerous_inner_html: "{s}" }
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
