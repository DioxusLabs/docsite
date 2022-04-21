use dioxus::prelude::*;

pub fn Nav(cx: Scope) -> Element {
    let show = use_state(&cx, || false);

    cx.render(rsx!(
        header {
            class: "sticky top-0 z-50",
            div { class: "pt-6 lg:pt-8 pb-4 flex items-center justify-between font-semibold text-sm leading-6 bg-white shadow dark:text-gray-200 dark:bg-gray-900 px-4 sm:px-6 md:px-8",
                Link {
                    class: "flex title-font font-medium items-center text-gray-900"
                    to: "/",
                    img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-10 w-auto" },
                    span { class: "ml-3 text-4xl dark:text-white", "dioxus" }
                }
                div { class: "flex items-center",
                    MobileNav { show: show }
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

type LinkPairs<'a> = &'a [(&'a str, &'a str)];
static LINKS: &[(&str, &str, LinkPairs)] = &[
    (
        "Platforms",
        "/platforms",
        &[
            ("Web", "https://dioxuslabs.com/reference/web"),
            ("Desktop", "https://dioxuslabs.com/reference/desktop"),
            ("Mobile", "https://dioxuslabs.com/reference/mobile"),
            ("SSR", "https://dioxuslabs.com/reference/ssr"),
            ("TUI", "https://github.com/dioxusLabs/rink"),
            //
            // todo: make dedicated pages for these platforms
            // ("Web", "/platforms/web"),
            // ("Desktop", "/platforms/desktop"),
            // ("Mobile", "/platforms/mobile"),
            // ("Liveview", "/platforms/liveview"),
            // ("SSR", "/platforms/ssr"),
            // ("TUI", "/platforms/tui"),
        ],
    ),
    (
        "Projects",
        "https://github.com/dioxuslabs",
        &[
            (
                "Fermi",
                "https://github.com/DioxusLabs/dioxus/tree/master/packages/fermi",
            ),
            (
                "Router",
                "https://github.com/DioxusLabs/dioxus/tree/master/packages/router",
            ),
            ("CLI", "https://github.com/DioxusLabs/cli"),
        ],
    ),
    (
        "Community",
        "https://github.com/DioxusLabs/awesome-dioxus#community",
        &[
            ("Discord", "https://discord.gg/XgGxMSkvUM"),
            ("Twitter", "https://twitter.com/dioxuslabs"),
            ("Reddit", "https://www.reddit.com/r/dioxus/"),
        ],
    ),
    ("Blog", "/blog", &[]),
    (
        "Guide",
        "https://dioxuslabs.com/guide/",
        &[
            ("Guide", "https://dioxuslabs.com/guide/"),
            ("Advanced Guides", "https://dioxuslabs.com/reference/"),
            ("Reference", "https://dioxuslabs.com/reference/"),
            (
                "Router",
                "https://github.com/DioxusLabs/dioxus/tree/master/packages/router",
            ),
        ],
    ),
];

fn LinkList(cx: Scope) -> Element {
    let hover = "hover:text-sky-500 dark:hover:text-sky-400";
    let hover_bg = "dark:hover:bg-gray-500 hover:bg-gray-200 rounded";

    let links = LINKS.iter().copied().map(|(name, link, links)| {
        if links.is_empty() {
            rsx! {
                li { key: "{link}",
                    Link { class: "py-1 px-2 {hover} {hover_bg}", to: "{link}",
                        "{name}"
                    }
                }
            }
        } else {
            rsx! {
                li { class: "group relative dropdown", key: "{link}",
                    Link { to: "{link}", class: "py-1 px-2 {hover} {hover_bg}", "{name}" }
                    nav { class: "dropdown-menu absolute h-auto -mt-64 group-hover:mt-0 opacity-0 group-hover:opacity-100 transition-opacity duration-250",
                        ul { class: "top-0 w-36 bg-white dark:bg-gray-800 shadow px-4 py-4 rounded",
                            links.iter().map(|(name, link)| rsx!{
                                Link {  to: "{link}", key: "{link}",
                                    li { class: "rounded px-1 py-1 {hover} {hover_bg}",
                                        "{name}"
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! { links })
}
