# Choosing a web renderer

Dioxus has three different renderers that target the web:

- [dioxus-web](../getting_started/web.md) allows you to render your application to HTML with WASM on the client
- [dioxus-liveview](../getting_started/liveview.md) allows you to render HTML generated on the server on the client with a websocket
- [dioxus-fullstack](../getting_started/ssr.md) allows you to initially render static HTML on the server and then update that html from the client with WASM

Each approach has its tradeoffs:

### Client-side (dioxus-web)

- With Client side rendering, you send the entire content of your application to the client, and then the client generates all of the HTML of the page dynamically.

- This means that the page will be blank until the JavaScript bundle has loaded and the application has initialized. This can result in **slower first render times and makes the page less SEO-friendly**.

> SEO stands for Search Engine Optimization. It refers to the practice of making your website more likely to appear in search engine results. Search engines like Google and Bing use web crawlers to index the content of websites. Most of these crawlers are not able to run JavaScript, so they will not be able to index the content of your page if it is rendered client-side.

- Client-side rendered applications need to use **weakly typed requests to communicate with the server**

> Client-side rendering is a good starting point for most applications. It is well supported and makes it easy to communicate with the client/browser APIs

### Liveview (dioxus-liveview)

- Liveview rendering communicates with the server over a WebSocket connection. It essentially moves all of the work that Client-side rendering does to the server.

- This makes it **easy to communicate with the server, but more difficult to communicate with the client/browser APIS**.

- Each interaction also requires a message to be sent to the server and back which can cause **issues with latency**.

- Because Liveview uses a websocket to render, the page will be blank until the WebSocket connection has been established and the first renderer has been sent form the websocket. Just like with client side rendering, this can make the page **less SEO-friendly**.

- Because the page is rendered on the server and the page is sent to the client piece by piece, you never need to send the entire application to the client. The initial load time can be faster than client-side rendering with large applications because Liveview only needs to send a constant small websocket script regardless of the size of the application.

> Liveview is a good fit for applications that already need to communicate with the server frequently (like real time collaborative apps), but don't need to communicate with as many client/browser APIs

### Fullstack (dioxus-fullstack)

Fullstack rendering happens in two parts:
1. The page is rendered on the server. This can include fetching any data you need to render the page.
2. The page is hydrated on the client. (Hydration is just taking the page from the server and adding all of the event listeners Dioxus needs on the client). Any updates to the page happen on the client after this point.

Because the page is initially rendered on the server, the page will be fully rendered when it is sent to the client. This results in a faster first render time and makes the page more SEO-friendly.

- **Fast initial render**
- **Works well with SEO**
- **Type safe easy communication with the server**
- **Access to the client/browser APIs**
- **Fast interactivity**

Finally, we can use [server functions](server_functions.md) to communicate with the server in a type-safe way.

This approach uses both the dioxus-web and dioxus-ssr crates. To integrate those two packages and `axum`, `warp`, or `salvo`, Dioxus provides the `dioxus-fullstack` crate.

There can be more complexity with fullstack applications because your code runs in two different places. Dioxus trys to midigate this with server functions and other helpers.

