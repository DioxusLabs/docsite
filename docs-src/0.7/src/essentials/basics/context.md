# Global Context

By now, you have the requisite knowledge to build large Dioxus apps! When your component tree grows to several layers deep, passing state through component props can be become tedious and repetitive.

```rust
fn app() -> Element {
    let name = use_signal(|| "bob".to_string());
    rsx! { Container { name } }
}

#[name]
fn Container(name: Signal<String>) -> Element {
    rsx! { Content { name } }
}

#[name]
fn Content(name: Signal<String>) -> Element {
    rsx! { Title { name } }
}

#[name]
fn Title(name: Signal<String>) -> Element {
    rsx! { h1 { "{name}" } }
}
```

Passing state through several layers of component properties is called "prop drilling." Wouldn't it be great if we could pass the `name` signal from the app root *directly* to the Title component?

This is where *global context* becomes useful. Components can insert values into the global context, allowing and any child components to "reach up" and read those values.

![Context Tree](/assets/07/context-tree.png)

Note that context is only available by "reaching up" the tree. Recall a fundamental pillar of reactivity: data flows down. In the one-way data-flow model, child components can freely read the state of their parent components. You can think of context as an invisible extra argument passed through the properties of each component from the root to every child.

```rust
fn Child(context: Context, ..props) -> Element { /* */ }
```

Context is roughly defined as a recursive definition of itself:

```rust
struct Context {
    contexts: Vec<Rc<dyn Any>>
    parent: Option<Context>,
}
```

When we "reach up" through the tree, we first walk the current component's context list, and then check each parent recursively until the target context is found.

## Providing and Consuming Context

Before we can start reaching up the component tree, we first need to *provide* context to child components. Dioxus exposes the `provide_context` and `use_context_provider` functions. You typically place your state initializer in the `use_context_provider` hook like so:

```rust
fn app() -> Element {
    // use_context_provider takes an initializer closure
    use_context_provider(|| "Hello world!".to_string());

    rsx! { Child {} }
}
```

Now, to read the context, we pair the provider with a consumer. To do so, we call `use_context` and set the return type to be the same type as the context we provided. In the example above, we provided a `String` type object; therefore, we consume it with the `String` generic:

```rust
fn Child() -> Element {
    let title = use_context::<String>();
    rsx! { "{title}" }
}
```

