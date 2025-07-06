#![allow(non_snake_case)]
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use futures::future::join_all;
use serde::{Deserialize, Serialize};

pub mod story_v1 {
    use super::*;

    // ANCHOR: story_v1
    fn main() {
        launch(App);
    }

    pub fn App() -> Element {
        rsx! {"story"}
    }
    // ANCHOR_END: story_v1
}

pub mod story_v2 {
    use dioxus::prelude::*;

    // ANCHOR: story_v2
    pub fn App() -> Element {
        let title = "title";
        let by = "author";
        let score = 0;
        let time = chrono::Utc::now();
        let comments = "comments";

        rsx! {"{title} by {by} ({score}) {time} {comments}"}
    }
    // ANCHOR_END: story_v2
}

pub mod story_v3 {
    use dioxus::prelude::*;

    // ANCHOR: story_v3
    pub fn App() -> Element {
        let title = "title";
        let by = "author";
        let score = 0;
        let time = chrono::Utc::now();
        let comments = "comments";

        rsx! { div { "{title} by {by} ({score}) {time} {comments}" } }
    }
    // ANCHOR_END: story_v3
}

pub mod story_v4 {
    use dioxus::prelude::*;

    // ANCHOR: story_v4
    pub fn App() -> Element {
        let title = "title";
        let by = "author";
        let score = 0;
        let time = chrono::Utc::now();
        let comments = "comments";

        rsx! {
            div { padding: "0.5rem", position: "relative",
                "{title} by {by} ({score}) {time} {comments}"
            }
        }
    }
    // ANCHOR_END: story_v4
}

pub mod story_v5 {
    use dioxus::prelude::*;

    // ANCHOR: app_v5
    pub fn App() -> Element {
        rsx! { StoryListing {} }
    }
    // ANCHOR_END: app_v5

    // ANCHOR: story_v5
    fn StoryListing() -> Element {
        let title = "title";
        let by = "author";
        let score = 0;
        let time = chrono::Utc::now();
        let comments = "comments";

        rsx! {
            div { padding: "0.5rem", position: "relative",
                "{title} by {by} ({score}) {time} {comments}"
            }
        }
    }
    // ANCHOR_END: story_v5
}

pub mod story_v6 {
    use dioxus::prelude::*;

    // ANCHOR: app_v6
    pub fn App() -> Element {
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
    // ANCHOR_END: app_v6

    // ANCHOR: story_v6
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    // Define the Hackernews types
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

        let comments = kids.len();

        rsx! {
            div { padding: "0.5rem", position: "relative",
                "{title} by {by} ({score}) {time} {comments}"
            }
        }
    }
    // ANCHOR_END: story_v6
}

pub mod story_final {
    // ANCHOR: story_final
    use dioxus::prelude::*;

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

    fn main() {
        launch(App);
    }

    pub fn App() -> Element {
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
                    time: Utc::now(),
                    kids: vec![],
                    r#type: "".to_string(),
                }
            }
        }
    }

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
    // ANCHOR_END: story_final
}
