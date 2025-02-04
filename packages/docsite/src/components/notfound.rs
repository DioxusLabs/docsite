use crate::icons::FERROUS_LOGO;
use crate::*;

#[allow(unused)]
#[component]
pub(crate) fn Err404(segments: Vec<String>) -> Element {
    rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto",
                div { class: "mb-12 text-center",
                    span { class: "text-xs font-semibold text-indigo-500 uppercase",
                        "Error 404"
                    }
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Page not found"
                    }
                    p { class: "mb-8 text-base leading-relaxed lg:text-xl lg:leading-relaxed text-gray-500",
                        "Sorry! We are unable to find the page you are looking for."
                    }
                    div {
                        Link {
                            to: Route::Homepage {},
                            class: "block md:inline-block px-5 py-3 md:mr-3 mb-3 md:mb-0 text-sm bg-indigo-500 hover:bg-indigo-600 text-white font-semibold border border-indigo-500 hover:border-indigo-600 rounded transition duration-200",
                            "Return to homepage"
                        }
                    }
                }
                div { class: "max-w-4xl h-64 md:h-96 mx-auto",
                    img {
                        class: "w-full rounded-lg object-cover",
                        src: FERROUS_LOGO,
                    }
                }
            }
        }
    )
}
