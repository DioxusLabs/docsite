# Reactive Stores and Collections

The `Signal` object is great for making your application state reactive. So far, we've demonstrated how signals can make any type reactive simply by wrapping its initializer in a `use_signal` hook:

```rust
let mut count: Signal<i32> = use_signal(|| 0);
```

Whenever we call `.read()` or `.write()` on the signal, we can easily access and modify the underlying value and queue components to be re-rendered.

As your data structures evolve, you might run into new challenges. Some questions you might ask:

- "How do I add reactivity to types I don't own?"
- "What exactly re-renders when I modify just *one* entry in a collection?"
- "What if I can't wrap every value in my app in a signal?"

As our apps become more interesting, we're more likely to pull in third-party libraries and organize our state into collections like HashMaps, BTrees, and Vectors. In these cases, we need to interact with data structures *we don't own.*

To make working with structs and collections easier, Dioxus provides the **Store**.

## Reactive Stores

In Dioxus, reactive stores are types that isolate reactivity to just a single field or entry in a collection. Stores allow us to "zoom in" on a smaller portion of our data, ignoring all other reads and writes.

The simplest stores are structs that derive the `Store` trait:

```rust
#[derive(Store)]
struct HeaderState {
    title: String,
    subtitle: String
}
```

To use stores as state, we use the `use_store` hook:

```rust
let header = use_store(|| HeaderState {
    title: "Hello, ".to_string(),
    subtitle: "world!".to_string(),
});
```

The `Store` drive macro generates additional methods on `HeaderState` that allow us to "zoom in" to fields of the struct. We access the fields by calling the field name like a method:

```rust
fn app() -> Element {
    let header = use_store(|| HeaderState {
        title: "Hello, ".to_string(),
        subtitle: "world!".to_string(),
    });

    // we can "zoom in" with the generated `.title()` method.
    let title = header.title();

    rsx! { "{title}" }
}
```

The `.title()` method returns an object called a "lens". Lenses are not unique to Dioxus -  they are common throughout functional programming and web development.

### The `Store<Value, Lens>` type

When we use the `.title()` method, we're creating a new `Store` object, but with an additional generic parameter that specifies how the lens "zooms in" on our state. Lenses are zero-cost, meaning that the cost of using the `.title()` method is the same as indexing the struct directly with `.title`. One side-effect is that the generic type can't be explicitly "named" or specified:

```rust
// the second generic is the unique type of the lens
let title: Store<HeaderState, _> = header.title();
```

Notice how the default `Store` we get from `use_store` has an elided default generic argument:

```rust
let title: Store<HeaderState> = use_store(|| HeaderState::new());
```

Because the lens is "unnamable," we can't easily add it to structs or pass it as a function argument. In these cases, we can use the `boxed` and `boxed_mut` methods to convert the lens into a ReadSignal or WriteSignal at the cost of an allocation:

```rust
let title: ReadSignal<HeaderState> = header.title().boxed();
```

On the boundary of components, this is done automatically by "decaying" lenses into ReadSignals:

```rust
fn app() -> Element {
    let header = use_store(|| HeaderState {
        title: "Hello, ".to_string(),
        subtitle: "world!".to_string(),
    });

    rsx! {
        // the lens returned by `.title()` decays into a `ReadSignal` automatically!
        Title { title: header.title() }
    }
}

#[component]
fn Title(title: ReadSignal<String>) -> Element {
    // ..
}
```

### Stores are Readable and Writable

The `Store` type implements the same `Readable` and `Writable` traits that signals implement, allowing us to use stores and lenses just like signals:

```rust
// we can use `.read()`
let title = header.title().read();

// we can use `.write()`
*header.title().write() = "goodbye".to_string();

// we can use `.set()`
header.title().set("goodbye!".to_string());
```

Stores and lenses also implement the same `Readable` and `Writable` extension traits, allowing us to work with them like regular Rust values:

```rust
fn app() -> Element {
    let mut header = use_store(|| HeaderState {
        title: "Hello, ".to_string(),
        subtitle: "world!".to_string(),
    });

    rsx! {
        // Stores implement display
        h1 { "{header.title()}" }

        // They are `Copy`, making them easy to share between scopes
        button { onclick: move |_| header.title().write().clear() }
    }
}
```

### Nested Stores

Stores can also be nested, allowing lenses to "see through" multiple levels of datastructures:

```rust
#[derive(Store)]
struct HeaderState {
    title: String,
    subtitle: String,
    other: OtherHeaderState,
}

#[derive(Store)]
struct OtherHeaderState {
    title2: String,
    subtitle2: String,
}

// now we can "zoom in" to nested fields:
let title2 = header.other().title2();
```

The ability for a store to "zoom in" through nested datastructures is dependent on whether or not types also implement `Store`. If, for example, or nested structs *didn't* implement the `Store` trait, then we can't lens them:

```rust
#[derive(Store)]
struct HeaderState {
    title: String,
    subtitle: String,
    other: OtherHeaderState,
}

struct OtherHeaderState {
    title2: String,
    subtitle2: String,
}

// ❌ we *can't* lens the OtherHeaderState
let title2 = header.other().title2();
```

