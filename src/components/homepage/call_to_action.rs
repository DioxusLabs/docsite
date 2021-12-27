use dioxus::prelude::*;
use crate::icons::FERROUS_LOGO;

pub static CallToAction: Component<()> = |cx| {
    cx.render(rsx!{
        section { class: "text-gray-400 bg-gray-800 body-font",
            div { class: "container px-5 py-12 mx-auto",
                div { class: "lg:w-2/3 flex flex-col sm:flex-row sm:items-center items-start mx-auto",
                    img { class: "h-12 mx-4" src: format_args!("{}", FERROUS_LOGO) }
                    h1 { class: "flex-grow sm:pr-16 text-2xl font-medium title-font text-white",
                        "Get started building robust and portable apps with Dioxus."
                    }
                    a {
                        href: "https://dioxuslabs.com/guide/",
                        button { class: "flex-shrink-0 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0",
                            "Get started"
                        }
                    }
                }
            }
        }        
    })
};
