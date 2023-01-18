/// # Global state: globalstate.rs
///
/// With Dioxus, it's possible to directly expose shared state to child components with the `use_provide_context` hook.
/// Components lower in the tree can then directly read and write to the shared state with runtime safety.
struct SharedState(&'static str);

fn GlobalState(cx: Scope) -> Element {
    use_provide_state(&cx, || SharedState("world!"));
    rsx!(
        cx,
        div {
            "Hello, "
            Child {}
        }
    )
}

fn Child(cx: Scope) -> Element {
    let name = use_shared_state::<SharedState>(cx)?;
    rsx!( cx, "{name}" )
}
