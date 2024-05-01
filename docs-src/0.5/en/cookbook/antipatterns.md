# Antipatterns

This example shows what not to do and provides a reason why a given pattern is considered an "AntiPattern". Most anti-patterns are considered wrong for performance or code re-usability reasons.

## Unnecessarily Nested Fragments

Fragments don't mount a physical element to the DOM immediately, so Dioxus must recurse into its children to find a physical DOM node. This process is called "normalization". This means that deeply nested fragments make Dioxus perform unnecessary work. Prefer one or two levels of fragments / nested components until presenting a true DOM element.

Only Component and Fragment nodes are susceptible to this issue. Dioxus mitigates this with components by providing an API for registering shared state without the Context Provider pattern.

```rust
{{#include src/doc_examples/anti_patterns.rs:nested_fragments}}
```

## Incorrect Iterator Keys

As described in the [dynamic rendering chapter](../reference/dynamic_rendering#the), list items must have unique keys that are associated with the same items across renders. This helps Dioxus associate state with the contained components and ensures good diffing performance. Do not omit keys, unless you know that the list will never change.

```rust
{{#include src/doc_examples/anti_patterns.rs:iter_keys}}
```

## Avoid Interior Mutability in Props

While it is technically acceptable to have a `Mutex` or a `RwLock` in the props, they will be difficult to use.

Suppose you have a struct `User` containing the field `username: String`. If you pass a `Mutex<User>` prop to a `UserComponent` component, that component may wish to write to the `username` field. However, when it does, the parent component will not be aware of the change, and the component will not re-render which causes the UI to be out of sync with the state. Instead, consider passing down a reactive value like a `Signal` or immutable data.

```rust
{{#include src/doc_examples/anti_patterns.rs:interior_mutability}}
```

## Avoid Updating State During Render

Every time you update the state, Dioxus needs to re-render the component – this is inefficient! Consider refactoring your code to avoid this.

Also, if you unconditionally update the state during render, it will be re-rendered in an infinite loop.

```rust
{{#include src/doc_examples/anti_patterns.rs:updating_state}}
```

## Avoid Large Groups of State

It can be tempting to have a single large state struct that contains all of your application's state. However, this can lead to issues:
- It can be easy to accidentally mutate the state in a way that causes an infinite loop
- It can be difficult to reason about when and how the state is updated
- It can lead to performance issues because many components will need to re-render when the state changes

Instead, consider breaking your state into smaller, more manageable pieces. This will make it easier to reason about the state, avoid update loops, and improve performance.

```rust
{{#include src/doc_examples/anti_patterns.rs:large_state}}
```

## Running Non-Deterministic Code in the Body of a Component

If you have a component that contains non-deterministic code, that code should generally not be run in the body of the component. If it is put in the body of the component, it will be executed every time the component is re-rendered which can lead to performance issues.

Instead, consider moving the non-deterministic code into a hook that only runs when the component is first created or an effect that reruns when dependencies change.

```rust
{{#include src/doc_examples/anti_patterns.rs:non_deterministic}}
```

## Overly Permissive PartialEq for Props

You may have noticed that `Props` requires a `PartialEq` implementation. That `PartialEq` is very important for Dioxus to work correctly. It is used to determine if a component should re-render or not when the parent component re-renders.

If you cannot derive `PartialEq` for your `Props`, you will need to implement it yourself. If you do implement `PartialEq`, make sure to return `false` any time the props change in a way that would cause the UI in the child component to change.

In general, returning `false` from `PartialEq` if you aren't sure if the props have changed or not is better than returning `true`. This will help you avoid out of date UI in your child components.

```rust
{{#include src/doc_examples/anti_patterns.rs:permissive_partial_eq}}
```