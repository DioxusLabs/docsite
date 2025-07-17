# Adding a Backend

Dioxus is a _fullstack_ framework, meaning it allows you to seamlessly build your frontend alongside your backend.

We provide a number of utilities like _Server Functions_, _Server Futures_, and _Server State_ for you to integrate into your apps. In this chapter, we'll cover loading and saving state to our backend with _Server Functions_. For an in-depth guide on fullstack, check out the dedicated [Fullstack Guide](../guides/fullstack/index.md).

## Enabling Fullstack

Before we can start using server functions, we need to enable the "fullstack" feature on Dioxus in our Cargo.toml.

```toml
[dependencies]
dioxus = { version = "0.6.0", features = ["fullstack"] }
```

We also need to add the "server" feature to our app's features in the Cargo.toml and remove the default web target.

```toml
[features]
default = [] # <----- remove the default web target
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"] # <----- add this additional target
```

If you selected _yes_ to the "use fullstack?" prompt when creating your app, you will already have this set up!

> 📣 Unfortunately, `dx` doesn't know how to hot-reload this change, so we'll need to kill our currently running `dx serve` process and launch it again.

Now instead of running `dx serve`, you need to run with a manual platform with `dx serve --platform web`. Give your app a moment to build again and make sure that the "fullstack" feature is enabled in the dashboard.

![Fullstack Enabled](/assets/06_docs/serve_with_fullstack.png)

## Server Functions: an inline RPC system

Dioxus integrates with the [server_fn](https://crates.io/crates/server_fn) crate to provide a simple inline communication system for your apps. The server_fn crate makes it easy to build your app's backend with just basic Rust functions. Server Functions are `async` functions annotated with the `#[server]` attribute.

A typical server function looks like this:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_v1}}
```

Every server function is an async function that takes some parameters and returns a `Result<(), ServerFnError>`. Whenever the client calls the server function, it sends an HTTP request to a corresponding endpoint on the server. The parameters of the server function are serialized as the body of the HTTP request. As a result, each argument must be serializable.

On the client, the server function expands to a `reqwest` call:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_client}}
```

On the server, the server function expands to an [axum](https://github.com/tokio-rs/axum) handler:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_server}}
```

When `dioxus::launch` is called, the server functions are automatically registered for you and set up as an Axum router.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_launch}}
```

As of Dioxus 0.6, we only support the `axum` server framework. We plan to build additional server features in the future and only support `axum` to ship faster.

In some cases, the `dioxus::launch` function might be too limiting for your use-case on the server. You can easily drop down to axum by changing your main.rs. The `dioxus::launch` function also handles setting up logging and reading environment variables, which you will have to handle yourself.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:separate_server_launch}}
```

## The Client/Server split

When Dioxus builds your fullstack apps, it actually creates two separate applications: the server and the client. To achieve this, `dx` passes different features to each build.

- The client is built with `--feature web`
- The server is built with `--feature server`

![Server Client Split](/assets/06_docs/server_split.png)

When embedding server code in our apps, we need to be careful about which code gets compiled. The body of the server function is designed to only be _executed on the server_ - not the client. Any code configured by the `"server"` feature will not be present in the final app. Conversely, any code not configured by the `"server"` feature _will_ make it into the final app.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:server_client_split_broken}}
```

Instead, we recommend placing server-only code within modules configured for the `"server"` feature.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:server_client_split_fixed}}
```

In addition to the "server" feature, Dioxus expects a client side rendering feature like "web" or "desktop". Some libraries like web-sys only work when running in the browser, so make sure to not run specific client code in your server functions or before your `launch`. You can place client only code under a config for a client target feature like "web".

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:server_client_split_broken_client_broken}}
```

## Managing Dependencies

Some dependencies like [Tokio](https://github.com/tokio-rs/tokio) only compile properly when targeting native platforms. Other dependencies like [jni-sys](https://github.com/jni-rs/jni-sys) only work properly when running on a specific platform. In these cases, you'll want to make sure that these dependencies are only compiled when a particular feature is enabled. To do this, we can use Rust's `optional` [flag on dependencies in our Cargo.toml](https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies).

```toml
[dependencies]
tokio = { version = "1", optional = true }

[features]
default = []
server = ["dep:tokio"]
```

Eventually, if your project becomes large enough, you might want to pull your server functions out into their own crate to be used across different apps. We'd create a `server` crate in our workspace:

```sh
├── Cargo.toml
└── crates
    ├── dashboard
    ├── marketplace
    └── server
```

And then we'd import the server functions in our app, disabling their `"server"` feature.

```toml
[dependencies]
server = { workspace = true, default-features = false }
```

We provide a longer guide about the details of managing dependencies across the server and the client [here](../guides/fullstack/managing_dependencies.md).

## Our HotDog Server Function

Revisiting our HotDog app, let's create a new server function that saves our favorite dog to a file called `dogs.txt`. In production, you'd want to use a proper database as covered in [the next chapter](databases.md), but for now we'll use a simple file to test things out.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_v2}}
```

### Calling the server function

Now, in our client code, we can actually call the server function.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/guide_backend.rs:save_dog_call}}
```

Wow, our app is really coming together!

![Working Server Functions](/assets/06_docs/dog-save-serverfn.mp4)

Server functions are extremely capable and can even be used during server-side-rendering. Check out the complete [fullstack guide](../guides/fullstack/index.md) for more information.
