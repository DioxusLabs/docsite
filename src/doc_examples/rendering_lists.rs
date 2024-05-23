#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct CommentData {
    content: String,
    id: usize,
}

pub fn App() -> Element {
    // ANCHOR: render_list
    let mut comment_field = use_signal(String::new);
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<CommentData>::new);

    let comments_lock = comments.read();
    let comments_rendered = comments_lock.iter().map(|comment| {
        rsx! { Comment { comment: comment.clone() } }
    });

    rsx! {
        form {
            onsubmit: move |_| {
                comments
                    .write()
                    .push(CommentData {
                        content: comment_field(),
                        id: next_id(),
                    });
                next_id += 1;
                comment_field.set(String::new());
            },
            input {
                value: "{comment_field}",
                oninput: move |event| comment_field.set(event.value())
            }
            input { r#type: "submit" }
        }
        {comments_rendered}
    }
    // ANCHOR_END: render_list
}

pub fn AppForLoop() -> Element {
    // ANCHOR: render_list_for_loop
    let mut comment_field = use_signal(String::new);
    let mut next_id = use_signal(|| 0);
    let mut comments = use_signal(Vec::<CommentData>::new);

    rsx! {
        form {
            onsubmit: move |_| {
                comments
                    .write()
                    .push(CommentData {
                        content: comment_field(),
                        id: next_id(),
                    });
                next_id += 1;
                comment_field.set(String::new());
            },
            input {
                value: "{comment_field}",
                oninput: move |event| comment_field.set(event.value())
            }
            input { r#type: "submit" }
        }
        for comment in comments() {
            // Notice the body of this for loop is rsx code, not an expression
            Comment { comment }
        }
    }
    // ANCHOR_END: render_list_for_loop
}

#[component]
fn Comment(comment: CommentData) -> Element {
    rsx! {
        div {
            "Comment by anon:"
            p { "{comment.content}" }
            button { "Reply" }
        }
    }
}
