use dioxus::prelude::*;

pub mod call_to_action;
pub mod featured_examples;
pub mod hero;
pub mod snippets;
pub mod value_add;

pub fn Homepage(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "dark:bg-black",
            hero::Hero {},
            ProjectCards {}
            // value_add::ValueAdd {}
            featured_examples::FeaturedExamples {}
            snippets::Snippets {}
            // crate::components::blog::RecentBlogPosts {}
            // call_to_action::CallToAction {}
            script { "Prism.highlightAll();" } // ensure Prism is able to highlight all our code elements
        }
    ))
}

const CARDS: &[(&str, &str)] = &[
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Atomic global state",
        "Reminiscent of RecoilJS, Fermi makes it dead-easy to manage global state.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
    (
        "Security in saving",
        "Take care to develop resources continually and integrity them with previous projects.",
    ),
];

fn ProjectCards(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            div { class: "container mx-auto px-4 py-12 px-6 lg:px-64",
                div { class: "flex flex-wrap -mx-3",
                    CARDS.iter().map(|(title, description)| rsx! {
                        div { class: "w-full md:w-1/2 lg:w-1/3 px-3 mb-6 text-xs dark:text-white",
                            div { class: "p-6 md:p-8 h-full rounded shadow-white hover:shadow-xl hover:border-transparent cursor-pointer",
                                div {
                                    h3 { class: "mb-4 text-2xl font-semibold font-heading", "{title}" }
                                    p { class: "text-base text-gray-500", "{description}" }
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}
