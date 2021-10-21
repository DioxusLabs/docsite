#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;

mod call_to_action;
mod featured_examples;
mod hero;
mod recent_blog_posts;
mod snippets;
mod value_add;

pub static Home: FC<()> = |(cx, _)| {
    cx.render(rsx! {
        hero::Hero {}
        value_add::ValueAdd {}
        featured_examples::FeaturedExamples {}
        snippets::Snippets {}
        recent_blog_posts::RecentBlogPosts {}
        call_to_action::CallToAction {}

        // ensure Prism is able to highlight all our code elements
        script { "Prism.highlightAll();" }
    })
};
