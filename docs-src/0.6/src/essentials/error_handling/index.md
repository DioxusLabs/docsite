# Error handling

A selling point of Rust for web development is the reliability of always knowing where errors can occur and being forced to handle them. Dioxus provides ErrorBoundarys to help you handle errors in a declarative way. This guide will teach you how to use ErrorBoundaries and other error handling strategies in Dioxus.

## Returning Errors from Components

Astute observers might have noticed that `Element` is actually a type alias for `Result<VNode, RenderError>`. The `RenderError` type can be created from an error type that implements `Error`. You can use `?` to bubble up any errors you encounter while rendering to the nearest error boundary:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:throw_error}}
```

## Capturing errors with ErrorBoundaries

When you return an error from a component, it gets sent to the nearest error boundary. That error boundary can then handle the error and render a fallback UI with the handle_error closure:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:capture_error}}
```

## Throwing Errors from Event Handlers

In addition to components, you can throw errors from event handlers. If you throw an error from an event handler, it will bubble up to the nearest error boundary just like a component:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:throw_error_event_handler}}
```

## Adding context to errors

You can add additional context to your errors with the [`Context`](https://docs.rs/dioxus/0.6/dioxus/prelude/trait.Context.html) trait. Calling `context` on a `Result` will add the context to the error variant of the `Result`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:add_context}}
```

If you need some custom UI for the error message, you can call `show` on a result to attach an Element to the error variant. The parent error boundary can choose to render this element instead of the default error message:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:show}}
```

## Local Error Handling

If you need more fine-grained control over error states, you can store errors in reactive hooks and use them just like any other value. For example, if you need to show a phone number validation error, you can store the error in a memo and show it below the input field if it is invalid:

```rust, no_run
{{#include ../docs-router/src/doc_examples/untested_06/error_handling.rs:phone_number_validation}}
```

```inject-dioxus
DemoFrame {
    error_handling::PhoneNumberValidation {}
}
```
