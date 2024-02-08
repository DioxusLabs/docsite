use crate::icons;
use crate::Link;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BlogPost {
    category: &'static str,
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    content: &'static str,
}

pub const POST_TEMPLATE: BlogPost = BlogPost {
    category: "Tech",
    date: "Dec 11, 2022",
    title: "Making Dioxus (almost) as fast as SolidJS",
    description:
        "Using a new technique called subtree memoization, Dioxus is now almost as fast as SolidJS.",
    link: "/blog/templates-diffing/",
    content: include_str!("../../../posts/templates.html"),
};

#[component]
pub fn PostTemplate() -> Element {
    rsx! { SinglePost { post: POST_TEMPLATE } }
}

pub const POST_FULLTINME: BlogPost = BlogPost {
    category: "Misc",
    date: "May 5 2023",
    title: "Going full time on Dioxus",
    description:
        "Dioxus is now my full time job! I'm so excited to be able to work on this full time.",
    link: "/blog/going-fulltime/",
    content: include_str!("../../../posts/fulltime.html"),
};

#[component]
pub fn PostFulltime() -> Element {
    rsx! { SinglePost { post: POST_FULLTINME } }
}

pub const POST_RELEASE_040: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Aug 1 2023",
    title: "Announcing Dioxus 0.4",
    description: "An overhauled router, fullstack, desktop hotreloading, and more!",
    link: "/blog/release-040/",
    content: include_str!("../../../posts/release040.html"),
};

#[component]
pub fn PostRelease040() -> Element {
    rsx! { SinglePost { post: POST_RELEASE_040 } }
}

pub const POST_RELEASE_030: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Feb 8 2023",
    title: "Announcing Dioxus 0.3",
    description: "The next big release of Dioxus is here! Templates, autoformatting, multiwindow support, and more!",
    link: "/blog/release-030/",
    content: include_str!("../../../posts/release030.html"),
};

#[component]
pub fn PostRelease030() -> Element {
    rsx! { SinglePost { post: POST_RELEASE_030 } }
}

pub const POST_RELEASE_020: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Mar 9 2022",
    title: "Announcing Dioxus 0.2",
    description: "Just over two months in, and we already have a ton of awesome changes to Dioxus!",
    link: "/blog/release-020/",
    content: include_str!("../../../posts/release020.html"),
};

#[component]
pub fn PostRelease020() -> Element {
    rsx! { SinglePost { post: POST_RELEASE_020 } }
}

pub const POST_RELEASE_010: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Jan 3 2022",
    title: "Announcing Dioxus 0.1",
    description: "After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust. It is built around a VirtualDOM, making it portable for the web, desktop, server, mobile, and more.",
    link: "/blog/introducing-dioxus/",
    content: include_str!("../../../posts/release.html"),
};

#[component]
pub fn PostRelease010() -> Element {
    rsx! { SinglePost { post: POST_RELEASE_010 } }
}

pub const POSTS: &[BlogPost] = &[
    POST_RELEASE_040,
    POST_FULLTINME,
    POST_RELEASE_030,
    POST_TEMPLATE,
    POST_RELEASE_020,
    POST_RELEASE_010,
];

#[component]
pub fn BlogList() -> Element {
    rsx!(
        section { class: "body-font overflow-hidden dark:bg-ideblack",
            div { class: "container max-w-screen-lg pt-12 pb-12 mx-auto",
                div { class: "-my-8 px-8 pb-12",
                    // Header
                    h2 { class: "dark:text-white mb-8 md:mb-16 sm:text-3xl text-2xl font-medium title-font font-sans",
                        "Recent Blog Posts"
                    }
                    section { class: "body-font overflow-hidden dark:bg-ideblack",
                        div { class: "container px-6 mx-auto",
                            div { class: "-my-8 divide-y-2 divide-gray-100",
                                for post in POSTS.iter() {
                                    BlogPostItem { post: post }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn SinglePost(post: BlogPost) -> Element {
    let BlogPost { content, .. } = post;

    rsx! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack",
            div { class: "container lg:px-20 xl:px-48 pt-12 pb-12 mx-auto",
                script { "Prism.highlightAll()" }
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body li {{ display: list-item; }}"
                        ".markdown-body img {{ max-height: 500px; margin-left: auto; margin-right: auto; padding-left: 4px; padding-right: 4px; }}"
                        ".markdown-body .highlight pre, .markdown-body pre {{ background-color: #1e1e1e }}"
                    }
                    article { class: "markdown-body", dangerous_inner_html: format_args!("{}", content) }
                    script { "Prism.highlightAll()" }
                }
            }
        }
    }
}

fn BlogHeader() -> Element {
    rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto dark:text-white",

                h2 { class: "mb-8 md:mb-16 text-5xl lg:text-6xl font-semibold font-heading font-sans",
                    "Dioxus Official Blog"
                }

                div { class: "flex flex-wrap items-center",
                    div { class: "inline-block max-w-xl mb-6 md:mb-0",
                        p { class: "text-xl pb-4 text-gray-500 dark:text-gray-300",
                            "Updates, changelogs, and general musings of the Dioxus community...."
                        }
                    }
                }
            }
        }
    )
}

pub static RecentBlogPosts: Component<()> = |cx| {
    rsx! {
        section { class: "body-font overflow-hidden dark:bg-ideblack",
            div { class: "container px-6 lg:px-40 pt-24 pb-36 mx-auto max-w-screen-xl",
                div { class: "flex flex-col w-full mb-10",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 dark:text-white",
                        "Recent Blog Posts"
                    }
                }
                div { class: "-my-8 divide-y-2 divide-gray-100",
                    for post in POSTS.iter() {
                        BlogPostItem { post: post }
                    }
                }
            }
        }
    }
};

#[component]
fn BlogPostItem(post: &'static BlogPost) -> Element {
    let BlogPost {
        category,
        date,
        title,
        description,
        link,
        ..
    } = post;

    rsx!(
        div { class: "py-8 flex flex-wrap md:flex-nowrap",
            div { class: "md:w-32 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
                span { class: "font-semibold title-font text-gray-700 dark:text-white",
                    "{category}"
                }
                span { class: "mt-1 text-gray-500 text-sm", "{date}" }
            }
            div { class: "md:flex-grow",
                h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2 dark:text-white",
                    "{title}"
                }
                p { class: "leading-relaxed dark:text-white text-base dark:opacity-75",
                    "{description}"
                }
                Link { class: "text-indigo-500 inline-flex items-center mt-4", to: *link,
                    "Read more"
                    icons::ArrowRight {}
                }
            }
        }
    )
}
