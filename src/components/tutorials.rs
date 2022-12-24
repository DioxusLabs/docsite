use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

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
    Tutorial {
        title: "Making an HTTP request",
        contents: "here's how to make an HTTP request",
        description: "here's how to make an HTTP request",
        author: "@jkelleyrtp",
        tags: &["desktop", "async", "state"],
    },
];

pub fn Tutorials(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "dark:bg-ideblack dark:text-white",
            div { class: "max-w-screen-lg mx-auto",
                section { class: "py-10",
                    div { class: "container px-4 mx-auto dark:text-white",
                        h2 { class: "mb-8 md:mb-16 text-5xl lg:text-6xl font-semibold font-heading font-mono", "Tutorials" }
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
    })
}

#[inline_props]
fn TutorialPreview(cx: Scope, id: usize) -> Element {
    let tutorial = &TUTORIALS[*id];

    cx.render(rsx! {
        li { class: "pb-8",
            Link { to: "/tutorials/{id}",
                div { class: "rounded p-4 shadow",
                    div { class: "flex justify-between",
                        h2 { class: "text-lg font-bold", tutorial.title }
                        div {
                            for tag in tutorial.tags {
                                span { class: "rounded p-2 bg-orange", *tag }
                            }
                        }
                    }
                    p { tutorial.description }
                }
            }
        }
    })
}

pub fn Tutorial(cx: Scope) -> Element {
    let tutorial = use_route(cx)
        .parse_segment_or_404::<usize>("id")
        .and_then(|f| TUTORIALS.get(f))?;

    render!(
        div {
            h1 { tutorial.title  }
            h3 { tutorial.author }
            ul {
                for tag in tutorial.tags {
                    li { *tag }
                }
            }
            p { tutorial.contents }
        }
    )
}
