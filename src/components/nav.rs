use dioxus::{
    prelude::*,
    router::{Link, Route, Router},
};

pub fn Nav(cx: Scope) -> Element {
    let (show, set_show) = use_state(&cx, || false);

    cx.render(rsx!(
        div {
            div { class: "relative pt-6 lg:pt-8 pb-4 flex items-center justify-between font-semibold text-sm leading-6 dark:text-gray-200 dark:bg-gray-900 px-4 sm:px-6 md:px-8",
                Link {
                    class: "flex title-font font-medium items-center text-gray-900"
                    to: "/",
                    img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
                    span { class: "ml-3 text-4xl dark:text-white", "dioxus" }
                }
                div { class: "flex items-center",
                    MobileNav { show: set_show }
                    FullNav {}
                }
            }
            show.then(|| {
                rsx!{
                    ul { class: "flex items-center flex-col", gap: "10px",
                        LinkList {}
                    }
                }
            })
        }
    ))
}

fn FullNav(cx: Scope) -> Element {
    cx.render(rsx!{
        div { class: "hidden md:flex items-center",
            nav {
                ul { class: "flex items-center space-x-8", LinkList {} }
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
    })
}

#[inline_props]
fn MobileNav<'a>(cx: Scope<'a>, show: &'a UseState<bool>) -> Element {
    cx.render(rsx!{
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
                    onclick: move |_| show.modify(|f| !f),
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
            },
        }
    })
}

fn LinkList(cx: Scope) -> Element {
    cx.render(rsx!{
        li {
            Link {
                class: "hover:text-sky-500 dark:hover:text-sky-400",
                to: "/",
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
                href: "https://dioxuslabs.com/reference/",
                "Reference"
            }
        }
        li {
            Link { class: "hover:text-sky-500 dark:hover:text-sky-400"
                to: "/blog",
                "Blog"
            }
        }
        li {
            a { class: "dark:hover:text-sky-400 p-2 rounded bg-gray-600 hover:bg-gray-300 text-white",
                href: "https://dioxuslabs.com/guide/",
                "Get Started"
            }
        }
    })
}
