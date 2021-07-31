use crate::icons;
use super::snippets::*;
use dioxus::prelude::*;

pub static Home: FC<()> = |cx| {
    cx.render(rsx! {
        section { class: "text-gray-400 bg-gray-800 body-font"
            div { class: "container mx-auto pt-24 pb-24",
                div { class: "flex flex-wrap w-full flex-col items-center text-center",
                    h1 { class: "sm:text-6xl text-8xl font-medium title-font mb-2 text-white",
                        "Dioxus"
                    }
                    p { class: "lg:w-1/2 w-full leading-relaxed text-opacity-80 text-4xl",
                        "A Rust library for building user interfaces."
                    }
                    div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center mt-8",
                        button { 
                            class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0 ml-auto text-xl",
                            "Get Started"                                
                        }
                        button { 
                            class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0 mr-auto text-xl",
                            "Take the tutorial"
                            icons::ArrowRight {}
                        }
                    }
                }
            }
        }
        section { class: "text-gray-400 bg-gray-900 body-font"
            div { class: "container mx-auto pb-12 px-40",
                div { class: "flex flex-wrap pt-5 sm:-m-4 -mx-4 -mb-10 -mt-4 md:space-y-0 space-y-6 ",
                    {[
                        ("Declarative", "Easily describe the layout of your application with HTML or RSX syntax - Dioxus will handle the rest."),

                        ("Component-Based", "Build encapsulated components that manage their own state, then compose them to make complex UIs."),

                        ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                        ("Concurrent and Async", "1st class support for powerful asynchronous coroutines."),

                        ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                        ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                        ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                        ("Learn Once, Write Anywhere", "Render on the web, desktop, mobile, terminal, and more!"),

                        
                    ].iter().map(|(title, content)| rsx!(HeroItem { title: title, content: content }))}
                }
            }
        }

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

                    FeaturedExample {}
                    FeaturedExample {}
                    FeaturedExample {}
                }
            }
        }

        section { class: "text-gray-400 bg-gray-900 body-font mx-auto px-48"
            {build_snippets().into_iter().map(|snip| rsx!(RenderSnippet { snippet: snip }))}
        }

        section { class: "text-gray-600 body-font",
            div { class: "container px-5 py-24 mx-auto",
                div { class: "text-center mb-20",
                    h1 { class: "sm:text-3xl text-2xl font-medium text-center title-font text-gray-900 mb-4",
                        "Dioxus is packed with features"
                    }
                    p { class: "text-base leading-relaxed xl:w-2/4 lg:w-3/4 mx-auto",
                        "Built on "
                    }
                }

                div { class: "flex flex-wrap lg:w-4/5 sm:mx-auto sm:mb-2 -mx-2",
                    {(0..10).map(|f| rsx!(CheckedFeature {}))}
                }
                button { class: "flex mx-auto mt-16 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                    "Button"
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

static HeroItem: FC<HeroItemProps> = |cx| {
    cx.render(rsx! {
        div { class: "p-4 md:w-1/4 flex",
            div { class: "flex-grow pl-6",
                icons::Icon3 {}
                h2 { class: "text-white text-lg title-font font-medium mb-2",
                    {[cx.title]}
                }
                {cx.content.split("\n").map(|line| rsx!{
                    p { class: "leading-relaxed text-base pb-4",
                        "{line}"
                    }
                })}
            }
        }
    })
};



#[derive(PartialEq, Props)]
pub struct SnippetProps {
    snippet: Snippet,
}
pub static RenderSnippet: FC<SnippetProps> = |cx| {
    let Snippet {
        title,
        body,
        code,
        caller_id,
    } = &cx.snippet;

    let body = body
        .split("\n")
        .map(|paragraph| rsx!( p{ class: "leading-relaxed text-base pb-4", "{paragraph}"} ))
        .take(3);

    cx.render(rsx! {
        section { class: "text-gray-400 body-font bg-gray-900",
            div { class: "container flex flex-wrap px-5 py-4 mx-auto",
                div { class: "md:w-1/2 md:pr-12 md:py-8 md:border-r md:border-b-0 md:mb-0 mb-10 pb-10 border-b border-gray-800",
                    h1 { class: "sm:text-3xl text-2xl font-medium title-font mb-2 text-white",
                        "{title}"
                    }
                    {body}
                }
                div { class: "flex flex-col md:w-1/2 md:pl-12",
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
};


pub static CheckedFeature: FC<()> = |cx| {
    cx.render(rsx!{
        div { class: "p-2 sm:w-1/2 w-full",
            div { class: "bg-gray-800 rounded flex p-4 h-full items-center",
                icons::IconCheck {}
                span { class: "title-font font-medium text-white",
                    "Coloring Book Ethical"
                }
            }
        }
    })    
};

static FeaturedExample: FC<()> = |cx| {
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
