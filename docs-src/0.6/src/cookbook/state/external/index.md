# Working with External State

This guide will help you integrate your Dioxus application with some external state like a different thread or a websocket connection.

## Working with non-reactive State 

[Coroutines](../../reference/use_coroutine.md) are great tool for dealing with non-reactive (state you don't render directly) state within your application.


You can store your state inside the coroutine async block and communicate with the coroutine with messages from any child components.

```rust
{{#include src/doc_examples/use_coroutine.rs:use_coroutine}}
```

## Making Reactive State External

If you have some reactive state (state that is rendered), that you want to modify from another thread, you can use a signal that is sync. Signals take an optional second generic value with information about syncness. Sync signals have a slightly higher overhead than thread local signals, but they can be used in a multithreaded environment.

```rust
{{#include src/doc_examples/sync_signal.rs}}
```
