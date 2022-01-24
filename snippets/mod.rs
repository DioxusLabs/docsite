use dioxus::prelude::*;

/// # A Simple Component
///
/// Dioxus components are declaratively defined using either the `rsx!` macro or the `html!` macro. Both these macros are
/// just helpful wrappers around the `NodeFactory` API - which can be used directly to create new elements, listeners,
/// attributes, and components.
fn Simple(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello world!"}
    ))
}

/// # A Stateful Component
///
/// Dioxus components use hooks to store state between renders. The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component. Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
fn Stateful(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx! (
        button {
            "Upvote counter: {count}",
            onclick: move |_| count += 1
        }
    ))
}

/// # Custom Props
///
/// Dioxus components use hooks to store state between renders. The `use_state` hooks make it easy to update state from
/// event listeners attached to elements in the component. Whenever the state is modified, the component will be re-rendered.
///
/// Thanks to Rust's ownership rules, it's impossible to misuse the `use_state` hook.
#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}
fn Stateful(cx: Scope<PropBased>) -> Element {
    cx.render(rsx! ( "Hello {props.name}, you are {props.age} years old!" ))
}

/// # Advanced Rendering
///
/// Dioxus accepts fragments, iterators, conditionals, listeners, matching, f_string iterpolation and more. Anything that can
/// be coerced into an iterator of VNodes can be used in the macro bodies. By default, `rsx!` is Lazy, meaning it won't allocate
/// until "rendered" with a `render` call.
///
/// Elements are created with a dedicated memory allocator that intelligently reuses memory between renders. A component
/// at "steady-state" performs zero global allocations, making rendering extremely fast and memory efficient.
fn AdvancedRendering(cx: Scope) -> Element {
    let should_show = use_state(&cx, || true);

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

/// # Built-in error handling
///
/// Because components return an `Option<VNode>`, errors can be handled gracefully through the use of the question mark
/// syntax. Components that fail to render will return will be frozen until the next successful render.
///
/// This is exceptionally useful for components that select optional values that will never be `None` while the component
/// is being viewed - IE a settings panel that can only be shown if a user is logged in.
fn ErrorHandling(cx: Scope) -> Element {
    let items = vec!["a", "b", "c", "d", "e"];
    let first_item = items.first()?;

    cx.render(rsx!(
        h1 { "First item: {first_item}" }
    ))
}

/// # Global state
///
/// With Dioxus, it's possible to directly expose shared state to child components with the `use_provide_context` hook.
/// Components lower in the tree can then directly read and write to the shared state with runtime safety.
struct SharedState(&'static str);

fn GlobalState(cx: Scope) -> Element {
    use_provide_state(&cx, || SharedState("world!"));
    rsx!(cx, div { "Hello, ", Child {} })
}

fn Child(cx: Scope) -> Element {
    let name = use_shared_state::<SharedState>(cx)?;
    rsx!(cx, "{name}" )
}

/// # Advanced State management
///
/// With Dioxus, it's possible to directly expose shared state to child components with the `use_provide_context` hook.
/// Components lower in the tree can then directly read and write to the shared state with runtime safety.
///
/// Dioxus also has 1st-class support for Fermi: a global state management toolkit modeled after RecoilJS.
static Name: Atom<Option<String>> = |_| Some("Dioxus".into());

fn UserInfo(cx: Scope) -> Element {
    let user_name = use_read(&cx, Name);

    match user_name {
        Some(name) => rsx!(cx, "Hello, {name}"),
        None => rsx!(cx, "Please log in")
    }
}

/// # Coroutines and tasks
///
/// Components may spawn a coroutine or task to perform asynchronous operations. These tasks may be started, stopped, or
/// reset by other logc in the component. Coroutines are extremely handy for asynchronous tasks like network requests,
/// websockets, and multi-threading.
fn Tasks(cx: Scope) -> Element {
    let count = use_state(&cx, || 0);

    use_future(cx, || {
        let mut count = count.to_owned();
        async move {
            loop {
                count += 1;
                timer::from_ms(500).await;
            }
        }
    });

    rsx!(cx, pre {"Count: {count}"})
}

/// # Suspense
///
/// Dioxus supports Suspense - a way of deferring rendering until a condition is met. Simply pass in a future and a callback,
/// and Dioxus will wait for the future to resolve before rendering the result. Suspense makes it possible to prevent
/// cascaded re-rendering and allows Dioxus to render the rest of the component while waiting for the future to complete.
fn Suspense(cx: Scope) -> Element {
    #[derive(serde::Deserialize)]
    struct DogApi {
        message: String,
    }

    let doggo = use_suspense(
        cx,
        || surf::get("https://dog.ceo/api/breeds/image/random").recv_json::<DogApi>(),
        |cx, res| match res {
            Ok(res) => rsx!(cx, img { src: "{res.message}" }),
            Err(_) => rsx!(cx, div { "No doggos for you :(" } ),
        },
    );

    cx.render(rsx!(
        h1 { "Waiting for doggos:" }
        {doggo}
    ))
}
