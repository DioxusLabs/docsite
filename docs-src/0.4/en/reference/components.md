# Components

Just like you wouldn't want to write a complex program in a single, long, `main` function, you shouldn't build a complex UI in a single `App` function. Instead, you should break down the functionality of an app in logical parts called components.

A component is a Rust function, named in UpperCammelCase, that takes a `Scope` parameter and returns an `Element` describing the UI it wants to render. In fact, our `App` function is a component!

```rust, no_run
{{#include src/doc_examples/hello_world_desktop.rs:component}}
```

> You'll probably want to add `#![allow(non_snake_case)]` to the top of your crate to avoid warnings about UpperCammelCase component names

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

## Reactivity and re-renders

So far, the examples shown only produce static output: the component function is run once for every time we wish to render the component, and that's it.

However, components in Dioxus are "reactive", meaning that - as much as possible - they will automatically re-render whenever there is a possible change in the data that was used to run the function.

The `Element` type returned by the function is a single description of how the UI must look at a given point in time.
Hence, whenever the UI needs to change - because data changes somewhere - we must provide a new, updated description of the UI. This is why the component function will likely be called many times!

You should avoid expensive computations within it.
Dioxus uses a technique called "memoization" to prevent excessive re-computations of the same data; you will learn more about this, as well as the lifetimes involed, in the chapters on [props](../component_props) and [hooks](../hooks).
