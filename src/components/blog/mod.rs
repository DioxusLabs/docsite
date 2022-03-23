use crate::icons;
use dioxus::prelude::*;
use dioxus::router::Link;

pub struct BlogPostDisplay {
    category: &'static str,
    date: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
    content: &'static str,
}

impl PartialEq for &'static BlogPostDisplay {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

pub static POSTS: &[BlogPostDisplay] = &[
        BlogPostDisplay {
            category: "Release Notes",
            date: "3 Jan 2022",
            title: "Announcing Dioxus 0.1",
            description: "After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust. It is built around a VirtualDOM, making it portable for the web, desktop, server, mobile, and more.",
            link: "/blog/introducing-dioxus/",
            content: include_str!("../../../posts/release.html"),
        },

        BlogPostDisplay {
            category: "Release Notes",
            date: "Mar 9 2022",
            title: "Announcing Dioxus 0.2",
            description: "Just over two months in, and we already a ton of awesome changes to Dioxus!",
            link: "/blog/release-020/",
            content: include_str!("../../../posts/release020.html"),
        },
    ];

pub static BlogList: Component = |cx| {
    cx.render(rsx!(
        section { class: "text-gray-600 body-font overflow-hidden",
            div { class: "container lg:px-48 pt-12 pb-12 mx-auto",
                div { class: "-my-8 divide-y-2 divide-gray-100",
                    // Header
                    blog_header(),

                    // Individual Post starts here
                    POSTS.iter().rev().enumerate().map(|(id, BlogPostDisplay { category, date, title, description, link, .. })| rsx!{
                        div { class: "py-8 flex flex-wrap md:flex-nowrap",
                            div { class: "md:w-64 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
                                span { class: "font-semibold title-font text-gray-700", "{category}" }
                                span { class: "mt-1 text-gray-500 text-sm", "{date}" }
                            }
                            div { class: "md:flex-grow",
                                h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2", "{title}" }
                                p { class: "leading-relaxed", "{description}" }
                                Link {
                                    class: "text-indigo-500 inline-flex items-center mt-4",
                                    to: "{link}",
                                    "Read more"
                                    icons::ArrowRight {}
                                }
                            }
                        }
                    })
                }
            }
        }
    ))
};

#[inline_props]
pub fn SinglePost(cx: Scope, id: usize) -> Element {
    let BlogPostDisplay { content, .. } = &POSTS[*id];

    cx.render(rsx! {

        section { class: "text-gray-600 body-font overflow-hidden",
            div { class: "container lg:px-20 xl:px-48 pt-12 pb-12 mx-auto",
                div { class: "-my-8 divide-y-2 divide-gray-100",
                    // Header
                    // blog_header()

                    div { class: "flex w-full mb-20 flex-wrap list-none",
                        style {
                            ".markdown-body ul {{ list-style: disc; }}"
                            ".markdown-body li {{ display: list-item; }}"
                        }
                        article { class: "markdown-body", dangerous_inner_html: format_args!("{}", content), }
                        script {"Prism.highlightAll()"}
                    }
                }
            }
        }
    })
}

fn blog_header(cx: Scope) -> Element {
    rsx!(cx,
        section { class: "py-20",
            div { class: "container px-4 mx-auto",

                Link { to: "/blog"
                    h2 { class: "mb-8 md:mb-16 text-5xl lg:text-6xl font-semibold font-heading",
                        "Dioxus Official Blog"
                    }
                }

                div { class: "flex flex-wrap items-center",
                    div { class: "inline-block max-w-xl mb-6 md:mb-0",
                        p { class: "text-xl text-gray-500",
                            "Updates, changelogs, and general musings of the Dioxus community."
                        }
                    }
                    // a { class: "inline-block ml-auto w-full md:w-auto px-12 py-4 text-center text-sm text-white font-medium leading-normal bg-red-400 hover:bg-red-300 rounded",
                    //     href: "#",
                    //     "Save to RSS (WIP)"
                    // }
                }
            }
        }
    )
}

pub static RecentBlogPosts: Component<()> = |cx| {
    cx.render(rsx! {
        section { class: "text-gray-600 body-font overflow-hidden",
            div { class: "container px-6 lg:px-40 py-12 mx-auto",
                div { class: "-my-8 divide-y-2 divide-gray-100",
                    POSTS.iter().enumerate().map(|(id, post)| rsx!{ BlogPostItem { post: post, id: id } })
                }
            }
        }
    })
};

#[inline_props]
fn BlogPostItem(cx: Scope, post: &'static BlogPostDisplay, id: usize) -> Element {
    let BlogPostDisplay {
        category,
        date,
        title,
        description,
        link,
        ..
    } = post;

    cx.render(rsx!(
        div { class: "py-8 flex flex-wrap md:flex-nowrap",
            div { class: "md:w-64 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
                span { class: "font-semibold title-font text-gray-700 dark:text-white", "{category}" }
                span { class: "mt-1 text-gray-500 text-sm", "{date}" }
            }
            div { class: "md:flex-grow",
                h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2 dark:text-white", "{title}" }
                p { class: "leading-relaxed dark:text-white text-base dark:opacity-75", "{description}" }
                Link {
                    class: "text-indigo-500 inline-flex items-center mt-4",
                    to: "{link}",
                    "Read more"
                    icons::ArrowRight {}
                }
            }
        }
    ))
}
