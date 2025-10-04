# Effects and Memos

Signals provide a foundation for mutable state in Dioxus apps. Calls to `.read()` subscribe reactive scopes and calls to `.write()` queue side-effects.

However, sometimes we want to run *our own* side-effects when a Signal's value changes. Other times, we want to isolate reactive scopes such that changes to a signal do not automatically queue a component to be re-rendered. In these cases, we reach for Memos with `use_memo` and Effects with `use_effect`.

## Multiple Reactive Scopes

To understand Effects and Memos, we need to first understand that a single Signal (or other reactive value) can be read in multiple reactive scopes simultaneously. For instance, a signal may be shared among several components via props. Each component that calls `.read()` on the signal value is automatically subscribed to any changes of the signal's value. When the signal value changes, it runs the re-render side-effect.

Effects and Memos allow us to observe changes in reactive values without re-rendering components. We can isolate smaller units of reactivity with memos and then queue our own side-effects with effects.

![Multiple Readers](/assets/07/multiple-scopes.png)

Memos implement the `Readable` trait (but not the Writable trait!) and thus implement the same ergonomic extensions as signals. Both Memos and Effects are `Copy` and have the same lifecycle and Drop semantics as signals.

## Derived State with Memo

`use_memo` is a reactive primitive that lets you derive state from any tracked value. It takes a closure that computes the new state and returns a tracked value that contains the current state of the memo. When a dependency of the memo changes, the memo will rerun, and a new value will be calculated.

The value returned from the closure will only cause the memo's value to update - and thus any side-effects - when they are not equal, determined by the `PartialEq` between the old and new value.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:memo}}
```

```inject-dioxus
DemoFrame {
    reactivity::MemoDemo {}
}
```

Memos can be useful to perform expensive computations outside the component's reactive scope, preventing re-renders when the inputs change. In this example, by performing our computation *inside* the memo, we prevent the component from re-rendering when either `loading` or `loading_text` changes. Instead, the component will only re-render when the computed memo value changes.

```rust
let mut loading = use_signal(|| false);
let mut loading_text = use_signal(|| "loading".to_string());

let subheading = use_memo(move || {
    if loading() && loading_text() == "loading" {
        return "The state is loading";
    }

    "The state is not loading"
});

rsx! {
    h1 { "{subheading}" }
}
```

## Derived Elements

The `use_memo` hook is particularly powerful. In addition to primitive values, it can even memoize `Element` objects! We can break up large components into a series of smaller memos for a performance boost.

In practice, you won't need to frequently use Element memoization, but it can be useful. Most commonly, we can transform the result of some expensive computation directly into an Element without needing to store the intermediate value:

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

## Running Side-Effects

By default, whenever a Tracked value changes, any reactive scopes observing the value with `.read()` will run side-effects. The classic example is a component: when a signal value changes, the component queues a side-effect that re-renders the component.

![Component renders are effects](/assets/07/component-effect.png)

We can attach our own side-effects to Signals and Memos using the `use_effect` hook. It creates a closure that is run any time a tracked value that is run inside the closure changes.

Any value you read inside the closure will become a dependency of the effect. If the value changes, the effect will rerun.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:effect}}
```

```inject-dioxus
DemoFrame {
    reactivity::EffectDemo {}
}
```

## Prefer Actions over Side-Effects

You might be wondering: "why should I ever run side-effects?" And, indeed, they should not be a frequently used tool in your toolbox. Side-effects can be difficult to reason about and are frequently misused when an action should be preferred.

The classic example of a side-effect is to synchronize UI state with some external state. For example, we might have a `Title {}` component that sets the window's title whenever the title changes:

```rust
fn Title() -> Element {
    let mut text = use_signal(|| "");

    // attach an effect to modify the document title whenever title changes
    use_effect(move || {
        window().document().set_title(text());
    });

    rsx! {
        input {
            oninput: move |e| text.set(e.value()),
            placeholder: "Set the document title"
        }
    }
}
```

This is a valid use case for side-effects. Dioxus guarantees side-effects will be run *after* the UI has been painted to the screen. If we instead set the document title from the oninput handler, another change in state during the same step might cause the `Title {}` component to be unmounted. In this case, the document title will have been set even though the `Title {}` component is no longer present.

However, some actions should *not* be effects. Effects are widely over-used in React and the source of many state headaches. If you can be reasonably sure that the `Title {}` component won't be unmounted, then it is better to set the document title directly in the handler:


```rust
fn Title() -> Element {
    rsx! {
        input {
            oninput: move |e| {
                window().document().set_title(e.value())
            },
            placeholder: "Set the document title"
        }
    }
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

## Making Props Reactive

To avoid losing reactivity with props, we recommend you wrap any props you want to track in a `ReadSignal`. Dioxus will automatically convert `T` into `ReadSignal<T>` when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:making_props_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::MakingPropsReactiveDemo {}
}
```
