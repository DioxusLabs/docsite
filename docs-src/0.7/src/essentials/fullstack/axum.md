# Axum Router

Dioxus fullstack is built on the popular backend crate Axum. The default `dioxus::launch` function will initialize a default Axum server for your fullstack project. If you need more control, you can easily customize the router with `dioxus::serve`.

The `dioxus::serve` function is the primary entrypoint for Dioxus apps that run on the server, as is standard in fullstack applications. For fullstack apps, you'll typically use both `dioxus::launch` and `dioxus::serve`, enabling each entrypoint based on the `"server"` feature.

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

## Adding `Layers`

Axum allows you to to attach middleware to many parts of your router:

- To entire routers with [Router::layer](https://docs.rs/axum/latest/axum/struct.Router.html#method.layer) and [Router::route_layer](https://docs.rs/axum/latest/axum/struct.Router.html#method.route_layer).
- To method routers with [MethodRouter::layer](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html#method.layer) and [MethodRouter::route_layer](https://docs.rs/axum/latest/axum/routing/method_routing/struct.MethodRouter.html#method.route_layer).
- To individual handlers with [Handler::layer](https://docs.rs/axum/latest/axum/handler/trait.Handler.html#method.layer).


## Adding State with Extensions

As you build out your app, you might want to expose state to your endpoints and to your requests. Axum provides two ways of adding state to endpoints: `Extension` and `State<T>`. Extensions enable you to attach extra data to requests as they are handled by your router.

You can use extensions *either* as a form of global state *or* as a way of attaching state to requests. To share a given piece of data with all endpoints, you can attach the extension as a layer to the router in `dioxus::serve`:

```rust
dioxus::serve(|| async move {
    use dioxus::server::axum::Extension;
    use tokio::sync::broadcast;

    let router = dioxus::server::router(app)
        .layer(Extension(broadcast::channel::<String>(16).0));

    Ok(router)
});
```

Now, in our handlers, we can extract the extension from the request:

```rust
#[post("/api/broadcast", ext: Extension<broadcast::Sender<String>>;)]
async fn broadcast_message() -> Result<()> {
    ext.send("New broadcast message".to_string())?;
    Ok(())
}
```

If we want to attach state to a single request - as in the case with a session middleware - we can attach a new middleware to the router that dynamically inserts a new extension into the request.

```rust
use axum::{extract::Request, middleware::Next, middleware};

let router = dioxus::server::router(app)
    .layer(middleware::from_fn(|req: Request, next: Next| async move {
        // Attach some extra state to the request
        req.extensions_mut().insert(Session::new());

        // And then return the response with `next.run()
        Ok::<_, Infallible>(next.run(req).await)
    }))
```

## Adding State with `State<T>`

As you migrate an existing Axum backend to Dioxus Fullstack, you might eventually need to use Axum's `State<T>` type parameter. In Axum, the `State<T>` type provides state to your axum endpoints using compile-time guarantees.

... todo - currently no migration pattern here

## Using `Lazy<T>` as Global State

As a simpler alternative to axyn extensions and `State<T>`, you can also use the built-in `Lazy<T>` type to access server resources without needing to set up a dedicated `dioxus::serve` entrypoint. The `Lazy<T>` type is very similar to the standard library's `LazyLock<T>` type, making it possible to initialize asynchronous data like database connections.

Simply create a new `Lazy<T>` instance as a `static` variable:


```rust
static DATABASE: Lazy<sqlx::SqlitePool> = Lazy::new(|| async move {
    dioxus::Ok(
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with("sqlite::memory:".parse().unwrap())
            .await?,
    )
});
```

When you access the `DATABASE` object in your code, Dioxus will ensure it's properly initialized, blocking the current thread until the initializer finishes. This lets you use asynchronous resources *synchronously* which makes them extremely ergonomic.

```rust
/// When using the `Lazy<T>` type, it implements `Deref<Target = T>`, so you can use it like a normal reference.
#[get("/api/users")]
async fn get_users() -> Result<Vec<String>> {
    let users = DATABASE
        .fetch_all(sqlx::query("SELECT name FROM users"))
        .await?
        .iter()
        .map(|row| row.get::<String, _>("name"))
        .collect::<Vec<_>>();

    Ok(users)
}
```

Typically, Rust discourages the use of global variables for managing state, but for apps like web-servers, it's generally okay to have a single shared object for the entire app.

Note that you can also use the built-in standard `LazyLock` type for simple synchronous data:

```rust
static MESSAGES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[post("/api/messages")]
async fn add_message() -> Result<()> {
    MESSAGES.lock().await.push("New message".to_string());
    Ok(())
}
```

## Nesting Routers

todo....
