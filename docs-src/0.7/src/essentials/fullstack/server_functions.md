# Server Functions

Dioxus Fullstack provides an ergonomic solution for quickly building your backend API and calling those endpoints on the client called *Server Functions*. Server Functions are regular Rust functions that define an Axum-compatible endpoint:

```rust
#[get("/api/hello-world")]
async fn hello_world() -> Result<String> {
	Ok("Hello world!".to_string())
}
```

Server functions automatically generate an HTTP endpoint for your app. After launching your app, you can `curl` your endpoint directly:

```sh
# returns "Hello world!"
curl http://127.0.0.1:8080/api/hello-world
```

Server functions can be called directly from the client as a function:

```rust
let onclick = move |_| async move {
	let msg = hello_world().await;
	// ...
}
```

Server functions can take all sorts of modifiers like server-only extractors and custom axum payloads, making them even more powerful than a plain axum handler:

```rust
#[get("/api/users/{user_id}", db: SqlDb)]
async fn get_user(user_id: Uuid) -> Result<UserData> {
    db.get(user_id)
}
```

Ultimately, a server function is just an axum endpoint - you can cleanly use the entirety of the Axum ecosystem with server functions!

## Anatomy of a Server Function

A server function is an HTTP endpoint in the form of a Rust fuction. We can transform a regular function into a server function by annotating it with one of a few procedural macros:

- Explicitly using the `#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]` macros
- Anonymously with the `#[server]` macro

To make a server function, simply add one of `#[get]`, `#[post]`, etc on top of your function. This function has a few restrictions - it must:

- Be an async function
- Return a `Result<T, E>`
- Take arguments that are either `Serialize + Deserialize` *or* `IntoRequest + FromRequest`
- Return a type that is either `Serialize + Deserialize` *or* `IntoResponse + FromResponse`

Dioxus uses some specialization "magic" to enable flexible input and output types, so the errors for types not satisfying these bounds might be rather unwieldly.

In essence, the non-URL inputs must either be a set of items that are obviously serializable (think strings, numbers, custom types):

```rust
// The function inputs create a single serializable object that looks like:
//
// ```
// #[derive(Serialize, Deserialize)]
// struct Body {
//     a: String,
//     b: i32,
//     c: serde_json::Value,
// }
// ```
#[get("/api/json-body")]
async fn json_body(a: String, b: i32, c: serde_json::Value) -> Result<()> {
	Ok(())
}
```

*or*, the inputs would be a single object that implements Axum's `FromRequest` trait and Dioxus' `IntoRequest` trait. Dioxus Fullstack provides a number of built-in types that implement these types and can be used across the client and the server:

```rust
// The `FileStream` type lets us stream file uploads from the client to the server
#[get("/api/upload")]
async fn upload(file: FileStream) -> Result<()> {
	// ....
}
```

Similarly, the output type can be either a serializable object (strings, numbers, custom strutures)

```rust
// Our custom payload implements `Serialize + Deserialize`
#[derive(Serialize, Deserialize)]
struct Payload {
	a: i32,
	b: String
}

