# Axum integration

The launch function will set up a default Axum server for your fullstack project, but if you need more control over the server you can drop down to a custom axum server.

## Adding axum to your project

First, we need to set up axum and tokio as dependencies. Since we only need these dependencies on the server, we need to make them optional and enable them in the server feature flag. More information about server only dependencies can be found in the [dependencies guide](./managing_dependencies.md#adding-server-only-dependencies):

```toml
[dependencies]
dioxus = { version = "0.6.0", features = ["fullstack"] }
# Axum is optional because we only use it on the server
axum = { version = "0.7", optional = true }
tokio = { version = "1.0", features = ["full"], optional = true }

[features]
# ...
web = ["dioxus/web"]
# The server feature enables the axum dependency
server = ["dioxus/server", "dep:axum", "dep:tokio"]
```

## Splitting up the main function

Next we to split up our main function into two parts: one that will start dioxus on the client with the launch function, and one that will start our axum server. We can use `#[cfg]` attribute to control what code compiles when the web feature or server feature is enabled. 

```rust
{{#include src/doc_examples/axum_integration.rs:main}}
```

## Starting the axum server

Now we can setup our axum server in the launch_server function. Since this code should only compile when the server is enabled, we still need to gate it behind a `#[cfg(feature = "server")]` attribute.


When we serve our app the CLI proxies the backend to add devtool endpoints for hot reloading and logging. We need to look for the port the server should run under with `server_ip` and `server_port` to make our new axum server compatible with the CLI.


Finally, when building our axum server, we need to call `serve_dioxus_application` on the router to add all the routes Dioxus needs to serve your app.


```rust
{{#include src/doc_examples/axum_integration.rs:server}}
```

## Running your application

Now we can run our application with the CLI. The CLI will automatically detect the server feature (that enables `dioxus/server`) and the client feature (which enables `dioxus/web`, `dioxus/desktop` or `dioxus/mobile`) and build once for each platform. To run the application, we can use the following command:

```bash
dx serve
```