It's important to know that stores can't lens through types that don't implement `Store`. When working with 3rd-party APIs and datastructures, foreign structs typically don't implement the `Store` trait, meaning we can't make them entirely reactive.

### Result, Option, and enumerated Lenses

Some fields of our store may be an enumerated value, like a [Result](https://doc.rust-lang.org/std/result/) or an [Option](https://doc.rust-lang.org/std/option/). Stores provide a few helpful extension methods that make working with the enum easier and make the enum itself reactive.

We can update our header example to include both `Result` and `Option` types:

```rust
#[derive(Store)]
struct HeaderState {
    title: Result<String, String>,
    subtitle: Option<String>,
}
```

Just like regular `Result` and `Option`, store lenses also implement `.unwrap()` (and a number of utility methods).

```rust
let title = header.title().unwrap();
let subtitle = header.subtitle().unwrap();
```

To properly gain access to the underyling `Option<String>` we can use one of two approaches:

- The `.transpose()` method which converts `Store<Option<String>>` to `Option<Store<String>>`
- Using `.read().as_ref()`

The `.transpose()` approach is useful since we keep the lens, allowing us to "zoom in" to nested structs and collections.

```rust
let len = match header.subtitle().transpose() {
    Some(subtitle) => subtitle.read().len(),
    None => 0,
};
```

Alternatively, we can use `.ref()` the lens to gain access to the underlying value, but we lose the ability to reactively "zoom in" further:

```rust
let len = match header.subtitle().as_ref() {
    Some(subtitle) => subtitle.chars().count(),
    None => 0,
};
```

You can usually choose either approach - just know that using `.ref()` calls `.read()` internally, and the "reactivity zoom" might not be perfectly precise.

## Reactive Collections

You might be wondering: "what's the best way to handle HashMaps and Vectors in signals?" So far, we've only showcased simple state in signals, like strings and integers. Just like strings and integers, you can place collections directly into signals:

```rust
type UserId = i32;
struct UserData {
    name: String,
    email: String,
}

let mut users = use_signal(|| HashMap::<UserId, UserData>::new());

rsx! {
    ul {
        for (id, data) in users.read().iter() {
            li { key: "{id}", "{data.name}" }
        }
    }
}
```

For small apps, this is usually fine. However, we might eventually want to move the `li {}` element into its own component. Unfortunately, we'll immediately run into lifetime issues with the `GenerationalRef` guard returned by `.iter()`:

```rust
// ❌ This won't compile!
fn app() -> Element {
    let mut users = use_signal(|| HashMap::<UserId, UserData>::new());

    rsx! {
        // the lifetime here is is not `'static` won't pass to the ListItem component
        for (id, user) in users.read().iter() {
            ListItem { key: "{id}", user }
        }
    }
}

// `UserData` cannot be borrowed here
#[component]
fn ListItem(user: ReadSignal<UserData>) -> Element {
    rsx! {
        li { "{user.read().name}" }
    }
}
```

In these scenarios, we have a few options:

- Deeply clone the `UserData` object, detaching its lifetime from the `for` loop
- Clone individual fields of the `UserData` object
- Use Stores and pass a Lens

The first two options require us to perform potentially expensive clones when rendering our component. Instead, we can use stores and lenses to find a zero-cost solution, simply by converting our `use_signal` to a `use_store` call:

```rust
fn app() -> Element {
    // switch to using `use_store`
    let mut users = use_store(|| HashMap::<UserId, UserData>::new());

    rsx! {
        for (id, user) in users.read() {
            ListItem { key: "{id}", user }
        }
    }
}

// And now we can accept a `ReadSignal<UserData>` as a prop
#[component]
fn ListItem(user: ReadSignal<UserData>) -> Element {
    rsx! {
        li { "{user.read().name}" }
    }
}
```

The `Store<HashMap<K, V>>` type is a special type that implements reactivity on a per-entry basis. When we insert or remove values from the `users` store, only *one* re-render is queued. If we edit an individual entry in the HashMap, only a single `ListItem` will re-render.

Alternatively, we could pass the entire `Store` to the ListItem, along with the `UserId` key, allowing us to further lens into specific fields of our UserData entries:

```rust
fn app() -> Element {
    // switch to using `use_store`
    let mut users = use_store(|| HashMap::<UserId, UserData>::new());

    rsx! {
        for (id, user) in users.read() {
            ListItem { key: "{id}", users, id }
        }
    }
}

#[component]
fn ListItem(users: Store<HashMap<UserId, UserData>>, id: UserId) -> Element {
    rsx! {
        li { "{users.get(id)).read()}" }
    }
}
```

## Making the Most of Stores

Hopefully, the examples here highlight when stores might be useful in your applications. It might be tempting to use stores *everywhere*, but in many cases, signals and structs are good enough. Most apps won't run into performance issues where precise control over the reactivity of struct fields makes a huge difference.

Stores are most useful when interfacing with foreign data types and optimizing huge collections - not something you need to do every day!

