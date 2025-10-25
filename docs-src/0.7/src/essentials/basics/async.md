# Async and Futures

Not all actions complete immediately. Some actions, like a network request, require waiting for system input/output (IO). While waiting for network response, we want to provide status updates, add a loading spinner, and most importantly: avoid blocking the UI thread. Code that blocks the UI thread will prevent further user input, making the UI feel janky and unintuitive.

Rust provides a built-in way of handling asynchronous work with its built-in async/await system. Dioxus provides a first-class integration with Rust's async/await system.

## Future: Rust's Async Primitive

The `Future` trait is the core of async Rust. A future represents a value that may not yet be ready. In other languages, this is sometimes called a *Promise* or *Task*. You can read more about Futures in the [Rust book](https://doc.rust-lang.org/book/ch17-00-async-await.html).

We won't cover all the details of futures here, but there are a few important things to know before using them in Dioxus:

- **Futures are lazy**: They do not do anything until you `await` them or `spawn` them.
- **Futures are concurrent but not always parallel**: In Dioxus, all futures run on the main thread.
- **Futures pause at await points**: You should not hold any locks across those await points.
- **Futures can be cancelled before they complete**: Your futures need to be "cancel safe."

Futures should be able to handle stopping at any time without panicking or leaving the application in an inconsistent state. They should also be careful not to run blocking operations that lock the main thread.

The lifecycle of a future follows a consistent structure:

- A callback calls an `async fn` or an async closure
- The async function returns a Future
- A `dioxus::spawn()` call submits the future to the Dioxus runtime, returning a `Task`
- The Future is polled in the background until it returns a `Ready` value
- If the Future is cancelled, Rust calls its `Drop` implementation

![Future Diagram](/assets/07/future-diagram.png)

## Lazy futures

Unlike JavaScript's Promises, Rust futures are *lazy*. This means that they do not start executing until you call `.await` or start them in the background with `spawn`.

This Future will never log "Ran" because it is never awaited:

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:async_block}}
```

To run this Future, you can either await it in another Future or spawn it:

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:await}}
```

You can stop polling a Future any time or customize how a Future is polled using the [futures](https://crates.io/crates/futures) crate.

## Running Futures with `spawn`

The Dioxus [`spawn`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn.html) function starts running a Future in the background and returns a `Task` that you can use to control the Future. It is the basis of all other async hooks in dioxus. You can use spawn to execute one-off tasks in event handlers, hooks or other Futures:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn}}
```

```inject-dioxus
DemoFrame {
    asynchronous::SpawnButton {}
}
```

Since spawning in event handlers is very common, Dioxus provides a more concise syntax. If you return a Future from an event handler, Dioxus will automatically `spawn` it:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:spawn_simplified}}
```

## Running Futures with `use_action`

You'll frequently want to spawn an action in response to some user input and store the result. For rapid user input, you'll also want to cancel previous actions to prevent race conditions. Dioxus provides a built-in hook that simplifies this pattern with a function called `use_action`.

The `use_action` hook combines signals and tasks into a single unified interface. Simply call `use_action` with a callback that returns a `Result<T>`:

```rust
// Whenever this action is called, it will re-run the future and return the result.
let mut breed = use_action(move |breed| async move {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct DogApi {
        message: String,
    }

    reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
        .await
        .unwrap()
        .json::<DogApi>()
        .await
});
```

You can call the action with `.call()`:

```rust
rsx! {
    button {
        onclick: move |_| {
            breed.call(cur_breed.clone());
        },
        "{cur_breed}"
    }
}
```

And then, elsewhere in your component, you can read the result with `.value()`:

```rust
match breed.value() {
    Some(Ok(res)) => rsx! {
        img { src: "{res.read().message}" }
    },
    Some(Err(_e)) => rsx! {
        div { "Failed to fetch a dog, please try again." }
    },
    None => rsx! {
        div { "Click the button to fetch a dog!" }
    },
}
```

If an action is pending, calling `.call()` will cancel the current action's `Task`, replacing it with the new task.

## Automatic Cancellation

The Future you pass to the `spawn` will automatically be cancelled when the component is unmounted. If you need to keep the Future running until it is finished, you can use [`spawn_forever`](https://docs.rs/dioxus/0.7/dioxus/prelude/fn.spawn_forever.html) instead:

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

## Cancel Safety

Async tasks can be cancelled at any time. The futures you spawn in dioxus may be canceled:
1. When the component they were spawned in is unmounted.
2. When the task is cancelled manually using the `cancel` method on the `Task` returned by `spawn` or `spawn_forever`.
3. When a resource restarts

This means that your futures need to be cancel safe. A cancel-safe future is one that can be stopped at any await point without causing issues. For example, if you are using a global state, you need to ensure that the state is restored when the future is dropped:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:not_cancel_safe}}
```

```inject-dioxus
DemoFrame {
    asynchronous::NotCancelSafe {}
}
```

You can mitigate issues with cancellation by cleaning up resources manually. For example, by making sure global state is restored when the future is dropped:
```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_safe}}
```

```inject-dioxus
DemoFrame {
    asynchronous::CancelSafe {}
}
```

Async methods will often mention if they are cancel safe in their documentation. Generally, most futures you'll encounter when building Dioxus apps *are* cancel safe.

## Concurrency vs Parallelism

Concurrency and parallelism are often confused, but the difference has important implications for how you write your applications. Multiple concurrent tasks may be in progress at the same time, but they don't necessarily run at the same time. In Rust, futures are concurrent. They can yield control to other tasks at await points, allowing other tasks to run while they wait for a value to become ready.

![concurrent](/assets/07/async_concurrent.png)

In contrast, multiple parallel tasks can run at exactly the same time on different threads. In Rust, you can spawn parallel tasks using the `std::thread` module or libraries like `rayon`.

![parallel](/assets/07/async_parallel.png)

Rust has multiple different async runtimes like `tokio` or `wasm-bindgen-futures`. Dioxus provides its own async runtime built on top of a platform specific runtime for each renderer. On desktop and mobile, we use Tokio to progress futures.

The Dioxus runtime is single threaded which means futures can use `!Send` types, but they need to be careful to never block the thread.

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:blocking}}
```

If you have an expensive task you need to run, you should spawn it on a separate thread using [`std::thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) on desktop/mobile or use a [web worker](https://docs.rs/gloo-worker/latest/gloo_worker/) on the web. This will allow the main thread to continue running and keep the UI responsive.

```rust
{{#include ../docs-router/src/doc_examples/async_crash_course.rs:thread}}
```

## Handling locks

Futures will pause execution at `.await` points, allowing other tasks to run until the future is ready to continue. You should never hold `read`/`write` locks across `.await` points because another async task could try to use the value while the future is paused and the lock is still open. Instead, you need to ensure that locks are only held for the duration of the critical section and released before awaiting.

![async locks](/assets/07/async_lock_await.png)

## Long-lived Futures

In some apps, you might want to include long-lived tasks that exist for the lifetime of the app. This might be a background sync engine or a thread listening to some system IO. For these use cases, we provide the `spawn_forever` function. This works exactly the same as `spawn`, but instead of spawning the future under the *current* component, the future is attached to the *root* component. Because the root component is never unmounted, the task continues until the app is closed.

```rust
use_hook(|| spawn_forever(async move {
    println!("Starting a background task!");
}));
```

This function does have its drawbacks and is meant for advanced use cases. If any resources like a Signal are used in this future, they must *also* be valid for the lifetime of the app. Using Signals after they have been dropped will lead to a panic and crash your app!
