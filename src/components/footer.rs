use dioxus::prelude::*;

pub fn Footer() -> Element {
    let categories = [
        (
            "Community",
            &[
                ("Github", "https://github.com/dioxuslabs"),
                ("Twitter", "https://twitter.com/dioxuslabs"),
                ("Discord", "https://discord.gg/XgGxMSkvUM"),
            ],
        ),
        (
            "Learning",
            &[
                ("docs.rs", "https://docs.rs/dioxus"),
                ("Guide", "/learn/0.5/guide"),
                ("Awesome", "/awesome"),
            ],
        ),
        (
            "Projects",
            &[
                ("Dioxus", "https://github.com/DioxusLabs/dioxus"),
                (
                    "CLI",
                    "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                ),
                ("Taffy", "https://github.com/DioxusLabs/taffy"),
            ],
        ),
    ];

    rsx! {
        footer { class: "sticky z-30 text-gray-400 bg-ghmetal body-font",
            div { class: "container px-5 py-24 mx-auto flex md:items-center lg:items-start md:flex-row md:flex-nowrap flex-wrap flex-col",
                div { class: "w-64 flex-shrink-0 md:mx-0 mx-auto text-center md:text-left",
                    a {
                        class: "flex title-font font-medium items-center md:justify-start justify-center text-white",
                        href: "https://github.com/DioxusLabs/dioxus",
                        img {
                            src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            class: "h-8 w-auto",
                            alt: "Dioxus Labs Icon",
                        }
                        span { class: "ml-3 text-xl", "Dioxus Labs" }
                    }
                    p { class: "mt-2 text-sm text-gray-500",
                        "An Open Source project dedicated to making Rust UI wonderful."
                    }
                }
                div { class: "flex-grow flex flex-wrap md:pl-20 -mb-10 md:mt-0 mt-10 md:text-left text-center",
                    for (name , links) in categories.iter() {
                        div {
                            key: "{name}",
                            class: "lg:w-1/4 md:w-1/2 w-full px-4",
                            h2 { class: "title-font font-medium text-white tracking-widest text-sm mb-3",
                                "{name}"
                            }
                            nav { class: "list-none mb-10",
                                ul {
                                    for f in links.iter() {
                                        li { key: "{f.0}",
                                            a {
                                                class: "text-gray-400 hover:text-white",
                                                href: "{f.1}",
                                                "{f.0}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "container mx-auto py-4 px-5 flex flex-wrap flex-col sm:flex-row",
                p { class: "text-gray-400 text-sm text-center sm:text-left",
                    "© 2024 Dioxus Labs —"
                    a {
                        class: "text-gray-500 ml-1",
                        rel: "noopener noreferrer",
                        href: "https://twitter.com/dioxuslabs",
                        target: "_blank",
                        "@dioxuslabs"
                    }
                }
            }
        }
    }
}
