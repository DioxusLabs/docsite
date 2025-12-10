# Custom Hooks

Hooks are a great way to encapsulate business logic. If none of the existing hooks work for your problem, you can write your own.

When writing your hook, you can make a function that starts with `use_` and takes any arguments you need. You can then use the `use_hook` method to create a hook that will be called the first time the component is rendered.

## Composing Hooks

To avoid repetition, you can encapsulate business logic based on existing hooks to create a new hook.

For example, if many components need to access an `AppSettings` struct, you can create a "shortcut" hook:

```rust
{{#include ../docs-router/src/doc_examples/hooks_composed.rs:wrap_context}}
```

Or if you want to wrap a hook that persists reloads with the storage API, you can build on top of the use_signal hook to work with mutable state:

```rust
{{#include ../docs-router/src/doc_examples/hooks_composed.rs:use_storage}}
```

## Custom Hook Logic

You can use [`use_hook`](https://docs.rs/dioxus/~0.6/dioxus/prelude/fn.use_hook.html) to build your own hooks. In fact, this is what all the standard hooks are built on!

`use_hook` accepts a single closure for initializing the hook. It will be only run the first time the component is rendered. The return value of that closure will be used as the value of the hook – Dioxus will take it, and store it for as long as the component is alive. On every render (not just the first one!), you will get a reference to this value.

> Note: You can use the `use_on_destroy` hook to clean up any resources the hook uses when the component is destroyed.

Inside the initialization closure, you will typically make calls to other dioxus runtime methods. For example:

- The `use_signal` hook tracks state in the hook value, and uses [`ReactiveContext`](https://docs.rs/dioxus/~0.6/dioxus/prelude/struct.ReactiveContext.html) to make Dioxus re-render any component that has observed it whenever the signal's value changes.

Here is a simplified implementation of the `use_signal` hook:

```rust
{{#include ../docs-router/src/doc_examples/hooks_custom_logic.rs:use_signal}}
```

- The `use_context` hook calls [`consume_context`](https://docs.rs/dioxus/~0.6/dioxus/prelude/fn.consume_context.html) (which would be expensive to call on every render) to get some context from the component

Here is an implementation of the `use_context` and `use_context_provider` hooks:

```rust
{{#include ../docs-router/src/doc_examples/hooks_custom_logic.rs:use_context}}
```


## Building Reactive Hooks

The `use_hook` primitive only provides a way to *store* a value. It does not directly integrate with the Dioxus runtime to allow *modifying* state or queueing effects.

To queue a component to re-render, you can use the `dioxus::core::needs_update` primitive. This sends a message to the internal Dioxus scheduler to queue the current component to be re-rendered.

```rust
log!("Re-rendering!");

rsx! {
    // Clicking this button will force a re-render
    button {
        onclick: move |_| dioxus::core::needs_update(),
        "Queue for re-rendering"
    }
}
```

We can combine `needs_update`, `use_hook`, and [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) to build hooks that work with the Dioxus reactivity system.

```rust
// We declare a new "ReactiveString" type that calls `needs_update` when modified
#[derive(Default)]
struct ReactiveString { inner: Rc<RefCell<String>> }
impl ReactiveString {
    fn get(&self) -> String {
        self.inner.borrow().to_string()
    }
    fn set(&mut self, new: String) {
        *self.inner.write() = new;
        dioxus::core::needs_update();
    }
}

// We store the ReactiveString in a hook
fn use_reactive_string(init: impl FnOnce() -> String) -> ReactiveString {
    let inner = use_hook(|| Rc::new(RefCell::new(init())));
    ReactiveString { inner }
}

// And then when can use it in our component
let mut name = use_reactive_string(|| "Jane".to_string());

rsx! {
    // Clicking the button will cause `needs_update` to be queue a re-render
    button {
        onclick: move |_| name.set("Bob".to_string()),
        "Name: {name.get()}"
    }
}
```

In practice, you should never need to build state management primitives yourself. We provide these examples to help you understand how they work.
