# Handling Asynchronous Tasks

Asynchronous tasks are a core part of any modern application. They let you define code waits for network requests, timers, or OS events without blocking the thread. 
- [`spawn`](./futures.md) starts an asynchronous task in the background.
- [`use_resource`](./resources.md) derives async state from a future.
- [`SuspenseBoundary`](./suspense.md) provides a unified way to handle loading states for async tasks.

## Futures

The `Future` trait is the core of async Rust. A future represents a value that may not be available yet. In other languages, this may be called a promise or task. You can read more about them in the [rust book](https://doc.rust-lang.org/book/ch17-00-async-await.html).


While we won't cover all of the details of futures here there are a few key points to understand about futures when using them in Dioxus:
- Futures are lazy. They do not do anything until you call `.await` on them or spawn them with [`spawn`](./futures.md).
- Futures are concurrent, but not always parallel. In dioxus, all futures run on the main thread, so you should not block inside a future.
- Futures will pause at await points which means you shouldn't hold any locks across await points.

## Lazy futures

Unlike javascript, Rust futures are lazy. This means that they do not start executing until you call `.await` on them or run them in the background with [`spawn`](./futures.md).


This future will never log "Ran" because it is never awaited:
```rust
let future = async {
    println!("Ran");
};
```

To run this future, you can either await it or spawn it:
```rust
let future = async {
    println!("Ran");
};
let other_future = async {
    future.await;
    println!("Ran Other");
};
spawn(other_future);
```

## Concurrency vs Parallelism

Concurrency and parallelism are often confused, but the difference has important implications for how you write your applications. Multiple concurrent tasks can be started and not yet finished at the same time, but they do not necessarily run at the same time. In Rust, this is achieved through the use of futures and the async/await syntax.

![concurrent](/assets/07/concurrent.png)

On the other hand, multiple parallel can run at exactly the same time on different threads. In rust, you can spawn parallel tasks using the `std::thread` module or libraries like `rayon`.

![parallel](/assets/07/parallel.png)

Rust has multiple different async runtimes like `tokio` or `wasm-bindgen-futures`. Dioxus has its own async runtime built on top of a platform specific runtime for each renderer. On desktop it is compatible with tokio.


The dioxus runtime is single threaded which means futures can use `!Send` types, but they need to be careful to never block the thread.

```rust
spawn(async {
    // This will block the main thread and make the UI unresponsive.
    // Do not do this!
    solve_for_the_answer_to_life_and_everything();
    println!("Ran");
});
```

If you have an expensive task you need to run, you should spawn it on a separate thread using `std::thread::spawn` on desktop or use a [web worker](https://docs.rs/gloo-worker/latest/gloo_worker/) on the web. This will allow the main thread to continue running and keep the UI responsive.

```rust
std::thread::spawn(|| {
    // This will run on a separate thread and not block the main thread.
    solve_for_the_answer_to_life_and_everything();
    println!("Ran");
});
```

## Handling locks

Futures will pause execution at `.await` points, allowing other tasks to run until the future is ready to continue. You should avoid holding locks across `.await` points because another async task could try to use the lock while the future is paused. Instead, you need to ensure that locks are only held for the duration of the critical section and released before awaiting.

![async locks](/assets/07/lock_await.png)

This guide has covered the basics of asynchronous tasks in Dioxus. More detailed documentation about specific hooks are available in docs.rs:
- [use_resource](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_resource.html)
- [use_server_future](https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_server_future.html)
- [SuspenseBoundary](https://docs.rs/dioxus/latest/dioxus/prelude/fn.SuspenseBoundary.html)
- [spawn](https://docs.rs/dioxus/latest/dioxus/prelude/fn.spawn.html)
- [spawn_forever](https://docs.rs/dioxus/latest/dioxus/prelude/fn.spawn_forever.html)

More examples of futures and asynchronous tasks are available in the [example folder](https://github.com/DioxusLabs/dioxus/tree/main/examples) in the dioxus repo.
