# Fetching Data

In this chapter, we will fetch data from the hacker news API and use it to render the list of top posts in our application.

## Defining the API

First we need to create some utilities to fetch data from the hackernews API using [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html):

```rust
{{#include src/doc_examples/hackernews_async.rs:api}}
```

The code above requires you to add the [reqwest](https://crates.io/crates/reqwest), [async_recursion](https://crates.io/crates/async-recursion), and [futures](https://crates.io/crates/futures) crate:

```bash
cargo add reqwest --features json
cargo add async_recursion
cargo add futures
```

A quick overview of the supporting crates:
- [reqwest](https://crates.io/crates/reqwest) allows us to create HTTP calls to the hackernews API. 
- [async_recursion](https://crates.io/crates/async-recursion) provides a utility macro to allow us to recursively use an async function.
- [futures](https://crates.io/crates/futures) provides us with utilities all around Rust's futures.


## Working with Async

[`use_resource`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_resource.html) is a [hook](./state.md) that lets you run an async closure, and provides you with its result.

For example, we can make an API request (using [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html)) inside `use_resource`:

```rust
{{#include src/doc_examples/hackernews_async.rs:use_resource}}
```

The code inside `use_resource` will be submitted to the Dioxus scheduler once the component has rendered.

We can use `.read()` to get the result of the future. On the first run, since there's no data ready when the component loads, its value will be `None`.  However, once the future is finished, the component will be re-rendered and the value will now be `Some(...)`, containing the return value of the closure.

We can then render the result by looping over each of the posts and rendering them with the `StoryListing` component.

```inject-dioxus
DemoFrame {
	hackernews_async::fetch::App {}
}
```

> You can read more about working with Async in Dioxus in the [Async reference](../reference/index.md)

## Lazily Fetching Data

Finally, we will lazily fetch the comments on each post as the user hovers over the post.


We need to revisit the code that handles hovering over an item. Instead of passing an empty list of comments, we can fetch all the related comments when the user hovers over the item.


We will cache the list of comments with a [use_signal](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_signal.html) hook. This hook allows you to store some state in a single component. When the user triggers fetching the comments we will check if the response has already been cached before fetching the data from the hackernews API.

```rust
{{#include src/doc_examples/hackernews_async.rs:resolve_story}}
```

```inject-dioxus
DemoFrame {
	hackernews_async::App {}
}
```
