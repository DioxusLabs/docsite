use crate::*;
use dioxus::prelude::*;

struct Tutorial {
    title: &'static str,
    description: &'static str,
    contents: &'static str,
    author: &'static str,
    tags: &'static [&'static str],
}

static TUTORIALS: &[Tutorial] = &[
    Tutorial {
        title: "Making an HTTP request",
        contents: "here's how to make an HTTP request",
        description: "here's how to make an HTTP request",
        author: "@jkelleyrtp",
        tags: &["desktop", "async", "state"],
    },
    Tutorial {
        title: "Making an HTTP request",
        contents: "here's how to make an HTTP request",
        description: "here's how to make an HTTP request",
        author: "@jkelleyrtp",
        tags: &["desktop", "async", "state"],
    },
    Tutorial {
        title: "Making an HTTP request",
        contents: "here's how to make an HTTP request",
        description: "here's how to make an HTTP request",
        author: "@jkelleyrtp",
        tags: &["desktop", "async", "state"],
    },
];

pub fn Tutorials() -> Element {
    rsx! {
        div { class: "dark:bg-ideblack dark:text-white",
            div { class: "max-w-screen-lg mx-auto",
                section { class: "py-10",
                    div { class: "container px-4 mx-auto dark:text-white",
                        h2 { class: "mb-8 md:mb-16 text-5xl lg:text-6xl font-semibold font-heading font-sans",
                            "Tutorials"
                        }
                        div { class: "flex flex-wrap items-center",
                            div { class: "inline-block mb-6 md:mb-0",
                                p { class: "text-xl text-gray-500 dark:text-gray-300",
                                    "Quick snippets of useful code to build basic app functions in Dioxus."
                                    "The official tutorials are simple and mainly focus on one core concept at the time, while the community tutorials are usually more complex."
                                }
                            }
                        }
                    }
                }

                div { class: "pt-4 pb-8",
                    ul {
                        for id in 0..TUTORIALS.len() {
                            TutorialPreview { id: id }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TutorialPreview(id: usize) -> Element {
    let tutorial = &TUTORIALS[id];

    rsx! {
        li { class: "pb-4 border-b border-gray-200 dark:border-gray-500",
            Link { to: Route::Tutorial { id },
                div { class: "rounded p-4 shadow",
                    div { class: "flex justify-between",
                        h2 { class: "text-lg font-bold", "{tutorial.title}" }
                        div {
                            for tag in tutorial.tags {
                                span { class: "rounded p-2 bg-orange", "{tag}" }
                            }
                        }
                    }
                    p { "{tutorial.description}" }
                }
            }
        }
    }
}

#[component]
pub fn Tutorial(id: usize) -> Element {
    let tutorial = TUTORIALS.get(id)?;

    rsx!(
        div {
            h1 { "{tutorial.title}" }
            h3 { "{tutorial.author}" }
            ul {
                for tag in tutorial.tags {
                    li { "{tag}" }
                }
            }
            p { "{tutorial.contents}" }
        }
    )
}
