# Your First Component

Now that we've initialized our *HotDog* app, we can start building out its components.

## What is a component?

In Dioxus, apps are comprised of individual functions called *Components* that take in some *Properties* and render an *Element*:

```rust
fn DogApp(props: DogAppProps) -> Element {
    // ...
}
```

## Component Properties

All components take an object that outlines which parameters the component can accept. All `Props` structs in Dioxus need to derive the `Properties` trait which requires both `Clone` and `PartialEq`:

```rust
#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String
}
```

Dioxus provides the `#[component]` macro for simplifying how components are defined. This macro converts the parameters of the annotated function into a hidden accompanying struct.

```rust
#[component]
fn DogApp(breed: String) -> Element {
    // ...
}
```

When building apps, you'll frequently use the `#[component]` macro. When building libraries, we generally suggest deriving Props instead.

## Properties are Immutable

If you're familiar with JavaScript, then you might also be familiar with libraries like [React](http://react.dev). Dioxus is *very* similar to React: if you know React then you will feel comfortable with Dioxus.

Just like React, Dioxus components are rendered by calling the function component. On every render, Dioxus makes a `.clone()` of the component's props. This ensures you can't accidentally modify your props which can lead to hard-to-track issues with state management.

```rust
fn DogApp(props: DogAppProps) -> Element {
    tracing::info!("Rendered with breed: {}", props.breed);

    // ...
}
```

Dioxus provides types that make `.clone()` cheaper to call, so don't worry about performance gotchas here.

## Component Functions are Called Multiple Times

Just like React, Dioxus will call your component function multiple times throughout its lifecycle. This is called *re-rendering*. In Dioxus, re-renders are extremely cheap (much cheaper than React!). In most cases you shouldn't worry about re-rendering too frequently.

When Dioxus re-renders your component, it compares the `Element` returned from the *last* render against the `Element` returned in the *current* render.

For example, when the `breed` property changes on the DogApp component, Dioxus will call the DogApp function a second time and compare the previous Element against the new Element.

```rust
#[component]
fn DogApp(breed: String) -> Element {
	rsx! {
		"Breed: {breed}"
	}
}
```

![Diffing](/assets/06_docs/diffing_diagram.png)

Dioxus will re-render your component in only two circumstances:

- When the `Props` change as determined by `PartialEq`
- When a function like `signal.set()` or `signal.write()` calls `Scope.needs_update()`

Unlike React, all Dioxus components are *memoized by default* meaning Dioxus will always compare `Props` before deciding to re-render your component. As an additional optimization, Dioxus only compares dynamic parts of your RSX. Elements that don't contain dynamic data won't be checked for changes.

## Composing Components

In Dioxus, *Components* are composed together to create *Apps*. Each component will hold onto its own state and handle its own updates. This makes it easy to abstract your app into different parts and even share pieces of your app as libraries for others to use.

To compose components together, we'll use the `rsx! {}` macro to define the structure of our app.

```rust
#[component]
fn App() -> Element {
    rsx! {
        Header {}
        DogApp { breed: "corgi" }
        Footer {}
    }
}
```

We'll cover `rsx! {}` in more depth in the [next chapter](rsx.md).
