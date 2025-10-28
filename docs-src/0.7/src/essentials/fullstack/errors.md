# Fullstack Error Handling

Errors are unfortunately inevitable in software development. Even in Rust, apps might not behave as expected and user requests might be malformed.

In these cases, you might want to show an error page to the user while also returning an appropriate status code.

## Creating an Error

Recall in the [Server Functions](./server_functions.md) chapter, that all server functions must return a `Result<T>`:

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<()> {
    // ...
}
```

Because server functions are called from the client, they need some way of expressiong *both* a request failure and a server failure. If the user is offline, we want to reliably return an offline error and status in the UI.

Also recall that server functions can return several different error types:

- `ServerFnError`: A broad error type encompasing request failures and server failures
- `anyhow::Error`: A general-purpose error type that downcasts its inner value
- `CapturedError`: A cheaply-clonable `anyhow::Error` wrapper
- `StatusCode`: A specific HTTP status code
- `HttpError`: A specific HTTP status code and a message
- Custom Errors: User errors that implement `Serialize + Deserialize + AsStatusCode`

Dioxus will attempt to downcast server function errors and captured errors into status codes such that the returned page receives an appropriate HTTP status.

If an error does not downcast to a known status-code-like error type, then the request will default to a `500 INTERNAL SERVER ERROR`.

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<()> {
    // This will return a 500 status code
    return Err(anyhow::anyhow!("Bad request!").into());

    // ...
}
```

The `OrHttpError` error type makes emitting status codes quite simple with its extension methods on `Result<T>`, `Option<T>`, and `bool`.

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<()> {
	authenticate_user()
        // this method comes from `OrHttpError`
		.or_unauthorized("You must be logged in to view this resource")?;
	// ..
}
```

## Error Status Codes Bubble

In the event of an error, Dioxus extracts the status code for the response by downcasting errors that bubble to root component.

For example, this example app does not provide a root error boundary, and thus all errors will bubble up to the root:

```rust
fn app() -> Element {
    let post_data = use_loader(move || get_post())?;

    rsx! {
        p { "{post_data}" }
    }
}

// This endpoint always throws an error
#[get("/api/post/")]
async fn get_post() -> Result<String, HttpError> {
    HttpError::not_found("Post not found")
}
```

If you `curl` the app, you'll notice that it returns a `404` status code.

If we want to catch the error and provide a nice fallback UI, we can use an `ErrorBoundary`. Note that when we catch the error, we must

```rust
fn app() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: move |err| {
                // To ensure the HTTP status is still set properly, we need to call `commit_error_status`
                let http_error = FullstackContext::commit_error_status(err.error().unwrap());

                // and then we can render some pretty fallback UI
                rsx! { "An error occured! {http_error:?}" }
            },
            Post {}
        }
    }
}

fn Post() -> Element {
    let post_data = use_loader(move || get_post())?;
    rsx! { p { "{post_data" } }
}
```

The `commit_error_status` function on `FullstackContext` extracts the HTTP status from a `CapturedError` and then modifies the outgoing response accordingly.


## Error Layout with Router

If you're using the Dioxus Router for your app's routing, then it might not be immediately clear how to integrate custom error pages into your app.

In these cases, we'd take a similar approach with an `ErrorBoundary`. We could either wrap our `Router {}` component in an error boundary, or add a layout to our app that renders the fallback UI.

```rust
// A router definition with `ErrorLayout` layout
#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[layout(ErrorLayout)]
    #[route("/")]
    Home,

    #[route("/blog/:id")]
    Blog { id: u32 },
}

// And then our Outlet is wrapped in a fallback UI
#[component]
fn ErrorLayout() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: move |err: ErrorContext| {
                let http_error = FullstackContext::commit_error_status(err.error().unwrap());
                match http_error.status {
                    StatusCode::NOT_FOUND => rsx! { div { "404 - Page not found" } },
                    _ => rsx! { div { "An unknown error occurred" } },
                }
            },
            Outlet::<Route> {}
        }
    }
}
```

Using layouts for error pages is extremely powerful. You can isolate fallback UI to specific parts of the page while also properly setting the returned status code.

For example, the GitHub UI retains most of its UI while isolate the 404 message to just the source code viewer:

![Github Fallback UI](/assets/07/github-fallbackui.avif)

It's a better user experience to render a web page that is visually consistent while also still delivering the appropriate status code.
