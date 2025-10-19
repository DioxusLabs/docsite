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

Before Dioxus 0.6, only the `#[server]` macro was supported, limiting our ability to customize things like query parameters and path segments. As of 0.7, Dioxus supports the `get`/`post`/`put` macros with full support for Axum types.

To make a server function, simply add one of `#[get]`, `#[post]`, etc on top of your function. This function has a few restrictions - it must:

- Be an async function
- Return a `Result<T, E>`
- Take arguments that are either `Serialize + Deserialize` *or* `IntoRequest + FromRequest`
- Return a type that is either `Serialize + Deserialize` *or* `IntoResponse + FromResponse`

Dioxus uses some specialization "magic" to enable flexible input and output types, so the errors for types not satisfying these bounds might be rather unwieldly.

In essence, the non-URL inputs must either be a set of items that are obviously serializable (think strings, numbers, custom types):

```rust
#[get("/api/json-body")]
async fn json_body(a: String, b: i32, c: serde_json::Value) -> Result<()> {
	Ok(())
}
```

*or*, the inputs would be a single object that implements Axum's `FromRequest` trait and Dioxus' `IntoRequest` trait. Dioxus fullstack provides a number of built-in types that implement these types and can be used across the client and the server:

```rust
#[get("/api/stream")]
async fn stream(upgrade: Streaming<String>) -> Result<()> {
	// ....
}
```

Similarly, the output type can be either a serializable object (strings, numbers, custom strutures)

```rust
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

We can combine custom payload bodies with query and path extractors, letting us build APIs that are suitable both for our Rust frontend and any other HTTP client. This can be particularly useful if your API is consumed by your own team and external customers.

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

## Custom Inputs

Dioxus provides a number of

## Custom Outputs

## Server Extractors

## Error Type


## Registering Server Functions

> If you are targeting WASM on the server with WASI, you must call `register` on the type you passed into the server macro in your main function before starting your server to tell Dioxus about the server function. For all other targets, the server function will be registered automatically.

Let's continue building on the app we made in the [getting started](../../getting_started/index.md) guide. We will add a server function to our app that allows us to double the count on the server.

First, add serde as a dependency:

```sh
cargo add serde
```

Next, add the server function to your `main.rs`:

```rust
{{#include ../docs-router/src/doc_examples/server_function.rs}}
```

Now, build your client-side bundle with `dx build --features web` and run your server with `cargo run --features ssr`. You should see a new button that multiplies the count by 2.