The `::<String>` syntax declares the return type of this use of `use_context`. In Dioxus, context objects are indexed by their [TypeId](https://doc.rust-lang.org/std/any/struct.TypeId.html). A type's TypeId is a  compile-time hash that uniquely identifies every Rust type. No two types will share the same TypeId unless the refer to the same underlying type declaration - ie, [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html) will share the same TypeId.

When providing context objects, we should wrap the data we want to store in a custom type or [new type](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). This ensures that we can store multiple `String` objects in the context tree and retrieve them by their wrapper type.

Declaring a new type requires a new struct declaration:

```rust
#[derive(Clone)]
struct Title(String);

#[derive(Clone)]
struct Subtitle(String);
```

The, we can provide the context using our wrapper types:

```rust
use_context_provider(|| Title("Hello world!".to_string()));
use_context_provider(|| Subtitle("Hello world!".to_string()));
```

To consume the context, we pass in the appropriate struct type:
```rust
let title = use_context::<Title>();
let title = use_context::<Subtitle>();
```

In practice, you don't need to wrap *every* field of your state in a newtype, usually just one struct to encapsulate a set of data is enough.

```rust
#[derive(Clone)]
struct HeaderContext {
    title: String,
    subtitle: String
}
```

## Context Provider Components

Sometimes you don't want context to apply to *every* component in your tree - just a subset is fine. If you want to provide state to a scoped portion of the elements in your `rsx!` macro, you can create a new component that provides context to its children called a **Context Provider**.

For example, we may write a `ThemeProvider` component that wraps its children and provides context:

```rust
#[component]
fn ThemeProvider(children: Element, color: ThemeColor) -> Element {
    use_context_provider(|| ThemeState::new(color));
    children
}
```

Then, we can use the context provider in our RSX multiple times, but with different parameters:

```rust
fn app() -> Element {
    rsx! {
        ThemeProvider {
            color: ThemeColor::Red,
            h1 { "Red theme!" }
        }
        ThemeProvider {
            color: ThemeColor::Blue,
            h1 { "Blue theme!" }
        }
    }
}
```

## Dynamically Providing and Consuming Context

Dioxus provides several different functions to provide and consume context. You do not necessarily need to use the `use_context` and `use_context_provider` hooks. Instead, you can dynamically provide and consume context *at runtime* with `provide_context` and `consume_context`.

Being able to dynamically consume context is powerful. We can directly access context in event handlers and async tasks without an additional hook.

```rust
fn ToggleTheme() -> Element {
    // no hooks required!
    rsx! {
        button {
            onclick: move |_| consume_context::<ThemeProvider>().set_theme(ThemeColor::Red),
            "Make Red"
        }
        button {
            onclick: move |_| consume_context::<ThemeProvider>().set_theme(ThemeColor::Blue),
            "Make Blue"
        }
    }
}
```

Because the Dioxus runtime always sets the theme context when running handlers and polling futures, this even works in async tasks:

```rust
rsx! {
    button {
        onclick: move |_| async move {
            let color = fetch_random_color().await;
            consume_context::<ThemeProvider>().set_theme(color);
        },
        "Make Random Color"
    }
}
```

Note that we can also dynamically provide context too, though this is less useful. Whenever we dynamically provide context, the current component's context entry is replaced (if it existed).

```rust
use_context_provider(|| ThemeProvider::new());
rsx! {
    button {
        onclick: move |_| dioxus::core::provide_context(ThemeProvider::reset()),
        "Reset Theme Provider"
    }
}
```

Replacing context is *not* a reactive operation which inherently limits its usefulness.

## Providing Signals

So far, we've only demonstrated how to provide simple values like Strings. As mentioned above, dynamically replacing context is not a reactive operation, so we shouldn't use it to update state in our application.

To make our context fully reactive, we should provide signals. Typically, you'll bundle a collection of signals together into a larger state object:

```rust
#[derive(Clone, Copy)]
struct HeaderContext {
    title: Signal<String>,
    subtitle: Signal<String>
}
```

Notice how our `HeaderContext` derives both `Clone` and `Copy`. For context to be shared throughout the component tree, each context entry must implement `Clone`. Dioxus signals are extremely ergonomic because the implement `Copy` too, which makes them easier to work with in async contexts. Similarly, our custom context structs can also implement `Copy` and gain the same ergonomic benefits.

To construct our HeaderContext, we use one of two approaches. The first is to build a Provider component:

```rust
#[component]
fn HeaderProvider(children: Element) -> Element {
    // create signals with `use_signal`
    let title = use_signal(|| "Title".to_string());
    let subtitle = use_signal(|| "Subtitle".to_string());

    // And then use them to build the HeaderContext directly
    use_context_provider(|| HeaderContext { title, subtitle });

    children
}
```

The second approach is to use `Signal::new()` directly, either in a `HeaderContext::new()` method or a `use_header_provider` function. The advantage here is that users can provide `HeaderContext` without needing to wrap their RSX elements.

```rust
fn app() -> Element {
    use_context_provider(|| HeaderContext::new());

    // ...
}

impl HeaderContext {
    fn new() -> HeaderContext {
        HeaderContext {
            title: Signal::new("Title")
            subtitle: Signal::new("Subtitle")
        }
    }
}
```

Using `new` methods is idiomatic Rust and lets us customize the initial parameters used to build the context.

When we bundle signals in a struct, we can make working with state a bit easier by adding accessor and mutation methods.

```rust
impl HeaderContext {
    pub fn reset(&mut self) {
        self.title.set("".to_string());
        self.subtitle.set("".to_string());
    }

    pub async fn fetch(&mut self) {
        let data = api::fetch_header_info().await;
        self.title.set(data.title);
        self.subtitle.set(data.subtitle);
    }

    pub fn uppercase_title(&self) -> String {
        self.title.cloned().to_ascii_uppercase()
    }
}
```

Now, when we want to interact with the header context, we use its methods:

```rust
let mut header = use_context::<HeaderContext>();

rsx! {
    h1 { "{header.uppercase_title()}" }
    button { onclick: move |_| header.reset(), "Reset" }
    button { onclick: move |_| header.fetch(), "Fetch New" }
}
```

With context and signals, you should have all the tools required to architect large reactive Dioxus apps. These simple primitives compose into a complete first-party state solution. You can say goodbye to libraries like [Redux](https://redux.js.org) and [Mobx](https://mobx.js.org/README.html)!

## Global Signals

We're not done with global state *just* yet! Dioxus provides an enhancement to the "signals in context" pattern with **global signals**. Global signals are signals available to *every* component in your application. Global signals are automatically mounted to the root component of your app.

Because the `GlobalSignalProvider` is automatically mounted to your app, you don't need to call `use_context_provider`. To create a new global signal, you use the `Signal::global` initializer in a `static`.

```rust
{{#include ../docs-router/src/doc_examples/guide_state.rs:initialize_global_signal}}
```

And then read and write to it from anywhere:

```rust
{{#include ../docs-router/src/doc_examples/guide_state.rs:use_global_signal}}
```

GlobalSignals are only global to one app - not the entire program. This means that in "multitenant" environments like server-side-rendering and multi-window desktop, every app gets its own *independent* global signal value.

```rust
// every separate instance of the app receive its own "COUNT" GlobalSignal
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);

fn app() -> Element {
    rsx! {
        div { "{COUNT}" }
        button { onclick: move |_| *COUNT.write() += 1 }
    }
}
```

While it may seem like `COUNT` is synchronized across *every* runnig app, it actually is just local to one app at a time. Global signals are roughly implemented as a HashMap of global signal key to value where the key is a unique compile-time identifier per instance.

In addition to global signals, you can also have global memos with GlobalMemo. These are similar to regular memos, allowing you to incrementally compute new values as the inner reactive values are updated.

```rust
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
static DOUBLE_COUNT: GlobalMemo<i32> = Memo::global(|| COUNT.cloned() * 2);

fn app() -> Element {
    rsx! {
        div { "count: {COUNT}" }
        div { "double: {DOUBLE_COUNT}" }
        button { onclick: move |_| *COUNT.write() += 1 }
    }
}
```

Note that the types for GlobalSignal and GlobalMemo are actually `Global<Signal<T>>` and `Global<Memo<T>>` - if you implemented the required traits, you can make any custom reactive type globally available!
