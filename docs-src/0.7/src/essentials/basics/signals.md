# Reactive Signals

In Dioxus, your app's UI is defined as a function of its current state. As the state changes, the components and effects that depend on that state will automatically re-run. Reactivity automatically *tracks* state and *derives* new state, making it easy to build large applications that are efficient and simple to reason about.

Dioxus combines the various primitives we've covered into a single source of mutable state: the Signal.

## State with Signals

In Dioxus, mutable state is stored in Signals. Signals are *tracked* values that automatically update *reactive contexts* that watch them. They are the source of state from which all other state is derived from. Signals are often modified directly by event handlers in response to user input or asynchronously in futures.

You can create a signal with the `use_signal` hook:

```rust
let mut signal = use_signal(|| 0);
```

Once you have your signal, you can gain an inner reference to the signal value by calling the `.read()`:

```rust
let mut signal = use_signal(|| 0);

// use `.read()` to access the inner value
let inner = signal.read();
```

For Signals whose inner can be cheaply cloneable, you can also use "function" syntax to get a direct `Clone` of the value.

```rust
let name = use_signal(|| "Bob".to_string());

// Call the signal like a function
let inner = name();
```

Finally, you can set the value of the signal with the `.set()` method or get a mutable reference to the inner value with the `.write()` method:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:signal_write}}
```

A simple component that uses `.read()` and `.write()` to update its own state with signals:

```rust
fn Demo() -> Element {
    let mut count = use_signal(|| 0);

    // read the current value
    let current = count.read().clone();

    rsx! {
        button {
            onclick: move |_| count.set(current + 1),
            "Increment ({current})"
        }
    }
}
```

## Ergonomic Methods on Signals

In some cases, wrapping your data in Signals can make accessing the inner state awkward. Mutable Signals implement two fundamental traits: `Readable` and `Writeable`. These traits provide a number of automatic ergonomic improvements.

- `Signal<T>` implements `Display` if `T` implements `Display`
- `Signal<bool>` implements `fn toggle()`
- `Signal<i32>` and other numbers implement math operators (+, -, /, etc)
- `Signal<T>` where `T` implements `IntoIterator` implements `.iter()`
- and many more!

The `Display` extension enables using signals in formatting expressions:

```rust
let mut count = use_signal(|| 0);
rsx! { "Count is: {count}" }
```

The toggle extension makes toggling boolean values simpler:
```rust
let mut enabled = use_signal(|| true);
rsx! {
    button {
        onclick: move |_| enabled.toggle(),
        if enabled() { "disable" } else { "enable" }
    }
}
```

Math operators simplify arithmetic operations:

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
```

You'll generally want to use the extension methods unless the inner state does *not* implement the required traits. There are several methods available *not* listed here, so peruse the docs reference for more information.

## Reactive Scopes

Whenever `.write()` or `.set()` is called on a Signal, any active *reactive scopes* tracking that run a callback as a side-efect.

The simplest reactive scope is a component. During a component render, components automatically subscribe to signals where `.read()` is called. The `.read()` method can be called *implicitly* in many circumstances - notably, the extension methods provided by `Readable` use the underlying `.read()` method and thus *also* contribute to the current reactive scope. When a signal's value changes, components queue a side-effect to re-render the component using `dioxus::core::needs_update`.

```rust
let mut name = use_signal(|| "abc".to_string());

rsx! {
    // An obvious call to `.read()`
    {name.read().to_string()}

    // An implicit call via `Display`
    "{name}"
}
```

If a component does not call `.read()` on a Signal while rendering, it does not subscribe to that signal's value. This provides us "zero cost reactivity" where we can freely modify signal values without worrying about unnecessary re-renders. If a value is not observed, it won't cause unnecessary re-renders.

```rust
let mut loading = use_signal(|| false);

rsx! {
    button {
        // Because we don't use "loading" in our markup, the component won't re-render!
        onclick: move |_| async move {
            if loading() {
                return;
            }
            loading.set(true);
            // .. do async work
            loading.set(false);
        }
    }
}
```

There are other uses of reactive scopes beyond component re-renders. Hooks like `use_effect`, `use_memo`, and `use_resource` all implement functionality by leveraging a reactive scope that exists *outside* the rendering lifecycle.

## Automatic Batching

