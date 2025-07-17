# Handling Asynchronous Tasks

Asynchronous tasks are a core part of any modern application. Dioxus provides a few different methods to handle asynchronous tasks. This guide will cover how to use each of them. If you already know what kind of asynchronous task you need, you can skip to the section for that task:
- [spawn](#running-futures-with-spawn) is great for futures you need to run in the background that don't return a value
- [use_resource](#asynchronous-state-with-use-resource) handles asynchronous state while retaining control of exactly what happens while the future is running
- It can be combined with [Suspense](#unified-loading-views-with-suspense) to handle many pending tasks with the same loading view

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

## Asynchronous State with `use_resource`

The [`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) can be used to derive asynchronous state. It takes an async closure to calculate the state and returns a tracked value with the current state of the future. Any time a dependency of the resource changes, the resource will rerun:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResource {}
}
```

The `use_resource` hook might look similar to the `use_memo` hook, but unlike `use_memo`, the resource's output is not memoized with `PartialEq`. That means any components/reactive hooks that read the output will rerun if the future reruns even if the value it returns is the same:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource_memo}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResourceDemo {}
}
```

> Note: The future you pass to `use_resource` must be cancel safe. Cancel safe futures are futures that can be stopped at any await point without causing causing issues. For example, this task is not cancel safe:
>
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:not_cancel_safe}}
> ```
>
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::NotCancelSafe {}
> }
> ```
>
> It can be fixed by making sure the global state is restored when the future is dropped:
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_safe}}
> ```
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::CancelSafe {}
> }
> ```
>
> Async methods will often mention if they are cancel safe in their documentation.

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

## Conclusion

This guide has covered the basics of asynchronous tasks in Dioxus. More detailed documentation about specific hooks are available in docs.rs:
- [use_resource](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_resource.html)
- [use_server_future](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_server_future.html)
- [SuspenseBoundary](https://docs.rs/dioxus/latest/dioxus/prelude/fn.SuspenseBoundary.html)
- [spawn](https://docs.rs/dioxus/latest/dioxus/prelude/fn.spawn.html)
- [spawn_forever](https://docs.rs/dioxus/latest/dioxus/prelude/fn.spawn_forever.html)

More examples of futures and asynchronous tasks are available in the [example folder](https://github.com/DioxusLabs/dioxus/tree/v0.7xamples) in the dioxus repo.
