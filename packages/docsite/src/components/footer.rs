use dioxus::prelude::*;

use crate::Route;

pub fn Footer() -> Element {
    // it's annoying to overscroll on the docs, so we just hide the footer
    let url: Route = use_route();
    if url.is_docs() {
        return VNode::empty();
    }

    let categories = [
        (
            "Community",
            vec![
                ("Github", "https://github.com/dioxuslabs"),
                ("Discord", "https://discord.gg/XgGxMSkvUM"),
                ("Twitter", "https://twitter.com/dioxuslabs"),
                ("YouTube", "https://www.youtube.com/@DioxusLabs"),
            ],
        ),
        (
            "Resources",
            vec![
                ("docs.rs", "https://docs.rs/dioxus"),
                ("crates.io", "https://crates.io/crates/dioxus"),
                ("Guide", "/learn/0.6/guide"),
                ("Awesome", "/awesome"),
                ("Playground", "/playground"),
            ],
        ),
        (
            "Projects",
            vec![
                ("Dioxus", "https://github.com/DioxusLabs/dioxus"),
                (
                    "CLI",
                    "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                ),
                ("Taffy", "https://github.com/DioxusLabs/taffy"),
                ("Blitz", "https://github.com/DioxusLabs/blitz"),
                ("SDK", "https://github.com/DioxusLabs/sdk"),
            ],
        ),
    ];

    rsx! {
        footer { class: "text-gray-700 dark:text-gray-400 w-full mx-auto max-w-screen-xl px-2",
            div { class: "py-8 md:py-24 flex flex-wrap justify-evenly items-start lg:items-start md:flex-row md:flex-nowrap  gap-x-24 gap-y-8 mx-auto",
                for (name , links) in categories.iter() {
                    div { key: "{name}",
                        h2 { class: "text-md mb-3 text-black dark:text-gray-100", "{name}" }
                        nav { class: "list-none font-extralight ",
                            ul { class: "space-y-2",
                                for f in links.iter() {
                                    li { key: "{f.0}",
                                        a { class: "", href: "{f.1}", "{f.0}" }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "text-left md:text-left",
                    a {
                        class: "flex items-center gap-1",
                        href: "https://github.com/DioxusLabs",
                        div {
                            span { class: "text-lg font-mono dark:text-gray-100", "DIOXUS" }
                        }
                        img {
                            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            class: "h-6 w-auto",
                            alt: "Dioxus Labs Icon",
                        }
                    }
                    span { class: "text-xs", "Build cool things ✌️" }
                }

            }
            div { class: "text-gray-400 text-sm text-center sm:text-left pb-2 mx-auto",
                "© 2024 Dioxus Labs"
            }
        }
    }
}
