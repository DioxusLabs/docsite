# Running Futures with `spawn`

The [`spawn`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn.html) method starts running a future in the background and returns a `Task` that you can use to control the future. It is the basis of all other async hooks in dioxus. You can use spawn to execute one off tasks in event handlers, hooks or other futures:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn}}
```

```inject-dioxus
DemoFrame {
    asynchronous::SpawnButton {}
}
```

Since spawning in event handlers is very common, Dioxus provides a more concise syntax. If you return a future from an event handler, Dioxus will automatically `spawn` it:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn_simplified}}
```

## Automatic Cancellation

The future you pass to the `spawn` will automatically be cancelled when the component is unmounted. If you need to keep the future running until it is finished, you can use [`spawn_forever`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn_forever.html) instead:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn_forever}}
```

## Manual Cancellation

If you want to cancel your future manually, you can call the `cancel` method on the `Task` returned by `spawn` or `spawn_forever`. This will stop the future from running and drop it.

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_button}}
```

```inject-dioxus
DemoFrame {
    asynchronous::CancelButton {}
}
```
