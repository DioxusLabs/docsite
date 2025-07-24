
## Unified Loading Views with suspense

`SuspenseBoundary` is a convenient way to bundle multiple async tasks into a single loading view. It accepts a loading closure and children. You can suspend tasks in children to pause rendering of that child until the future is finished. The suspense boundary will show the loading view instead of the children while any of its children are suspended. Once that suspense is resolved, it will show the children again.


We can use a suspense boundary to show a grid of different breeds of dogs without handling each loading state individually:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:suspense_boundary}}
```

```inject-dioxus
DemoFrame {
    asynchronous::DogGridView {}
}
```

If you need to change the loading view while a specific task is loading, you can provide a different loading view with the `with_loading_placeholder` method. The loading placeholder you return from the method will be passed to the suspense boundary and may choose to render it instead of the default loading view:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:suspense_boundary_with_loading_placeholder}}
```

```inject-dioxus
DemoFrame {
    asynchronous::DogGridViewWithLoadingPlaceholder {}
}
```

## Suspense with Fullstack

To use suspense in your fullstack application, you need to use the `use_server_future` hook instead of `use_resource`. `use_server_future` handles serialization of the result of the future for hydration. It will also suspend automatically, so you don't need to call `.suspend()` on the future.

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_server_future}}
```

```inject-dioxus
DemoFrame {
    asynchronous::DogGridFullstack {}
}
```

Unlike `use_resource`, `use_server_future` is only reactive in the closure, not the future itself. If you need to subscribe to another reactive value, you need to read it in the closure before passing it to the future:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_server_future_reactive}}
```

When you use suspense with fullstack without streaming enabled, dioxus will wait until all suspended futures are resolved before sending the resolved html to the client. If you [enable](https://docs.rs/dioxus/0.7/dioxus/prelude/struct.ServeConfigBuilder.html#method.enable_out_of_order_streaming) out of order streaming, dioxus will send the finished HTML chunks to the client one at a time as they are resolved:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_server_future_streaming}}
```

![Out of order streaming](/assets/06_docs/streaming_dogs.mp4)
