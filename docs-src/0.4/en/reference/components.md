# Components

Just like you wouldn't want to write a complex program in a single, long, `main` function, you shouldn't build a complex UI in a single `App` function. Instead, you should break down the functionality of an app in logical parts called components.

A component is a Rust function, named in UpperCamelCase, that takes a `Scope` parameter and returns an `Element` describing the UI it wants to render. In fact, our `App` function is a component!

```rust, no_run
{{#include src/doc_examples/hello_world_desktop.rs:component}}
```

> `#[component]` is not strictly necessary, but it makes creating components easier. For example, you won't be warned about using UpperCamelCase for functions.

A Component is responsible for some rendering task – typically, rendering an isolated part of the user interface. For example, you could have an `About` component that renders a short description of Dioxus Labs:

```rust, no_run
{{#include src/doc_examples/components.rs:About}}
```
```inject-dioxus
DemoFrame {
	components::About {}
}
```

Then, you can render your component in another component, similarly to how elements are rendered:

```rust, no_run
{{#include src/doc_examples/components.rs:App}}
```
```inject-dioxus
DemoFrame {
	components::App {}
}
```

> At this point, it might seem like components are nothing more than functions. However, as you learn more about the features of Dioxus, you'll see that they are actually more powerful!
