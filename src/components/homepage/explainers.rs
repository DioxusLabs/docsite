use dioxus::prelude::*;

#[component]
pub fn Explainer(invert: bool, title: &'static str, content: Element, flasher: Element) -> Element {
    // pt-5 sm:pt-24 lg:pt-24

    let mut right = rsx! {
        div { class: "relative w-1/2", {flasher} }
    };

    let align = match invert {
        true => "mr-auto ml-16",
        false => "ml-auto mr-16",
    };

    let mut left = rsx! {
        div { class: "relative w-1/2 {align} max-w-md leading-8",
            h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-heading font-sans font-bold",
                "{title}"
            }
            {content}
        }
    };

    if invert {
        std::mem::swap(&mut left, &mut right);
    }

    rsx! {
        div { class: "flex flex-wrap items-center dark:text-white py-16 border-t font-light",
            {left},
            {right}
        }
    }
}

/*

Content
-------
1. Similar React, small learning curve
2. Very quick, optimized
3. Multiplatform as a priority
4. Good Documentation
5. Good Dev Tooling
6. Investing in the ecosystem

*/
pub fn Explainers() -> Element {
    rsx! {
        section { class: "dark:text-white py-16 border-t font-light",
            div { class: "max-w-screen-xl mx-auto flex flex-wrap items-center",
                div { class: "w-1/2 relative bg-red",
                    h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-heading font-sans font-bold",
                        "Inspired by React, made for Rust"
                    }
                    p { class: "mb-6",
                        "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components."
                    }
                    p { class: "mb-3",
                        strong { class: "font-bold", "Know React? You already know Dioxus." }
                        " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
                    }
                }
                div { class: "w-1/2 relative bg-blue",
                    div { class: "w-full relative flex-auto bg-slate-800 dark:bg-slate-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 rounded",
                        pre { class: "flex min-h-full text-sm leading-6",
                            code { class: "language-rust line-numbers rounded",
                                {include_str!("../../../snippets/homepage.rs")}
                            }
                        }
                    }
                }
            }

            div { class: "max-w-screen-xl mx-auto flex flex-wrap items-center",
                div { class: "w-1/2 relative bg-red",
                    h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-heading font-sans font-bold",
                        "Inspired by React, made for Rust"
                    }
                    p { class: "mb-6",
                        "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components."
                    }
                    p { class: "mb-3",
                        strong { class: "font-bold", "Know React? You already know Dioxus." }
                        " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
                    }
                }
                div { class: "w-1/2 relative bg-blue",
                    div { class: "w-full relative flex-auto bg-slate-800 dark:bg-slate-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 rounded",
                        pre { class: "flex min-h-full text-sm leading-6",
                            code { class: "language-rust line-numbers rounded",
                                {include_str!("../../../snippets/homepage.rs")}
                            }
                        }
                    }
                }
            }
        }
    }
}

// Explainer { invert: false, title: "Inspired by React, made for Rust",
//     flasher: rsx! {
//         div { class: "w-full relative flex-auto bg-slate-800 dark:bg-slate-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10",
//             pre { class: "flex min-h-full text-sm leading-6",
//                 code { class: "language-rust line-numbers", [include_str!("../../../snippets/homepage.rs")] }
//             }
//         }
//     },
//     content: rsx! {
//         p { class: "mb-6", "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components." }
//         p { class: "mb-3",
//             strong { class: "font-bold", "Know React? You already know Dioxus." }
//             " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
//         }
//     }
// }

// Explainer { invert: true, title: "A fresh take on multiplatform",
//     flasher: rsx!{ img { class: "w-24", src: "/static/Untitled.png" } },
//     content: rsx!{
//         p { class: "mb-6", "Why learn an entirely new framework or language just to add mobile support to your app? Dioxus makes multi-platform apps a high priority." }
//         p { class: "mb-3", " Easily bridge into any webview (desktop, mobile), render via WASM (web), pre-render to a string (HTML/SSR), or stream DOM manipulation over the network (LiveView). Try TUI or WGPU for a native experience!" }
//     }
// }

// Explainer { invert: false, title: "Top notch documentation",
//     flasher: rsx!{
//         video {
//             muted: "true",
//             autoplay: "true",
//             "loop": "true",
//             playsinline: "true",
//             src: "https://d33wubrfki0l68.cloudfront.net/b5f28d71a3dead99d229d442c3ca9e9a51b06777/d0bf9/img/homepage/reactrefresh.mp4"
//         }
//         // img { class: "w-24",
//         //     src: "https://d33wubrfki0l68.cloudfront.net/d74da08f08b4a17c368b58d36ee23c368b4a6819/fff62/img/homepage/phones.png"
//         // }
//     },
//     content: rsx!{
//         p { class: "mb-6", "Plenty of guides and tutorials to help you get started with Dioxus." }
//         p { class: "mb-3", " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more." }
//     }
// }

// Explainer { invert: true, title: "Powerful dev tooling",
//     flasher: rsx!{
//         video {
//             muted: "true",
//             autoplay: "true",
//             "loop": "true",
//             playsinline: "true",
//             src: "https://d33wubrfki0l68.cloudfront.net/b5f28d71a3dead99d229d442c3ca9e9a51b06777/d0bf9/img/homepage/reactrefresh.mp4"
//         }
//     },
//     content: rsx!{
//         p { class: "mb-6",
//             "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components."
//         }

//         p { class: "mb-3",
//             strong { "Know React? You already know Dioxus." }
//             " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
//         }
//     }
// }

// Explainer { invert: false,title: "Fun to use",
//     flasher: rsx!{
//         img { class: "w-24",
//             src: "https://d33wubrfki0l68.cloudfront.net/d74da08f08b4a17c368b58d36ee23c368b4a6819/fff62/img/homepage/phones.png"
//         }
//     },
//     content: rsx!{
//         p { class: "mb-6",
//             "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components."
//         }

//         p { class: "mb-3",
//             strong { "Know React? You already know Dioxus." }
//             " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
//         }
//     }
// }

// Explainer { invert: true, title: "Completing the ecosystem",
//     flasher: rsx!{
//         img { class: "w-24",
//             src: "https://d33wubrfki0l68.cloudfront.net/d74da08f08b4a17c368b58d36ee23c368b4a6819/fff62/img/homepage/phones.png"
//         }
//     },
//     content: rsx!{
//         p { class: "mb-6",
//             "Dioxus apps are built declaratively with an ergonomic JSX-like syntax. Reusability is achieved through functional hooks and encapsulated components."
//         }

//         p { class: "mb-3",
//             strong { "Know React? You already know Dioxus." }
//             " Dioxus borrows the same concepts like hooks, components, elements, context, suspense, and more."
//         }
//     }
// }
