# Fullstack development

Dioxus fullstack helps you integrate a server into your application. Depending on your client it has two primary features:
- **Server functions**: Dioxus provides an IPC layer that lets you define functions that always run on the server. When you call these functions from the client, Dioxus will handle the request and response with the server. This allows you to easily communicate with the server in a type-safe way.
- **Server-side rendering (SSR)**: Dioxus will render your application on the server and send the generated HTML to the client. This allows for faster initial page loads and better search engine optimization (SEO).

## Enabling fullstack

To enable fullstack in your application, you need add the `"fullstack"` feature to your `dioxus` dependency in your `Cargo.toml` and create a separate feature for the client and server:

```toml
[dependencies]
dioxus = { version = "0.7.0", features = ["fullstack"] }

[features]
web = ["dioxus/web"] # This feature is enabled in the web client
server = ["dioxus/server"] # This feature is enabled in the server
```

Now when you run `dx serve`, Dioxus will build your app once for the client and once for the server:

![Server Client Split](/assets/06_docs/server_split.png)

If you have dependencies you only want to include on either the server or client, you can make those dependencies depend on the `web` or `server` feature:

```toml
[dependencies]
dioxus = { version = "0.7.0", features = ["fullstack"] }
tokio = { version = "1.0", features = ["full"], optional = true }

[features]
default = []
web = ["dioxus/web"]
server = ["dioxus/server", "dep:tokio"] # Only include tokio on the server
```

<!-- For more information on how to manage dependencies and conditional compilation in fullstack applications, see the [managing dependencies guide](./managing_dependencies.md). -->

## Server functions

Fullstack also provides an easy way to communicate with the server from any client. Server functions let you define a function that always runs on the server. When you call that function from the client, Dioxus will automatically serialize the arguments, send them to the server, run the function on the server, serialize the return value, and send it back to the client.

Server functions are described in more detail in the [server functions guide](./server_functions.md).

```rust
fn app() -> Element {
    // When we call fetch_dog, it will either run the async function directly on the server, or send
    // a request to the server to run the function from the client.
    let url = use_server_future(|| fetch_dog("poodle".to_string()));

    rsx! {
        div {
            h1 { "Dog of the day" }
            img {
                src: "{url}",
                alt: "A cute dog"
            }
        }
    }
}

#[server]
async fn fetch_dog(breed: String) -> Result<String, ServerFnError> {
    // The body of the function will always run on the server so we can do server-side operations like database queries.
    let image = DB.with(|f| f.execute("SELECT url FROM dogs WHERE id = ?1", &breed))?;
    Ok(image)
}
```

> In addition to this guide, you can find more examples of fullstack apps in the [examples directory](https://github.com/DioxusLabs/dioxus/tree/main/examples).

## Server Side Rendering

Dioxus web defaults to client-side rendering (CSR). When you load a client-side rendered application, the server sends an empty HTML page to the browser along with some scripts. The browser then downloads your entire application and runs it to generate the HTML for the page.

When fullstack is enabled, the server will generate the HTML for the page and send that to the client. The client will then download your application and "hydrate" the page. Hydration is the process of taking the HTML that was generated on the server and adding all of the event listeners and other things that Dioxus needs to make the page interactive. Your server and client code must render the exact same HTML for hydration to work correctly, as described in the [hydration guide](./ssr.md).

Since all of the data loading can happen on the server during the initial render and the page is visible before the wasm bundle is downloaded, fullstack applications can generally load the initial content of the page much faster than client-side rendered applications.

Lets take a look at what rendering looks like for the dioxuslabs.com website with and without fullstack enabled:

![Fullstack vs client side rendering load diagram](/assets/07/fullstack-request-lifecycle.png)

### Search engine optimization (SEO)

In addition to loading your application faster, server side rendering is especially important for applications that need to be indexed by search engines. Most search engine crawlers do not execute JavaScript, so they will not be able to see the content of a client-side rendered application. By rendering the page on the server, we can ensure that the crawlers will be able to see the content of the page. This is one of the main reasons dioxuslabs.com uses fullstack rendering:

![Fullstack vs client side rendering load diagram for crawlers](/assets/07/fullstack-crawler-request-lifecycle.png)
<!--
## Table of Contents

This guide is covers two main topics:

Server integration with server functions for any client
- [Server Functions](./server_functions.md): How to use server functions to communicate with the server in a type-safe way.
- [Managing Dependencies](./managing_dependencies.md): How to include server or client specific dependencies in your fullstack application.
- [Extractors](./extractors.md): Using extractors to access request data in server functions.
- [Middleware](./middleware.md): Wrapping server functions with middleware for additional functionality.
- [Authentication](./authentication.md): Securing your fullstack application with authentication.
- [Axum Integration](./axum.md): Integrating Dioxus fullstack with your existing Axum server.

And server side rendering with a web client
- [Hydration](./hydration.md): Understanding the process of hydration for fullstack web applications.
- [Routing](./routing.md): Integrating the dioxus router with your fullstack application.
- [Streaming](./streaming.md): Starting rendering faster with streaming.
- [Static Site Generation](./static_site_generation.md): Generating static sites with Dioxus. -->
