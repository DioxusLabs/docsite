use crate::icons::FERROUS_LOGO;
use crate::*;
use dioxus::prelude::*;

pub static CallToAction: Component<()> = |cx| {
    rsx! {
        section { class: "text-gray-400 bg-ideblack body-font",
            div { class: "container px-5 py-12 mx-auto",
                div { class: "lg:w-2/3 flex flex-col sm:flex-row items-start mx-auto",
                    h1 { class: "flex-grow pb-4 sm:pr-16 text-2xl font-medium title-font text-white text-center sm:text-left",
                        "Start building today, with Dioxus."
                    }
                    img { class: "h-12 mx-4", src: "{FERROUS_LOGO}" }
                    Link {
                        to: Route::Docs {
                            child: BookRoute::GettingStartedIndex {},
                        },
                        button { class: "flex-shrink-0 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0",
                            "Get started"
                        }
                    }
                }
            }
        }
    }
};
