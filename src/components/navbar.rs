use crate::icons;
use dioxus::prelude::*;

pub static NavBar: Component<()> = |cx| {
    cx.render(rsx! {
        section { class: "py-8 px-4 lg:px-10 bg-gray-900",
            nav { class: "relative flex justify-between items-center",
                a { class: "text-2xl text-white font-bold",
                    href: "#",
                    // img { class: "h-7",
                    //     width: "auto",
                    //     alt: "",
                    //     src: "zospace-assets/logos/zospace-logo.svg",
                    // }
                }
                div { class: "lg:hidden",
                    button { class: "p-2 navbar-burger",
                        svg { class: "w-10 h-3",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            height: "13",
                            width: "39",
                            view_box: "0 0 39 13",
                            rect {
                                width: "39",
                                height: "2",
                                rx: "1",
                                fill: "#C4C4C4",
                            }
                            rect {
                                fill: "#C4C4C4",
                                x: "19",
                                height: "2",
                                rx: "1",
                                width: "20",
                                y: "11",
                            }
                        }
                    }
                }
                div { class: "hidden lg:block ml-auto mr-16",
                    ul { class: "flex items-center text-white space-x-10",
                        li {
                            a { class: "text-white font-bold text-lg",
                                href: "#",
                                "Product"
                            }
                        }
                        span {
                            svg {
                                width: "5",
                                view_box: "0 0 5 5",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                height: "5",
                                circle {
                                    cy: "2.5",
                                    r: "2.5",
                                    cx: "2.5",
                                    fill: "#726B6B",
                                }
                            }
                        }
                        li {
                            a { class: "text-white font-bold text-lg",
                                href: "#",
                                "Story"
                            }
                        }
                        span {
                            svg {
                                view_box: "0 0 5 5",
                                width: "5",
                                height: "5",
                                fill: "none",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle {
                                    fill: "#726B6B",
                                    cy: "2.5",
                                    cx: "2.5",
                                    r: "2.5",
                                }
                            }
                        }
                        li {
                            a { class: "text-white font-bold text-lg",
                                href: "#",
                                "Features"
                            }
                        }
                        span {
                            svg {
                                height: "5",
                                width: "5",
                                fill: "none",
                                view_box: "0 0 5 5",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle {
                                    fill: "#726B6B",
                                    cy: "2.5",
                                    cx: "2.5",
                                    r: "2.5",
                                }
                            }
                        }
                        li {
                            a { class: "text-white font-bold text-lg",
                                href: "#",
                                "Contact"
                            }
                        }
                    }
                }
                div { class: "hidden lg:block",
                    a { class: "inline-block px-12 py-4 text-white font-bold border border-gray-200 hover:border-white rounded-full",
                        href: "#",
                        "Sign Up"
                    }
                }
            }
            div { class: "hidden navbar-menu fixed top-0 left-0 bottom-0 w-5/6 max-w-sm z-50",
                div { class: "navbar-backdrop fixed inset-0 bg-gray-800 opacity-80",
                }
                nav { class: "relative flex flex-col py-8 h-full w-full bg-white overflow-y-auto",
                    div { class: "flex items-center mb-16 pr-6",
                        a { class: "ml-10 text-2xl text-gray-800 font-bold",
                            href: "#",
                            img { class: "h-7",
                                width: "auto",
                                alt: "",
                                src: "zospace-assets/logos/zospace-dark-logo.svg",
                            }
                        }
                    }
                    div {
                        ul {
                            li { class: "mb-1 px-10",
                                a { class: "block pl-8 py-4 text-xl text-gray-800 hover:bg-blueGray-50 rounded-xl",
                                    href: "#",
                                    "Product"
                                }
                            }
                            li { class: "mb-1 px-10",
                                a { class: "block pl-8 py-4 text-xl text-gray-800 hover:bg-blueGray-50 rounded-xl",
                                    href: "#",
                                    "Story"
                                }
                            }
                            li { class: "mb-1 px-10",
                                a { class: "block pl-8 py-4 text-xl text-gray-800 hover:bg-blueGray-50 rounded-xl",
                                    href: "#",
                                    "Features"
                                }
                            }
                            li { class: "mb-1 px-10",
                                a { class: "block pl-8 py-4 text-xl text-gray-800 hover:bg-blueGray-50 rounded-xl",
                                    href: "#",
                                    "Contact"
                                }
                            }
                        }
                    }
                    div { class: "mt-auto px-10",
                        div { class: "pt-6",
                            a { class: "block mb-4 py-4 px-12 text-gray-800 text-center font-bold border border-gray-50 hover:border-gray-100 rounded-full",
                                href: "#",
                                "Sign in"
                            }
                            a { class: "block py-4 px-12 text-white text-center font-bold bg-blue-500 hover:bg-blue-600 rounded-full transition duration-200",
                                href: "#",
                                "Sign up"
                            }
                        }
                        p { class: "mt-6 mb-4 text-lg text-center",
                            span {
                                "2021 © Zospace. All rights reserved."
                            }
                        }
                    }
                }
            }
        }



        //     header { class: "text-gray-400 bg-gray-900 body-font",
        //         div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
        //             a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
        //                 img { src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4", class: "h-8 w-auto" },
        //                 span { class: "ml-3 text-xl", "Dioxus Labs" }
        //             }
        //             nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
        //                 a {
        //                     class: "mr-5 hover:text-white",
        //                     href: "https://github.com/DioxusLabs/awesome-dioxus#community",
        //                     "Community"
        //                 }
        //                 a {
        //                     class: "mr-5 hover:text-white",
        //                     href: "/docs/getting-started",
        //                     "Guide"
        //                 }
        //                 a {
        //                     class: "mr-5 hover:text-white",
        //                     href: "https://docs.rs/dioxus",
        //                     "Reference"
        //                 }
        //                 a {
        //                     class: "mr-5 hover:text-white",
        //                     href: "#",
        //                     // onclick: move |_| url.write().0 = "blog",
        //                     "Blog"
        //                 }
        //                 a {
        //                     class: "mr-5 hover:text-white",
        //                     href: "#",
        //                     // onclick: move |_| url.write().0 = "home",
        //                     "Home"
        //                 }
        //             }
        //             a {
        //                 class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
        //                 href: "/book/"
        //                 "Get Started"
        //                 icons::ArrowRight {}
        //             }
        //         }
        //     }
        // })
        })
};
