# Data Fetching

One of the most common asynchronous operations in applications is making network requests. This guide will cover how to fetch data in Dioxus, how to avoid waterfalls, and using libraries to manage caching and invalidating requests.

The hooks and techniques we cover here are built on top of the Future and Signal primitives.

## Library Dependencies

While Dioxus does not provide a built-in HTTP client, you can use the popular [reqwest](https://docs.rs/reqwest/latest/reqwest/) library to make asynchronous network requests. We will be using the reqwest library throughout the examples in this page. Before we start, make sure to add the `reqwest` and `serde` libraries to your `Cargo.toml`:

```sh
cargo add reqwest --features json
cargo add serde --features derive
```

Your Cargo.toml should have the reqwest and serde libraries:
```toml
[dependencies]
# ... dioxus and other dependencies
reqwest = { version = "*", features = ["json"] }
serde = { version = "1", features = ["derive"] }
```

We are planning on eventually integrating a library like [dioxus-query](https://crates.io/crates/dioxus-query) directly into Dioxus for better integration with the app router.

## Requests from Event Handlers

The simplest way to request data is simply by attaching an async closure to an EventHandler.

```rust
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

let mut img_src = use_signal(|| "image.png".to_string());

let fetch_new = move |_| async move {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<DogApi>()
        .await
        .unwrap();

    img_src.set(response.message);
};

rsx! {
    img { src: img_src }
    button { onclick: fetch_new, "Fetch a new dog!" }
}
```

Whenever the user clicks the button, the `fetch_new` closure is fired, a new Future is spawned, and the network request is made. When the response is complete, we set `img_src` to the return value.

Unfortunately, data fetching is not always quite this simple. If the user rapidly presses the fetch button, multiple requests will be made simultaneously, and the image source will overwritten multiple times. To mitigate this, we can add a "loading" Signal to prevent multiple requests:

```rust
let mut img_src = use_signal(|| "image.png".to_string());
let mut loading = use_signal(|| false);

let fetch_new = move |_| async move {
    if loading() {
        return;
    }

    loading.set(true);
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<DogApi>()
        .await
        .unwrap();

    img_src.set(response.message);
    loading.set(false);
};

// ...
```

Manually handling edge cases of data loading can be tedious, so we've built a more general solution for futures with `use_resource`.

## Asynchronous State with `use_resource`

The [`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) hook can be used to *derive* asynchronous state. This function accepts an async closure that returns a Future. As the future is polled, `use_resource` tracks `.read()` calls of any contained Signals. If another action calls `.write()` on the tracked signals, the `use_resource` immediately restarts.

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResource {}
}
```

The `use_resource` hook might look similar to the `use_memo` hook. Unlike `use_memo`, the resource's output is not memoized with `PartialEq`. That means any components/reactive hooks that read the output will rerun if the future reruns even if the value it returns is the same:

```rust
{{#include ../docs-router/src/doc_examples/asynchronous.rs:use_resource_memo}}
```

```inject-dioxus
DemoFrame {
    asynchronous::UseResourceDemo {}
}
```

> Note: The future you pass to `use_resource` must be cancel safe. Cancel safe futures are futures that can be stopped at any await point without causing causing issues. For example, this task is not cancel safe:
>
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:not_cancel_safe}}
> ```
>
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::NotCancelSafe {}
> }
> ```
>
> It can be fixed by making sure the global state is restored when the future is dropped:
> ```rust
> {{#include ../docs-router/src/doc_examples/asynchronous.rs:cancel_safe}}
> ```
>
> ```inject-dioxus
> DemoFrame {
>     asynchronous::CancelSafe {}
> }
> ```
>
> Async methods will often mention if they are cancel safe in their documentation.

## Asynchronous State with `use_loader`

The `use_resource` hook is great for loading arbitrary values. However, working with resources that return results can be a little cumbersome. In some cases, the `use_loader` hook is a better choice.

The `use_loader` hook is designed to work with reactive futures that return `Result<T, E>`. Instead of returning a `Resource<T>`, like `use_resource`, the `use_loader` hook *actually* returns a `Result<Loader<T>, Loading>`. The `Loading` return type tightly integrates with Error Boundaries and Suspense - both of which are very useful in server-side-rendering (SSR).

Because `use_loader` returns a Result, you can use the `?` syntax to early return if the resource is pending or errored:

```rust
// Fetch the list of breeds from the Dog API, using the `?` syntax to suspend or throw errors
let breed_list = use_loader(move || async move {
    reqwest::get("https://dog.ceo/api/breeds/list/all")
        .await?
        .json::<ListBreeds>()
        .await
})?;
```

Generally, we recommend using `use_resource` when doing client-side fetching and `use_loader` when doing hybrid client/server fetching.

## Avoiding Waterfalls

One common issue when fetching data is the "waterfall" effect, where requests run sequentially. This can lead to slow loading times and a poor user experience. To avoid waterfalls, you can hoist your data loading logic to a higher level in your component tree and avoid returning early before unrelated requests.

Lets look at an app that causes a waterfall effect:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:waterfall_effect}}
```

In this example, we return early from the component when any of the requests are still loading. The request for the golden retriever and pug images will not start until the poodle image is loaded, causing a waterfall effect.

![waterfall effect](/assets/07/waterfall_effect.png)

We can avoid this issue by moving all of the early returns after the data fetching for all three images has started. This way, all requests will start at the same time which means they can execute in parallel:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:no_waterfall_effect}}
```

![no waterfall effect](/assets/07/no_waterfall_effect.png)

## Organizing Data Fetching

While it might be tempting to place `use_resource` calls *everywhere* in your app, we strongly recommend limiting yourself to just a few sources of data fetching. It is generally easier to reason about centralized loading states rather than many fragmented sources.

As we add more sources of data fetching, we also add a larger combination of loading states. If possible, it's better to load a users's "name" and "id" in *one* request, rather than two.

## Libraries for Data Fetching

`use_resource` is a great way to fetch data in dioxus, but it can be cumbersome to manage complex data fetching scenarios. Libraries like [Dioxus query](https://docs.rs/dioxus-query/latest/dioxus_query/) provide more advanced features for data fetching, such as caching, invalidation, and polling. We won't cover the api of these libraries in detail here, but you can check out the [dioxus awesome](https://dioxuslabs.com/awesome/) list for more libraries that can help you with data fetching.

