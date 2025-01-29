# Handling Asynchronous Tasks  

Asynchronous tasks are a core part of any modern application. Dioxus provides a few different methods to handle asynchronous tasks. This guide will cover how to use each of them. If you already know what kind of asynchronous task you need, you can skip to the section for that task:
- [spawn](#running-futures-with-spawn) is great for futures you need to run in the background that don't return a value
- [use_resource](#asynchronous-state-with-use-resource) handles asynchronous state while retaining control of exactly what happens while the future is running
- It can be combined with [Suspense](#unified-loading-views-with-suspense) to handle many pending tasks with the same loading view

## Running Futures with `spawn`

The [`spawn`](https://docs.rs/dioxus/0.6.2/dioxus/prelude/fn.spawn.html) method spawns a future in the background and returns a `Task` that you can use to cancel the future. Spawn is great for futures you want to start and then forget about like sending analytics data to a server:

```rust
{{#include src/doc_examples/asynchronous.rs:spawn}}
```

```inject-dioxus
DemoFrame {
    asynchronous::SpawnButton {}
}
```

Dioxus will automatically call `spawn` if you return a future from an event handler:

```rust
{{#include src/doc_examples/asynchronous.rs:spawn_simplified}}
```

<div class="warning">

The future you pass to the `spawn` will automatically be cancelled when the component is unmounted. If you need to keep the future running until it is finished, you can use [`spawn_forever`](https://docs.rs/dioxus/0.6.2/dioxus/prelude/fn.spawn_forever.html) instead.

</div>

## Asynchronous State with `use_resource`

The [`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) can be used to derive asynchronous state. It takes a closure that returns a future and returns a tracked value with the current state of the future. Any time a dependency of the resource changes, the resource will rerun:

```rust
{{#include src/doc_examples/asynchronous.rs:use_resource}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResource {}
}
```

> Note: The future you pass to `use_resource` must be cancel safe. Cancel safe futures are futures that can be stopped at any await point without causing causing issues. For example, this task is not cancel safe:
> 
> ```rust
> {{#include src/doc_examples/asynchronous.rs:not_cancel_safe}}
> ```
> 
> 
> ```inject-dioxus
> DemoFrame {
>     asynchronous::NotCancelSafe {}
> }
> ```
>
> Async methods will often mention if they are cancel safe in their documentation.

The `use_resource` hook might look similar to the `use_memo` hook. Both hooks derive a new state and will rerun when any value used to compute that state is written to. However unlike `use_memo`, the resource's output is not memoized with `PartialEq`:

```rust
{{#include src/doc_examples/asynchronous.rs:use_resource_memo}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResourceDemo {}
}
```

## Unified Loading Views with suspense

`SuspenseBoundary` is a convenient way to bundle multiple async tasks into a single loading view. It accepts a loading closure and children. If any of the tasks underneath the suspense boundary are suspended, the loading view will be shown instead of the children. When all of the tasks are resolved, the children will be rendered:

```rust
// {{#include src/doc_examples/asynchronous.rs:suspense_boundary}}
fn Article() -> Element {
    rsx! {
        SuspenseBoundary {
            // When any child components (like ArticleContents) are suspended, this closure will be called and the loading view will be rendered instead of the children
            fallback: |_| rsx! {
                "Loading..."
            },
            ArticleContents {}
        }
    }
}

fn ArticleContents() -> Element {
    let resource = use_resource(|_| todo!())
        // Calling .suspend()? will suspend the component and return early while the future is running
        .suspend()?;

    // Then you can just handle the happy path with the resolved future
    rsx! {
        "Article Contents: {resource}"
    }
}
```
<!-- 
```inject-dioxus
DemoFrame {
    asynchronous::SuspenseBoundary {}
}
``` -->

If you nest multiple suspense boundaries, the closest suspense boundary will capture the suspense and show the loading view:

```rust
{{#include src/doc_examples/asynchronous.rs:suspense_boundary_nested}}
```

<!-- ```inject-dioxus
DemoFrame {
    asynchronous::SuspenseBoundaryNested {}
}
``` -->

You may want more control over what is shown in the loading view depending on what future is blocking the suspense boundary from loading. You can use the `with_loading_placeholder` method to provide a Element to the suspense boundary that it may choose to render in the loading view:

```rust
{{#include src/doc_examples/asynchronous.rs:suspense_boundary_with_loading_placeholder}}
```

<!-- ```inject-dioxus
DemoFrame {
    asynchronous::SuspenseBoundaryWithLoadingPlaceholder {}
}
``` -->

## Suspense with Fullstack

To use suspense in your fullstack application, you need to use the `use_server_future` hook instead of `use_resource`. `use_server_future` handles serialization of the result of the future for hydration. It will also suspend automatically, so you don't need to call `.suspend()` on the future.

```rust
{{#include src/doc_examples/asynchronous.rs:use_server_future}}
```

<!-- ```inject-dioxus
DemoFrame {
    asynchronous::UseServerFuture {}
}
``` -->

When you use suspense with fullstack without streaming enabled, dioxus will wait until all suspended futures are resolved before sending the resolved html to the client.


If you [enable](https://docs.rs/dioxus/0.6.2/dioxus/prelude/struct.ServeConfigBuilder.html#method.enable_out_of_order_streaming) out of order streaming, dioxus will send the finished HTML chunks to the client one at a time as they are resolved:

```rust
{{#include src/doc_examples/asynchronous.rs:use_server_future_streaming}}
```

<!-- ```inject-dioxus
DemoFrame {
    asynchronous::UseServerFutureStreaming {}
}
``` -->
