# Reactive Signals

In Dioxus, your app's UI is defined as a function of its current state. As the state changes, the components and effects that depend on that state will automatically re-run. Reactivity automatically *tracks* state and *derives* new state, making it easy to build large applications that are efficient and simple to reason about.

Dioxus provides a single source of mutable state: the **Signal**.

## State with Signals

In Dioxus, mutable state is stored in Signals. Signals are *tracked* values that automatically update *reactive contexts* that watch them. They are the source of state from which all other state is derived from. Signals are modified directly by event handlers in response to user input or asynchronously in futures.

You can create a signal with the `use_signal` hook:

```rust
let mut signal = use_signal(|| 0);
```

Once you have your signal, you can gain an inner reference to the signal value by calling the `.read()`:

```rust
let mut signal = use_signal(|| 0);

// use `.read()` to access the inner value
let inner = signal.read();
```

For Signals whose inner can be cheaply cloneable, you can also use "function" syntax to get a direct `Clone` of the value.

```rust
let name = use_signal(|| "Bob".to_string());

// Call the signal like a function
let inner = name();

// Or use `.cloned()`
let inner = name.cloned();
```

Finally, you can set the value of the signal with the `.set()` method or get a mutable reference to the inner value with the `.write()` method:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:signal_write}}
```

A simple component that uses `.read()` and `.write()` to update its own state with signals may look like:

```rust
fn Demo() -> Element {
    let mut count = use_signal(|| 0);

    // read the current value
    let current = count.read().clone();

    rsx! {
        button {
            onclick: move |_| *count.write() = current,
            "Increment ({current})"
        }
    }
}
```

When assigning values to a `.write()` call, note that we use the [*dereference operator*](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) which let's us write a value directly into the mutable reference.

## Ergonomic Methods on Signals

In some cases, wrapping your data in Signals can make accessing the inner state awkward. Mutable Signals implement two fundamental traits: `Readable` and `Writable`. These traits provide a number of automatic ergonomic improvements.

- `Signal<T>` implements `Display` if `T` implements `Display`
- `Signal<bool>` implements `fn toggle()`
- `Signal<i32>` and other numbers implement math operators (+, -, /, etc)
- `Signal<T>` where `T` implements `IntoIterator` implements `.iter()`
- and many more!

The `Display` extension enables using signals in formatting expressions:

```rust
let mut count = use_signal(|| 0);
rsx! { "Count is: {count}" }
```

The toggle extension makes toggling boolean values simpler:
```rust
let mut enabled = use_signal(|| true);
rsx! {
    button {
        onclick: move |_| enabled.toggle(),
        if enabled() { "disable" } else { "enable" }
    }
}
```

Math operators simplify arithmetic operations:

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    }
}
```

The iterator extension makes iterating through collections easier:

```rust
fn app() -> Element {
    let names = use_signal(|| vec!["bob", "bill", "jane", "doe"]);

    rsx! {
        ul {
            for name in names.iter() {
                li { "hello {name}" }
            }
        }
    }
}
```

You'll generally want to use the extension methods unless the inner state does *not* implement the required traits. There are several methods available not listed here, so peruse the docs reference for more information.

## ReadSignal and WriteSignal

Dioxus provides two variations of the base Signal type: `ReadSignal` and `WriteSignal`.

- **`ReadSignal`**: a read-only version of the base Signal type
- **`WriteSignal`**: a read-write version of the base Signal type, equivalent to `Signal` itself

ReadSignals are reactive values that are implement the `Readable` trait. `WriteSignals` are reactive values that implement the `Writable` trait.

These two variations are useful when writing functions that need to be generic over their input types. If a function only needs the `.read()` method and its extensions, then it can specify a `ReadSignal` as an argument.


```rust
fn app() -> Element {
    let mut name: Signal<String> = use_signal(|| "abc".to_string());

    // `.into()` converts the Signal into a ReadSignal
    do_read_only_things(name.into());

    // ...
}

// we can accept anything that implements `Into<ReadSignal>`
fn do_read_only_things(sig: ReadSignal<String>) {
    println!("{}", sig.read());
}
```

