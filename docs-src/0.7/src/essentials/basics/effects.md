
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
