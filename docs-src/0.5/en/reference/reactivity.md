# Reactivity

In Dioxus, your app is defined as a function of the current state. As the state changes, the parts of your app that depend on that state will automatically re-run. Reactivity automatically tracks state and updates derived state in your application.

## Mutable State

You can create mutable state in Dioxus with Signals. Signals are tracked values that automatically update your app when you change them. They form the skeleton of your app's state from which you can derive other state. Signals are often driven directly from user input through event handlers or async tasks.

You can create a signal with the `use_signal` hook:

```rust
{{#include src/doc_examples/reactivity.rs:signal}}
```

Once you have your signal, you can clone it by calling the signal like a function or get a reference to the inner value with the `.read()` method:

```rust
{{#include src/doc_examples/reactivity.rs:signal_read}}
```

Finally, you can set the value of the signal with the `.set()` method or get a mutable reference to the inner value with the `.write()` method:

```rust
{{#include src/doc_examples/reactivity.rs:signal_write}}
```

## Reactive Scopes

The simplest reactive primitive in Dioxus is the `use_effect` hook. It creates a closure that is run any time a tracked value that is run inside the closure changes.


Any value you read inside the closure will become a dependency of the effect. If the value changes, the effect will rerun.

```rust
{{#include src/doc_examples/reactivity.rs:effect}}
```

```inject-dioxus
DemoFrame {
    reactivity::EffectDemo {}
}
```

## Derived State

`use_memo` is a reactive primitive that lets you derive state from any tracked value. It takes a closure that computes the new state and returns a tracked value with the current state of the memo. Any time a dependency of the memo changes, the memo will rerun.

The value you return from the closure will only change when the output of the closure changes (`PartialEq` between the old and new value returns false).

```rust
{{#include src/doc_examples/reactivity.rs:memo}}
```

```inject-dioxus
DemoFrame {
    reactivity::MemoDemo {}
}
```

## Derived Async State

`use_resource` is a reactive primitive that lets you derive state from any async closure. It takes an async closure that computes the new state and returns a tracked value with the current state of the resource. Any time a dependency of the resource changes, the resource will rerun.

The value you return from the closure will only change when the state of the future changes. Unlike `use_memo`, the resource's output is not memoized with `PartialEq`.

```rust
{{#include src/doc_examples/reactivity.rs:resource}}
```

```inject-dioxus
DemoFrame {
    reactivity::ResourceDemo {}
}
```

## Derived UI

Components are functions that return some UI. They memorize the output of the function just like memos. Components keep track of any dependencies you read inside the component and rerun when those dependencies change.

```rust
{{#include src/doc_examples/reactivity.rs:component}}
```

```inject-dioxus
DemoFrame {
    reactivity::ComponentDemo {}
}
```

## Working with Untracked State

Most of the state in your app will be tracked values. All built in hooks return tracked values, and we encourage custom hooks to do the same. However, there are times when you need to work with untracked state. For example, you may receive a raw untracked value in props. When you read an untracked value inside a reactive context, it will not subscribe to the value:

```rust
{{#include src/doc_examples/reactivity.rs:non_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::NonReactiveDemo {}
}
```

You can start tracking raw state with the `use_reactive` hook. This hook takes a tuple of dependencies and returns a reactive closure. When the closure is called in a reactive context, it will track subscribe to the dependencies and rerun the closure when the dependencies change.

```rust
{{#include src/doc_examples/reactivity.rs:use_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::UseReactiveDemo {}
}
```

## Making Props Reactive

To avoid loosing reactivity with props, we recommend you wrap any props you want to track in a `ReadOnlySignal`. Dioxus will automatically convert `T` into `ReadOnlySignal<T>` when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:

```rust
{{#include src/doc_examples/reactivity.rs:making_props_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::MakingPropsReactiveDemo {}
}
```