In Dioxus, `Signal` is not the only reactive type. The entire ecosystem is full of custom reactive types. Dioxus itself also provides additional reactive types like `Memo` and `Resource`. To integrate well with the broader ecosystem, it's best to prefer using `ReadSignal` and `WriteSignal` in your interfaces rather than specific reactive types.

This ensures we can pass both `Signal` and `Memo` to the same function:

```rust
let name: Signal<String> = use_signal(|| "abc".to_string());
let memo: Memo<String> = use_memo(move || name.to_string());

// `.into()` converts the Signal into a ReadSignal
do_read_only_things(name.into());

// `.into()` converts the Memo into a ReadSignal
do_read_only_things(name.into());
```

## Reactive Scopes

A **Reactive Scope** is a block of Rust code that observes reads and writes of reactive values. Whenever `.write()` or `.set()` is called on a Signal, any active reactive scopes tracking that Signal run a callback as a side-efect.

The simplest reactive scope is a component. During a component render, components automatically subscribe to signals where `.read()` is called. The `.read()` method can be called *implicitly* in many circumstances - notably, the extension methods provided by `Readable` use the underlying `.read()` method and thus *also* contribute to the current reactive scope. When a signal's value changes, components queue a side-effect to re-render the component using `dioxus::core::needs_update`.

```rust
let mut name = use_signal(|| "abc".to_string());

rsx! {
    // An explicit call to `.read()`
    {name.read().to_string()}

    // An implicit call via `Display`
    "{name}"
}
```

If a component does not call `.read()` on a Signal while rendering, it does not subscribe to that signal's value. This provides us "zero cost reactivity" where we can freely modify signal values without worrying about unnecessary re-renders. If a value is not observed, it won't cause unnecessary re-renders.

```rust
let mut loading = use_signal(|| false);

rsx! {
    button {
        // Because we don't use "loading" in our markup, the component won't re-render!
        onclick: move |_| async move {
            if loading() {
                return;
            }
            loading.set(true);
            // .. do async work
            loading.set(false);
        }
    }
}
```

Calls to `.read()` access the current reactive scope, adding this scope to the list of subscribers to the Signal. By default, components attach a re-render effect to all signals accessed from witin their reactive scope.

![Component effects](/assets/07/component-effect.png)

There are other uses of reactive scopes beyond component re-renders. Hooks like `use_effect`, `use_memo`, and `use_resource` all implement functionality by leveraging a reactive scope that exists *outside* the rendering lifecycle.

## Automatic Batching

By default, all `.write()` calls are batched in one "step" of your app. Dioxus does not synchronously run side-effects when you call `.write()`. Instead, the runtime waits for all events to be handled *first* before determining what side-effects need to be run. This provides automatic batching of `.write()` calls which is important both for performance and consistency in the UI.

By batching `.write()` calls, Dioxus ensures that our example UI always displays one of two states:
- **"loading?: false -> Complete"**
- **"loading?: true -> Loading"**

```rust
let mut loading = use_signal(|| false);
let mut text = use_signal(|| "Complete!");

rsx! {
    button {
        onclick: move |_| async move {
            // these writes are batched and side-effects are de-duplicated
            text.set("Loading");
            loading.set(true);

            // awaiting a future allows the runtime to continue
            do_async_work().await

            // these writes are also batched - only one re-render is queued
            text.set("Complete!");
            loading.set(false);
        },
        "loading?: {loading:?} -> {text}"
    }
}
```

Dioxus uses `await` boundaries as barriers between steps. If state is modified during a step, Dioxus prefers to paint the new UI first before polling additional futures. This ensures changes are flushed as fast as possible and pending states aren't missed.

## Signals are Borrowed at Runtime

In Rust, the `&T` and `&mut T` reference types statically assert that the underlying value is either immutable or mutable *at compile time*. This assertion brings a number of guarantees, enabling Rust to generate fast and correct code.

Unfortunately, these static assertions do not mix well with asynchronous background tasks. If our `onclick` handler spawns a long-running Future that captures an `&mut T`, we can not safely handle any *other* events until that Future completes:

![Mutability Over Time](/assets/07/mutable-diagram.png)

