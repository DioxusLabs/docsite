#![allow(non_snake_case, non_upper_case_globals)]

use super::snippets::*;
use crate::icons;
use dioxus::prelude::*;

pub static Home: FC<()> = |(cx, props)| {
    cx.render(rsx! {
        Hero {}
        ValueAdd {}
        FeaturedExamples {}
        Snippets {}
    })
};

fn Hero((cx, _): Component<()>) -> DomTree {
    cx.render(rsx!{
        section { class: "text-gray-400 bg-gray-800",
            div { class: "container flex flex-col md:flex-row w:2/3 px-24 py-20 mx-auto",
                div { class: "flex flex-col md:pr-10 md:mb-0 mb-6 pr-0 w-full md:w-auto md:text-left text-center w:1/2",
                    h1 { class: "sm:text-6xl text-16xl font-medium title-font mb-2 text-white", "Dioxus" }
                    p { class: "leading-relaxed text-white text-4xl", "A Rust library for building user interfaces." }
                    p {
                        class: "leading-relaxed text-opacity-90 text-xl py-5",
                        "Build reliable, fast, and scalable user interfaces that run on the web, desktop, mobile, server,
                        and more."
                    }
                    div { class: "container flex flex-wrap p-5 flex-col md:flex-row mx-0 px-0",
                        button {
                            class: "inline-flex items-center text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600",
                            "Read the book"
                            icons::ArrowRight {}
                        }
                    }
                }
                div { class: "flex flex-col mx-auto w:1/2 flex-shrink-0",
                    div { class: "pt-4"
                        pre {
                            padding_bottom: "0px"
                            margin_bottom: "0px"
                            code { class: "language-rust", {[include_str!("../snippets/homepage.rs")]} }
                        }
                    }
                    InteractiveHeader {}
                }
            }
        }
    })
}
static InteractiveHeader: FC<()> = |(cx, props)| {
    let mut count = use_state(cx, || 0);

    cx.render(rsx!{
        div { class: "flex flex-col items-center py-3" background_color: "hsl(220, 13%, 18%)"
            div { class: "pb-3"
                h1 { "High-Five counter: {count}" }
            }
            div { class: "flex flex-row items-center"
                button {
                    class: "inline-flex items-center text-white bg-green-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count += 1, "Up high!"
                }
                img { class: "h-12 mx-4" src: "https://rustacean.net/assets/rustacean-flat-gesture.png" }
                button {
                    class: "inline-flex items-center text-white bg-red-500 border-0 py-1 px-4 focus:outline-none hover:bg-gray-600"
                    onclick: move |_| count -= 1, "Down low!"
                }
            }
        }
    })
};

fn ValueAdd((cx, props): Component<()>) -> DomTree {
    rsx!(cx, section { class: "text-gray-400 body-font"
        // section { class: "text-gray-400 bg-gray-900 body-font"
        div { class: "container mx-auto pb-12 px-40",
            div { class: "flex flex-wrap pt-5 sm:-m-4 -mx-4 -mb-10 -mt-4 md:space-y-0 space-y-6 ",
                {[
                    ("Declarative", "Easily describe the layout of your application with HTML or RSX syntax - Dioxus will handle the rest."),

                    ("Component-Based", "Build encapsulated components that manage their own state, then compose them to make complex UIs."),

                    ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                    ("Concurrent and Async", "1st class support for powerful asynchronous coroutines."),

                    ("Static Types Everywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                    ("If it compiles, it works", "Render on the web, desktop, mobile, terminal, and more!"),

                    ("First-class error handling", "Render on the web, desktop, mobile, terminal, and more!"),

                    ("Incredible inline documentation", "Render on the web, desktop, mobile, terminal, and more!"),


                ].iter().map(|(title, content)| rsx!(HeroItem { title: title, content: content }))}
            }
        }
    })
}

fn FeaturedExamples((cx, props): Component<()>) -> DomTree {
    cx.render(rsx!{
            // section { class: "text-gray-600 body-font",
            //     div { class: "container px-5 py-24 mx-auto",
            //         div { class: "flex flex-wrap w-full mb-20",
            //             div { class: "lg:w-1/2 w-full mb-6 lg:mb-0",
            //                 h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-2 text-gray-900",
            //                     "Pitchfork Kickstarter Taxidermy"
            //                 }
            //                 div { class: "h-1 w-20 bg-indigo-500 rounded",
            //                 }
            //             }
            //             p { class: "lg:w-1/2 w-full leading-relaxed text-gray-500",
            //                 "Whatever cardigan tote bag tumblr hexagon brooklyn asymmetrical gentrify, subway tile poke farm-to-table. Franzen you probably haven't heard of them man bun deep jianbing selfies heirloom prism food truck ugh squid celiac humblebrag."
            //             }
            //         }
            //         div { class: "flex flex-wrap -m-4",
            //             FeaturedExample {}
            //             FeaturedExample {}
            //             FeaturedExample {}
            //             FeaturedExample {}
            //         }
            //     }
            // }



        section { class: "text-gray-400 bg-gray-900 body-font"
            div { class: "container px-40 py-24 mx-auto",
                div { class: "flex flex-col text-center w-full mb-20",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-4 text-white-900",
                        "Feature-packed examples"
                    }
                    p { class: "lg:w-2/3 mx-auto leading-relaxed text-base",
                        "Whatever cardigan tote bag tumblr hexagon brooklyn asymmetrical gentrify, subway tile poke farm-to-table. Franzen you probably haven't heard of them man bun deep jianbing selfies heirloom."
                    }
                }
                div { class: "flex flex-wrap -m-4"

                    FeaturedExample {}
                    FeaturedExample {}
                    FeaturedExample {}

                    // FeaturedExample {}
                    // FeaturedExample {}
                    // FeaturedExample {}
                }
            }
        }
    })
}