#[get("/api/json-out")]
async fn json_body() -> Result<Payload> {
	Ok(Payload {
		a: 123,
		b: "hello".to_string(),
	})
}
```

*or* an object that implements Axum's `IntoResponse` trait and Dioxus' `FromResponse` trait. Many built-in types implement these traits and can be returned to the client:

```rust
#[get("/api/stream")]
async fn stream() -> Result<Streaming<String>> {
	// ...
}
```

If you want to use a 3rd-party Axum response type but it doesn't implement `FromResponse`, then you need to call `.into_response()` and return an `axum::response::Response` type instead:

```rust
#[get("/api/video", range: RangeHeader)]
async fn video_endpoint() -> Result<axum::response::Response> {
	let chunk = get_chunk_from_range(range);
	Ok(chunk.into_response())
}
```

### Path and Query Extractors

We can combine custom payload bodies with query and path extractors, enabling us to build APIs that are suitable both for our Rust frontend and any other HTTP client. This can be particularly useful if your API is consumed both by your own app and external customers.

To add query and path extractors, we can use the Axum route syntax in the macro. The macro will parse the route and generate the associated axum extractors for you:

```rust
#[get("/api/products/{product}?color&quantity", range: RangeHeader)]
async fn get_product_data(product: String, color: String, quantity: Option<i32>) -> Result<Vec<Product>> {
	// ...
}
```

Under the hood, we generate `axum::extract::Query<T>` and `axum::extract::Path<T>` objects, so you can use any valid types, like `Option<T>`. When extracting from the URL, values are URL-encoded and URL-decoded. Note that not all structures can be cleanly URL-encoded, so we recommend sticking with simple data types where possible.

We can combine path and query extractors with the body extractor. This is especially useful when sending additional data alongside custom payloads.

```rust
// we can pass along additional data to objects like streams!
#[post("/api/photos/upload?name&rating")]
async fn upload_photo(name: String, rating: i32, image: FileStream) -> Result<i32> {
	// ...
}
```

### Custom Inputs

We mentioned earlier that the non-query arguments of a server function must be one of two types:

- A group of serializable types (strings, ints, custom serializable structs)
- A single type that implements `FromRequest` and `IntoRequest`

The second type - `FromRequest + IntoRequest` - is extremely powerful. This lets us create new bodies that abstract over the client request with Rust methods, making things like the built-in `WebsocketOptions` and `Websocket` types possible.

```rust
#[get("/api/ws")]
async fn get_updates(options: WebsocketOptions) -> Result<Websocket> {
	Ok(options.on_upgrade(|mut socket| {
		// ...
	}))
}
```

The `WebsocketOptions` type implements the two key Rust traits mentioned above: `FromRequest` and `IntoRequest`.

The first trait, [`FromRequest`](https://docs.rs/axum/latest/axum/extract/trait.FromRequest.html), comes from Axum, the library that Dioxus fullstack is built on.

To implement the `FromRequest` trait, we need to define our new type and then implement the `from_request` method. If you aren't sure which `Rejection` type to use in the implementation, you can use the built-in `ServerFnError` type which integrates with the rest of Dioxus fullstack.

```rust
struct WebsocketOptions {}

impl<S: Send> FromRequest<S> for WebSocketOptions {
    type Rejection = axum::response::Response;

    fn from_request(
        request: Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
		async move {
			// .. implementation for our type
		}
	}
}
```

Perfect - this lets use the `WebsocketOptions` type as an Axum extractor. Now, we need to implement `IntoRequest` which lets create `WebsocketOptions` on the client before passing it off to the server.

The `IntoRequest` trait is generic over a hidden "state" type parameter. Generally, you'll implement the plain `IntoRequest` type, but for complex types like Websockets, we need a custom state object that the response (`Websocket`) will use to initialize with. In this case, we create a new state type called `UpgradingWebsocket` which will hold the state from the original request to properly upgrade the server's response into a `Websocket` handle.

```rust
struct UpgradingWebsocket {
	/// .. state for the connection
}

// IntoRequest is generic over `UpgradingWebsocket`
impl IntoRequest<UpgradingWebsocket> for WebSocketOptions {
    fn into_request(
        self,
        request: ClientRequest,
    ) -> impl Future<Output = std::result::Result<UpgradingWebsocket, RequestError>> + 'static {
		async move {
			let stream = send_request(request).await?;

			return Ok(UpgradingWebsocket {
				// ... pass along the stream
			})
		}
	}
}
```

For bodies that don't need custom state, you can just use the default `IntoRequest` type which is generic over the Dioxus Fullstack `ClientResponse` type

```rust
// the default state is `ClientResponse`:
pub trait IntoRequest<R = ClientResponse>: Sized {
    fn into_request(
        self,
        req: ClientRequest,
    ) -> impl Future<Output = Result<R, RequestError>> + 'static;
}
```

Now, when the client makes a request to our endpoint, the `WebsocketOptions` struct can be used to store state for the connection:

```rust
// We can now use `WebsocketOptions` as a custom body:
#[get("/api/ws/")]
async fn get_updates(options: WebsocketOptions) -> Result<()> {
	// ...
}

