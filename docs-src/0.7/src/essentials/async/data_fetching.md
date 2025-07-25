# Data Fetching

One of the most common asynchronous operations in applications is making network requests. This guide will cover how fetch data in dioxus, how to avoid waterfalls, and using libraries to manage caching and invalidating requests.

## Dependencies

While dioxus does not provide a built-in HTTP client, you can use the popular [reqwest](https://docs.rs/reqwest/latest/reqwest/) library to make asynchronous network requests. We will be using the reqwest library throughout the examples in this page. Before we start, make sure to add the `reqwest` and `serde` libraries to your `Cargo.toml`:

```sh
cargo add reqwest --features json
cargo add serde --features derive
```

## Requests with `use_resource`

The easiest way to fetch data in dioxus is inside a [`use_resource`](./resources.md) hook. The async closure you pass to `use_resource` will be called when the component is created, and will automatically rerun when the dependencies change. For example, we can fetch a dog from the Dog API like this:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:fetch_resource}}
```

```inject-dioxus
DemoFrame {
    data_fetching::DogViewResource {}
}
```

Most requests will depend on state in your application. `use_resource` is reactive so it will automatically rerun when the dependencies change. For example, if we read the breed signal inside of the resource, the resource will rerun whenever the breed changes:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:fetch_resource_reactive}}
```

```inject-dioxus
DemoFrame {
    data_fetching::DogViewResourceReactive {}
}
```

## Avoiding Waterfalls

One common issue when fetching data is the "waterfall" effect, where requests run sequentially. This can lead to slow loading times and a poor user experience. To avoid waterfalls, you can hoist your data loading logic to a higher level in your component tree and avoid returning early before unrelated requests.

Lets look at at an app that causes a waterfall effect:

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

## Libraries for Data Fetching

`use_resource` is a great way to fetch data in dioxus, but it can be cumbersome to manage complex data fetching scenarios. Libraries like [dioxus query](https://docs.rs/dioxus-query/latest/dioxus_query/) provide more advanced features for data fetching, such as caching, invalidation, and polling. We won't cover the api of these libraries in detail here, but you can check out the [dioxus awesome](https://dioxuslabs.com/awesome/) list for more libraries that can help you with data fetching.
