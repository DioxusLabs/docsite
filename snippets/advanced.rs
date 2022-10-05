/// # Advanced Rendering: advanced.rs
///
/// Dioxus accepts fragments, iterators, conditionals, listeners, matching, f_string iterpolation and more. Anything that can
/// be coerced into an iterator of VNodes can be used in the macro bodies. By default, `rsx!` is Lazy, meaning it won't allocate
/// until "rendered" with a `render` call.
///
/// Elements are created with a dedicated memory allocator that intelligently reuses memory between renders. A component
/// at "steady-state" performs zero global allocations, making rendering extremely fast and memory efficient.
fn AdvancedRendering(cx: Scope) -> Element {
    let (should_show, _) = use_state(&cx, || true);

    cx.render(rsx! (
        button {
            onclick: move |_| should_show.set(!should_show),
            match *should_show {
                true => rsx!("Click to show"),
                false => rsx!("Click to hide"),
            },
        }

        should_show.then(|| rsx!(
            ul {
                (0..10).map(|i| match (i % 3, i % 5) {
                    (0, 0) => rsx!( li { "FizzBuzz" } ),
                    (0, _) => rsx!( li { "Fizz" } ),
                    (_, 0) => rsx!( li { "Buzz" } ),
                    (_, _) => rsx!( li { "{i}" } ),
                })
            }
        ))
    ))
}
