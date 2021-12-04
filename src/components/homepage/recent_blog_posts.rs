use crate::icons;
use dioxus::prelude::*;

pub static RecentBlogPosts: FC<()> = |cx, _props| {
    struct BlogPostDisplay {
        category: &'static str,
        date: &'static str,
        title: &'static str,
        description: &'static str,
        link: &'static str,
    }

    let posts = [
        BlogPostDisplay {
            category: "Release Notes",
            date: "21 Oct 2021",
            title: "Announcing Dioxus 0.1",
            description: "After months of work, we're very excited to release the first version of Dioxus! Dioxus is a new library for building interactive user interfaces with Rust. It is built around a VirtualDOM, making it portable for the web, desktop, server, mobile, and more.",
            // description: "Glossier echo park pug, church-key sartorial biodiesel vexillologist pop-up snackwave ramps cornhole. Marfa 3 wolf moon party messenger bag selfies, poke vaporware kombucha lumbersexual pork belly polaroid hoodie portland craft beer.",
            link: "/blog/announcing_dioxus_01",
        },
        BlogPostDisplay {
            category: "Technical Notes",
            date: "28 Oct 2021",
            title: "WASM, Allocators, and Performance Deep Dive",
            description: "WebAssembly is a promising new web technology that brings a wide variety of programming languages to the web. However, WASM performance is a poorly understood topic - especially when considering memory allocators and interaction with JS APIs.",
            link: "/blog/wasm_allocators_performance",
        },
    ];

    cx.render(rsx!{
        section { class: "text-gray-600 body-font overflow-hidden",
            div { class: "container px-40 pt-0 pb-12 mx-auto",
                div { class: "-my-8 divide-y-2 divide-gray-100",
                    // Individual Post starts here
                    {posts.iter().map(|BlogPostDisplay { category, date, title, description, link }| rsx!{
                        div { class: "py-8 flex flex-wrap md:flex-nowrap",
                            div { class: "md:w-64 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
                                span { class: "font-semibold title-font text-gray-700",
                                    "{category}"
                                }
                                span { class: "mt-1 text-gray-500 text-sm",
                                    "{date}"
                                }
                            }
                            div { class: "md:flex-grow",
                                h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2",
                                    "{title}"
                                }
                                p { class: "leading-relaxed",
                                    "{description}"
                                    // "Glossier echo park pug, church-key sartorial biodiesel vexillologist pop-up snackwave ramps cornhole. Marfa 3 wolf moon party messenger bag selfies, poke vaporware kombucha lumbersexual pork belly polaroid hoodie portland craft beer."
                                }
                                a { class: "text-indigo-500 inline-flex items-center mt-4",
                                    href: "{link}",
                                    "Read more"
                                    icons::ArrowRight {}
                                }
                            }
                        }
                    })}
                }
            }
        }
    })
};
