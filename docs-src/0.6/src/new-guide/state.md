# Interactivity

In this chapter, we will add a preview for articles you hover over or links you focus on.

## Creating a Preview

First, let's split our app into a Stories component on the left side of the screen, and a preview component on the right side of the screen:

```rust
{{#include src/doc_examples/hackernews_state.rs:app_v1}}
```

```inject-dioxus
DemoFrame {
    hackernews_state::app_v1::App {}
}
```

## Event Handlers

Next, we need to detect when the user hovers over a section or focuses a link. We can use an [event listener](../reference/event_handlers.md) to listen for the hover and focus events.

Event handlers are similar to regular attributes, but their name usually starts with `on`- and they accept closures as values. The closure will be called whenever the event it listens for is triggered. When an event is triggered, information about the event is passed to the closure through the [Event](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html) structure.

Let's create a [`onmouseenter`](https://docs.rs/dioxus/latest/dioxus/events/fn.onmouseenter.html) event listener in the `StoryListing` component:

```rust
{{#include src/doc_examples/hackernews_state.rs:story_listing_listener}}
```

> You can read more about Event Handlers in the [Event Handler reference](../reference/event_handlers.md)

## State

So far our components have had no state like normal rust functions. To make our application change when we hover over a link we need state to store the currently hovered link in the root of the application.

You can create state in dioxus using hooks. Hooks are Rust functions you call in a constant order in a component that add additional functionality to the component.

In this case, we will use the `use_context_provider` and `use_context` hooks:

- You can provide a closure to `use_context_provider` that determines the initial value of the shared state and provides the value to all child components
- You can then use the `use_context` hook to read and modify that state in the `Preview` and `StoryListing` components
- When the value updates, the `Signal` will cause the component to re-render, and provides you with the new value

> Note: You should prefer local state hooks like use_signal or use_signal_sync when you only use state in one component. Because we use state in multiple components, we can use a [global state pattern](../reference/context.md)

```rust
{{#include src/doc_examples/hackernews_state.rs:shared_state_app}}
```

```rust
{{#include src/doc_examples/hackernews_state.rs:shared_state_stories}}
```

```rust
{{#include src/doc_examples/hackernews_state.rs:shared_state_preview}}
```

```inject-dioxus
DemoFrame {
    hackernews_state::App {}
}
```

> You can read more about Hooks in the [Hooks reference](../reference/hooks.md)

### The Rules of Hooks

Hooks are a powerful way to manage state in Dioxus, but there are some rules you need to follow to insure they work as expected. Dioxus uses the order you call hooks to differentiate between hooks. Because the order you call hooks matters, you must follow these rules:

1. Hooks may be only used in components or other hooks (we'll get to that later)
2. On every call to the component function
   1. The same hooks must be called
   2. In the same order
3. Hooks name's should start with `use_` so you don't accidentally confuse them with regular functions

These rules mean that there are certain things you can't do with hooks:

#### No Hooks in Conditionals
```rust
{{#include src/doc_examples/hooks_bad.rs:conditional}}
```

#### No Hooks in Closures
```rust
{{#include src/doc_examples/hooks_bad.rs:closure}}
```

#### No Hooks in Loops
```rust
{{#include src/doc_examples/hooks_bad.rs:loop}}
```
