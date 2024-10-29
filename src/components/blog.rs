use crate::icons;
use crate::Link;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) struct BlogPost {
    category: &'static str,
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    content: &'static str,
}

pub(crate) const POST_RELEASE_050: BlogPost = BlogPost {
    category: "Release Notes",
    date: "March 21, 2024",
    title: "Announcing Dioxus 0.5",
    description: "A signal rewrite, zero unsafe, no lifetimes, unified launch, and more! ",
    link: "/blog/release-050/",
    content: include_str!("../../posts/release050.html"),
};

#[component]
pub(crate) fn PostRelease050() -> Element {
    rsx! {
        SinglePost { post: POST_RELEASE_050 }
    }
}

pub(crate) const POST_TEMPLATE: BlogPost = BlogPost {
    category: "Tech",
    date: "Dec 11, 2022",
    title: "Making Dioxus (almost) as fast as SolidJS",
    description:
        "Using a new technique called subtree memoization, Dioxus is now almost as fast as SolidJS.",
    link: "/blog/templates-diffing/",
    content: include_str!("../../posts/templates.html"),
};

#[component]
pub(crate) fn PostTemplate() -> Element {
    rsx! {
        SinglePost { post: POST_TEMPLATE }
    }
}

pub(crate) const POST_FULLTINME: BlogPost = BlogPost {
    category: "Misc",
    date: "May 5 2023",
    title: "Going full time on Dioxus",
    description:
        "Dioxus is now my full time job! I'm so excited to be able to work on this full time.",
    link: "/blog/going-fulltime/",
    content: include_str!("../../posts/fulltime.html"),
};

#[component]
pub(crate) fn PostFulltime() -> Element {
    rsx! {
        SinglePost { post: POST_FULLTINME }
    }
}

pub(crate) const POST_RELEASE_040: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Aug 1 2023",
    title: "Announcing Dioxus 0.4",
    description: "An overhauled router, fullstack, desktop hotreloading, and more!",
    link: "/blog/release-040/",
    content: include_str!("../../posts/release040.html"),
};

#[component]
pub(crate) fn PostRelease040() -> Element {
    rsx! {
        SinglePost { post: POST_RELEASE_040 }
    }
}

pub(crate) const POST_RELEASE_030: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Feb 8 2023",
    title: "Announcing Dioxus 0.3",
    description: "The next big release of Dioxus is here! Templates, autoformatting, multiwindow support, and more!",
    link: "/blog/release-030/",
    content: include_str!("../../posts/release030.html"),
};

#[component]
pub(crate) fn PostRelease030() -> Element {
    rsx! {
        SinglePost { post: POST_RELEASE_030 }
    }
}

pub(crate) const POST_RELEASE_020: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Mar 9 2022",
    title: "Announcing Dioxus 0.2",
    description: "Just over two months in, and we already have a ton of awesome changes to Dioxus!",
    link: "/blog/release-020/",
    content: include_str!("../../posts/release020.html"),
};

#[component]
pub(crate) fn PostRelease020() -> Element {
    rsx! {
        SinglePost { post: POST_RELEASE_020 }
    }
}

pub(crate) const POST_RELEASE_010: BlogPost = BlogPost {
    category: "Release Notes",
    date: "Jan 3 2022",
    title: "Announcing Dioxus 0.1",
    description: "After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust. It is built around a VirtualDOM, making it portable for the web, desktop, server, mobile, and more.",
    link: "/blog/introducing-dioxus/",
    content: include_str!("../../posts/release.html"),
};

#[component]
pub(crate) fn PostRelease010() -> Element {
    rsx! {
        SinglePost { post: POST_RELEASE_010 }
    }
}

pub(crate) const POSTS: &[BlogPost] = &[
    POST_RELEASE_050,
    POST_RELEASE_040,
    POST_FULLTINME,
    POST_RELEASE_030,
    POST_TEMPLATE,
    POST_RELEASE_020,
    POST_RELEASE_010,
];

#[component]
pub(crate) fn BlogList() -> Element {
    rsx! {
        section { class: "body-font overflow-hidden dark:bg-ideblack font-light",
            div { class: "container max-w-screen-md pt-12 pb-12 mx-auto",
                div { class: "-my-8 px-8 pb-12",
                    // Header
                    h2 { class: "dark:text-white my-8 md:mb-16 sm:text-3xl text-2xl font-medium title-font font-sans",
                        "Blog"
                    }
                    section { class: "body-font overflow-hidden dark:bg-ideblack",
                        div { class: "container px- mx-auto",
                            div { class: "-my-8 divide-y divide-neutral-400",
                                for post in POSTS.iter() {
                                    BlogPostItem { post }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn SinglePost(post: BlogPost) -> Element {
    let BlogPost { content, .. } = post;

    rsx! {
        section { class: "text-gray-600 body-font dark:bg-ideblack max-w-screen-md mx-auto pt-24 font-light",
            script { "Prism.highlightAll()" }
            article {
                class: "markdown-body px-2  dioxus-blog-post",
                dangerous_inner_html: format_args!("{}", content),
            }
            script { "Prism.highlightAll()" }
        }
    }
}

#[component]
fn BlogPostItem(post: &'static BlogPost) -> Element {
    let BlogPost {
        date,
        title,
        description,
        link,
        ..
    } = post;

    rsx! {
        div { class: "py-8 flex flex-wrap md:flex-nowrap",

            div { class: "md:flex-grow pl-8",
                div { class: "flex flex-row justify-between gap-4",
                    h2 { class: "text-2xl font-medium text-gray-900 title-font mb-4 dark:text-white",
                        "{title}"
                    }
                    span { class: "my-2 text-gray-500 text-sm", "{date}" }
                }
                p { class: "leading-relaxed dark:text-white text-base dark:opacity-75",
                    "{description}"
                }
                Link {
                    class: "text-indigo-500 inline-flex items-center mt-4",
                    to: *link,
                    "Read more"
                    icons::ArrowRight {}
                }
            }
        }
    }
}
