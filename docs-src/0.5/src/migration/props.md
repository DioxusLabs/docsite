# Props Migration

In dioxus 0.4, props are passed into the component through the scope. In dioxus 0.5, props are passed into the component through the props struct directly.

## Owned Props

The props were borrowed with the lifetime from the scope. The props are cloned every render, and passed into the component as an owned value.

Dioxus 0.4:
```rust
#[component]
fn Comp(cx: Scope, name: String) -> Element {
    // You pass in an owned prop, but inside the component, it is borrowed (name is the type &String inside the function)
    let owned_name: String = name.clone();

    cx.render(rsx! {
        "Hello {owned_name}"
    })
}
```
Dioxus 0.5:
```rust
{{#include src/doc_examples/migration_props.rs:owned_props}}
```

Because props are cloned every render, making props Copy is recommended. You can easily make a field Copy by accepting `ReadOnlySignal<T>` instead of `T` in the props struct:

```rust
{{#include src/doc_examples/migration_props.rs:copy_props}}
```

## Borrowed Props

Borrowed props are removed in dioxus 0.5. Mapped signals can act similarly to borrowed props if your props are borrowed from state.

Dioxus 0.4:
```rust
fn Parent(cx: Scope) -> Element {
    let state = use_state(cx, || (1, "World".to_string()));
    rsx! {
        BorrowedComp {
            name: &state.get().1
        }
    }
}

#[component]
fn BorrowedComp<'a>(cx: Scope<'a>, name: &'a str) -> Element<'a> {
    rsx! {
        "Hello {name}"
    }
}
```

Dioxus 0.5:
```rust
{{#include src/doc_examples/migration_props.rs:borrowed_props}}
```

## Manual Props

Manual prop structs in dioxus 0.5 need to derive `Clone` in addition to `Props` and `PartialEq`:

Dioxus 0.4:
```rust
#[derive(Props, PartialEq)]
struct ManualProps {
    name: String,
}

// Functions accept the props directly instead of the scope
fn ManualPropsComponent(cx: Scope<ManualProps>) -> Element {
    render! {
        "Hello {cx.props.name}"
    }
}
```

Dioxus 0.5:
```rust
{{#include src/doc_examples/migration_props.rs:manual_props}}
```
