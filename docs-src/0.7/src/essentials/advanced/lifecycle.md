# Component Lifecycle

## Initializing State with `use_hook`

`use_hook` lets you create new state for your component. The closure you pass to `use_hook` will be called once the first time the component is rendered. Every time the component is re-rendered, the value that was created the first run will be re-used.

```rust, no_run
{{#include ../docs-router/src/doc_examples/component_lifecycle.rs:use_hook}}
```
```inject-dioxus
DemoFrame {
    component_lifecycle::UseHookDemo {}
}
```

## Rerendering

You can use tracked values to re-render your component whenever a value changes.

```rust, no_run
{{#include ../docs-router/src/doc_examples/component_lifecycle.rs:rerenders}}
```
```inject-dioxus
DemoFrame {
    component_lifecycle::RerenderDemo {}
}
```

### ⚠️ Don't mutate state in the body of a component

You should avoid changing state in the body of a component. If you read and write to state in the body of a component, you can cause an infinite loop as the component tries to rerender because of the change which triggers another state change.

```rust, no_run
{{#include ../docs-router/src/doc_examples/component_lifecycle.rs:dont_mutate}}
```

Instead, derive state with `use_memo`, `use_resource`, or mutate state in a effect.

## Using Effects

You can use effects to run code whenever a component is rendered.


```rust, no_run
{{#include ../docs-router/src/doc_examples/component_lifecycle.rs:effect}}
```
```inject-dioxus
DemoFrame {
    component_lifecycle::EffectDemo {}
}
```

## Cleaning Up Components with Drop

Before a component is dropped, it will drop all of its hooks. You can use this drop behavior to clean up any resources that your component is using. If you just need the drop effect, you can use the [`use_drop`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_drop.html) hook.

```rust, no_run
{{#include ../docs-router/src/doc_examples/component_lifecycle.rs:drop}}
```
```inject-dioxus
DemoFrame {
    component_lifecycle::DropDemo {}
}
```

## Cleaning Up Elements with `onunmounted`

> **Note:** This feature is coming soon in a future Dioxus release. See [PR #5113](https://github.com/DioxusLabs/dioxus/pull/5113) for progress.

While `use_drop` fires when an entire component is dropped, sometimes you need to know when a specific element within a component is removed from the DOM. For this, you can use the `onunmounted` event.

This is particularly useful when you have conditional elements or lists where individual items can be added or removed while the parent component remains mounted:

```rust, no_run
div {
    onmounted: move |_| {
        // Register the element with some external system
    },
    onunmounted: move |_| {
        // Clean up when the element is removed
    },
}
```

See the [Breaking Out of Dioxus](./breaking_out.md#cleaning-up-when-elements-are-removed-with-onunmounted) chapter for a complete example.
