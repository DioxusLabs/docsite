use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_router::use_route;
use dioxus_router::Link;
use use_mdbook::{include_mdbook, LazyMdbook};

pub fn Learn(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full pt-12 text-sm dark:bg-ideblack", min_height: "100vh",
            // do a typical three-column flex layout with a single centered then pin the nav items on top
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto dark:text-white",
                Content {}
                LeftNav {}
                RightNav {}
            }
        }
    })
}

static DOCS: LazyMdbook = include_mdbook!("docs");

fn LeftNav(cx: Scope) -> Element {
    let sections = DOCS.summary.numbered_chapters.iter().filter_map(|chapter| {
        let link = chapter.maybe_link()?;
        let sections = link.nested_items.iter().filter_map(|link| {
            let link = link.maybe_link()?;
            let show_chevron = link.nested_items.len() > 0;
            let url = link.location.as_ref().unwrap().to_string_lossy();

            render! {
                li { class: "pt-1",
                    if show_chevron {
                        rsx! {
                            dioxus_material_icons::MaterialIcon {
                                name: "chevron_right",
                                color: "gray",
                            }
                        }
                    }
                    Link {
                        to: "/docs/0.4/en/{url}",
                        "{link.name}"
                    }
                    if show_chevron {
                        rsx! {
                            ul {
                                for nest in link.nested_items.iter().filter_map(|link| link.maybe_link()) {
                                    li { class: "pl-8",
                                        Link {
                                            to: "/docs/0.4/en/{link.location.as_ref().unwrap().to_string_lossy()}",
                                            "{nest.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        render! {
            div {  class: "pb-4",
                h2 { class: "font-semibold", "{link.name}" }
                ul { class: "pl-2", sections }
            }
        }
    });

    render! {
        // Now, pin the nav to the left
        nav { class: "pl-6 z-20 text-base hidden md:block fixed top-0 pt-36 pb-16 pl-3.5 md:-ml-3.5 w-[calc(100%-1rem)] md:w-60 h-full max-h-screen md:text-[13px] text-navy content-start overflow-y-auto leading-5",
            BreadCrumbs {}
            sections
        }
    }
}

fn RightNav(cx: Scope) -> Element {
    render! {
        div { class: "overflow-y-auto hidden xl:block fixed top-0 pt-36 pb-16 pl-3.5 -ml-3.5 w-60 h-full md:text-[13px] leading-5 text-navy docs-right-sidebar",
            right: "calc(40vw - 40.875rem)",
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul { class: "",
                li { class: "pb-2", "Run Your Entire Stack Near Your Users" }
                li { class: "pb-2", "Grow at Your Own Pace" }
                li { class: "pb-2", "Color Outside the Lines" }
                li { class: "pb-2", "Draw Your Own Lines" }
            }
        }
    }
}

fn Content(cx: Scope) -> Element {
    let url: PathBuf = "index.md".parse().unwrap();
    let page = DOCS.pages.get(&url).unwrap();

    // let BlogPost { content, .. } = crate::blog::POST_RELEASE_030;

    render! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack mx-auto container pt-12 pb-12",
            div { class: "-my-8",
                script { "Prism.highlightAll()" }
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body li {{ display: list-item; }}"
                    }
                    article { class: "markdown-body pt-1", dangerous_inner_html: format_args!("{}", page.content) }
                    script { "Prism.highlightAll()" }
                }
            }
        }
    }
}

fn BreadCrumbs(cx: Scope) -> Element {
    // parse out the route after the version and language
    let route = use_route(cx);

    render! {
        h2 { class: "font-semibold pb-4",
            // dioxus_router::Link { to: "https://google.com", class: "text-blue-600", "Learn" }
            for segment in route.url().path_segments().unwrap().skip(3).filter(|f| !f.is_empty()).map(|f| f.trim_end_matches(".md")) {
                rsx! {
                    if segment != "index" {
                        rsx! {
                            dioxus_router::Link { to: "https://google.com", class: "text-blue-600", "{segment}" }
                            " / "
                        }
                    }
                }
            }
        }
    }
}
