# Interactivity

In this chapter, we will add a preview that loads an article when you hover over a post.

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

Next, we need to detect when the user hovers over a section or focuses a link. We can use an [event listener](../reference/interactivity/event_handlers.md) to listen for a hover event.

Event handlers are similar to regular attributes, but their name usually starts with `on`- and they accept closures as values. The closure will be called whenever the event it listens for is triggered. When an event is triggered, information about the event is passed to the closure though the [Event](https://docs.rs/dioxus/latest/dioxus/prelude/struct.Event.html) structure.

Let's create a [`onmouseenter`](https://docs.rs/dioxus/latest/dioxus/events/fn.onmouseenter.html) event listener in the `StoryListing` component:

```rust
{{#include src/doc_examples/hackernews_state.rs:story_listing_listener}}
```

## State

So far our components have had no state like a normal rust functions. To make our application change when we hover over a link we need state to store the currently hovered link in the root of the application.

You can create state in dioxus using hooks. Hooks are Rust functions that take a reference to `ScopeState` (in a component, you can pass `cx`), and provide you with functionality and state.

We can use the `use_shared_state_provider` to create state in the root of our application that is available to all child components. We can then use the `use_shared_state` hook to read and modify that state in the `Preview` and `StoryListing` components.

> Note: You should prefer local state hooks like use_state or use_ref when you only use state in one component. Because we use state in multiple components, we can use a [global state pattern](../cookbook/state/global.md)

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

### The Rules of Hooks

Dioxus uses the order you call hooks to differentiate between hooks. Because the order you call hooks matters, you must follow certain rules when using hooks:

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
