# Dioxus Fullstack

Almost all apps need a remote server to store and update user data. Dioxus provides a number of fullstack utilities for building your app's server alongisde the client. With Dioxus Fullstack, you can build *both* your app's frontend and backend entirely in Rust!

Dioxus Fullstack deeply integrates with the popular [Axum](https://docs.rs/axum/latest/axum/) framework, making it easy to quickly add complex fullstack functionality to your app, including:

- **Server-Side-Rendering**: Render HTML on the server and hydrate it on the client
- **Server Functions**: Type-safe Axum HTTP endpoints directly callable from the client
- **Ergonomic Typed Routing**: Easily extract queries and paths from the URL
- **Multi-part Forms**: Capture multipart form data from the client into type Rust structs
- **Binary Streams**: Easily add file upload/download backend capability
- **SSE and WebSockets**: Complex, stateful datatypes for server communication
- **Asset Management**: Automatically optimizes assets for deployment to CDNs
- **WASM Support**: Deploy to WASM-based providers like Cloudflare Workers
- **Hot-Reload**: Rapid Rust hot-reload during development powered by [subsecond](https://crates.io/crates/subsecond)

Currently, Dioxus Fullstack does not provide built-in utilities for things like Databases, Caches, Sessions, and Mailers. Our current focus is to finish polishing the fullstack integration before branching out into a more "complete" fullstack solution. You'll need to pull in 3rd-party crates like `Sqlx` and `tower-sessions` to use such features. To help, we provide a [few examples in the Dioxus GitHub repo](https://github.com/DioxusLabs/dioxus/tree/main/examples/07-fullstack) to get started.

## Hot-Reload

With Dioxus, our goal is to maximize your developer productivity. Dioxus Fullstack ships with full Rust hot-reload support built-in thanks to our hot-patch engine [subsecond](https://crates.io/crates/subsecond). Subsecond uses advanced assembly and linker techniques to allow modifying Rust functions at runtime. You can add new endpoints, pages, and logic to your app without manually rebuilding.

![Dual Serve Hot-Reload](/assets/07/dual-serve-hotreload.mp4)

Subsecond currently has a few limitations. For the best experience, we recommend only modifying code in the "tip" of your app. Note that code that runs only once will not be hot-reloadable and will require a restart of the app.

## Server functions

Dioxus Fullstack provides an easy way to communicate with the server from any client. Server functions let you define a function that always runs on the server. When you call that function from the client, Dioxus will automatically serialize the arguments, send them to the server, run the function on the server, serialize the return value, and send it back to the client.

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

// The body of the function will always run on the server so we can do server-side operations like database queries
#[get("/api/dog/:breed")]
async fn fetch_dog(breed: String) -> Result<String> {
    let image = DB.with(|f| f.execute("SELECT url FROM dogs WHERE id = ?1", &breed))?;
    Ok(image)
}
```

> In addition to this guide, you can find more examples of fullstack apps in the [examples directory](https://github.com/DioxusLabs/dioxus/tree/main/examples).

## Server Side Rendering

Dioxus web defaults to client-side rendering (CSR). When you load a client-side rendered application, the server sends an empty HTML page to the browser along with some scripts. The browser then downloads your entire application and runs it to generate the HTML for the page.

When fullstack is enabled, the server will generate the HTML for the page and send that to the client. The client will then download your application and "hydrate" the page. Hydration is the process of taking the HTML that was generated on the server and adding all of the event listeners and other things that Dioxus needs to make the page interactive. Your server and client code must render the exact same HTML for hydration to work correctly, as described in the [hydration guide](./hydration.md).

Since all of the data loading can happen on the server during the initial render and the page is visible before the wasm bundle is downloaded, fullstack applications can generally load the initial content of the page much faster than client-side rendered applications.

Lets take a look at what rendering looks like for the dioxuslabs.com website with and without fullstack enabled:

![Fullstack vs client side rendering load diagram](/assets/07/fullstack-request-lifecycle.png)

## Search engine optimization (SEO)

In addition to loading your application faster, server side rendering is especially important for applications that need to be indexed by search engines. Most search engine crawlers do not execute JavaScript, so they will not be able to see the content of a client-side rendered application. By rendering the page on the server, we can ensure that the crawlers will be able to see the content of the page. This is one of the main reasons dioxuslabs.com uses fullstack rendering:

![Fullstack vs client side rendering load diagram for crawlers](/assets/07/fullstack-crawler-request-lifecycle.png)

## Websockets and Streams

Dioxus Fullstack comes with full support for Axum, and with it, special client handlers for things like WebSockets and HTTP Streams. We provide a number of utilities like `use_websocket` to reactively manage these resources on the client.

![Fullstack websockets](/assets/07/fullstack-websockets.avif)

Our `Streaming<T>` wrapper allows you to easily send arbitrary bytes, text, JSON, and chunked file contents to and from the server.

## Assets

Because Dioxus Fullstack integrates with our build tool DX, your fullstack apps come pre-optimized for deploying onto infrastructure like content-distribution-networks (CDNs). CDNs reduce your bandwidth usage and speed up your app's time-to-first byte for maximum performance. Assets bundled with DX are hashed, letting the server infinitely cache their contents.
