
## Running Futures with `spawn`

The [`spawn`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn.html) method spawns a future in the background and returns a `Task` that you can use to cancel the future. Spawn is great for futures you want to start and then forget about like sending analytics data to a server:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn}}
```

```inject-dioxus
DemoFrame {
    asynchronous::SpawnButton {}
}
```

Since spawning in event handlers is very common, Dioxus provides a more concise syntax for async event handlers. If you return a future from an event handler, Dioxus will automatically `spawn` it:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn_simplified}}
```

<div class="warning">

The future you pass to the `spawn` will automatically be cancelled when the component is unmounted. If you need to keep the future running until it is finished, you can use [`spawn_forever`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn_forever.html) instead.

</div>

