# Error handling

A selling point of Rust for web development is the reliability of always knowing where errors can occur and being forced to handle them

However, we haven't talked about error handling at all in this guide! In this chapter, we'll cover some strategies in handling errors to ensure your app never crashes.



## The simplest – returning None

Astute observers might have noticed that `Element` is actually a type alias for `Option<VNode>`. You don't need to know what a `VNode` is, but it's important to recognize that we could actually return nothing at all:

```rust
{{#include src/doc_examples/error_handling.rs:none}}
```

This lets us add in some syntactic sugar for operations we think *shouldn't* fail, but we're still not confident enough to "unwrap" on.

> The nature of `Option<VNode>` might change in the future as the `try` trait gets upgraded.

```rust
{{#include src/doc_examples/error_handling.rs:try_hook}}
```

## Early return on result

Because Rust can't accept both Options and Results with the existing try infrastructure, you'll need to manually handle Results. This can be done by converting them into Options or by explicitly handling them. If you choose to convert your Result into an Option and bubble it with a `?`, keep in mind that if you do hit an error you will lose error information and nothing will be rendered for that component.

```rust
{{#include src/doc_examples/error_handling.rs:try_result_hook}}
```

Notice that while hooks in Dioxus do not like being called in conditionals or loops, they *are* okay with early returns. Returning an error state early is a completely valid way of handling errors.


## Match results

The next "best" way of handling errors in Dioxus is to match on the error locally. This is the most robust way of handling errors, but it doesn't scale to architectures beyond a single component.

To do this, we simply have an error state built into our component:

```rust
{{#include src/doc_examples/error_handling.rs:use_error}}
```

Whenever we perform an action that generates an error, we'll set that error state. We can then match on the error in a number of ways (early return, return Element, etc).


```rust
{{#include src/doc_examples/error_handling.rs:match_error}}
```

## Passing error states through components

If you're dealing with a handful of components with minimal nesting, you can just pass the error handle into child components.

```rust
{{#include src/doc_examples/error_handling.rs:match_error_children}}
```

Much like before, our child components can manually set the error during their own actions. The advantage to this pattern is that we can easily isolate error states to a few components at a time, making our app more predictable and robust.

## Throwing errors

Dioxus provides a much easier way to handle errors: throwing them. Throwing errors combines the best parts of an error state and early return: you can easily throw and error with `?`, but you keep information about the error so that you can handle it in a parent component.

You can call `throw` on any `Result` type that implements `Debug` to turn it into an error state and then use `?` to return early if you do hit an error. You can capture the error state with an `ErrorBoundary` component that will render the a different component if an error is thrown in any of its children.

```rust
{{#include src/doc_examples/error_handling.rs:throw_error}}
```

You can even nest `ErrorBoundary` components to capture errors at different levels of your app.

```rust
{{#include src/doc_examples/error_handling.rs:nested_throw}}
```

This pattern is particularly helpful whenever your code generates a non-recoverable error. You can gracefully capture these "global" error states without panicking or handling state for each error yourself.
