# Introduction

> If you are not familiar with Dioxus itself, check out the [Dioxus guide](../guide/index.md) first.

Whether you are building a website, desktop app, or mobile app, splitting your app's views into "pages" can be an effective method for organization and maintainability.

For this purpose, Dioxus provides a router. Use the `cargo add` command to add the dependency:

```sh
cargo add dioxus@0.5.0 --features router
```

Then, add this to your `Dioxus.toml` (learn more about configuration [here](../CLI/configure)):

```toml
[web.watcher]
index_on_404 = true
```

> This configuration only works when using `dx serve`. If you host your app in a different way (which you most likely do in production), you need to find out how to add a fallback 404 page to your app, and make it a copy of the generated `dist/index.html`.

This will instruct `dx serve` to redirect any unknown route to the index, to then be resolved by the router.
The router works on the client. If we connect through the index route (e.g., `localhost:8080`, then click a link to go to `localhost:8080/contact`), the app renders the new route without reloading.
However, when we go to a route *before* going to the index (go straight to `localhost:8080/contact`), we are trying to access a static route from the server, but the only static route on our server is the index (because the Dioxus frontend is a Single Page Application) and it will fail unless we redirect all missing routes to the index.

This book is intended to get you up to speed with Dioxus Router. It is split
into two sections:

1. The [reference](reference/index.md) section explains individual features in 
   depth. You can read it from start to finish, or you can read individual chapters 
   in whatever order you want.
2. If you prefer a learning-by-doing approach, you can check out the 
   _[example project](example/index.md)_. It guides you through 
   creating a dioxus app, setting up the router, and using some of its 
   functionality.

> Please note that this is not the only documentation for the Dioxus Router. You
> can also check out the [API Docs](https://docs.rs/dioxus-router/).
