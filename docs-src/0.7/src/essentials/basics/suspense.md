# Suspense

[Resources](../basics/resources.md) let you load data asynchronously in Dioxus, but it can be cumbersome to handle the loading state of each resource individually. Dioxus provides a `SuspenseBoundary` component to group multiple asynchronous tasks and show a loading view while any of them are suspended.

You can create a `SuspenseBoundary` with a loading closure and children. Then you can call `.suspend()?` on any resource inside the children to pause rendering of that component until the future is finished. The suspense boundary will show the loading view while any of its children are suspended. Once that suspense is resolved, it will show the children again.

We can use a suspense boundary to show a grid of different breeds of dogs without handling each loading state individually:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:suspense_boundary}}
```

```inject-dioxus
DemoFrame {
    asynchronous::DogGridView {}
}
```

## Customizing the loading view from children

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

Dioxus fullstack will wait for suspended futures during server-side rendering. This means your async data loading starts sooner and search engines can see the resolved version of your page. However, using suspense in fullstack does require some changes for hydration compatibility.

To use suspense in your fullstack application, you need to switch every suspended resource to the `use_server_future` hook. `use_server_future` handles serializing the result of the future on the server and deserializing that result on the client. It will also suspend automatically, so you don't need to call `.suspend()` on the resource.

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

### Streaming Suspense

The default behavior for server side rendering is to wait for all suspended futures then send the fully resolved page. If you [enable](https://docs.rs/dioxus/~0.7/dioxus/prelude/struct.ServeConfig.html#method.enable_out_of_order_streaming) out of order streaming, dioxus will send the finished HTML chunks to the client one at a time as they are resolved. This lets you show the loading views in your suspense boundaries while you are still waiting for other futures to resolve on the server:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_server_future_streaming}}
```

![Out of order streaming](/assets/06_docs/streaming_dogs.mp4)


For more information on streaming, see the [streaming documentation](../../essentials/fullstack/streaming.md).
