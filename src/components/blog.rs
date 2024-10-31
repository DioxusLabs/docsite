use crate::icons;
use crate::Link;
use crate::{docs::router_blog::BookRoute as BlogRoute, Route};
use dioxus::prelude::*;

#[component]
pub(crate) fn BlogList() -> Element {
    rsx! {
        section { class: "body-font overflow-hidden font-light",
            div { class: "container max-w-screen-md pt-12 pb-12 mx-auto",
                div { class: "-my-8 px-8 pb-12",
                    h2 { class: "dark:text-white my-8 md:mb-16 sm:text-3xl text-2xl font-medium title-font font-sans",
                        "Blog"
                    }
                    section { class: "body-font overflow-hidden",
                        div { class: "container px- mx-auto",
                            div { class: "-my-8 divide-y divide-neutral-400",
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
}

#[component]
pub(crate) fn BlogPost() -> Element {
    rsx! {
        section { class: "text-gray-600 body-font max-w-screen-md mx-auto pt-24 font-light",
            div { class: "markdown-body px-2  dioxus-blog-post", Outlet::<Route> {} }
        }
    }
}

#[component]
fn BlogPostItem(route: BlogRoute) -> Element {
    let post = &route.page().title;
    let items = post.splitn(4, " $ ").collect::<Vec<_>>();
    let [title, _category, date, description, ..] = items.as_slice() else {
        panic!("Invalid post structure:");
    };

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
                    to: Route::BlogPost { child: route },
                    "Read more"
                    icons::ArrowRight {}
                }
            }
        }
    }
}