static Snippets: FC<()> = |(cx, _)| {
    let (snippets, _) = use_state(cx, build_snippets).classic();
    let selected_snippet = use_state(cx, || 0);

    let chosen_snippet = snippets.get(*selected_snippet).unwrap();

    let snip_list = snippets.iter().enumerate().map(|(id, s)| {
        rsx!(li {
            onclick: move |_| selected_snippet.set(id),
            "{s.title}"
        })
    });

    cx.render(rsx! {
        section { class: "text-gray-400 bg-gray-900 body-font mx-auto px-48 "
            div {
                ul { {snip_list} }
            }
            div {
                {
                    {snippets.iter().enumerate().map(|(id, f)| {
                        let show = if id == *selected_snippet {"block"} else {"none"};
                        rsx!(div { style: "display: {show};"
                            RenderSnippet { snippet: f }
                        })
                    })}
                }
            }
        }
    })
};

#[derive(PartialEq, Props)]
struct HeroItemProps {
    title: &'static str,
    content: &'static str,
}

static HeroItem: FC<HeroItemProps> = |(cx, props)| {
    cx.render(rsx! {
        div { class: "p-4 md:w-1/4 flex",
            div { class: "flex-grow pl-6",
                icons::Icon3 {}
                h2 { class: "text-black text-lg title-font font-medium mb-2",
                    "{props.title}"
                }
                {props.content.split('\n').map(|line| rsx!{
                    p { class: "leading-relaxed text-base pb-4",
                        "{line}"
                    }
                })}
            }
        }
    })
};

#[derive(PartialEq, Props)]
pub struct SnippetProps<'a> {
    snippet: &'a Snippet,
}
fn RenderSnippet<'a>((cx, props): Component<'a, SnippetProps>) -> DomTree<'a> {
    let Snippet {
        title,
        body,
        code,
        caller_id: _,
    } = &props.snippet;

    let body = body
        .split("\n")
        .map(|paragraph| rsx!( p{ class: "leading-relaxed text-base pb-4", "{paragraph}"} ))
        .take(3);

    cx.render(rsx! {
        section { class: "text-gray-400 body-font bg-gray-900",
            div { class: "container px-5 py-4 mx-auto",
            // div { class: "container flex flex-wrap px-5 py-4 mx-auto",
                // div { class: "md:pr-12 md:py-8 md:border-r md:border-b-0 md:mb-0 mb-10 pb-10 border-b border-gray-800",
                div { class: "md:w-1/3 md:pr-12 md:py-8 md:border-r md:border-b-0 md:mb-0 mb-10 pb-10 border-b border-gray-800",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-2 text-white",
                        "{title}"
                    }
                    {body}
                }
                // div { class: "flex flex-col md:pl-12",
                div { class: "flex flex-col md:w-2/3 md:pl-12",
                    div { class: "pt-4"
                        pre {
                            code {
                                class: "language-rust"
                                "{code}"
                            }
                        }
                    }
                }
            }
        }
    })
}

static FeaturedExample: FC<()> = |(cx, props)| {
    cx.render(rsx!{
        div { class: "lg:w-1/3 sm:w-1/2 p-4",
            div { class: "flex relative",
                img { class: "absolute inset-0 w-full h-full object-cover object-center",
                    alt: "gallery",
                    src: "https://dummyimage.com/606x366",
                }
                div { class: "px-8 py-10 relative z-10 w-full border-4 border-gray-200 bg-white opacity-0 hover:opacity-100",
                    h2 { class: "tracking-widest text-sm title-font font-medium text-indigo-500 mb-1",
                        "THE SUBTITLE"
                    }
                    h1 { class: "title-font text-lg font-medium text-gray-900 mb-3",
                        "Alper Kamu"
                    }
                    p { class: "leading-relaxed",
                        "Photo booth fam kinfolk cold-pressed sriracha leggings jianbing microdosing tousled waistcoat."
                    }
                }
            }
        }
    })
};
