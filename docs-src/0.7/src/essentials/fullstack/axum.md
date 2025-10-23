# Custom Axum Router

Dioxus fullstack is built on the popular backend crate Axum. The default `dioxus::launch` function will initialize a default Axum server for your fullstack project. If you need more control, you can easily customize the router with `dioxus::serve`.

The `dioxus::serve` function is the primary entrypoint for Dioxus apps that run on the server, as is standard in fullstack applications. For fullstack apps, we'll typically use both `dioxus::launch` and `dioxus::serve`, enabling each entrypoint based on the `"server"` feature.

```rust
fn main() {
    // Run `serve()` on the server only
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new router for our app using the `router` function
        let mut router = dioxus::server::router(app);

        // .. customize the router, adding layers and new routes

        // And then return the router
        Ok(router)
    });

    // When not on the server, just run `launch()` like normal
    #[cfg(not(feature = "server"))]
    dioxus::launch(app);
}
```

The `dioxus::server::router` function creates a new axum router that sets up a few imporant pieces:

- Static Assets: automatically serve the `public` directory, index.html and assets
- SSR: automatically run the app, render it to HTML, and serialize data for hydration
- Server Functions: automatically initialize the API endpoints

Dioxus uses extension methods on the Axum router (given by `DioxusRouterExt`) that is equivalent to enabling each of these items manually:

```rust
axum::Router::new()
	.register_server_functions()
	.serve_static_assets()
	.fallback(
		get(render_handler).with_state(RenderHandleState::new(cfg, app)),
	)
```


## Adding New Routes

One common use-case of a custom axum router is to add new routes to the router that are *not* defined with server functions. We might want to include special endpoints that respond dynamically or that return non-HTML data types.

This example adds three new routes to our app:

```rust
dioxus::serve(|| async move {
    use dioxus::server::axum::routing::{get, post};

    let router = dioxus::server::router(app)
        .route("/submit", post(|| async { "Form submitted!" }))
        .route("/about", get(|| async { "About us" }))
        .route("/contact", get(|| async { "Contact us" }));

    Ok(router)
});
```

Note that the server-side-rendering handler is registered as a *fallback* handler. Any routes we manually register will take priority over the Dioxus app. Since these handlers are axum handlers, they can take the typical modifiers, like `.with_state()`, `.layer()`, etc.

```rust
let router = dioxus::server::router(app)
    .route(
        "/submit",
        post(
            |state: State<FormSubmitter>, ping: Extension<Broadcast>, cookie: TypedHeader<Cookie>| async {
                // ... endpoint logic
            },
        ),
    )
    .with_state(FormSubmitter::new())
    .layer(Extension(Broadcast::new()));
```

## Adding State with Extensions

As you build out your app, you might want to expose state
