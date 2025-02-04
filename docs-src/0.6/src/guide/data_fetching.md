# Fetching Data

Our *HotDog* app has some basic interactivity but does not yet fetch new dog images. In this chapter, we'll interact with async and fetching data from an API.

## Adding Dependencies

Dioxus does not provide any built-in utilities for fetching data. Crates like [dioxus-query](https://github.com/marc2332/dioxus-query) exist, but for this tutorial we'll implement data-fetching from scratch.

First, we need to add two new dependencies to our app: [serde](https://crates.io/crates/serde) and [reqwest](https://crates.io/crates/reqwest).

- Reqwest provides an HTTP client for fetching.
- Serde will let us derive a JSON Deserializer to decode the response.

In a new terminal window, add these crates to your app with `cargo add`.

```bash
cargo add reqwest --features json
cargo add serde --features derive
```

## Defining a Response Type

We'll be using the amazing [dog.ceo/dog-api](https://dog.ceo/dog-api/) to fetch images of dogs for *HotDog*. Fortunately, the API response is quite simple to deserialize.

Let's create a new Rust struct that matches the format of the API and derive `Deserialize` for it.

The Dog API docs outline a sample API response:
```json
{
    "message": "https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg",
    "status": "success"
}
```

Our Rust struct needs to match that format, though for now we'll only include the "message" field.
```rust
{{#include src/doc_examples/guide_data_fetching.rs:dog_api}}
```

## Using `reqwest` and `async`

Dioxus has stellar support for asynchronous Rust. We can simply convert our `onclick` handler to be `async` and then set the `img_src` after the future has resolved.

![Dog Fetching](/assets/06_docs/fetch-dog.mp4)

The changes to our code are quite simple - just add the `reqwest::get` call and then call `.set()` on `img_src` with the result.

```rust
{{#include src/doc_examples/guide_data_fetching.rs:dog_view_reqwest}}
```

Dioxus automatically calls `spawn` on asynchronous closures. You can also use `spawn` to perform async work *without* async closures - just call `spawn()` on any async block.

```rust
{{#include src/doc_examples/guide_data_fetching.rs:spawn}}
```

The futures passed to `spawn` must not contain latent references to data outside the async block. Data that is `Copy` *can* be captured by async blocks, but all other data must be *moved*, usually by calling `.clone()`.

## Managing Data Fetching with use_resource

Eventually, using bare `async` calls might lead to race conditions and weird state bugs. For example, if the user clicks the *fetch* button too quickly, then two requests will be made in parallel. If the request is updating data somewhere else, the wrong request might finish early and causes a race condition.

In Dioxus, *Resources* are pieces of state whose value is dependent on the completion of some asynchronous work. The `use_resource` hook provides a `Resource` object with helpful methods to start, stop, pause, and modify the asynchronous state.

Let's change our component to use a resource instead:

```rust
{{#include src/doc_examples/guide_data_fetching.rs:dog_view_resource}}
```

Resources are very powerful: they integrate with Suspense, Streaming HTML, reactivity, and more.

The details of the `Resource` API are not terribly important right now, but you'll be using Resources frequently in larger apps, so it's a good idea to [read the docs](../reference/use_resource.md).
