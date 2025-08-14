'#' Rendering Lists

## Iterators and inline `for`

In the [Conditional Rendering](./conditional.md) chapter we showed how Rust expressions become Dynamic Nodes in RSX blocks.

Any iterator that returns `Element` or an object that implements `IntoVnode` can be used inside RSX blocks to render lists:

```rust
rsx! {
    // mapping existing iterators
    {(0..10).map(|idx| rsx! { "item {idx}" })}

    // or calling .iter()
    {users.iter().map(|user| rsx!{ User { id: user.id } })}
}
```

Dioxus provides a small amount of syntax sugar to make using iterators a bit nicer. Instead of wrapping your iterator in an expression, you can instead move it to an inline `for` block:

```rust
rsx! {
    for idx in 0..10 {
        "Item {idx}"
    }

    for user in users.iter() {
        User { id: user.id }
    }
}
```

If we're using iterators alongside wrapper types like a `Signal<T>`, we need to call `.iter()` first:


```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::ForLoop {}
}
```

## Keys are Required

Each item in your list should have unique value that is stable across rerenders called a key. Keys are used to identify how items move between renders. Without keys, it is easy to accidentally lose or move state when you reorder items in a list. We can add keys to our list items by using the `key` attribute:

```rust, no_run
{{#include ../docs-router/src/doc_examples/building_uis_with_rsx.rs:keyed_for_loop}}
```

```inject-dioxus
DemoFrame {
    building_uis_with_rsx::KeyedForLoop {}
}
```

You can usually find a suitable key from your state itself. Most data in your UI has some unique identifier that can be used to disambiguate it from other entries in the collection. For example, users typically have a unique ID, time samples have a unique timestamp, and emails have a unique sender address.

```rust
rsx! {
    for user in users {
        div { key: "{user.id}", "{user.name}" }
    }
}
```

Note that some fields might *seem* like a suitable unique ID, but are not. Two users might share the same name, two data points might share a timestamp, or not every entry actually has the required data present. In these cases, we recommend creating a unique ID for each item based on a global "generation" counter. Typically, a database might provide this, but we can also use state to manage this:

```rust
let mut generation = use_signal(|| 0);
let mut items = use_signal(|| vec![]);

let mut add_item = move |name| {
    let id = generation();
    generation.set(id + 1);
    items.write().push(Item { id, name })
};

rsx! {
    for item in items.iter() {
        div { key: "{item.id}", "{item.name}" }
    }
    // ...
}
```

The *worst* key to use is the index of the item in the collection. This is *guaranteed* to never be unique and almost certainly will cause either performance issues or loss of state in components.

```rust
// ❌ do not do this!
rsx! {
    for (idx, user) in users.iter().enumerate() {
        div { key: "{idx}", "{user.name}" }
    }
}
```

## The Fragment Component

In some cases, your iterators might return multiple root elements. This doesn't give us a great location to place a key. RSX automatically uses the first element's key as iterator key, if it exists:

```rust
rsx! {
    for item in items.iter() {
        h1 { key: "{item.id}", "{item.name}" }
        p { "content" }
    }
}
```

If there's no easy way to attach a key to the first element, you can use a `Fragment` component to wrap the elements and give that the key instead:

```rust
rsx! {
    for item in items.iter() {
        Fragment {
            h1 { "Item {item.id}" }
        }
    }
}
```

The `Fragment` component is nothing special. It is plainly a component that forwards its children as its body. However, RSX sees it as valid element syntax and is able to assign the key properly.

```rust
fn Fragment(children: Element) -> Element {
    children
}
```


## Borrowed State

When rendering lists, you might want to borrow some of data contents into child components. The common Dioxus container, Signal, is not capable of providing references to child components.

```rust
let items = use_signal(|| vec![]);

rsx! {
    for item in items.iter() {
        Card {
            // ❌ not possible to forward a reference to the child.
            content: item.name.as_str()
        }
    }
}
```

While earlier versions of Dioxus did allow forwarding references, we eventually disabled this feature since it relied on some unsound Rust semantics. Instead, we recommend three strategies:

- Use owned data and `clone` if its cheap to do so
- Pass the collection *and* the index to the component
- Use a `MappedSignal` to map the entry out of the collection

MappedSignals make it possible to map items out of collections using a key as index:

```rust
fn app() -> Element {
    let mut vec = use_signal(|| vec![0]);

    rsx! {
        div {
            for i in 0..vec.len() {
                // we call `.map()` on the signal to create a MappedSignal
                Child { count: vec.map(move |v| &v[i]) }
            }
        }
    }
}

#[component]
fn Child(count: MappedSignal<i32>) -> Element {
    rsx! { "{count}" }
}
```

For complex apps, we recommend using building a system around global context or using a MappedSignal.