By default, all `.write()` calls are batched in one "step" of your app. Dioxus does not synchronously run side-effects when you call `.write()`. Instead, it waits for all events to be handled before first determining what side-effects need to be run. This provides automatic batching of `.write()` calls which is important both for performance and consistency in the UI.

For example, by batching `.write()` calls, we ensure that our UI always display one of two states:
- "false -> Complete"
- "true -> Loading"

```rust
let mut loading = use_signal(|| false);
let mut text = use_signal(|| "Complete!");

rsx! {
    button {
        onclick: move |_| async move {
            text.set("Loading");
            loading.set(true);

            do_async_work().await

            text.set("Complete!");
            loading.set(false);
        },
        "{loading:?} -> {text}"
    }
}
```

Dioxus uses `await` boundaries as barriers between steps. If state is modified during a step, Dioxus prefers to paint the new UI first before polling additional more futures. This ensures changes are flushed as fast as possible and pending states aren't missed.

## Derived State with Memo

`use_memo` is a reactive primitive that lets you derive state from any tracked value. It takes a closure that computes the new state and returns a tracked value with the current state of the memo. Any time a dependency of the memo changes, the memo will rerun.

The value you return from the closure will only change when the output of the closure changes (`PartialEq` between the old and new value returns false).

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:memo}}
```

```inject-dioxus
DemoFrame {
    reactivity::MemoDemo {}
}
```

This can be useful to perform expensive computations outside the component's reactive scope, preventing re-renders when the inputs change. By performing our computation *inside* the memo, we prevent the component from re-rendering when either `loading` or `loading_text` changes. Instead, the component will only re-render when the computed memo value changes.

```rust
let mut loading = use_signal(|| false);
let mut loading_text = use_signal(|| "loading".to_string());

let subheading = use_memo(move || {
    if load() && load_text() == "loading" {
        return "The state is loading"
    };

    "The state is not loading"
});


rsx! {
    h1 { "{subheading}" }
}
```

## Derived Elements

The `use_memo` hook is particularly powerful. In addition to primitive values, it can even memoize `Element` objects! We can break up large components into a series of smaller memos for a performance boost.

In practice, you won't need to frequently use Element memoization, but it can be useful in some cases. Most commonly, we can transform the result of some expensive computation directly into an Element without needing to store the intermediate vlaue

```rust
let mut loading_text = use_signal(|| "loading".to_string());

let loading_ui = use_memo(move || {
    let num_chars = loading_text.read().chars().count();
    rsx! { "there are {num_chars} characters!" }
});

rsx! {
    h1 { "Demo" }
    {loading_ui}
}
```

Astute readers will recognize that memoized UI and components are essentially the same concept - components are simply functions of memoized state that return an Element.

## Running Side-effects

The simplest reactive primitive in Dioxus is the `use_effect` hook. It creates a closure that is run any time a tracked value that is run inside the closure changes.


Any value you read inside the closure will become a dependency of the effect. If the value changes, the effect will rerun.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:effect}}
```

```inject-dioxus
DemoFrame {
    reactivity::EffectDemo {}
}
```

## Working with Untracked State

Most of the state in your app will be tracked values. All built in hooks return tracked values, and we encourage custom hooks to do the same. However, there are times when you need to work with untracked state. For example, you may receive a raw untracked value in props. When you read an untracked value inside a reactive context, it will not subscribe to the value:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:non_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::NonReactiveDemo {}
}
```

You can start tracking raw state with the `use_reactive` hook. This hook takes a tuple of dependencies and returns a reactive closure. When the closure is called in a reactive context, it will track subscribe to the dependencies and rerun the closure when the dependencies change.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:use_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::UseReactiveDemo {}
}
```

## Opting Out of Subscriptions

In some situations, you may need to read a reactive value without subscribing to it. You can use the `peek` method to get a reference to the inner value without registering the value as a dependency of the current reactive context:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:peek}}
```

```inject-dioxus
DemoFrame {
    reactivity::PeekDemo {}
}
```

## Making Props Reactive

To avoid losing reactivity with props, we recommend you wrap any props you want to track in a `ReadOnlySignal`. Dioxus will automatically convert `T` into `ReadOnlySignal<T>` when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:making_props_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::MakingPropsReactiveDemo {}
}
```

