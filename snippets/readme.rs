//! Simple, familiar, React-like syntax

#[component]
fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    )
}
