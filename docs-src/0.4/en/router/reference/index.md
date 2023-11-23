# Adding the router to your application

In this chapter, we will learn how to add the router to our app. By itself, this
is not very useful. However, it is a prerequisite for all the functionality
described in the other chapters.

> Make sure you added the `dioxus-router` dependency as explained in the
> [introduction](../index.md).

First, add this to your `Dioxus.toml`:

```toml
[web.watcher]
index_on_404 = true
```

> This configuration only works when using `dx serve`. If you host your app in a different way (which you most likely do in production), you need to find out how to add a fallback 404 page to your app, and make it a copy of the generated `dist/index.html`.

This will instruct `dx serve` to redirect any unknown route to the index, to then be resolved by the router.
You see, the router works on the client, meaning that if we connect through the index route (e.g., `localhost:8080`, then click a button to go to `localhost:8080/contact`), the app is launched and routes are resolved.
However, when we go to a route *before* going to the index (go straight to `localhost:8080/contact`), we are trying to access a static route from the server.
And here's the thing; the only route on our server is the index, because the Dioxus frontend is a SPA (Single Page Application).

In most cases, we want to add the router to the root component of our app. This
way, we can ensure that we have access to all it's functionality everywhere.

First, we define the router with the router macro:

```rust
{{#include src/doc_examples/first_route.rs:router}}
```

Then we render the router with the [`Router`] component.

```rust
{{#include src/doc_examples/first_route.rs:app}}
```
