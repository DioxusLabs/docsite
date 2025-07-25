# Data Fetching

One of the most common asynchronous operations in applications is making network requests. This guide will cover how fetch data in dioxus, avoiding waterfalls, and using libraries to manage caching and invalidating requests.

## Dependencies

While dioxus does not provide a built-in HTTP client, you can use the popular [reqwest](https://docs.rs/reqwest/latest/reqwest/) library to make asynchronous network requests. We will be using the reqwest library throughout the examples in this page. Before we start, make sure to add the `reqwest` and `serde` libraries to your `Cargo.toml`:

```sh
cargo add reqwest --features json
cargo add serde --features derive
```

## Requests with `use_resource`

The easiest way to fetch data in dioxus is to fetch your data in a [`use_resource`](./resources.md) hook. The async closure you pass to `use_resource` will be called when the component mounts, and it will automatically rerun when the dependencies change. For example, we can fetch a dog from the Dog API like this:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:fetch_resource}}
```

```inject-dioxus
DemoFrame {
    data_fetching::DogViewResource {}
}
```

Most requests will depend on state in your application. `use_resource` is reactive so it will automatically rerun when the dependencies change. For example, we can fetch a dog from the Dog API based on a breed selected by the user:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:fetch_resource_reactive}}
```

```inject-dioxus
DemoFrame {
    data_fetching::DogViewResourceReactive {}
}
```

## Avoiding Waterfalls

One common issue when fetching data is the "waterfall" effect, where each request depends on the previous one. This can lead to slow loading times and a poor user experience. To avoid waterfalls, you can hoist your data loading logic to a higher level in your component tree and avoid returning early before unrelated requests.

Lets look at at an app that causes a waterfall effect:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:waterfall_effect}}
```

In this example, we return early from the component if any of the requests are still loading or if there is an error. The request for the golden retriever and pug images will not start until the poodle image is loaded, causing a waterfall effect.

![waterfall effect](/assets/07/waterfall_effect.png)

We can avoid this issue by moving all of the early returns after the data fetching for all three images has started. This way, all requests will start at the same time which means they can execute in parallel:

```rust
{{#include ../docs-router/src/doc_examples/data_fetching.rs:no_waterfall_effect}}
```

![no waterfall effect](/assets/07/no_waterfall_effect.png)

## Libraries for Data Fetching

Libraries like [dioxus query](https://docs.rs/dioxus-query/latest/dioxus_query/) provide more advanced features for data fetching, such as caching, invalidation, and polling. These libraries can help you manage complex data fetching scenarios and improve the performance of your application.