// Calling the endpoint is still quite simple:
_ = get_updates(WebsocketOptions::new()).await?;
```

### Custom Outputs

The `IntoRequest` and `FromRequest` traits allow us to send arbitrary data types to the Server, but sometimes we need to return arbitrary data types to the Client. In our example above, this would be the `Websocket` return type:

```rust
#[get("/api/ws")]
async fn get_updates(options: WebsocketOptions) -> Result<Websocket> {
	Ok(options.on_upgrade(|mut socket| {
		// ...
	}))
}
```

As mentioned above, the return type of a server function must be one of two types:
- An obviously serializable object (string, int, custom struct)
- A type that implements `IntoResponse` and `FromResponse`

The [`IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html) trait comes from Axum and is quite simple to implement. To implement the `IntoResponse` type, we just need to implement the `into_response` method for our custom type. The return type here is an Axum `Response` which is very simple to construct:

```rust
impl IntoResponse for Websocket {
	fn into_response(self) -> Response {
        Response::builder()
			.status(200)
			.header(/* */)
			.body(/* */)
			.unwrap()
	}
}
```

The Response here is directly passed along to the client. Dioxus Fullstack might attach some additional headers to the response, but the response body will remain untouched as its returned through the Axum router.

Now, to use our `Websocket` type on the client, we need to implement `FromResponse`. The `FromResponse` trait is an analog to the `IntoResponse` trait, with a similar definition:

```rust
pub trait FromResponse<R = ClientResponse>: Sized {
    fn from_response(res: R) -> impl Future<Output = Result<Self, ServerFnError>>;
}
```

Just like `IntoRequest`, the `FromResponse` trait is generic over a default state parameter (usually `ClientResponse`). For our `Websocket` type, we need to match the same state parameter as our `WebsocketOptions` type. Usually, we *aren't* generic over the state paramter since the `ClientResponse` type is quite useful on its own, but for `Websocket`, we want to make sure the input request has the required state at compile time.

To implement `FromResponse`, we need to create a new instance of our type from the stored state:

```rust
impl FromResponse<UpgradingWebsocket> for Websocket {
    fn from_response(res: UpgradingWebsocket) -> impl Future<Output = Result<Self, ServerFnError>> {
		async move {
			// ...
		}
	}
}
```

Note that the error type here is `ServerFnError`. This type ensures that the client code can properly downcast any errors that occur while making the request into a standard error type. The `ServerFnError` type includes a number of useful error variants, allowing us to express all sorts of failure modes, some with a standardized HTTP status code and details.

### Server Extractors

As you build increasingly complex backends, you might need more control over extracting data from the request. This could be handling things like auth tokens, cookies, range headers, or any number of tasks related to the request and its headers. Sometimes, these values cannot be sent directly from the client.

In the case of authentication, we might want to extract a stateful extension from the request or reader a specific header like the auth-bearer. In many cases, the client does not explicitly pass these types to the server as they are either extracted using server-only state or implicitly attached like cookies.

To extract arbitrary data from the request, we can "hoist" the function arguments into the macro. The types here must implement Axum's `FromRequestParts` trait - or `FromRequest` if there's no client-only body.

```rust
// Our `auth` argument is a function argument hoisted to the argument list in the proc macro
#[post("/api/user/login", auth: auth::Session)]
pub async fn login() -> Result<()> {
    auth.login_user(2);
    Ok(())
}
```

Because the types here must implement `FromRequestParts`, we can use a wide variety of built-in extractors. For example, we can extract the entire `HeaderMap` object from the request:

```rust
#[get("/api/headers", headers: dioxus::fullstack::HeaderMap)]
async fn get_headers() -> Result<String> {
    Ok(format!("{:#?}", headers))
}
```

We can stack as many of these extractors as we'd like:

```rust
#[post("/api/user/login", header: TypedHeader<Cookie>, auth: Session)]
pub async fn login() -> Result<()> {
    // ...
}
```

Server-only extractors make it easy to migrate existing axum handlers to Server Functions without too many code modifications.

