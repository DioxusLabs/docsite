use crate::*;

pub fn Hero(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "w-full dark:bg-ideblack",
            div { class: "flex flex-wrap items-center pb-12 px-3 md:px-12 max-w-screen-2xl mx-auto text-center md:text-left",
                div { class: "relative w-full md:w-1/2 mx-4 sm:mx-auto pt-5 sm:pt-24 lg:pt-24 text-gray-600",
                    h1 { class: "text-[5em] md:text-[8em] font-mono dark:text-white text-ghdarkmetal",
                        "Dioxus"
                    }
                    h1 { class: "text-xl md:text-3xl tracking-tight dark:text-white font-mono text-ghdarkmetal flex flex-row",
                        "User interfaces that run anywhere. 🦀"
                    }

                    h3 { class: "text-md md:text-xl tracking-tight dark:text-white font-thin font-mono text-ghdarkmetal pt-4",
                        "Written in Rust, inspired by React"
                    }

                    div { class: "mt-6 sm:mt-10 flex space-x-6 text-sm",
                        Link {
                            to: Route::Docs { child: BookRoute::GettingStartedIndex {} },
                            class: "bg-gray-900 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-gray-50 text-white font-semibold h-12 px-6 rounded-lg w-full flex items-center justify-center md:w-auto dark:bg-sky-500 dark:highlight-white/20 dark:hover:bg-sky-400",
                            "Get started"
                        }
                        SaveClipboard {}
                    }
                }
                div { class: "w-full md:w-1/2 px-4 flex flex-col pt-5 sm:pt-24 lg:pt-24 justify-end",
                    AnimatedIcon {}
                }
            }
        }
    })
}

static ADD_TO_CLIPBOARD: &str = r#"navigator.clipboard.writeText("cargo add dioxus")"#;

fn SaveClipboard(cx: Scope) -> Element {
    let saved = use_state(cx, || false);

    // funny that we can just default to some javascript like this
    // might want to do the same thing in rust so we can display a selected state
    cx.render(rsx! {
        button {
            class: "w-full sm:w-auto flex-none bg-gray-50 text-gray-400 hover:text-gray-900 font-mono leading-6 py-3 sm:px-6 border border-gray-200 rounded-xl items-center justify-center space-x-2 sm:space-x-4 focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-gray-300 focus:outline-none transition-colors duration-200 hidden md:flex",
            "onclick": "{ADD_TO_CLIPBOARD}",
            "type": "button",
            onclick: move |_| saved.set(true),
            span { class: "text-gray-900",
                span { class: "hidden sm:inline text-gray-500", aria_hidden: "true", "$ " }
                span { class: "text-red-400", "cargo " }
                "add dioxus"
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
