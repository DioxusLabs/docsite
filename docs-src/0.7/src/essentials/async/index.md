# Handling Asynchronous Tasks

Asynchronous tasks are a core part of any modern application. They let you define code waits for network requests, timers, or OS events without blocking the thread. Dioxus provides three core apis for handling asynchronous tasks:
- [`spawn`](./futures.md) starts an asynchronous task in the background.
- [`use_resource`](./resources.md) derives async state from a future.
- [`SuspenseBoundary`](./suspense.md) provides a unified way to handle loading states for async tasks.

Each of these APIs rely on some fundamental knowledge of async in Rust. If you're not familiar with async programming in Rust, we recommend checking out the [async programming chapter](https://doc.rust-lang.org/book/ch17-00-async-await.html) in the Rust book. You can also read the [async crash course](./crash_course.md) in this section for a quick overview of the concepts.
