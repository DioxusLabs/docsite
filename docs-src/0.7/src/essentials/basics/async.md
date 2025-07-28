# Async Crash Course

The `Future` trait is the core of async Rust. A future represents a value that may not be available yet. In other languages, this is sometimes called a promise or task. You can read more about them in the [rust book](https://doc.rust-lang.org/book/ch17-00-async-await.html).


We won't cover all of the details of futures here, but there are a few important things to know before using them in Dioxus:
- Futures are lazy. They do not do anything until you `await` them or `spawn` them.
- Futures are concurrent, but not always parallel. In dioxus, all futures run on the main thread, so you should not block inside a future.
- Futures will pause at await points. You should not hold any locks across those await points.
- Futures can be cancelled before they complete. Your futures need to be cancel safe. They should be able to handle stopping at any time without panicking or leaving the application in an inconsistent state.

## Lazy futures

Unlike javascript, Rust futures are lazy. This means that they do not start executing until you call `.await` on them or start them in the background with `spawn`.


This future will never log "Ran" because it is never awaited:

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:async_block}}
```

To run this future, you can either await it in another future or spawn it:

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:await}}
```

## Concurrency vs Parallelism

Concurrency and parallelism are often confused, but the difference has important implications for how you write your applications. Multiple concurrent tasks may be in progress at the same time, but they don't necessarily run at the same time. In Rust, futures are concurrent. They can yield control to other tasks at await points, allowing other tasks to run while they wait for a value to become available.

![concurrent](/assets/07/async_concurrent.png)

On the other hand, multiple parallel tasks can run at exactly the same time on different threads. In rust, you can spawn parallel tasks using the `std::thread` module or libraries like `rayon`.

![parallel](/assets/07/async_parallel.png)

Rust has multiple different async runtimes like `tokio` or `wasm-bindgen-futures`. Dioxus has its own async runtime built on top of a platform specific runtime for each renderer. On desktop it is compatible with tokio.


The dioxus runtime is single threaded which means futures can use `!Send` types, but they need to be careful to never block the thread.

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:blocking}}
```

If you have an expensive task you need to run, you should spawn it on a separate thread using [`std::thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) on desktop or use a [web worker](https://docs.rs/gloo-worker/latest/gloo_worker/) on the web. This will allow the main thread to continue running and keep the UI responsive.

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:thread}}
```

## Handling locks

Futures will pause execution at `.await` points, allowing other tasks to run until the future is ready to continue. You should never hold `read`/`write` locks across `.await` points because another async task could try to use the value while the future is paused and the lock is still open. Instead, you need to ensure that locks are only held for the duration of the critical section and released before awaiting.

![async locks](/assets/07/async_lock_await.png)

## Cancel Safety

Async tasks can be cancelled at any time. The futures you spawn in dioxus may be canceled:
1. When the component they were spawned in is unmounted.
2. When the task is cancelled manually using the `cancel` method on the `Task` returned by `spawn` or `spawn_forever`.
3. When a resource restarts

This means that your futures need to be cancel safe. A cancel safe future is one that can be stopped at any await point without causing issues. For example, if you are using a global state, you need to ensure that the state is restored when the future is dropped:


```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:not_cancel_safe}}
```

```inject-dioxus
DemoFrame {
    asynchronous::NotCancelSafe {}
}
```

It can be fixed by making sure the global state is restored when the future is dropped:
```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_safe}}
```

```inject-dioxus
DemoFrame {
    asynchronous::CancelSafe {}
}
```

Async methods will often mention if they are cancel safe in their documentation.


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
