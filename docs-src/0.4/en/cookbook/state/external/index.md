# Working with External State

This guide will help you integrate your Dioxus application with some external state like a different thread or a websocket connection.

## Working with non-reactive State

[Coroutines](../../../reference/use_coroutine.md) are great tool for dealing with non-reactive (state you don't render directly) state within your application.


You can store your state inside the coroutine async block and communicate with the coroutine with messages from any child components.

```rust
{{#include ../docs-router/src/doc_examples/untested_04/use_coroutine.rs:use_coroutine}}
```

## Making Reactive State External

If you have some reactive state (state that is rendered), that you want to modify from another thread, you can use the [use_rw](https://github.com/DioxusLabs/dioxus-std/blob/master/src/utils/rw/use_rw.rs) hook in the [dioxus-std](https://github.com/DioxusLabs/dioxus-std) crate.


The use_rw hook works like the use_ref hook, but it is Send + Sync which makes it possible to move the hook into another thread.
