# Custom Hooks

Hooks are a great way to encapsulate business logic. If none of the existing hooks work for your problem, you can write your own.

When writing your hook, you can make a function that starts with `use_` and takes any arguments you need. You can then use the `use_hook` method to create a hook that will be called the first time the component is rendered.

## Composing Hooks

To avoid repetition, you can encapsulate business logic based on existing hooks to create a new hook.

For example, if many components need to access an `AppSettings` struct, you can create a "shortcut" hook:

```rust
{{#include src/doc_examples/hooks_composed.rs:wrap_context}}
```

Or if you want to wrap a hook that persists reloads with the storage API, you can build on top of the use_signal hook to work with mutable state:

```rust
{{#include src/doc_examples/hooks_composed.rs:use_storage}}
```

## Custom Hook Logic

You can use [`use_hook`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_hook.html) to build your own hooks. In fact, this is what all the standard hooks are built on!

`use_hook` accepts a single closure for initializing the hook. It will be only run the first time the component is rendered. The return value of that closure will be used as the value of the hook – Dioxus will take it, and store it for as long as the component is alive. On every render (not just the first one!), you will get a reference to this value.

> Note: You can use the `use_on_destroy` hook to clean up any resources the hook uses when the component is destroyed.

Inside the initialization closure, you will typically make calls to other `cx` methods. For example:

- The `use_signal` hook tracks state in the hook value, and uses [`schedule_update`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.schedule_update.html) to make Dioxus re-render the component whenever it changes.

Here is a simplified implementation of the `use_signal` hook:

```rust
{{#include src/doc_examples/hooks_custom_logic.rs:use_signal}}
```

- The `use_context` hook calls [`consume_context`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.consume_context.html) (which would be expensive to call on every render) to get some context from the component

Here is an implementation of the `use_context` and `use_context_provider` hooks:

```rust
{{#include src/doc_examples/hooks_custom_logic.rs:use_context}}
```
