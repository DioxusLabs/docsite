use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full dark:bg-ideblack mx-auto dark:text-white flex flex-col justify-between items-center pt-8 md:pt-32 border-b md:mb-32 mb-16 pb-4 min-h-[640px] flex-1",
            div { class: "mx-auto flex w-full max-w-screen-2xl flex-col text-center md:min-h-[640px] min-h-[760px] md:px-12",
                div { class: "flex-1" }
                div { class: "flex-1 flex flex-row justify-start w-full px-8 md:px-0",
                    div { class: "text-center md:text-left flex-grow",
                        div { class: "text-[2.5em] md:text-[4em] font-semibold dark:text-white text-ghdarkmetal font-sans",
                            span { "One codebase, " }
                            span { " every platform." }
                        }
                        h3 { class: "text-[1.25em] dark:text-white font-extralight text-ghdarkmetal md:pt-4 pt-8 max-w-screen-sm md:max-w-screen-md md:text-left text-center",
                            span {
                                "Build fullstack web, desktop, and mobile apps with one unified codebase."
                            }
                        }

                        div { class: "pt-12 text-white text-[1em] md:text-[1.2em] font-sans font-bold flex flex-col md:flex-row md:space-x-4 md:space-y-0 space-y-4 md:w-full max-w-[200px] md:max-w-[360px] mx-auto md:mx-0",
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::Index {},
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
                    AnimatedLogo {}
                }
                div { class: "flex-1" }
                div { class: "flex-1 max-w-screen-2xl flex-col justify-end md:flex gap-4",
                    h1 { class: "text-md md:text-left text-center", "Trusted by top companies" }
                    div { class: "flex flex-row flex-wrap md:justify-start justify-center invert dark:invert-0 mb-4 md:mb-12 md:gap-12 gap-4 px-4 my-4 min-h-0",
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/airbuslogo.svg",
                        }
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/ESA_logo.svg",
                        }
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/xailogo.svg",
                        }
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/yclogo.svg",
                        }
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/futurewei_bw.png",
                        }
                        img {
                            class: "h-6 md:h-8",
                            src: "/assets/static/satellite.webp",
                        }
                    }
                }
            }
        }
    }
}

fn AnimatedLogo() -> Element {
    rsx! {
        div { class: "flex-grow",
            img {
                src: asset!("/assets/static/multiplatform-dark.svg"),
                class: "mx-auto max-h-48 lg:max-h-96 dark:hidden",
                alt: "Animated Icon",
            }
            img {
                src: asset!("/assets/static/multiplatform-light.svg"),
                class: "mx-auto max-h-48 lg:max-h-96 hidden dark:block",
                alt: "Animated Icon",
            }
        }
    }
}
