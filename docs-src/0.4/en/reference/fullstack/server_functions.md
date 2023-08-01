# Communicating with the server

`dioxus-fullstack` provides server functions that allow you to call an automatically generated API on the server from the client as if it were a local function.

To make a server function, simply add the `#[server(YourUniqueType)]` attribute to a function. The function must:

- Be an async function
- Have arguments and a return type that both implement serialize and deserialize (with [serde](https://serde.rs/)).
- Return a `Result` with an error type of ServerFnError

You must call `register` on the type you passed into the server macro in your main function before starting your server to tell Dioxus about the server function.

Let's continue building on the app we made in the [getting started](../../../getting_startedw/web/fullstack.md) guide. We will add a server function to our app that allows us to double the count on the server.

First, add serde as a dependancy:

```shell
cargo add serde
```

Next, add the server function to your `main.rs`:

```rust
{{#include src/doc_examples/server_function.rs}}
```

Now, build your client-side bundle with `dx build --features web` and run your server with `cargo run --features ssr`. You should see a new button that multiplies the count by 2.

## Cached data fetching

One common use case for server functions is fetching data from the server:

```rust
{{#include src/doc_examples/server_data_fetch.rs}}
```

If you navigate to the site above, you will first see `server data is None`, then after the `WASM` has loaded and the request to the server has finished, you will see `server data is Some(Ok("Hello from the server!"))`.


This approach works, but it can be slow. Instead of waiting for the client to load and send a request to the server, what if we could get all of the data we needed for the page on the server and send it down to the client with the initial HTML page?


This is exactly what the `use_server_future` hook allows us to do! `use_server_future` is similar to the `use_future` hook, but it allows you to wait for a future on the server and send the result of the future down to the client.


Let's change our data fetching to use `use_server_future`:

```rust
{{#include src/doc_examples/server_data_prefetch.rs}}
```

> Notice the `?` after `use_server_future`. This is what tells Dioxus fullstack to wait for the future to resolve before continuing rendering. If you want to not wait for a specific future, you can just remove the ? and deal with the `Option` manually.

Now when you load the page, you should see `server data is Ok("Hello from the server!")`. No need to wait for the `WASM` to load or wait for the request to finish!

```inject-dioxus
SandBoxFrame {
	url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-server-future-64vk22?file=%2Fsrc%2Fmain.rs%3A6%2C1"
}
```
