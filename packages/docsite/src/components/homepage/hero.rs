use crate::docs::router_06::BookRoute;
use crate::*;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center  border-b  border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] max-h-[960px] px-4",
            div { class: "flex w-full max-w-screen-xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)] gap-2 justify-evenly",
                div { class: "flex flex-col-reverse lg:flex-row items-center justify-end lg:justify-between lg:flex-1 flex-none",
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
                                to: Route::Docs06 {
                                    child: BookRoute::Index {
                                        section: Default::default(),
                                    },
                                },
                                class: "bg-ghdarkmetal dark:bg-[#EDEDED] text-white dark:text-black border border-[#a4a9ac7d] m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto  dark:shadow-white",
                                "Get started"
                            }
                            Link {
                                to: "https://www.youtube.com/watch?v=WgAjWPKRVlQ",
                                new_tab: true,
                                class: "bg-[#EDEDED] dark:bg-ghdarkmetal text-black dark:text-white border border-[#a4a9ac7d] m-0 ml-2 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto gap-2 flex flex-row items-center justify-center",
                                "Take a tour"
                                span {
                                    svg {
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "1.5rem",
                                        height: "1.5rem",
                                        circle {
                                            "stroke-width": "1.5",
                                            cx: "12",
                                            r: "10",
                                            stroke: "currentColor",
                                            cy: "12",
                                        }
                                        path {
                                            stroke: "currentColor",
                                            fill: "currentColor",
                                            "stroke-width": "1.5",
                                            d: "M15.4137 10.941C16.1954 11.4026 16.1954 12.5974 15.4137 13.059L10.6935 15.8458C9.93371 16.2944 9 15.7105 9 14.7868L9 9.21316C9 8.28947 9.93371 7.70561 10.6935 8.15419L15.4137 10.941Z",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8",
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
                        // img {
                        //     class: "h-6 ",
                        //     src: asset!("/assets/static/xailogo.svg"),
                        // }
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

// pub(crate) fn Hero() -> Element {
//     let abc = 21;
//     rsx! {
//         section { class: "w-full mx-auto dark:text-white flex flex-col justify-between items-center  border-b  border-gray-300 min-h-[760px] flex-1 dark:border-[#a4a9ac7d] ",
//             div { class: "flex w-full max-w-screen-2xl flex-col text-center md:min-h-[520px] min-h-[760px] h-[calc(100vh-4rem)] lg:px-8 pb-12",
//                 div { class: "flex flex-col lg:flex-1 items-center flex-none pt-20",
//                     div { class: "text-center",
//                         div { class: "text-[2.5em] md:text-[4.5em] lg:text-[5.5em] dark:text-white text-ghdarkmetal tracking-tight font-sans leading-relaxed text-balance",
//                             span { "One codebase, " }
//                             span { " every platform." }
//                         }
//                         h3 { class: "text-[1.25em] dark:text-white font-light text-ghdarkmetal max-w-screen-sm md:max-w-screen-md text-center flex flex-col mx-auto pb-8",
//                             span { class: "max-w-screen-md leading-loose",
//                                 "Rapidly build fullstack web, desktop, and mobile apps in Rust."
//                             }
//                         }
//                         div { class: "text-[1em] flex flex-row space-x-4 mx-auto lg:mx-0 justify-center pb-16 ",
//                             Link {
//                                 to: Route::Docs06 {
//                                     child: BookRoute::Index {
//                                         section: Default::default(),
//                                     },
//                                 },
//                                 class: "bg-ghdarkmetal dark:bg-[#EDEDED] text-white dark:text-black border border-[#a4a9ac7d] m-0 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto  dark:shadow-white",
//                                 "Get started"
//                             }
//                             Link {
//                                 to: "https://www.youtube.com/watch?v=WgAjWPKRVlQ",
//                                 new_tab: true,
//                                 class: "bg-[#EDEDED] dark:bg-ghdarkmetal text-black dark:text-white border border-[#a4a9ac7d] m-0 ml-2 p-2 px-4 rounded md:hover:-translate-y-1 transition-transform duration-300 w-full md:w-auto flex flex-row items-center justify-center",
//                                 "Take a tour"
//                                 span {
//                                     svg {
//                                         fill: "none",
//                                         "viewBox": "0 0 24 24",
//                                         xmlns: "http://www.w3.org/2000/svg",
//                                         circle {
//                                             "stroke-width": "1.5",
//                                             cx: "12",
//                                             r: "10",
//                                             stroke: "currentColor",
//                                             cy: "12",
//                                         }
//                                         path {
//                                             stroke: "currentColor",
//                                             fill: "currentColor",
//                                             "stroke-width": "1.5",
//                                             d: "M15.4137 10.941C16.1954 11.4026 16.1954 12.5974 15.4137 13.059L10.6935 15.8458C9.93371 16.2944 9 15.7105 9 14.7868L9 9.21316C9 8.28947 9.93371 7.70561 10.6935 8.15419L15.4137 10.941Z",
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     div { class: "w-full max-w-[1180px]",
//                         img {
//                             src: asset!("/assets/static/image-splash.avif"),
//                             alt: "Animated Icon",
//                         }
//                     }
//                     div { class: "lg:pb-12 h-screen max-h-40 lg:max-h-80 my-8 mx-auto hidden",
//                         // img {
//                         //     src: asset!("/assets/static/multiplatform-dark.svg"),
//                         //     class: "dark:hidden h-full",
//                         //     alt: "Animated Icon",
//                         // }
//                         // img {
//                         //     src: asset!("/assets/static/multiplatform-light.svg"),
//                         //     class: "hidden dark:block h-full",
//                         //     alt: "Animated Icon",
//                         // }
//                         img {
//                             src: asset!("/assets/static/image-splash.avif"),
//                             class: "hidden dark:block h-full",
//                             alt: "Animated Icon",
//                         }
//                     }
//                 }
//             }
//             div { class: "flex max-w-screen-2xl flex-col justify-end gap-4 pb-8",
//                 h1 { class: "lg:text-left text-center font-extralight text-sm",
//                     "Trusted by top companies"
//                 }
//                 div { class: "flex flex-row flex-wrap lg:justify-start justify-center invert dark:invert-0  gap-8  min-h-0",
//                     img {
//                         class: "h-6",
//                         src: asset!("/assets/static/airbuslogo.svg"),
//                     }
//                     img {
//                         class: "h-6 ",
//                         src: asset!("/assets/static/ESA_logo.svg"),
//                     }
//                     // img {
//                     //     class: "h-6 ",
//                     //     src: asset!("/assets/static/xailogo.svg"),
//                     // }
//                     img {
//                         class: "h-6 ",
//                         src: asset!("/assets/static/yclogo.svg"),
//                     }
//                     img {
//                         class: "h-6 ",
//                         src: asset!("/assets/static/futurewei_bw.png"),
//                     }
//                     img {
//                         class: "h-6 ",
//                         src: asset!("/assets/static/satellite.webp"),
//                     }
//                 }
//             }
//         }
//     }
// }
