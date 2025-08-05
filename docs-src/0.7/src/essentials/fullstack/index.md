# Fullstack development

- Fullstack rendering model
- Fullstack IPC model
- Integrating fullstack with your existing server

Fullstack provides an extra layer on top of your client-side web, desktop or mobile application that makes it easy to interact with the server.

## Fullstack rendering model

Dioxus web defaults to client-side rendering (CSR). When you load a client-side rendered application, the server sends a mostly empty HTML page to the client. The client then downloads your entire application and runs it to generate the HTML for the page.

When fullstack is enabled, dioxus will first render the page on the server. The server will generate the HTML for the page and send that to the client. The client will then download your application and "hydrate" the page. Hydration is the process of taking the HTML that was generated on the server and adding all of the event listeners and other things that Dioxus needs to make the page interactive.

Hydration requires some additional guarantees about your application described in the [hydration guide](../reference/fullstack/hydration.md). Your server and client code must render the exact same HTML for hydration to work correctly.

Since all of the data loading can happen on the server during the initial render and the page is visible before the wasm bundle is downloaded, fullstack applications generally have a faster time to first render.

Lets take a look at what rendering looks like for the dioxuslabs.com website with and without fullstack enabled:

![Fullstack vs client side rendering load diagram](/assets/07/fullstack.png)

### Search engine optimization (SEO)

Server side rendering with fullstack is especially important for applications that need to be indexed by search engines. Most search engine crawlers do not execute JavaScript, so they will not be able to see the content of a client-side rendered application. By rendering the page on the server, we can ensure that the crawlers will be able to see the content of the page. This is one of the main reasons dioxuslabs.com uses fullstack rendering:

![Fullstack vs client side rendering load diagram for crawlers](/assets/07/fullstack.png)

## Fullstack IPC model

In addition to rendering optimizations, fullstack also provides an ergonomic way to communicate with the server from any client. Server functions let you define a function that always runs on the server. When you call that function from the client, Dioxus will automatically serialize the arguments, send them to the server, run the function on the server, serialize the return value, and send it back to the client.

Server functions are described in more detail in the [server functions guide](../reference/fullstack/server_functions.md).

> In addition to this guide, you can find more examples of full-stack apps and information about how to integrate with other frameworks and desktop renderers in the [examples directory](https://github.com/DioxusLabs/dioxus/tree/main/examples).
