use dioxus::prelude::*;

pub mod call_to_action;
pub mod featured_examples;
pub mod hero;
pub mod snippets;
pub mod value_add;

pub fn Homepage(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "dark:bg-gray-900",
            hero::Hero {},
            value_add::ValueAdd {}
            featured_examples::FeaturedExamples {}
            snippets::Snippets {}
            crate::components::blog::RecentBlogPosts {}
            call_to_action::CallToAction {}
            crate::components::footer::Footer {}
            script { "Prism.highlightAll();" } // ensure Prism is able to highlight all our code elements
        }
    ))
}