## Handling Errors

### Acceptable Error Types

By default, Dioxus exports a custom `Result<T>` type in the prelude. Whenever you call `use dioxus::prelude::*`, you import this `Result<T>` type into the module's scope. This `Result<T>` type is actually a re-export of anyhow's `Result<T>` type.

Anyhow's Result type is a widely used "dynamic" error type used in Rust applications. It is extremely flexible, allowing you to use Rust's powerful question-mark (`?`) syntax with any error type that implements `std::Error`.

This means that the above examples are equivalent to using the anyhow error type directly:

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<(), anyhow::Error> {
    // ...
}
```

Unfortunately, when errors are created on the server, Dioxus Fullstack cannot preserve the error's type on the client. Therefore, all errors from endpoints that use the plain `Result<T>` will always downcast to the Dioxus Fullstack `ServerFnError` type:

```rust
// Make the request, assuming it always fails, unwrapping the error
let res = login().await.unwrap_err();

// We can only downcast this error to `ServerFnError`
let error = res.downcast_ref::<ServerFnError>().unwrap();
```

If you want more detail about the error type, you can use the `ServerFnError` type directly, or use `ServerFnResult`:

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<(), ServerFnError> {
    // ...
}
```

The `ServerFnError` type is a special error type that integrates cleanly with the rest of Dioxus. Its many variants represent various failure points of handling a given request. Its two most imporant variants are `ServerError` and `RequestError`.

```rust
pub enum ServerFnError {
    /// Occurs when there is an error while actually running the function on the server.
    #[error("error running server function: {message} (details: {details:#?})")]
    ServerError {
        /// A human-readable message describing the error.
        message: String,

        /// HTTP status code associated with the error.
        code: u16,

		/// Serialized custom error type
        details: Option<serde_json::Value>,
    },

    /// Occurs on the client if there is a network error while trying to run function on server.
    #[error("error reaching server to call server function: {0} ")]
    Request(RequestError),

	// ...
}
```

If an endpoint returns a `ServerFnError`, you can match the result on the client, providing more useful feedback to the user in the event of a failure:

```rust
match login().await {
	Err(ServerFnError::ServerError { code, .. }) => {
		if code == 404 {
			// .. handle not found
		}

		if code == 401 {
			// .. handle unauthorized
		}
	}
	_ => { /* */ }
}
```

Endpoints can accept a wide variety of error types, including:

- `anyhow::Error`: a simple, flexible error type to build quickly
- `ServerFnError`: a structured error for granularly handling types of errors
- `StatusCode`: a simple wrapper around the HTTP status code
- `HttpError`: the error type returned from the `OrHttpError` type
- Custom Errors: user-defined errors (see below)

### Custom Errors

In addition to `anyhow::Error`, `ServerFnError`, and `HttpError`, Server Functions can return custom, user-defined errors. These errors must implement `Serialize`, `Deserialize`, and an additional trait called `AsStatusCode`. `AsStatusCode` requires the error implement `From<ServerFnError>` and a method to get the actual status code from the error itself.

You can easily create new error types using the `thiserror` crate's `Error` macro. The `#[from]` attribute makes it possible to easily convert `ServerFnError` into the custom error type.

```rust
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
enum MyCustomError {
    #[error("bad request")]
    BadRequest { custom_name: String },

    #[error("not found")]
    NotFound,

    #[error("internal server error: {0}")]
    ServerFnError(#[from] ServerFnError),
}
```

We must then implement `AsStatusCode` so Dioxus Fullstack knows which status code to return to the client in the event of an error.

```rust
impl AsStatusCode for MyCustomError {
    fn as_status_code(&self) -> StatusCode {
        match self {
            MyCustomError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            MyCustomError::NotFound => StatusCode::NOT_FOUND,
            MyCustomError::ServerFnError(e) => e.as_status_code(),
        }
    }
}
```

### Ergonomic Error Handling

