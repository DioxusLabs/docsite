# Fetching Data

In this chapter, we will fetch data from the hacker news API and use it to render the list of top posts in our application.

## Defining the API

First we need to create some utilities to fetch data from the hackernews API:

```rust
{{#include src/doc_examples/hackernews_async.rs:api}}
```

## Working with Async

[`use_future`](https://docs.rs/dioxus-hooks/latest/dioxus_hooks/fn.use_future.html) is a [hook](./state.md) that lets you run an async closure, and provides you with its result.

For example, we can make an API request (using [reqwest](https://docs.rs/reqwest/latest/reqwest/index.html)) inside `use_future`:

```rust
{{#include src/doc_examples/hackernews_async.rs:use_future}}
```

The code inside `use_future` will be submitted to the Dioxus scheduler once the component has rendered.

We can use `.value()` to get the result of the future. On the first run, since there's no data ready when the component loads, its value will be `None`.  However, once the future is finished, the component will be re-rendered and the value will now be `Some(...)`, containing the return value of the closure.

We can then render the result by looping over each of the posts and rendering them with the `StoryListing` component.

```inject-dioxus
DemoFrame {
	hackernews_async::fetch::App {}
}
```

## Lazily Fetching Data

Finnaly, we will lazily fetch data on each post as the user hovers over the post.


We need to revisit the code that handles hovering over an item. Instead of 

```rust
{{#include src/doc_examples/hackernews_async.rs:resolve_story}}
```

```inject-dioxus
DemoFrame {
	hackernews_async::App {}
}
```
