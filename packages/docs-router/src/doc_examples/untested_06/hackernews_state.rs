#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod app_v1 {
    use super::{CommentData, StoryItem, StoryPageData};
    use dioxus::prelude::*;

    // ANCHOR: app_v1
    pub fn App() -> Element {
        rsx! {
            div { display: "flex", flex_direction: "row", width: "100%",
                div { width: "50%", Stories {} }
                div { width: "50%", Preview {} }
            }
        }
    }

    // New
    fn Stories() -> Element {
        rsx! {
            StoryListing {
                story: StoryItem {
                    id: 0,
                    title: "hello hackernews".to_string(),
                    url: None,
                    text: None,
                    by: "Author".to_string(),
                    score: 0,
                    descendants: 0,
                    time: chrono::Utc::now(),
                    kids: vec![],
                    r#type: "".to_string(),
                }
            }
        }
    }

    // New
    #[derive(Clone, Debug)]
    enum PreviewState {
        Unset,
        Loading,
        Loaded(StoryPageData),
    }

    // New
    fn Preview() -> Element {
        let preview_state = PreviewState::Unset;
        match preview_state {
            PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
            PreviewState::Loading => rsx! {"Loading..."},
            PreviewState::Loaded(story) => {
                rsx! {
                    div { padding: "0.5rem",
                        div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                        div { dangerous_inner_html: story.item.text }
                        for comment in &story.comments {
                            Comment { comment: comment.clone() }
                        }
                    }
                }
            }
        }
    }

    // NEW
    #[component]
    fn Comment(comment: CommentData) -> Element {
        rsx! {
            div { padding: "0.5rem",
                div { color: "gray", "by {comment.by}" }
                div { dangerous_inner_html: "{comment.text}" }
                for kid in &comment.sub_comments {
                    Comment { comment: kid.clone() }
                }
            }
        }
    }

    // ANCHOR_END: app_v1
    #[component]
    fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
        let StoryItem {
            title,
            url,
            by,
            score,
            time,
            kids,
            ..
        } = &*story.read();

        let url = url.as_deref().unwrap_or_default();
        let hostname = url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_start_matches("www.");
        let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });
        let comments = format!(
            "{} {}",
            kids.len(),
            if kids.len() == 1 {
                " comment"
            } else {
                " comments"
            }
        );
        let time = time.format("%D %l:%M %p");

        rsx! {
            div { padding: "0.5rem", position: "relative",
                div { font_size: "1.5rem",
                    a { href: url, "{title}" }
                    a {
                        color: "gray",
                        href: "https://news.ycombinator.com/from?site={hostname}",
                        text_decoration: "none",
                        " ({hostname})"
                    }
                }
                div { display: "flex", flex_direction: "row", color: "gray",
                    div { "{score}" }
                    div { padding_left: "0.5rem", "by {by}" }
                    div { padding_left: "0.5rem", "{time}" }
                    div { padding_left: "0.5rem", "{comments}" }
                }
            }
        }
    }
}

mod story_listing_listener {
    use super::{Comment, CommentData, StoryItem, StoryPageData};
    use dioxus::prelude::*;

    pub fn App() -> Element {
        use_context_provider(|| Signal::new(PreviewState::Unset));

        rsx! {
            div { display: "flex", flex_direction: "row", width: "100%",
                div { width: "50%", Stories {} }
                div { width: "50%", Preview {} }
            }
        }
    }

    fn Stories() -> Element {
        rsx! {
            StoryListing {
                story: StoryItem {
                    id: 0,
                    title: "hello hackernews".to_string(),
                    url: None,
                    text: None,
                    by: "Author".to_string(),
                    score: 0,
                    descendants: 0,
                    time: chrono::Utc::now(),
                    kids: vec![],
                    r#type: "".to_string(),
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    enum PreviewState {
        Unset,
        Loading,
        Loaded(StoryPageData),
    }

    fn Preview() -> Element {
        let preview_state = PreviewState::Unset;
        match preview_state {
            PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
            PreviewState::Loading => rsx! {"Loading..."},
            PreviewState::Loaded(story) => {
                rsx! {
                    div { padding: "0.5rem",
                        div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                        div { dangerous_inner_html: story.item.text }
                        for comment in &story.comments {
                            Comment { comment: comment.clone() }
                        }
                    }
                }
            }
        }
    }

    #[component]
    fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
        let mut preview_state = consume_context::<Signal<PreviewState>>();
        let StoryItem {
            title,
            url,
            by,
            score,
            time,
            kids,
            ..
        } = &*story.read();

        let url = url.as_deref().unwrap_or_default();
        let hostname = url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_start_matches("www.");
        let score = format!("{score} point{}", if *score > 1 { "s" } else { "" });
        let comments = format!(
            "{} {}",
            kids.len(),
            if kids.len() == 1 {
                " comment"
            } else {
                " comments"
            }
        );
        let time = time.format("%D %l:%M %p");

        // ANCHOR: story_listing_listener
        rsx! {
            div {
                padding: "0.5rem",
                position: "relative",
                onmouseenter: move |_| {},
                div { font_size: "1.5rem",
                    a { href: url, onfocus: move |_event| {}, "{title}" }
                    a {
                        color: "gray",
                        href: "https://news.ycombinator.com/from?site={hostname}",
                        text_decoration: "none",
                        " ({hostname})"
                    }
                }
                div { display: "flex", flex_direction: "row", color: "gray",
                    div { "{score}" }
                    div { padding_left: "0.5rem", "by {by}" }
                    div { padding_left: "0.5rem", "{time}" }
                    div { padding_left: "0.5rem", "{comments}" }
                }
            }
        }
        // ANCHOR_END: story_listing_listener
    }
}

// ANCHOR: shared_state_app
pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    // ANCHOR_END: shared_state_app
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}

// ANCHOR: shared_state_stories
#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let mut preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} point{}", if *score > 1 { "s" } else { "" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            onmouseenter: move |_event| {
                *preview_state
                    .write() = PreviewState::Loaded(StoryPageData {
                    item: story(),
                    comments: vec![],
                });
            },
            div { font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| {
                        *preview_state
                            .write() = PreviewState::Loaded(StoryPageData {
                            item: story(),
                            comments: vec![],
                        });
                    },
                    // ANCHOR_END: shared_state_stories
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}

// ANCHOR: shared_state_preview
fn Preview() -> Element {
    // New
    let preview_state = consume_context::<Signal<PreviewState>>();

    // New
    match preview_state() {
        // ANCHOR_END: shared_state_preview
        PreviewState::Unset => rsx! {"Hover over a story to preview it here"},
        PreviewState::Loading => rsx! {"Loading..."},
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}" } }
                    div { dangerous_inner_html: { story.item.text } }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}

#[component]
fn Comment(comment: CommentData) -> Element {
    rsx! {
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}" }
            div { dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}

// Define the Hackernews types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<CommentData>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentData {
    pub id: i64,
    /// there will be no by field if the comment was deleted
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<CommentData>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}
