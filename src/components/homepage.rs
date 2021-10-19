use crate::icons;
use super::snippets::*;
use dioxus::prelude::*;

pub static Home: FC<()> = |(cx, props)| {
    cx.render(rsx! {
        {Broadtitle(cx)}

        {ValueAdd(cx)}

        {FeaturedExamples(cx)}

        Snippets {}

        // {Features(cx)}
    })
};


fn Broadtitle(cx: Context) -> DomTree {
    cx.render(rsx! {
        section { class: "text-blueGray-700",
            div { class: "container flex flex-col items-center px-5 py-8 mx-auto",
                div { class: "flex flex-col w-full mb-12 text-left lg:text-center",
                    h1 { class: "mx-auto mb-12 text-2xl font-semibold leading-none tracking-tighter text-black lg:w-1/2 sm:text-6xl title-font",
                        "A Long headline to switch your visitors into users."
                    }
                    p { class: "mx-auto text-base font-medium leading-relaxed text-blueGray-700 lg:w-1/2",
                        "You're about to launch soon and must be 100% focused on your product. Don't loose precious days"
                            "designing, coding the landing page and testing ."
                    }
                    div { class: "grid w-full grid-cols-2 gap-8 mx-auto my-16 text-center lg:grid-cols-4 lg:w-1/2",
                        div { class: "flex items-center justify-center",
                            img { class: "block object-contain h-12",
                                src: "https://d33wubrfki0l68.cloudfront.net/e018a9cb3c198579040cd7c76efc4319d9d73065/78dee/logos/notion.svg",
                                alt: "Todoist Logo",
                            }
                        }
                        div { class: "flex items-center justify-center",
                            img { class: "block object-contain h-12",
                                src: "https://d33wubrfki0l68.cloudfront.net/3c4dca12c4e61997e31aa2810408ec46ea6b300c/35012/logos/jb_rider.svg",
                                alt: "Slack Logo",
                            }
                        }
                        div { class: "flex items-center justify-center",
                            img { class: "block object-contain h-12",
                                alt: "Typeform Logo",
                                src: "https://d33wubrfki0l68.cloudfront.net/f89a5bbfd36f091aa28386a9a5d00beeac739466/a67a2/logos/marvel.svg",
                            }
                        }
                        div { class: "flex items-center justify-center",
                            img { class: "block object-contain h-12",
                                src: "https://d33wubrfki0l68.cloudfront.net/00e9e5a3abb0f4a67423ee14f9822db4f0e22172/46c6b/logos/paypal.svg",
                                alt: "Algolia Logo",
                            }
                        }
                    }
                }
            }
        }
    })
    // // section { class: "text-gray-400 body-font"
    // rsx!(cx, section { class: "text-gray-400 bg-gray-800 body-font"
    //     div { class: "container mx-auto pt-24 pb-24",
    //         div { class: "flex flex-wrap w-full flex-col items-center text-center",
    //             h1 { class: "sm:text-6xl text-8xl font-medium title-font mb-2 text-white",
    //                 "Dioxus"
    //             }
    //             p { class: "lg:w-1/2 w-full leading-relaxed text-opacity-80 text-4xl",
    //                 "A Rust library for building user interfaces."
    //             }
    //             div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center mt-8",
    //                 button { 
    //                     class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0 ml-auto text-xl",
    //                     "Get Started"                                
    //                 }
    //                 button { 
    //                     class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0 mr-auto text-xl",
    //                     "Take the tutorial"
    //                     icons::ArrowRight {}
    //                 }
    //             }
    //         }
    //     }
    // })
}
fn ValueAdd(cx: Context) -> DomTree {
    rsx!(cx, section { class: "text-gray-400 body-font"
        // section { class: "text-gray-400 bg-gray-900 body-font"
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
    })
}

fn FeaturedExamples(cx: Context) -> DomTree {
    cx.render(rsx!{
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
    })
}

fn Features(cx: Context) -> DomTree {
    rsx!(cx, section { class: "text-gray-600 body-font",
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
    })
}


static Snippets: FC<()> = |(cx, props)| {
    let (snippets, _) = use_state(cx, || build_snippets()).classic();
    let selected_snippet = use_state(cx, || 0);


    let chosen_snippet = snippets.get(*selected_snippet).unwrap();

    let snip_list = snippets.iter().enumerate().map(|(id, s)| {
        rsx!(li {
            onclick: move |_| selected_snippet.set(id),
            "{s.title}"
        })
    });

    cx.render(rsx!{
        section { class: "text-gray-400 bg-gray-900 body-font mx-auto px-48"
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
                // h2 { class: "text-white text-lg title-font font-medium mb-2",
                    "{props.title}"
                }
                {props.content.split("\n").map(|line| rsx!{
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
        caller_id,
    } = &props.snippet;

    let body = body
        .split("\n")
        .map(|paragraph| rsx!( p{ class: "leading-relaxed text-base pb-4", "{paragraph}"} ))
        .take(3);

    cx.render(rsx! {
        section { class: "text-gray-400 body-font bg-gray-900",
            div { class: "container flex flex-wrap px-5 py-4 mx-auto",
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


pub static CheckedFeature: FC<()> = |(cx, props)| {
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
