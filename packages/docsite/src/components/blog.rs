use crate::icons;
use crate::Link;
use crate::{docs::router_blog::BookRoute as BlogRoute, Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn BlogList() -> Element {
    rsx! {
        section { class: "body-font overflow-hidden font-light",
            div { class: "container max-w-screen-md pt-12 pb-12 mx-auto",
                div { class: "",
                    h2 { class: "dark:text-white my-8 text-2xl sm:text-5xl font-medium title-font font-sans",
                        // h2 { class: "dark:text-white my-8 md:mb-16 sm:text-3xl text-2xl font-medium title-font font-sans",
                        "Blog"
                    }
                    section { class: "body-font overflow-hidden",
                        div { class: "container mx-auto space-y-4",
                            for route in BlogRoute::static_routes().into_iter().rev() {
                                BlogPostItem { route }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BlogPostItem(route: BlogRoute) -> Element {
    // coming in the form of:
    // "Announcing Dioxus 0.6 [draft] $ Release Notes $ November 18, 2024 $ Android/iOS simulator, Interactive CLI, RSX Autocomplete, Props Hotreloading, and more!"
    let raw_title = &route.page().title;

    if raw_title.contains("[draft]") {
        return rsx! {};
    }

    let items = raw_title.splitn(4, " $ ").collect::<Vec<_>>();
    let [title, _category, date, description, ..] = items.as_slice() else {
        panic!("Invalid post structure:");
    };

    // "Read more"
    // icons::ArrowRight {}
    // class: "text-indigo-500 inline-flex items-center mt-4",

    rsx! {
        Link {
            to: Route::BlogPost { child: route },
            class: "flex flex-wrap md:flex-nowrap pb-8 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors duration-200 rounded p-2",
            div { class: "md:flex-grow",
                div { class: "flex flex-row justify-between gap-4",
                    h2 { class: "text-2xl font-bold text-gray-900 title-font mb-2 dark:text-white",
                        "{title}"
                    }
                    span { class: "my-2 text-gray-500 text-sm", "{date}" }
                }
                p { class: "leading-relaxed dark:text-white text-base dark:opacity-75",
                    "{description}"
                }
            }
        
        }
    }
}

#[component]
pub(crate) fn BlogPost() -> Element {
    let meta = use_current_blog().unwrap();

    rsx! {
        section { class: "text-gray-600 body-font pt-12 font-light w-full",
            div { class: "flex flex-row justify-center pt-4 md:pt-[3.125rem] lg:gap-12",
                div { class: "hidden xl:block w-72" }
                div { class: "text-gray-600 dark:text-gray-300 body-font overflow-hidden container pb-12 max-w-screen-md px-4 grow min-h-[100vh] md:block",
                    div { class: "px-2 border-b border-gray-200 my-4 mb-8 pb-8  dark:text-white",
                        Link { to: Route::BlogList {},
                            p { class: "pb-12 text-sm flex flex-row gap-2 items-center",
                                svg {
                                    "viewBox": "0 0 16 16",
                                    width: "16",
                                    style: "width: 12px; height: 12px; color: currentcolor;",
                                    "data-testid": "geist-icon",
                                    height: "16",
                                    "stroke-linejoin": "round",
                                    path {
                                        d: "M10.5 14.0607L9.96966 13.5303L5.14644 8.7071C4.75592 8.31658 4.75592 7.68341 5.14644 7.29289L9.96966 2.46966L10.5 1.93933L11.5607 2.99999L11.0303 3.53032L6.56065 7.99999L11.0303 12.4697L11.5607 13L10.5 14.0607Z",
                                        "clip-rule": "evenodd",
                                        fill: "currentColor",
                                        "fill-rule": "evenodd",
                                    }
                                }
                                "Back to blog"
                            }
                        }

                        h1 { class: "text-[2.75rem] font-semibold text-black dark:text-white",
                            "{meta.title}"
                        }
                        p { class: "text-gray-500 text-sm pb-4 dark:text-white",
                            "{meta.date}"
                            " - "
                            "{meta.author}"
                        }
                        h3 { class: "text-[1.5rem] pb-2  dark:text-white", "{meta.description}" }
                    }
                    div { class: "markdown-body px-2  dioxus-blog-post", Outlet::<Route> {} }
                }
                crate::learn::RightNav::<BlogRoute> {}
            }
        }
    }
}

fn use_current_blog() -> Option<BlogMeta> {
    let route = use_route::<Route>();
    let blog_route = match route {
        Route::BlogPost { child } => child,
        _ => return None,
    };
    let page = blog_route.page();
    Some(page_to_meta(page))
}

struct BlogMeta {
    title: &'static str,
    category: &'static str,
    description: &'static str,
    date: &'static str,
    author: &'static str,
}

fn page_to_meta(page: &'static use_mdbook::mdbook_shared::Page<BlogRoute>) -> BlogMeta {
    let raw_title = &page.title;

    let items = raw_title.splitn(4, " $ ").collect::<Vec<_>>();
    let [title, category, date, description, ..] = items.as_slice() else {
        panic!("Invalid post structure:");
    };

    BlogMeta {
        title,
        category,
        description,
        date,
        author: "Jonathan Kelley",
    }
}
