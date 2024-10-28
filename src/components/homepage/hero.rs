use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full dark:bg-ideblack mx-auto dark:text-white flex flex-col justify-between items-center  border-b  min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[960px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)]",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between flex-1 ",
                    div { class: "text-center lg:text-left lg:flex-1",
                        div { class: "text-[2.5em] md:text-[3.5em] font-semibold dark:text-white text-ghdarkmetal font-sans leading-snug text-balance",
                            span { "One codebase, " }
                            span { " every platform." }
                        }
                        h3 { class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md md:text-left text-center flex flex-col",
                            span { class: "max-w-screen-md leading-loose",
                                "Dioxus is "
                                em { "the " }
                                "Rust framework for building fullstack web, desktop, and mobile apps. Iterate with live hotreloading, add server functions, and deploy in record time."
                            }
                        }
                        div { class: "pt-8 lg:pt-16 text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center lg:justify-start",
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::GuideIndex {},
                                },
                                class: "bg-ghdarkmetal dark:bg-[#EDEDED] text-white dark:text-black border border-[#a4a9ac7d] m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto  dark:shadow-white",
                                "Quickstart"
                            }
                            Link {
                                to: Route::Docs {
                                    child: BookRoute::Index {},
                                },
                                class: "bg-[#EDEDED] dark:bg-ghdarkmetal  text-black dark:text-white border border-[#a4a9ac7d]  m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto",
                                "Read the docs"
                            }
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-80",
                        img {
                            src: asset!("/assets/static/multiplatform-dark.svg"),
                            class: "dark:hidden w-full h-full",
                            alt: "Animated Icon",
                        }
                        img {
                            src: asset!("/assets/static/multiplatform-light.svg"),
                            class: "hidden dark:block w-full h-full",
                            alt: "Animated Icon",
                        }
                    }
                }
                div { class: "flex max-w-screen-2xl flex-col justify-end md:flex gap-4 pb-8",
                    h1 { class: "lg:text-left text-center font-extralight text-sm",
                        "Trusted by top companies"
                    }
                    div { class: "flex flex-row flex-wrap lg:justify-start justify-center invert dark:invert-0  gap-8  min-h-0",
                        img {
                            class: "h-6",
                            src: asset!("/assets/static/airbuslogo.svg"),
                        }
                        img {
                            class: "h-6 ",
                            src: asset!("/assets/static/ESA_logo.svg"),
                        }
                        img {
                            class: "h-6 ",
                            src: asset!("/assets/static/xailogo.svg"),
                        }
                        img {
                            class: "h-6 ",
                            src: asset!("/assets/static/yclogo.svg"),
                        }
                        img {
                            class: "h-6 ",
                            src: asset!("/assets/static/futurewei_bw.png"),
                        }
                        img {
                            class: "h-6 ",
                            src: asset!("/assets/static/satellite.webp"),
                        }
                    }
                }
            }
        }
    }
}
