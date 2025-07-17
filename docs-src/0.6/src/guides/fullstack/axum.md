# Axum integration

Dioxus fullstack is built on top of Axum under the hood. The launch function will set up a default Axum server for your fullstack project, but if you need more control you can drop down to a custom axum server. This guide will show you how to set up an Axum server with your Dioxus fullstack project.

## Adding axum to your project

First, we need to add axum and tokio as dependencies. Since we only need these dependencies on the server, we need to make them optional and enable them in the server feature flag. More information about server only dependencies can be found in the [dependencies guide](./managing_dependencies.md#adding-server-only-dependencies):

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

Next we to split up our main function into two parts: one that will start dioxus on the client with the launch function, and one that will serve our Axum router. We can use `#[cfg]` attribute to control what code compiles when the web feature or server feature is enabled:

```rust
{{#include ../docs-router/src/doc_examples/untested_06/axum_integration.rs:main}}
```

## Starting the axum server

Now we can setup our axum server in the launch_server function. Since this code should only compile when the server is enabled, we need to gate it behind a `#[cfg(feature = "server")]` attribute again.


When we serve our app the CLI will try to proxy the backend under a random port. The proxy adds devtool endpoints for hot reloading and logging to make it easier to develop your app. We need to look for the port the CLI gives us in dev mode with `server_ip` and `server_port`. If we don't find a port from the CLI, we can serve at our own ip and port for production.


Finally, when building our Axum server, we need to call `serve_dioxus_application` on the router to add all the routes Dioxus needs to serve your app.

```rust
{{#include ../docs-router/src/doc_examples/untested_06/axum_integration.rs:server}}
```

## Running your application

Now we can run our application with the CLI. The CLI will automatically detect the server feature (that enables `dioxus/server`) and the client feature (which enables `dioxus/web`, `dioxus/desktop` or `dioxus/mobile`) and build once for each platform. To run the application, we can use the following command:

```bash
dx serve
```

## Conclusion

This guide showed you how to set up a custom Axum server with your Dioxus fullstack project. You can now use all of the features of Axum in your Dioxus app, including middleware, routing, and more. If you need even more granular control over your router, you can split up the `serve_dioxus_application` function into [`serve_static_assets`](https://docs.rs/dioxus-fullstack/0.6.3/dioxus_fullstack/server/trait.DioxusRouterExt.html#tymethod.serve_static_assets), [`register_server_functions`](https://docs.rs/dioxus-fullstack/0.6.3/dioxus_fullstack/server/trait.DioxusRouterExt.html#method.register_server_functions), and [`render_handler`](https://docs.rs/dioxus-fullstack/0.6.3/dioxus_fullstack/server/fn.render_handler.html). See the documentation on each method for more details


If you want to learn more about Axum, check out the [Axum documentation](https://docs.rs/axum/latest/axum/). Axum is built on top of the tower ecosystem which means you can use any tower service with your Axum server. The [tower-http](https://docs.rs/tower-http/latest/tower_http/) crate contains many useful utilities for your server like logging, compression, and file serving.
