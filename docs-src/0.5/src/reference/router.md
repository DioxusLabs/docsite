# Router

In many of your apps, you'll want to have different "scenes". For a webpage, these scenes might be the different webpages with their own content. For a desktop app, these scenes might be different views in your app.

To unify these platforms, Dioxus provides a first-party solution for scene management called Dioxus Router.


## What is it?

For an app like the Dioxus landing page (https://dioxuslabs.com), we want to have several different scenes:

- Homepage
- Blog

Each of these scenes is independent – we don't want to render both the homepage and blog at the same time.

The Dioxus router makes it easy to create these scenes. To make sure we're using the router, add the `router` feature to your `dioxus` dependency:

```shell
cargo add dioxus@0.5.0 --features router
```


## Using the router

Unlike other routers in the Rust ecosystem, our router is built declaratively at compile time. This makes it possible to compose our app layout simply by defining an enum.

```rust
{{#include src/doc_examples/router_reference.rs:router_definition}}
```

Whenever we visit this app, we will get either the Home component or the Blog component rendered depending on which route we enter at. If neither of these routes match the current location, then nothing will render.

We can fix this one of two ways:

- A fallback 404 page

```rust
{{#include src/doc_examples/router_reference.rs:router_definition_404}}
```

- Redirect 404 to home

```rust
{{#include src/doc_examples/router_reference.rs:router_404_redirect}}
```

## Links

For our app to navigate these routes, we can provide clickable elements called Links. These simply wrap `<a>` elements that, when clicked, navigate the app to the given location. Because our route is an enum of valid routes, if you try to link to a page that doesn't exist, you will get a compiler error.

```rust
{{#include src/doc_examples/router_reference.rs:links}}
```

## More reading

This page is just a very brief overview of the router. For more information, check out the [router book](../router/index.md) or some of the [router examples](https://github.com/DioxusLabs/dioxus/blob/master/examples/router.rs).
