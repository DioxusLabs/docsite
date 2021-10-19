use crate::icons;
use dioxus::prelude::*;

pub static NavBar: FC<()> = |(cx, props)| {
    cx.render(rsx! {
        header { class: "text-gray-400 bg-gray-900 body-font",
            div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                    icons::Stacks {}
                    span { class: "ml-3 text-xl", "DioxusLabs" }
                }
                nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                    a { class: "mr-5 hover:text-white", href: "/community/support", "Community" }
                    a { class: "mr-5 hover:text-white", href: "/docs/getting-started", "Guides" }
                    a { class: "mr-5 hover:text-white", href: "/docs/getting-started", "Reference" }
                    a { class: "mr-5 hover:text-white", href: "/tutorial/tutorial", "Tutorial" }
                    a { class: "mr-5 hover:text-white", href: "/blog/", "Blog" }                
                }
                button { 
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    "Get Started"
                    icons::ArrowRight {}
                }
            }
        }
    })
};
