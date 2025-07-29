# Hooks

In Dioxus, state that is local to a component is stored in *hooks*. Hooks provide a simple way for components to store and retrieve state while rendering.

Dioxus hooks work similarly to React's hooks. If you haven't done much web development, hooks might seem particularly unusual. Hooks provide a way of storing state, attaching effects, and enabling composability that integrates well with the full Dioxus reactivity system. Even better - they're less verbose than declaring structs and implementing "render" traits!

## The `use_hook` primitive

All hooks in Dioxus are built on the `use_hook` primitive. While you might never directly use this primitive, it's good to know where all state eventually resides. The `use_hook` primitive is a function that takes an initializer and returns a `.clone()` of the value.

```rust
fn Simple() -> Element {
    let count = use_hook(|| 123);
    rsx! { "{count}" }
}
```

Whenever `use_hook` is called, one of two things happens:
- if this `use_hook` has never been called before, the initializer is ran and a new slot is created
- otherwise, `use_hook` returns a clone of the current value in the slot.

Internally, the "hook index" is incremented by 1 on every call to `use_hook` and reset to 0 before the component re-renders.

![Hook List](/assets/07/hook-list.png)

## Rules of Hooks

In Dioxus, we are transparent with the inner workings of the framework. Because hooks are implemented by walking an internal "hook list," they have certain rules that would cause walking the list to fail and your app to panic. It's important to note that these rules are not arbitrary - they are the intended result of how hooks are implemented.

Hooks use their call order to keep track of what state belongs to which hook. You must call hooks in the same order every time the component is run. To make sure the order is always the same, **you should only call hooks at the top level of a component or another hook**.

These rules mean that there are certain things you can't do with hooks:

### No Hooks in Conditionals
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:conditional}}
```

### No Hooks in Closures
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:closure}}
```

### No Hooks in Loops
```rust
{{#include ../docs-router/src/doc_examples/hooks_bad.rs:loop}}
```

### Prefix hook names with `use_`

By convention, hooks are rust functions that have the `use_` prefix. When you see a function with the `use_` prefix, you should be aware that it internally walks the component's hook list and must follow the Rules of Hooks.

## Why Hooks?

You might be wondering - why use hooks? Aren't structs and traits enough?

Hooks are useful because they compose exceptionally well. We can extract a set of hook primitives together to build complex yet modular interactions with a consistent interface. With one interface, we can encapsulate state *and* effects in just a simple function.

```rust
// This hook is *derived* from an initializer
fn use_document_title(initial: impl FnOnce() -> String) -> Signal<String> {
    let mut title = use_signal(initial);

    // Whenever the title signal changes, we queue a side-effect to modify the window state
    use_effect(move || {
        window().document().set_title(title());
    });

    // We return the reactive String
    title
}
```

Another perk of hooks: we don't need to declare the boilerplate that a struct-based approach might require. A simple component that stores user name and email is simple with hooks:

```rust
#[component]
fn Card(default_name: String) -> Element {
    let mut name = use_signal(|| default_name);
    let mut email = use_signal(|| "".to_string());
    rsx! {
        span { "default: {default_name}" }
        input { oninput: move |e| name.set(e.value()), }
        input { oninput: move |e| email.set(e.value()), }
    }
}
```

whereas struct compnents might be quite verbose:

```rust
struct Card {
    default_name: String
    name: Signal<String>,
    email: Signal<String>
}

#[derive(PartialEq, Clone)]
struct CardProps {
    default_name: String
}
impl Component for Card {
    type Props = CardProps;
    fn new(props: Self::Props) -> Self {
        Self {
            name: Signal::new(props.default_name)
            email: Signal::new("".to_string())
        }
    }

    fn change(&mut self, props: Self::Props) {
        self.default_name = props.defalt_name;
    }

    fn render(mut state: Handle<Self>) -> Element {
        rsx! {
            span { "default: {self.default_name}" }
            input { oninput: move |e| state.name.set(e.value()) }
            input { oninput: move |e| state.email.set(e.value()) }
        }
    }
}
```

With a single function, we are able to express a value initializer, establish automatic value tracking, and handle changes to component properties. We can easily encapsulate shared behavior, queue side-effects, and compose modular primitives.

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

When building your own apps, you should prefer Signals which are covered next.
