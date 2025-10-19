# Dioxus Fullstack

Almost all apps need a remote server to store and update user data. Dioxus provides a number of fullstack utilities for building your app's server alongisde the client. With Dioxus Fullstack, you can build *both* your app's frontend and backend entirely in Rust!

Dioxus Fullstack deeply integrates with the popular [Axum](https://docs.rs/axum/latest/axum/) framework, making it easy to quickly add complex fullstack functionality to your app, including:

- **Server-Side-Rendering**: Render HTML on the server and hydrate on the client
- **Server Functions**: Type-safe Axum HTTP endpoints directly callable from the client
- **Ergonomic Typed Routing**: Easily extract queries and paths from the URL
- **Typed Forms**: Capture multipart form data from the client into type Rust structs
- **Binary Streams**: Easily add file upload/download backend capability
- **SSE and WebSockets**: Complex, stateful datatypes for server communication
- **Asset Management**: Automatically optimizes assets for deployment to CDNs
- **WASM Support**: Deploy to WASM-based providers like Cloudflare Workers
- **Hot-Reload**: Rapid Rust hot-reload during development powered by [subsecond](https://crates.io/crates/subsecond)

Currently, Dioxus Fullstack does not provide built-in utilities for things like Databases, Caches, Sessions, and Mailers. Our current focus is to finish polishing the fullstack integration before branching out into a more "complete" fullstack solution. As such, you'll need to pull in 3rd-party crates like `Sqlx` and `tower-sessions` to use such features.

## A Quick Example

Dioxus Fullstack is designed to be simple, intuitive, and powerful. To add new endpoints to your app, you can use the built-in procedural macros (`#[get]`, `#[post]`, etc) on Axum handlers:

```rust
#[get("/api/users/{user_id}", db: SqlDb)]
async fn get_user(user_id: Uuid) -> Result<UserData> {
    db.get(user_id)
}
```

Here, we define a new endpoint for our app that takes `user_id` as a URL path parameter and `SqlDb` as an Axum extractor. This endpoint can be directly called by the client just like any Rust function:

```rust
fn app() -> Element {
    let user_id = use_current_user()?;

    rsx! {
        button {
            onclick: move |_| async move {
                let user = get_user(user_id).await;
            },
            "Get user"
        }
    }
}
```


## Solid Building BLocks

Dioxus Fullstack is designed to be composable with the broader Axum ecosystem, building on top of foundational crates like Axum, Tower, Hyper, and HTTP. Axum is an extraordinarily performant backend solution and fosters a wide ecosystem add-ons.

If something is not directly built-in to Dioxus Fullstack, definitely look online for an axum-compatible crate to drop in!

> In addition to this guide, you can find more examples of full-stack apps and information about how to integrate with other frameworks and desktop renderers in the [examples directory](https://github.com/DioxusLabs/dioxus/tree/main/examples).