Dioxus Fullstack provides a utility trait called `OrHttpError` to convert common failure cases into proper HTTP status codes and error messages. This trait makes it simpler to follow proper web semantics (like 404 for not-found, 401 for not-authorized, etc) while keeping inline with ergonomic Rust error handling.

You can use `OrHttpError` methods on any `Result<T>`, `Option<T>`, or `bool`, to return an `Err(HttpError)`.

For example, we might write an `authorize` method that throws an error if authorization fails. We can use the method `or_unauthorized()?` to convert the error into an appropriate status code.

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<(), ServerFnError> {
	authenticate_user()
		.or_unauthorized("You must be logged in to view this resource")?;
	// ..
}
```

To prevent polluting the global scope, only a few utility methods are available by default. You can use the `or_http_error` to return any status code:

```rust
#[post("/api/user/login")]
pub async fn login() -> Result<(), ServerFnError> {
	authenticate_user()
		.or_http_error(StatusCode::UNAUTHORIZED, "Log in first!")?;
	// ..
}
```

Note that even when we use `anyhow::Error`, Dioxus will automatically extract the appropriate status code from the error:

```rust
// our `Result<T>` contains an `HttpError` object
#[post("/api/user/login")]
pub async fn login() -> Result<()> {
	authenticate_user()
		.or_http_error(StatusCode::UNAUTHORIZED, "Log in first!")?;
	// ..
}
```

This is true for `HttpError`, `StatusCode`, and `ServerFnError`, all of which are downcasted from the anyhow Error type.

## Customizing Headers

### Customizing Response Headers

todo ...

### Customizing Request Headers

todo ...

## Registering Server Functions

Dioxus Fullstack automatically registers all server functions for you automatically. This means you can quickly build your backend without needing to explicitly wire up endpoints to a central router.

By default, this is done when you call `dioxus::launch`. If you wish to customize the underyling Axum router, you can instead use `dioxus::serve` which lets you manually construct the router.

For simplicity, you can use the `dioxus::server::router` function to create the very same Axum router that `dioxus::launch` initializes:

```rust
fn main() {
	#[cfg(feature = "server")]
	dioxus::serve(|| async move {
		// We create a new axum router with server functions and SSR automatically wired up
		let mut router = dioxus::server::router(app);

		// We can customize the router however we want with layers and nested routers, etc
		// ...

		// And then return the router
		Ok(router)
	});

	#[cfg(not(feature = "server"))]
	dioxus::launch(app);
}
```

Note how we use Rust's built-in `#[cfg]` macro to conditionally launch the app based on the `server` feature. When `server` feature is enabled, we enable `dioxus::serve`, and when it is disabled, we enable `dioxus::launch`.

Within the `dioxus::serve` closure, we can run setup for our app and customize the axum router. For example, we could add new axum routes to the router:

```rust
#[cfg(feature = "server")]
dioxus::serve(|| async move {
	use dioxus::server::axum::routing::{get, post};

	Ok(dioxus::server::router(app)
		.route("/submit", post(|| async { "Form submitted!" }))
		.route("/about", get(|| async { "About us" }))
		.route("/contact", get(|| async { "Contact us" })))
});
```

The `dioxus::server::router` function creates a new axum router that sets up a few imporant pieces:

- Static Assets: automatically server the `public` directory, index.html and assets
- SSR: automatically run the app, render it to HTML, and serialize data for hydration
- Server Functions: automatically initialize the API endpoints

Dioxus uses extension methods on the Axum router (given by `DioxusRouterExt`):

```rust
axum::Router::new()
	.register_server_functions()
	.serve_static_assets()
	.fallback(
		get(render_handler).with_state(RenderHandleState::new(cfg, app)),
	)
```

The `.register_server_functions()` method iterates through all global server functions and then registers them on the given router. You can manually iterate through the list of global server functions and register single endpoints, or create new routers with a subset of routes with `ServerFunction::collect()`:


```rust
// We can iterate through all server functions:
for func in ServerFunction::collect() {
	// Read their data
	tracing::info!(
		"Registering server function: {} {}",
		func.method(),
		func.path()
	);

	// And add them to our router
	router = func.register_server_fn_on_router(router);
}
```