At times, our UIs can be very concurrent. There *are* ways to re-orient how we concurrently access state that are compatible with Rust's static mutablity assertions - unfortunately, they are not easy to program.

Instead, Signals provide a `.write()` method that checks *at runtime* if the value is safe to access. If you're not careful, you can combine a `.read()` and a `.write()` in the same scope, leading to a runtime borrow failure (panic).

This is most frequently encountered when holding `.read()` or `.write()` refs across await points:

```rust
let mut state = use_signal(|| 0);

rsx! {
    button {
        // Clicking this buttom quickly will cause multiple `.write()` calls to be active
        onclick: move |_| async move {
            let mut writer = state.write();
            sleep(Duration::from_millis(1000)).await;
            *writer = 10;
        }
    }
}
```

Fortunately, this code fails `cargo clippy` because the `writer` type should not be held across an await point.

Thankfully, Signals guard against the "trivial" case because the `.write()` method takes an `&mut Signal`. While the `.write()` guard is active in a scope (block), no other `.read()` or `.write()` guards can be held:

```rust, no_run
let mut state = use_signal(|| 0);

rsx! {
    button {
        // rust prevents this code from compiling since `.write()` takes `&mut T`
        onclick: move |_| {
            let cur = state.read();
            *state.write() = *cur + 1;
        }
    }
}
```

We get a very nice error from the Rust compiler explaining why this code does not compile:

```text
error[E0502]: cannot borrow `state` as mutable because it is also borrowed as immutable
  --> examples/readme.rs:22:18
   |
21 |                 let cur = state.read();
   |                           ----- immutable borrow occurs here
22 |                 *state.write() = *cur + 1;
   |                  ^^^^^^^^^^^^^ mutable borrow occurs here
23 |             }
   |             - immutable borrow might be used here, when `cur` is dropped and runs the destructor for type `GenerationalRef<Ref<'_, i32>>`
```

If we *do* want to read and write in the same scope, we need to stage our operations in the correct order such that the `.read()` and `.write()` guards do not overlap. Usually, this is done by deriving an owned value from the `.read()` operation to be used in the `.write()` operation.

```rust
let cur = state.read().clone(); // calling `.clone()` releases the `.read()` guard immediately.
*state.write() = *cur + 1;
```

Note that Rust automatically drops items *at the end* of a scope, unless they are manually dropped sooner. We can use the `.read()` guard provided it's dropped before `.write()` is called.

This is done either by creating a new, shorter scope to access the `.read()` guard -
```rust
// The .read() guard is only alive for a shorter scope
let next = {
    let cur = state.read();
    println!("{cur}");
    cur.clone() + 1
};

// we can assign `state` to `next` since `next` is not referencing `.read()`.
*state.write() = next;
```

or, simply by calling `drop()` on the guard
```rust
let cur1 = state.read();
let cur2 = *cur1 + 1;
drop(cur1); // dropping early asserts we can `.write()` the signal safely
*state.write() = cur2 + 1;
```

In very advanced usecases, you can make a copy of the signal or use the `read_unchecked` method to relax the borrowing rules:

```rust
match result.read_unchecked().as_ref() {
    Ok(resp) => rsx! { "success! {resp}" }
    Err(err) => rsx! { "err: {err:?}" },
}
```

> Rust 2021 had issues with `.read()` in match statements, whereas Rust 2024 fixes this issue, meaning you no longer need to use `read_unchecked`

While this might seem scary or error prone, you will *very rarely* run into these issues when building apps. The `.read()` and `.write()` guards respect Rust's ownership rules within a given scope and concurrent scopes are protected by the [Clippy `await_holding_refcell_ref` lint](https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref).


## Signals implement `Copy`

If you've used Rust to build other projects - like a webserver or a command line tool - you might have encountered situations with closures, threads, and async tasks that required an `Arc` or `Rc` to satisfy the borrow checker.

![Concurrent Access](/assets/07/concurrent-arc.png)

If our data is used across several parallel threads, or even just held in a callback, we might need to wrap it in an `Arc` or `Rc` smart pointer and `.clone()` it. This can lead to cumbersome code where we constantly call `.clone()` to share data into callbacks and async tasks.

```rust
let state = Arc::new(123);

