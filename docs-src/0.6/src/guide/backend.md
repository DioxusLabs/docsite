# Adding a Backend

Dioxus is a *fullstack* framework, meaning it allows you to seamlessly build your frontend alongside your backend.

We provide a number of utilities like *Server Functions*, *Server Futures*, and *Server State* for you to integrate into your apps. In this chapter, we'll cover loading and saving state to our backend with *Server Functions*. For an in-depth guide on fullstack, check out the dedicated [Fullstack Guide](../guides/fullstack/index.md).

## Enabling Fullstack

Before we can start using server functions, we need to enable the "fullstack" feature on Dioxus in our Cargo.toml.

```toml
[dependencies]
dioxus = { version = "0.6.0", features = ["fullstack"] }
```

We also need to add the "server" feature to our app's features, also in the Cargo.toml

```toml
[features]
default = ["web"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server"] # <----- add this additional platform
```

If you selected *yes* to the "use fullstack?" prompt when creating your app, you will already have this set up!

> 📣 Unfortunately, `dx` doesn't know how to hot-reload this change, so we'll need to kill our currently running `dx serve` process and launch it again.

Give your app a moment to build again and make sure that the "fullstack" feature is enabled in `dx serve`.

![Fullstack Enabled](/assets/06_docs/serve_with_fullstack.png)

## Server Functions: an inline RPC system

Dioxus integrates with the [server_fn](https://crates.io/crates/server_fn) crate to provide a simple inline communication system for your apps. The server_fn crate makes it easy to build your app's backend with just basic Rust functions. Server Functions are `async` functions annotated with the `#[server]` attribute.

A typical server function looks like this:

```rust
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    Ok(())
}
```

Every server function is an async function that takes some parameters and returns a `Result<(), ServerFnError>`. Whenever the client calls the server function, it sends an HTTP request to a corresponding endpoint on the server. The parameters of the server function are serialized as the body of the HTTP request. As a result, each argument must be serializable.

On the client, the server function expands to a `reqwest` call:

```rust
// on the client:
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    reqwest::Client::new()
        .post("http://localhost:8080/api/save_dog")
        .json(&image)
        .send()
        .await
}
```

On the server, the server function expands to an [axum](https://github.com/tokio-rs/axum) handler:

```rust
// on the server:
struct SaveDogArgs {
    image: String,
}

async fn save_dog(Json(args): Json<SaveDogArgs>) -> Result<(), ServerFnError> {
    Ok(())
}
```

When `dioxus::launch` is called, the server functions are automatically registered for you and set up as an Axum router.

```rust
async fn launch(config: Config, app: Component<()>) {
    // register server functions
    let router = axum::Router::new().serve_dioxus_application(config, app);

    // start server
    axum::serve(listener, router).await
}
```

As of Dioxus 0.6, we only support the `axum` server framework. We plan to build additional server features in the future and  only support `axum` to ship faster.

In some cases, the `dioxus::launch` function might be too limiting for your use-case on the server. You can easily drop down to axum by changing your main.rs. The `dioxus::launch` function also handles setting up logging and reading environment variables, which you will have to handle yourself.

```rust
fn main() {
    if cfg!(feature = "server") {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(launch_server);
    } else {
        dioxus::launch(app);
    }
}

#[cfg(feature = "server")]
async fn launch_server() {
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application("", ServeConfigBuilder::new(app, ()))
        .into_make_service();

    // And launch it!
    axum::Server::bind(&addr).serve(router).await
}
```

## The Client/Server split

When Dioxus builds your fullstack apps, it actually creates two separate applications: the server and the client. To achieve this, `dx` passes different features to each build.

- The client is built with `--feature web`
- The server is built with `--feature server`

![Server Client Split](/assets/06_docs/server_split.png)

When embedding server code in our apps, we need to be careful about which code gets compiled. The body of the server function is designed to only be *executed on the server* - not the client. Any code configured by the `"server"` feature will not be present in the final app. Conversely, any code not configured by the `"server"` feature *will* make it into the final app.

```rust
// ❌ this will leak your DB_PASSWORD to your client app!
static DB_PASSWORD: &str = "1234";

fn app() -> Element { }

#[server]
async fn DoThing() -> Result<(), ServerFnError> {
    db.connect(DB_PASSWORD).await
    // ...
}
```

Instead, we recommend placing server-only code within modules configured for the `"server"` feature.

```rust
// ✅ code in this module can only be accessed on the server
#[cfg(feature = "server")]
mod server_utils {
    pub static DB_PASSWORD: &str = "1234";
}
```

While Dioxus expects a "server" feature, it does not expect a "client" feature. It is assumed that all client code will make it to the server. However, some libraries like web-sys only work when running in the browser, so make sure to not run specific client code in your server functions or before your `launch`.

```rust
#[server]
async fn DoThing() -> Result<(), ServerFnError> {
    // ❌ attempting to use web_sys on the server will panic!
    let document = web_sys::document();

    // ..
}
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
// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // And then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
```

### Calling the server function

Now, in our client code, we can actually call the server function.

```rust
fn DogView() -> Element {
    let mut img_src = use_resource(/**/);

    // ...
    rsx! {
        // ...
        div { id: "buttons",
            // ...
            button {
                id: "save",
                onclick: move |_| async move {
                    // Clone the current image
                    let current = img_src.cloned().unwrap();

                    // Start fetching a new image
                    img_src.restart();

                    // And call the `save_dog` server function
                    save_dog(current).await;
                },

                "save!"
            }
        }
    }
}
```

Wow, our app is really coming together!

![Working Server Functions](/assets/06_docs/dog-save-serverfn.mp4)

Server functions are extremely capable and can even be used during server-side-rendering. Check out the complete [fullstack guide](../guides/fullstack/) for more information.
