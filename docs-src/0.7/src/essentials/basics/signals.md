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

In some cases, wrapping your data in Signals can make accessing the inner state awkward. Mutable Signals implement two fundamental traits: `Readable` and `Writeable`. These traits provide a number of automatic ergonomic improvements.

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

You'll generally want to use the extension methods unless the inner state does *not* implement the required traits. There are several methods available not listed here, so peruse the docs reference for more information.

## Reactive Scopes

Whenever `.write()` or `.set()` is called on a Signal, any active *reactive scopes* tracking that run a callback as a side-efect.

The simplest reactive scope is a component. During a component render, components automatically subscribe to signals where `.read()` is called. The `.read()` method can be called *implicitly* in many circumstances - notably, the extension methods provided by `Readable` use the underlying `.read()` method and thus *also* contribute to the current reactive scope. When a signal's value changes, components queue a side-effect to re-render the component using `dioxus::core::needs_update`.

```rust
let mut name = use_signal(|| "abc".to_string());

rsx! {
    // An obvious call to `.read()`
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

There are other uses of reactive scopes beyond component re-renders. Hooks like `use_effect`, `use_memo`, and `use_resource` all implement functionality by leveraging a reactive scope that exists *outside* the rendering lifecycle.

## Automatic Batching

By default, all `.write()` calls are batched in one "step" of your app. Dioxus does not synchronously run side-effects when you call `.write()`. Instead, it waits for all events to be handled first before determining what side-effects need to be run. This provides automatic batching of `.write()` calls which is important both for performance and consistency in the UI.

For example, by batching `.write()` calls, we ensure that our UI always display one of two states:
- "loading?: false -> Complete"
- "loading?: true -> Loading"

```rust
let mut loading = use_signal(|| false);
let mut text = use_signal(|| "Complete!");

rsx! {
    button {
        onclick: move |_| async move {
            text.set("Loading");
            loading.set(true);

            do_async_work().await

            text.set("Complete!");
            loading.set(false);
        },
        "loading?: {loading:?} -> {text}"
    }
}
```

Dioxus uses `await` boundaries as barriers between steps. If state is modified during a step, Dioxus prefers to paint the new UI first before polling additional futures. This ensures changes are flushed as fast as possible and pending states aren't missed.

## Opting Out of Subscriptions

In some situations, you may need to read a reactive value without subscribing to it. You can use the `peek` method to get a reference to the inner value without registering the value as a dependency of the current reactive context:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:peek}}
```

```inject-dioxus
DemoFrame {
    reactivity::PeekDemo {}
}
```

## Working with Untracked State

Most of the state in your app will be tracked values. All built in hooks return tracked values, and we encourage custom hooks to do the same. However, there are times when you need to work with untracked state. For example, you may receive a raw untracked value in props. When you read an untracked value inside a reactive context, it will not subscribe to the value:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:non_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::NonReactiveDemo {}
}
```

You can start tracking raw state with the `use_reactive` hook. This hook takes a tuple of dependencies and returns a reactive closure. When the closure is called in a reactive context, it will track subscribe to the dependencies and rerun the closure when the dependencies change.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:use_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::UseReactiveDemo {}
}
```

## Making Props Reactive

To avoid losing reactivity with props, we recommend you wrap any props you want to track in a `ReadOnlySignal`. Dioxus will automatically convert `T` into `ReadOnlySignal<T>` when you pass props to the component. This will ensure your props are tracked and rerun any state you derive in the component:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/reactivity.rs:making_props_reactive}}
```

```inject-dioxus
DemoFrame {
    reactivity::MakingPropsReactiveDemo {}
}
```

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

With the right clippy lints, this code fails linting because the `writer` type should not be held across an await point.

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

This is done either by creating a new, smaller scope to access the `.read()` guard -
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

While this might seem scary or error prone, you will *very rarely* run into these issues when building apps. The `.read()` and `.write()` guards respect Rust's ownership rules within a given scope and concurrent scopes are protected by the [Clippy `await_holding_refcell_ref` lint](https://rust-lang.github.io/rust-clippy/master/index.html#await_holding_refcell_ref).


## Signals implement `Copy`

If you've used Rust to build other projects - like a webserver or a command line tool - you might have encountered situations with closures, threads, and async tasks that required an `Arc` or `Rc` to satisfy the borrow checker.

![Concurrent Access](/assets/07/concurrent-arc.png)

If our data is used in parallel across several threads, or even just held in a callback for future use, we might need to wrap it in an `Arc` or `Rc` smart pointer and `.clone()` it. This can lead to cumbersome code where we constantly need to call `.clone()` to share data into callbacks and async tasks.

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

To solve this, we built the [generational-box crate](https://crates.io/crates/generational-box) that provides a [`GenerationalBox`] type that implements [Rust's `Copy` trait](https://doc.rust-lang.org/std/marker/trait.Copy.html). The `Copy` trait is very important: Rust automatically copies `Copy` types (when needed) on boundaries of scopes.

```rust
let state = GenerationalBox::new(123);

// the `move` keyword automatically copies the GenerationalBox!
std::thread::spawn(move |_| println!("{state:?}"));

// we can easily share across threads with no `.clone()` noise
std::thread::spawn(move |_| println!("{state:?}"));
```

Instead of copying the underlying value, the `GenerationalBox` simply copies a *handle* to the value. This handle is essentially a smart pointer verified at runtime. Accessing the contents of a signal is not as efficient as reading a pointer directly - there is an extra pointer indirection and lock check - but we don't expect most code to be bottlenecked by reading `GenerationalBox` contents.

Dioxus Signals are built directly on top of `GenerationalBox`. They share the same `Copy` semantics and ergonomics, but with the same tradeoffs.

## Signals are Disposed

Signals implementing `Copy` is a huge win for ergonomics. However, there is a tradeoff. The `GenerationalBox` type does not have automatic [RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html) support. This means when a `GenerationalBox` is dropped, its resources are **not immediately cleaned up**. It can be tricky to correctly use `GenerationalBox` directly. Dioxus handles the *resource* lifecycle cleaning up resources using the *component* lifecycle.

The Signal type is built on `GenerationalBox`. Whenever you call `use_signal`, we automatically:

- Call `Signal::new()`
- Register `signal.dispose()` on the component's `on_drop`

Whenever a component is unmounted, its hooks are *dropped*. When you create Signals in a component, each Signal is registered with a Signal "owner" on that component. When the component is unmounted, the owner drops, and in its Drop implementation, it calls `.dispose()` on all Signals that were created in its scope.

Effectively, we connected the `.dispose()` method of the Signals to the unmount of the component.

Because the Signal is disposed when the component unmounts, reading it will cause a runtime panic. This very rarely happens in practice, but *is* possible if you "save" the signal in a component higher up the tree. Doing so would violate the one-way-data flow pillar of reactivity, but is technically possible.

![Concurrent Access](/assets/07/use-after-free.png)

Reading a Signal after it's been disposed is similar to the "use-after-free" bug with pointers, but reading a Signal *is not* undefined behavior. In debug mode, the Signal will be hoisted to its reader and you'll receive a warning in the logs that a Signal is being read after it's been disposed.

A similar issue can arise when you call `Signal::new()` directly. Dioxus creates an implicit Signal owner that is owned by the current component. The contents of this Signal will only be dropped when the current component is unmounted. Creating Signals manually in a loop can lead to unbounded memory usage until the component is dropped. It's rare to do this in normal application code but can crop up in library development.

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

