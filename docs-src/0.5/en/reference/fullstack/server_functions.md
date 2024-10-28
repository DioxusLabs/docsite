# Communicating with the server

`dioxus-fullstack` provides server functions that allow you to call an automatically generated API on the server from the client as if it were a local function.

To make a server function, simply add the `#[server(YourUniqueType)]` attribute to a function. The function must:

- Be an async function
- Have arguments and a return type that both implement serialize and deserialize (with [serde](https://serde.rs/)).
- Return a `Result` with an error type of ServerFnError

You must call `register` on the type you passed into the server macro in your main function before starting your server to tell Dioxus about the server function.

Let's continue building on the app we made in the [getting started](../../getting_started/fullstack.md) guide. We will add a server function to our app that allows us to double the count on the server.

First, add serde as a dependency:

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


This is exactly what the `use_server_future` hook allows us to do! `use_server_future` is similar to the `use_resource` hook, but it allows you to wait for a future on the server and send the result of the future down to the client.


Let's change our data fetching to use `use_server_future`:

```rust
{{#include src/doc_examples/server_data_prefetch.rs}}
```

> Notice the `?` after `use_server_future`. This is what tells Dioxus fullstack to wait for the future to resolve before continuing rendering. If you want to not wait for a specific future, you can just remove the ? and deal with the `Option` manually.

Now when you load the page, you should see `server data is Ok("Hello from the server!")`. No need to wait for the `WASM` to load or wait for the request to finish!

```inject-dioxus
SandBoxFrame {
	url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-server-future-qwpp4p?file=/src/main.rs:3,24"
}
```


## Running the client with dioxus-desktop

The project presented so far makes a web browser interact with the server, but it is also possible to make a desktop program interact with the server in a similar fashion. (The full example code is available in the [Dioxus repo](https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack/examples/axum-desktop))

First, we need to make two binary targets, one for the desktop program (the `client.rs` file), one for the server (the `server.rs` file). The client app and the server functions are written in a shared `lib.rs` file.

The desktop and server targets have slightly different build configuration to enable additional dependencies or features. 
The Cargo.toml in the full example has more information, but the main points are:
- the client.rs has to be run with the `desktop` feature, so that the optional `dioxus-desktop` dependency is included
- the server.rs has to be run with the `ssr` features; this will generate the server part of the server functions and will run our backend server.

Once you create your project, you can run the server executable with:
```bash
cargo run --bin server --features ssr
```
and the client desktop executable with:
```bash
cargo run --bin client --features desktop
```

### Client code

The client file is pretty straightforward. You only need to set the server url in the client code, so it knows where to send the network requests. Then, dioxus_desktop launches the app.

For development, the example project runs the server on `localhost:8080`. **Before you release remember to update the url to your production url.**


### Server code

In the server code, first you have to set the network address and port where the server will listen to.
```rust
{{#include src/doc_examples/server_function_desktop_client.rs:server_url}}
```

Then, you have to register the types declared in the server function macros into the server.
For example, consider this server function:
```rust
{{#include src/doc_examples/server_function_desktop_client.rs:server_function}}
```

The `GetServerData` type has to be registered in the server, which will add the corresponding route to the server.
```rust
{{#include src/doc_examples/server_function_desktop_client.rs:function_registration}}
```

Finally, the server is started and it begins responding to requests.
