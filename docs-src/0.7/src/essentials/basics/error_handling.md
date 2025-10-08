# Error handling

A major selling point of using Rust for web development its renowned reliability. A common sentiment by developers deploying Rust services:

> "We deployed our Rust service and then forgot about it because it just kept running without any issues"

Rust provides developers powerful tools to track where errors occur and easy ways to handle them. Similarly, in Dioxus, we provide additional tools like early returns, a special RenderError type, and ErrorBoundaries to help you handle errors in a declarative way.

## Returning Errors from Components

Recall that Dioxus components are functions that take props and return an `Element`. Astute observers might recognize that the `Element` type is actually a type alias for `Result<VNode, RenderError>`!

The `RenderError` type can be created from an error type that implements `Error`. You can use `?` to bubble up any errors you encounter while rendering to the nearest error boundary:

```rust
{{#include ../docs-router/src/doc_examples/error_handling.rs:throw_error}}
```

The [`RenderError`](https://docs.rs/anyhow/latest/anyhow/) is very similar to the crate `anyhow`'s error type. Unlike anyhow's error type, the `RenderError` lets you attach additional Dioxus-specific metadata when throwing errors.

## Capturing errors with ErrorBoundaries

In JavaScript, you might have used `try` and `catch` to throw and catch errors in your code:

```js
try {
    // Some code that might throw an error
    let result = riskyOperation();
    console.log(result);
} catch (error) {
    // Handle the error
    console.error("Something went wrong:", error.message);
}
```

In Dioxus, you can take a similar try/catch approach within the component tree with error boundaries. Error boundaries let us catch and handle errors produced while rendering our app.

[Error Boundaries](/assets/07/error-boundaries.png)


When you return an error from a component, it gets thrown to the nearest error boundary. That error boundary can then handle the error and render a fallback UI with the handle_error closure:

```rust
{{#include ../docs-router/src/doc_examples/error_handling.rs:capture_error}}
```

## Throwing Errors from Event Handlers

In addition to components, you can throw errors from event handlers. If you throw an error from an event handler, it will bubble up to the nearest error boundary just like a component:

```rust
{{#include ../docs-router/src/doc_examples/error_handling.rs:throw_error_event_handler}}
```

This is extremely useful when handling async work or work that fails frequently.

## Adding context to errors

You can add additional context to your errors with the [`Context`](https://docs.rs/dioxus/0.6/dioxus/prelude/trait.Context.html) trait. Calling `context` on a `Result` will add the context to the error variant of the `Result`:

```rust, no_run
{{#include ../docs-router/src/doc_examples/error_handling.rs:add_context}}
```

## Downcasting Specific Errors

When handling errors in Error Boundaries, we can match on specific types of errors, optionally choosing to capture the error and prevent it from bubbling.

By default, errors are caught by the nearest Error Boundary. In some scenarios, we might not want to catch a specific type of error, like a NetworkError.

In our handler code, we can iterate through the list of captured errors with `.errors()` and then re-throw the error if necessary:

```rust
rsx! {
    ErrorBoundary {
        handle_error: |errors: ErrorContext| {
            // Network errors need to be handled by a different error boundary!
            if let Err(e) = errors.errors().iter().find(|s| s.downcast::<NetworkError>().is_some()) {
                return Err(e.into())
            }

            // Otherwise, handle this error here
            rsx! {
                div { "Oops, we encountered an error" }
            }
        },
        // ...
    }
}
```

## Local Error Handling

If you need more fine-grained control over error states, you can store errors in reactive hooks and use them just like any other value. For example, if you need to show a phone number validation error, you can store the error in a memo and show it below the input field if it is invalid:

```rust, no_run
{{#include ../docs-router/src/doc_examples/error_handling.rs:phone_number_validation}}
```

```inject-dioxus
DemoFrame {
    error_handling::PhoneNumberValidation {}
}
```
