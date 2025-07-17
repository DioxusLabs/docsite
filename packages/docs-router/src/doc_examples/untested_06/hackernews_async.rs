#![allow(non_snake_case)]
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ANCHOR: api
// Define the Hackernews API
use futures::future::join_all;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i64 = 2;

pub async fn get_story_preview(id: i64) -> Result<StoryItem, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    reqwest::get(&url).await?.json().await
}

pub async fn get_stories(count: usize) -> Result<Vec<StoryItem>, reqwest::Error> {
    let url = format!("{}topstories.json", BASE_API_URL);
    let stories_ids = &reqwest::get(&url).await?.json::<Vec<i64>>().await?[..count];

    let story_futures = stories_ids[..usize::min(stories_ids.len(), count)]
        .iter()
        .map(|&story_id| get_story_preview(story_id));
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story(id: i64) -> Result<StoryPageData, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    let mut story = reqwest::get(&url).await?.json::<StoryPageData>().await?;
    let comment_futures = story.item.kids.iter().map(|&id| get_comment(id));
    let comments = join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|c| c.ok())
        .collect();

    story.comments = comments;
    Ok(story)
}

#[async_recursion::async_recursion(?Send)]
pub async fn get_comment_with_depth(id: i64, depth: i64) -> Result<CommentData, reqwest::Error> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    let mut comment = reqwest::get(&url).await?.json::<CommentData>().await?;
    if depth > 0 {
        let sub_comments_futures = comment
            .kids
            .iter()
            .map(|story_id| get_comment_with_depth(*story_id, depth - 1));
        comment.sub_comments = join_all(sub_comments_futures)
            .await
            .into_iter()
            .filter_map(|c| c.ok())
            .collect();
    }
    Ok(comment)
}

pub async fn get_comment(comment_id: i64) -> Result<CommentData, reqwest::Error> {
    let comment = get_comment_with_depth(comment_id, COMMENT_DEPTH).await?;
    Ok(comment)
}
// ANCHOR_END: api

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

pub mod fetch {
    use super::{get_comment, get_stories, get_story, PreviewState, StoryItem, StoryPageData};
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
            id,
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

    fn Preview() -> Element {
        let preview_state = consume_context::<Signal<PreviewState>>();

        match preview_state() {
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
    fn Comment(comment: super::CommentData) -> Element {
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

    // ANCHOR: use_resource
    fn Stories() -> Element {
        // Fetch the top 10 stories on Hackernews
        let stories = use_resource(move || get_stories(10));

        // check if the future is resolved
        match &*stories.read_unchecked() {
            Some(Ok(list)) => {
                // if it is, render the stories
                rsx! {
                    div {
                        // iterate over the stories with a for loop
                        for story in list {
                            // render every story with the StoryListing component
                            StoryListing { story: story.clone() }
                        }
                    }
                }
            }
            Some(Err(err)) => {
                // if there was an error, render the error
                rsx! {"An error occurred while fetching stories {err}"}
            }
            None => {
                // if the future is not resolved yet, render a loading message
                rsx! {"Loading items"}
            }
        }
    }
    // ANCHOR_END: use_resource
}

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

// ANCHOR: resolve_story
// New
async fn resolve_story(
    mut full_story: Signal<Option<StoryPageData>>,
    mut preview_state: Signal<PreviewState>,
    story_id: i64,
) {
    if let Some(cached) = full_story.as_ref() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_story(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story);
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
        id,
        ..
    } = story();
    // New
    let full_story = use_signal(|| None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if score == 1 { " point" } else { " points" });
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
            onmouseenter: move |_event| { resolve_story(full_story, preview_state, id) },
            div { font_size: "1.5rem",
                a {
                    href: url,
                    onfocus: move |_event| { resolve_story(full_story, preview_state, id) },
                    // ...

                    // ANCHOR_END: resolve_story
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

#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();

    match preview_state() {
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

fn Stories() -> Element {
    let stories = use_resource(move || get_stories(10));

    match &*stories.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                for story in list {
                    StoryListing { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! {"An error occurred while fetching stories {err}"},
        None => rsx! {"Loading items"},
    }
}
