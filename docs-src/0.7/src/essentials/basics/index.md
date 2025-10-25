# The Basics of State

Now that you know how to create user interfaces in Dioxus, it's time to learn about creating and update your app's state.

**Managaing state is, by far, the hardest part of building an app.**

This guide will walk you through the core principles of state management in Dioxus with an early emphasis on theory and then a shift into application.


## The Theory of State Management

Ultimately, "state management" refers to the act of:

1. Initializing data for the UI
2. Handling events from the user
3. Updating the data and re-rendering the UI

Managing this cycle is easy at first, but can become more challenging as apps grow in size, add more asynchronous work, and interact with external resources.

## For Experienced Web Developers

If you're coming to Dioxus as an experienced web developer, we hope you'll feel comfortable rather quickly. State management in Dioxus was heavily inspired by projects like React, Preact, SolidJS, and Svelte.

Dioxus uses signal-based reactivity. Unlike SolidJS, *reads* and *writes* of value are **explicit**. Rust does not have an equivalent to JavaScript's Proxy, so reactivity is traced by calls to `.read()` and `.write()`.

```rust
let mut count = use_signal(|| 0);

rsx! {
    button {
        onclick: move |_| *count.write() += 1,
        "Increment"
    }
    {count.read().to_string()}
}
```

In many cases, `.read()` is subtle, like using the value inside a string format:

```rust
rsx! { "Count: {count}" }
```

Unlike Svelte, Dioxus does not do compile-time transformation of your state. Unlike SolidJS, Dioxus components run multiple times. You can think of state management in Dioxus to be a hybrid of React and SolidJS, where reactivity is automatically tracked, but components are free to run multiple times throughout their lifetime.

Dioxus also employs "common sense" optimizations like automatic property memoization and automatic reactivity tracking - two huge improvements over React. Dioxus also allows you to early return, optionally with a Suspended future, giving you an `async/await`-like model for data loading in components.

## For Experienced Rustaceans

If you're coming to Dioxus as an experienced Rustacean, you might be intimidated by our use of "unusual" primitives like `use_signal`, `use_memo`, `use_resource`, etc. Dioxus uses stateful "hooks" - a paradigm that has its origins in web development (in particular, React).

If you don't want to write React-like code in Dioxus, you can opt to using structs to store state and handle updates to the UI imperatively. You can store your state in a struct:

```rust
struct EditorState {
    text: String
}

impl EditorState {
    fn handle_input(&mut self, event: FormEvent) {
        self.text = event.value();
    }
}
```

And then use that state in a component with a single `use_signal`:

```rust
/// Define a component
#[component]
fn TextEditor() -> Element {
    // Use a single "state" object, wrapped in a `Signal`
    let mut state = use_signal(EditorState::new);

    rsx! {
        input {
            // In event handlers, call `.write()` to get `&mut self` on the "state" object
            oninput: move |event| state.write().handle_input(event)
        }
    }
}
```

Rust's borrow checker adds complexity to holding values across async tasks and callbacks. Using our built-in primitives solves a number of issues:

- Ability to reference data in `'static` tasks and callbacks
- Calls to `.write()` automatically queue a component to re-render
- Values are never "stale", even in async contexts.

Dioxus exposes its core runtime functions like `spawn` and `needs_update` - an approach Rustaceans might be more familiar with. The built-in reactive primitives leverage these functions intelligently, and thus will likely be more efficient than the simplistic state approach, but do come with a learning curve.
