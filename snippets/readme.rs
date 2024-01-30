//! Simple, familiar, React-like syntax

fn app() -> Element {
    let mut count = use_state(|| 0);

    render!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    )
}
