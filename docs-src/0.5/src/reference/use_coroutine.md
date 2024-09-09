# Coroutines

Another tool in your async toolbox are coroutines. Coroutines are futures that can have values sent to them.

Like regular futures, code in a coroutine will run until the next `await` point before yielding. This low-level control over asynchronous tasks is quite powerful, allowing for infinitely looping tasks like WebSocket polling, background timers, and other periodic actions.

## `use_coroutine`

The `use_coroutine` hook allows you to create a coroutine. Most coroutines we write will be polling loops using await.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:component}}
```

For many services, a simple async loop will handle the majority of use cases.

## Yielding Values

To yield values from a coroutine, simply bring in a `Signal` handle and set the value whenever your coroutine completes its work.

The future must be `'static` – so any values captured by the task cannot carry any references to `cx`, such as a `Signal`.

You can use [to_owned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned) to create a clone of the hook handle which can be moved into the async closure.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:to_owned}}
```

To make this a bit less verbose, Dioxus exports the `to_owned!` macro which will create a binding as shown above, which can be quite helpful when dealing with many values.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:to_owned_continued}}
```

## Sending Values

You might've noticed the `use_coroutine` closure takes an argument called `rx`. What is that? Well, a common pattern in complex apps is to handle a bunch of async code at once. With libraries like Redux Toolkit, managing multiple promises at once can be challenging and a common source of bugs.

With Coroutines, we can centralize our async logic. The `rx` parameter is an Channel that allows code external to the coroutine to send data _into_ the coroutine. Instead of looping on an external service, we can loop on the channel itself, processing messages from within our app without needing to spawn a new future. To send data into the coroutine, we would call "send" on the handle.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:send}}
```

> Note: In order to use/run the `rx.next().await` statement you will need to extend the [`Stream`] trait (used by [`UnboundedReceiver`]) by adding 'futures_util' as a dependency to your project and adding the `use futures_util::stream::StreamExt;`.

For sufficiently complex apps, we could build a bunch of different useful "services" that loop on channels to update the app.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:services}}
```

We can combine coroutines with Global State to emulate Redux Toolkit's Thunk system with much less headache. This lets us store all of our app's state _within_ a task and then simply update the "view" values stored in Atoms. It cannot be understated how powerful this technique is: we get all the perks of native Rust tasks with the optimizations and ergonomics of global state. This means your _actual_ state does not need to be tied up in a system like `Signal::global` or Redux – the only Atoms that need to exist are those that are used to drive the display/UI.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:global}}
```

Now, in our sync service, we can structure our state however we want. We only need to update the view values when ready.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:global_continued}}
```

## Automatic injection into the Context API

Coroutine handles are automatically injected through the context API. You can use the `use_coroutine_handle` hook with the message type as a generic to fetch a handle.

```rust, no_run
{{#include src/doc_examples/use_coroutine_reference.rs:injection}}
```
