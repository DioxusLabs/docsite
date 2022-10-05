/// stateful.rs
/// # A Stateful Component
///
/// Dioxus components use hooks to store state between renders.
/// The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component.
/// Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
fn Stateful(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx! (
        button {
            "Upvote counter: {count}",
            onclick: move |_| set_count(count + 1)
        }
    ))
}
