use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full dark:bg-ideblack h-fit [@media(min-height:720px)]:h-[calc(100vh)] flex flex-col justify-between items-center py-16",
            div { class: "pb-12 md:px-12 text-center w-full flex flex-col h-full max-w-screen-2xl",
                div { class: "flex-grow" }
                div { class: "flex flex-row justify-start w-full pb-24",
                    div { class: "text-left flex-grow",
                        div { class: "text-[3em] md:text-[3em] font-semibold dark:text-white text-ghdarkmetal font-sans flex flex-col",
                            span {
                                "Dioxus is "
                                em { "the " }
                                "the Rust framework for"
                            }
                            span { "fullstack web, desktop, and mobile apps." }
                        }
                        h3 { class: "text-[1.5em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md text-left",
                            "Ship beautiful, robust, cross-platform apps in record time."
                        }

                        div { class: "pt-24 text-white text-[1.2em] font-sans font-bold flex flex-col md:flex-row md:space-x-4 md:space-y-0 space-y-4",
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::GettingStartedIndex {},
                                },
                                class: "bg-dxorange m-0 p-2 px-4 rounded md:hover:-translate-y-2 transition-transform duration-300 w-full md:w-auto",
                                "Quickstart"
                            }
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::ReferenceIndex {},
                                },
                                class: "bg-dxblue m-0 p-2 px-4 rounded md:hover:-translate-y-2 transition-transform duration-300 w-full md:w-auto",
                                "Read the docs"
                            }
                        }
                    }
                    div { class: "flex-grow",
                        img {
                            src: "/static/multiplatform-dark.svg",
                            class: "mx-auto max-h-48 lg:max-h-96 dark:hidden",
                            alt: "Animated Icon",
                        }
                        img {
                            src: "/static/multiplatform-light.svg",
                            class: "mx-auto max-h-48 lg:max-h-96 hidden dark:block",
                            alt: "Animated Icon",
                        }
                    }
                }
                div { class: "flex-grow max-w-screen-2xl flex flex-col justify-end",
                    h1 { class: "text-md text-left", "Trusted by top companies" }
                    div { class: "pt-4 flex flex-row flex-wrap justify-start invert dark:invert-0",
                        div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                            img { src: "/static/futurewei_bw.png" }
                        }
                        div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                            img { src: "/static/airbuslogo.svg" }
                        }
                        div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                            img { src: "/static/ESA_logo.svg" }
                        }
                        div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                            img { src: "/static/yclogo.svg" }
                        }
                        div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                            img { src: "/static/satellite.webp" }
                        }
                    }
                }
            }
        }
    }
}