// thread 1
std::thread::spawn({
    let state = state.clone();
    move |_| println!("{state:?}"),
})

// thread 2
std::thread::spawn({
    let state = state.clone();
    move |_| println!("{state:?}"),
})
```

Unfortunately, UI code constantly encounters this problem - **this is why Rust does not have a great reputation for building GUI apps**!

To solve this, we built the [generational-box crate](https://crates.io/crates/generational-box) that provides a `GenerationalBox` type that implements [Rust's `Copy` trait](https://doc.rust-lang.org/std/marker/trait.Copy.html). The `Copy` trait is very important: Rust automatically copies `Copy` types (when needed) on boundaries of scopes.

```rust
let state = GenerationalBox::new(123);

// the `move` keyword automatically copies the GenerationalBox!
std::thread::spawn(move |_| println!("{state:?}"));

// we can easily share across threads with no `.clone()` noise
std::thread::spawn(move |_| println!("{state:?}"));
```

Instead of copying the underlying value, the `GenerationalBox` simply copies a *handle* to the value. This handle is essentially a runtime-verified smart pointer. Accessing the contents of a signal is not as efficient as reading a pointer directly - there is an extra pointer indirection and lock - but we expect most code to not be bottlenecked by reading the contents of a `GenerationalBox`.

Dioxus Signals are built directly on top of `GenerationalBox`. They share the same `Copy` semantics and ergonomics, but with the same tradeoffs.

## Signals are Disposed

Signals implementing `Copy` is a huge win for ergonomics. However, there is a tradeoff. The `GenerationalBox` type does not have automatic [RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html) support. This means when a `GenerationalBox` is dropped, its resources are **not immediately cleaned up**. It can be tricky to correctly use `GenerationalBox` directly. Dioxus manages the *resource* lifecycle by cleaning up resources using the *component* lifecycle.

The Signal type is built on `GenerationalBox`. Whenever you call `use_signal`, we automatically:

- Call `Signal::new()`
- Register `signal.dispose()` on the component's `on_drop`

Whenever a component is unmounted, its hooks are dropped. When you create Signals in a component, each Signal is registered with a Signal "owner" on that component. When the component is unmounted, the owner drops, and in its `Drop` implementation, it calls `.dispose()` on all Signals that were created in its scope.

Effectively, we connected the `.dispose()` method of the Signals to the unmount of the component.

Because the Signal is disposed when the component unmounts, reading it after will cause a runtime panic. This very rarely happens in practice, but *is* possible if you "save" the signal in a component higher up the tree. Doing so would violate the one-way-data flow pillar of reactivity, but is technically possible.

![Concurrent Access](/assets/07/use-after-free.png)

Reading a Signal after it's been disposed is similar to the "use-after-free" bug with pointers, but reading a Signal *is not* undefined behavior. In debug mode, the Signal will be hoisted to its reader and you'll receive a warning in the logs that a Signal is being read after it's been disposed.

A similar issue can arise when you call `Signal::new()` directly. Dioxus creates an implicit Signal owner that is owned by the current component. The contents of this Signal will only be dropped when the current component is unmounted. Calling `Signal::new()` can lead to unbounded memory usage until the component is dropped. It's rare to do this in normal application code but can crop up in library development.

```rust
let mut users = use_signal(|| vec![]);

rsx! {
    button {
        // the underlying strings won't be dropped until the component is unmounted, or you call `.dispose()` manually
        onclick: move |_| {
            users.write().push(Signal::new("bob".to_string()));
        },
        "Add a new user"
    }
}
```

When mapping Signals or creating them on-the-fly, it's best to prefer the built-in methods and reactive collections.


## Effects, Memos, and More

Signals are just one piece of the Dioxus reactivity system. Hooks like `use_effect` and `use_memo` are able to isolate their reactive scopes to just callbacks and futures. This means `.read()` and `.write()` in these scopes won't queue re-render side-effects in their containing component.

We cover these hooks in more depth in a [later chapter](./effects.md).
