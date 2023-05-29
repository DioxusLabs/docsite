use dioxus::prelude::*;

use crate::{learn::learn_hook::use_mdbook, BlogPost, SinglePost, POST_RELEASE_030};

mod learn_hook;

pub fn Learn(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full pt-12 text-sm dark:bg-ideblack", min_height: "100vh",
        // How we do the nav:
        // - do a typical three-column flex layout with a single centered
        // then pin the nav items on top

            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto dark:text-white",
                Content {}
                LeftNav {}
                RightNav {}
            }
        }
    })
}

fn LeftNav(cx: Scope) -> Element {
    let book = use_mdbook(cx);

    let sections = book
        .summary
        .numbered_chapters
        .iter_mut()
        .filter_map(|chapter| {
            //
            let link = chapter.maybe_link_mut()?;

            let sections = link.nested_items.iter_mut().filter_map(|link| {
                let link = link.maybe_link_mut()?;
                render! {
                    li { class: "pt-1",
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_right",
                            color: "gray",
                        }
                        "{link.name}"
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
    let BlogPost {
        category,
        date,
        title,
        description,
        link,
        content,
    } = crate::blog::POST_RELEASE_030;

    render! {
        section { class: "text-gray-600 body-font overflow-hidden dark:bg-ideblack mx-auto container pt-12 pb-12",
            div { class: "-my-8",
                script { "Prism.highlightAll()" }
                div { class: "flex w-full mb-20 flex-wrap list-none",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body li {{ display: list-item; }}"
                    }
                    article { class: "markdown-body pt-1", dangerous_inner_html: format_args!("{}", content) }
                    script { "Prism.highlightAll()" }
                }
            }
        }
    }
}

fn BreadCrumbs(cx: Scope) -> Element {
    use dioxus_router::Link;

    render! {
        h2 { class: "font-semibold pb-4",
            Link { to: "https://google.com", class: "text-blue-600", "Learn" }
            " / "
            Link { to: "https://google.com", class: "text-blue-600", "Platforms" }
            " / "
            Link { to: "https://google.com", class: "text-blue-600", "Mobile" }
        }
    }
}

#[inline_props]
fn Section(cx: Scope, title: &'static str, items: &'static [&'static str]) -> Element {
    render! {
        div {  class: "pb-4",
            h2 { class: "font-semibold", "Deploying" }
            ul { class: "pl-2",
                for item in items.iter() {
                    li { class: "pt-1",
                        dioxus_material_icons::MaterialIcon {
                            name: "chevron_right",
                            color: "gray",
                        }
                        "Desktop"
                    }
                }
            }
        }
    }
}
