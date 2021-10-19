use crate::icons::*;
use crate::sitemap::SECTIONS;
use dioxus::prelude::*;

pub static Footer: FC<()> = |(cx, props)| {
    let sections = SECTIONS.iter().map(|(section, raw_links)| {
        let links = raw_links.iter().map(|(link_name, href)| {
            rsx! (
                a { href: "{href}",
                    "{link_name}",
                    {href.starts_with("http").then(|| rsx!( ExternalLinkIcon {} ))}
                }
            )
        });
        rsx! {
            div {
                div { "{section}" }
                {links}
            }
        }
    });

    let categories = [
        ("Articles", &[("a", "articles/index.html")]),
        ("Articles", &[("b", "articles/index.html")]),
        ("Articles", &[("c", "articles/index.html")]),
    ]
    .into_iter()
    .map(|(name, links)| {
        rsx!(Category {
            name: name,
            links: *links
        })
    });

    cx.render(rsx! {
        footer { class: "text-gray-600 body-font",
            div { class: "container px-5 py-24 mx-auto flex md:items-center lg:items-start md:flex-row md:flex-nowrap flex-wrap flex-col",
                div { class: "w-64 flex-shrink-0 md:mx-0 mx-auto text-center md:text-left",
                    a { class: "flex title-font font-medium items-center md:justify-start justify-center text-gray-900",
                        svg {}
                        span { class: "ml-3 text-xl",
                            "Tailblocks"
                        }
                    }
                    p { class: "mt-2 text-sm text-gray-500",
                        "Air plant banjo lyft occupy retro adaptogen indego"
                    }
                }
                div { class: "flex-grow flex flex-wrap md:pl-20 -mb-10 md:mt-0 mt-10 md:text-left text-center",
                    {categories}
                }
            }
        }
    })
};

#[derive(PartialEq, Props)]
struct CategoryProps {
    name: &'static str,
    links: &'static [(&'static str, &'static str)],
}

static Category: FC<CategoryProps> = |(cx, props)| {
    let links = props.links.iter().map(|(link_name, href)| {
        rsx! {
            li {
                a { href: "{href}", "{link_name}"}
            }
        }
    });
    cx.render(rsx! {
        div { class: "lg:w-1/4 md:w-1/2 w-full px-4",
            h2 { class: "title-font font-medium text-gray-900 tracking-widest text-sm mb-3",
                "{props.name}"
            }
            nav { class: "list-none mb-10",
                {links}
            }
        }
    })
};

// div { class: "bg-gray-100",
//             div { class: "container mx-auto py-4 px-5 flex flex-wrap flex-col sm:flex-row",
//                 p { class: "text-gray-500 text-sm text-center sm:text-left",
//                     "© 2020 Tailblocks —"
//                     a { class: "text-gray-600 ml-1",
//                         target: "_blank",
//                         rel: "noopener noreferrer",
//                         href: "https://twitter.com/knyttneve",
//                         "@knyttneve"
//                     }
//                 }
//                 span { class: "inline-flex sm:ml-auto sm:mt-0 mt-2 justify-center sm:justify-start",
//                     a { class: "text-gray-500",
//                         svg { class: "w-5 h-5",
//                             stroke-width: "2",
//                             stroke-linecap: "round",
//                             fill: "currentColor",
//                             stroke-linejoin: "round",
//                             viewBox: "0 0 24 24",
//                             path {
//                                 d: "M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z",
//                             }
//                         }
//                     }
//                     a { class: "ml-3 text-gray-500",
//                         svg { class: "w-5 h-5",
//                             fill: "currentColor",
//                             viewBox: "0 0 24 24",
//                             stroke-linejoin: "round",
//                             stroke-width: "2",
//                             stroke-linecap: "round",
//                             path {
//                                 d: "M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z",
//                             }
//                         }
//                     }
//                     a { class: "ml-3 text-gray-500",
//                         svg { class: "w-5 h-5",
//                             fill: "none",
//                             stroke-linejoin: "round",
//                             stroke-width: "2",
//                             viewBox: "0 0 24 24",
//                             stroke: "currentColor",
//                             stroke-linecap: "round",
//                             rect {
//                                 y: "2",
//                                 width: "20",
//                                 x: "2",
//                                 rx: "5",
//                                 height: "20",
//                                 ry: "5",
//                             }
//                             path {
//                                 d: "M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37zm1.5-4.87h.01",
//                             }
//                         }
//                     }
//                     a { class: "ml-3 text-gray-500",
//                         svg { class: "w-5 h-5",
//                             viewBox: "0 0 24 24",
//                             fill: "currentColor",
//                             stroke: "currentColor",
//                             stroke-linecap: "round",
//                             stroke-linejoin: "round",
//                             stroke-width: "0",
//                             path {
//                                 d: "M16 8a6 6 0 016 6v7h-4v-7a2 2 0 00-2-2 2 2 0 00-2 2v7h-4v-7a6 6 0 016-6zM2 9h4v12H2z",
//                                 stroke: "none",
//                             }
//                             circle {
//                                 cx: "4",
//                                 r: "2",
//                                 stroke: "none",
//                                 cy: "4",
//                             }
//                         }
//                     }
//                 }
//             }
//         }
