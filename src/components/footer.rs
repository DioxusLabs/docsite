use dioxus::prelude::*;

pub fn Footer() -> Element {
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
                ("Guide", "/learn/0.5/guide"),
                ("Awesome", "/awesome"),
                ("Playground", "/play"),
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
        footer { class: "text-gray-700 dark:text-gray-400 w-full mx-auto max-w-screen-lg",
            div { class: "container py-24 mx-auto flex md:items-center lg:items-start md:flex-row md:flex-nowrap flex-wrap flex-col justify-between ",
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

                div { class: "text-center md:text-left ",
                    a {
                        class: "flex items-center md:justify-start justify-evenly gap-1",
                        href: "https://github.com/DioxusLabs",
                        img {
                            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            class: "h-6 w-auto",
                            alt: "Dioxus Labs Icon",
                        }
                        div {
                            span { class: "text-xl font-mono dark:text-gray-100", "Dioxus" }
                        }
                    }
                    span { class: "text-xs", "Build cool things ✌️" }
                }
            }
            p { class: "text-gray-400 text-sm text-center sm:text-left pb-2", "© 2024 Dioxus Labs" }
        }
    }
}
