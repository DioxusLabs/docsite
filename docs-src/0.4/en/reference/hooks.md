# Hooks and component state

So far, our components have had no state like a normal Rust function. However, in a UI component, it is often useful to have stateful functionality to build user interactions. For example, you might want to track whether the user has opened a drop-down and render different things accordingly.

Hooks allow us to create state in our components. Hooks are Rust functions that take a reference to [`ScopeState`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.ScopeState.html) (in a component, you can pass `cx`), and provide you with functionality and state.

Dioxus provides many built-in hooks, but if those hooks don't fit your specific use case, you also can [create your own hook](../cookbook/state/custom_hooks/index.md)

## use_state hook

[`use_state`](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_state.html) is one of the simplest hooks.

- You provide a closure that determines the initial value: `let mut count = use_state(cx, || 0);`
- `use_state` gives you the current value, and a way to update it by setting it to something else
- When the value updates, `use_state` makes the component re-render (along with any other component that references it), and then provides you with the new value.


For example, you might have seen the counter example, in which state (a number) is tracked using the `use_state` hook:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_counter.rs:component}}
```
```inject-dioxus
DemoFrame {
   hooks_counter::App {}
}
```

Every time the component's state changes, it re-renders, and the component function is called, so you can describe what you want the new UI to look like. You don't have to worry about "changing" anything – describe what you want in terms of the state, and Dioxus will take care of the rest!

> `use_state` returns your value wrapped in a smart pointer of type [`UseState`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.UseState.html). This is why you can both read the value and update it, even within an event handler.

You can use multiple hooks in the same component if you want:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_counter_two_state.rs:component}}
```

```inject-dioxus
DemoFrame {
  hooks_counter_two_state::App {}
}
```

### Out-of-date UseState

The value `UseState` dereferences to is only set when the use_state hook is called every render. This means that if you move the state into a future, or you write to the state and then immediately read the state, it may return an out-of-date value.

```rust
{{#include ../docs-router/src/doc_examples/untested_04/hooks_out_of_date.rs:component}}
```
```inject-dioxus
DemoFrame {
   // original: hooks_out_of_date::App {}
   __interactive_04::hooks_out_of_date {}
}
```

Instead of using deref to get the inner value from UseState, you can use the [`current`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.UseState.html#method.current) function. This function will always return the current value of the state.

```rust
{{#include ../docs-router/src/doc_examples/untested_04/hooks_out_of_date.rs:fixed}}
```
```inject-dioxus
DemoFrame {
   // original: hooks_out_of_date::fixed::App {}
   __interactive_04::hooks_out_of_date_fixed {}
}
```

## Rules of hooks

The above example might seem a bit magic since Rust functions are typically not associated with state. Dioxus allows hooks to maintain state across renders through a reference to `ScopeState`, which is why you must pass `&cx` to them.

But how can Dioxus differentiate between multiple hooks in the same component? As you saw in the second example, both `use_state` functions were called with the same parameters, so how come they can return different things when the counters are different?

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_counter_two_state.rs:use_state_calls}}
```

This is only possible because the two hooks are always called in the same order, so Dioxus knows which is which. Because the order you call hooks matters, you must follow certain rules when using hooks:

1. Hooks may be only used in components or other hooks (we'll get to that later).
2. On every call to a component function.
3. The same hooks must be called (except in the case of early returns, as explained later in the [Error Handling chapter](../cookbook/error_handling.md)).
4. In the same order.
5. Hook names should start with `use_` so you don't accidentally confuse them with regular
   functions (`use_state()`, `use_ref()`, `use_future()`, etc...).

These rules mean that there are certain things you can't do with hooks:

### No hooks in conditionals

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_bad.rs:conditional}}
```

### No hooks in closures

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_bad.rs:closure}}
```

### No hooks in loops

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_bad.rs:loop}}
```

## use_ref hook

`use_state` is great for tracking simple values. However, in the [`UseState` API](https://docs.rs/dioxus/latest/dioxus/hooks/struct.UseState.html), you may notice that the only way to modify its value is to replace it with something else (e.g., by calling `set`, or through one of the `+=`, `-=` operators). This works well when it is cheap to construct a value (such as any primitive). But what if you want to maintain more complex data in the component's state?

For example, suppose we want to maintain a `Vec` of values. If we stored it with `use_state`, the
only way to add a new value to the list would be to copy the existing `Vec`, add our value to it,
and then replace the existing `Vec` in the state with it. This is expensive! We want to modify the
existing `Vec` instead.

Thankfully, there is another hook for that, `use_ref`! It is similar to `use_state`, but it lets you get a mutable reference to the contained data.

Here's a simple example that keeps a list of events in a `use_ref`. We can acquire write access to the state with `.with_mut()`, and then just `.push` a new value to the state:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_04/hooks_use_ref.rs:component}}
```
```inject-dioxus
DemoFrame {
   // original: hooks_use_ref::App {}
   __interactive_04::hooks_use_ref {}
}
```



> The return values of `use_state` and `use_ref` (
> &nbsp;[`UseState`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.UseState.html) and
> &nbsp;[`UseRef`](https://docs.rs/dioxus/latest/dioxus/prelude/struct.UseRef.html), respectively) are in
> &nbsp;some ways similar to [`Cell`](https://doc.rust-lang.org/std/cell/) and
> &nbsp;[`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) – they provide interior
> &nbsp;mutability. However, these Dioxus wrappers also ensure that the component gets re-rendered
> &nbsp;whenever you change the state.

## Additional resources

- [dioxus_hooks API docs](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/)
- [dioxus_hooks source code](https://github.com/DioxusLabs/dioxus/tree/master/packages/hooks)
