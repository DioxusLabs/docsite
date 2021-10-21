use crate::{icons, AppRoute};
use dioxus::prelude::*;

pub static NavBar: FC<()> = |(cx, props)| {
    let url = use_shared_state::<crate::AppRoute>(cx)?;

    cx.render(rsx! {
        header { class: "text-gray-400 bg-gray-900 body-font",
            div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                    img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-8 w-auto" },
                    span { class: "ml-3 text-xl", "Dioxus Labs" }
                }
                nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                    a {
                        class: "mr-5 hover:text-white",
                        href: "https://github.com/DioxusLabs/awesome-dioxus#community",
                        "Community"
                    }
                    a {
                        class: "mr-5 hover:text-white",
                        href: "/docs/getting-started",
                        "Guide"
                    }
                    a {
                        class: "mr-5 hover:text-white",
                        href: "https://docs.rs/dioxus",
                        "Reference"
                    }
                    a {
                        class: "mr-5 hover:text-white",
                        href: "#",
                        onclick: move |_| url.write().0 = "blog",
                        "Blog"
                    }
                    a {
                        class: "mr-5 hover:text-white",
                        href: "#",
                        onclick: move |_| url.write().0 = "home",
                        "Home"
                    }
                }
                a {
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    href: "/book/"
                    "Get Started"
                    icons::ArrowRight {}
                }
            }
        }
    })
};
