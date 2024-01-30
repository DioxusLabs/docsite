#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct Comment {
    content: String,
    id: usize,
}

pub fn App() -> Element {
    // ANCHOR: render_list
    let comment_field = use_state(String::new);
    let mut next_id = use_state(|| 0);
    let comments = use_ref(Vec::<Comment>::new);

    let comments_lock = comments.read();
    let comments_rendered = comments_lock.iter().map(|comment| {
        rsx!(CommentComponent {
            key: "{comment.id}",
            comment: comment.clone(),
        })
    });

    rsx!(
        form {
            onsubmit: move |_| {
                comments.write().push(Comment {
                    content: comment_field.get().clone(),
                    id: *next_id.get(),
                });
                next_id += 1;

                comment_field.set(String::new());
            },
            input {
                value: "{comment_field}",
                oninput: move |event| comment_field.set(event.value.clone()),
            }
            input {
                r#type: "submit",
            }
        },
        {comments_rendered}
    )
    // ANCHOR_END: render_list
}

pub fn AppForLoop() -> Element {
    // ANCHOR: render_list_for_loop
    let comment_field = use_state(String::new);
    let mut next_id = use_state(|| 0);
    let comments = use_ref(Vec::<Comment>::new);

    rsx!(
        form {
            onsubmit: move |_| {
                comments.write().push(Comment {
                    content: comment_field.get().clone(),
                    id: *next_id.get(),
                });
                next_id += 1;

                comment_field.set(String::new());
            },
            input {
                value: "{comment_field}",
                oninput: move |event| comment_field.set(event.value.clone()),
            }
            input {
                r#type: "submit",
            }
        },
        for comment in &*comments.read() {
            // Notice the body of this for loop is rsx code, not an expression
            CommentComponent {
                key: "{comment.id}",
                comment: comment.clone(),
            }
        }
    )
    // ANCHOR_END: render_list_for_loop
}

#[component]
fn CommentComponent(comment: Comment) -> Element {
    rsx!(div {
        "Comment by anon:",
        p { "{comment.content}" }
        button { "Reply" },
    })
}
