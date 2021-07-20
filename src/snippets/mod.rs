use dioxus::prelude::*;

/// # A Simple Component
///
/// Dioxus components are declaratively defined using either the `rsx!` macro or the `html!` macro. Both these macros are
/// just helpful wrappers around the `NodeFactory` API - which can be used directly to create new elements, listeners,
/// attributes, and components.
///
/// The `rsx!` macro is designed to feel like writing nested structs with optional values, taking advantage of deep
/// integration with Rust-Analyzer. The `html!` macro is designed to feel like writing HTML - it's possible to drop in
/// HTML templates without any additional work.
pub static Simple: FC<()> = |cx| {
    cx.render(rsx! {
        div { "Hello world!"}
    })
};

/// # A Stateful Component
///
/// Dioxus components use hooks to store state between renders. The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component. Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
pub static Stateful: FC<()> = |cx| {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        button { "Upvote counter: {count}", onclick: move |_| count += 1 }
    })
};

/// # Advanced rendering
///
/// Dioxus accepts fragments, iterators, conditionals, listeners, matching, f_string iterpolation and more. Anything that can
/// be coerced into an iterator of VNodes can be used in the macro bodies.
///
///
pub static AdvancedRendering: FC<()> = |cx| {
    let should_show = use_state(cx, || true);

    let button_text = match *should_show {
        true => "Click to show",
        false => "Click to hide",
    };

    let fizzes = (0..10).map(|i| match (i % 3, i % 5) {
        (0, 0) => rsx!(in cx, li {"FizzBuzz"} ),
        (0, _) => rsx!(in cx, li {"Fizz"} ),
        (_, 0) => rsx!(in cx, li {"Buzz"} ),
        (_, _) => rsx!(in cx, li {"{i}"} ),
    });

    cx.render(rsx! {
        "Advanced rendering"
        button { "{button_text}", onclick: move |_| should_show.set(!*should_show)}
        {should_show.then(|| rsx!( ul { {fizzes} } ))}
    })
};

/// # Powerful IDE integration
///
///
pub static IDEIntegration: FC<()> = |cx| None;

/// # Built-in error handling
pub static ErrorHandling: FC<()> = |cx| {
    //
    None
};
