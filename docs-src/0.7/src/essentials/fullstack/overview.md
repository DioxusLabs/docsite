# Fullstack development

- Fullstack rendering model
- Fullstack IPC model
- Integrating fullstack with your existing server

Fullstack provides an extra layer on top of your client-side web, desktop or mobile application that makes it easy to interact with the server.

## Enabling fullstack

To enable fullstack in your application, you need add the `"fullstack"` feature to your `dioxus` dependency in your `Cargo.toml` and create a separate feature for the client and server:

```toml
[dependencies]
dioxus = { version = "0.7.0", features = ["fullstack"] }

[features]
client = ["dioxus/web"]
server = ["dioxus/server"]
```

Now when you run `dx serve`, Dioxus will build your app twice: once for the client and once for the server:

![Server Client Split](/assets/06_docs/server_split.png)

If you have dependencies you only want to include on either the server or client, you can make those dependencies conditional on the `client` or `server` feature:

```toml
[dependencies]
dioxus = { version = "0.7.0", features = ["fullstack"] }
tokio = { version = "1.0", features = ["full"], optional = true }

[features]
default = []
client = ["dioxus/web"]
server = ["dioxus/server", "dep:tokio"]
```

For more information on how to manage dependencies and conditional compilation in fullstack applications, see the [managing dependencies guide](./managing_dependencies.md).

## Fullstack rendering model

Dioxus web defaults to client-side rendering (CSR). When you load a client-side rendered application, the server sends a mostly empty HTML page to the client. The client then downloads your entire application and runs it to generate the HTML for the page.

When fullstack is enabled, dioxus will first render the page on the server. The server will generate the HTML for the page and send that to the client. The client will then download your application and "hydrate" the page. Hydration is the process of taking the HTML that was generated on the server and adding all of the event listeners and other things that Dioxus needs to make the page interactive.

Hydration requires some additional guarantees about your application described in the [hydration guide](./hydration.md). Your server and client code must render the exact same HTML for hydration to work correctly.

Since all of the data loading can happen on the server during the initial render and the page is visible before the wasm bundle is downloaded, fullstack applications generally have a faster time to first render.

Lets take a look at what rendering looks like for the dioxuslabs.com website with and without fullstack enabled:

![Fullstack vs client side rendering load diagram](/assets/07/fullstack-request-lifecycle.png)

### Search engine optimization (SEO)

Server side rendering with fullstack is especially important for applications that need to be indexed by search engines. Most search engine crawlers do not execute JavaScript, so they will not be able to see the content of a client-side rendered application. By rendering the page on the server, we can ensure that the crawlers will be able to see the content of the page. This is one of the main reasons dioxuslabs.com uses fullstack rendering:

![Fullstack vs client side rendering load diagram for crawlers](/assets/07/fullstack-crawler-request-lifecycle.png)

## Fullstack IPC model

In addition to rendering optimizations, fullstack also provides an easy way to communicate with the server from any client. Server functions let you define a function that always runs on the server. When you call that function from the client, Dioxus will automatically serialize the arguments, send them to the server, run the function on the server, serialize the return value, and send it back to the client.

Server functions are described in more detail in the [server functions guide](./server_functions.md).

> In addition to this guide, you can find more examples of full-stack apps and information about how to integrate with other frameworks and desktop renderers in the [examples directory](https://github.com/DioxusLabs/dioxus/tree/main/examples).

## Table of Contents

This section contains guides and information about fullstack development with Dioxus:
- [Server Functions](essentials/fullstack/server_functions.md): How to use server functions to communicate with the server in a type-safe way.
- [Hydration](essentials/fullstack/hydration.md): Understanding the process of hydration for fullstack web applications.
- [Managing Dependencies](essentials/fullstack/managing_dependencies.md): How to include server or client specific dependencies in your fullstack application.
- [Extractors](essentials/fullstack/extractors.md): Using extractors to access request data in server functions.
- [Middleware](essentials/fullstack/middleware.md): Wrapping server functions with middleware for additional functionality.
- [Authentication](essentials/fullstack/authentication.md): Securing your fullstack application with authentication.
- [Routing](essentials/fullstack/routing.md): Integrating the dioxus router with your fullstack application.
- [Streaming](essentials/fullstack/streaming.md): Starting rendering faster with streaming.
- [Static Site Generation](essentials/fullstack/static_site_generation.md): Generating static sites with Dioxus.
- [Axum Integration](essentials/fullstack/axum.md): Integrating Dioxus fullstack with your existing Axum server.
