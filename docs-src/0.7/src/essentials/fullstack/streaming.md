# Streaming

This guide will show you how to use out-of-order streaming in Dioxus Fullstack to make your apps interactive sooner while waiting for async data.

## What is Streaming?

The default rendering mode in dioxus fullstack waits for all [suspense boundaries](../../essentials/async/index.md#suspense-with-fullstack) to resolve before sending the entire page as HTML to the client. If you have a page with multiple chunks of async data, the server will wait for all of them to complete before rendering the page.

When streaming is enabled, the server can send chunks of HTML to the client as soon as each suspense boundary resolves. You can start interacting with a page as soon as the first part of the HTML is sent, instead of waiting for the entire page to be ready. This can lead to a much faster initial load time.

Bellow is the same [hackernews example](https://github.com/DioxusLabs/dioxus/tree/main/example-projects/fullstack-hackernews) rendered with and without streaming enabled. While both pages take the same amount of time to load all the data, the page with streaming enabled on the left shows you the data streaming in as it becomes available.

```inject-dioxus
DemoFrame {
    overflow: "hidden",
    FakePage {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-around",
            align_items: "center",
            height: "100%",
            width: "100%",
            img {
                max_height: "100%",
                max_width: "50%",
                aria_label: "Hackernews with streaming enabled",
                src: asset!("/assets/static/streaming-enabled-hackernews"),
            }
            img {
                max_height: "100%",
                max_width: "50%",
                aria_label: "Hackernews with streaming disabled",
                src: asset!("/assets/static/streaming-disabled-hackernews"),
            }
        }
    }
}
```

### SEO and No JS

When streaming is enabled, all of the contents of the page are still rendered into the html document, so search engines can still crawl and index the full content of the page. However, the content will not be visible to users unless they have JavaScript enabled. If you want to support users without JavaScript, you will need to disable streaming and use the default rendering mode.

## Enabling Streaming

You can enable streaming in the ServeConfig builder with the `enable_out_of_order_streaming` method. If you are launching your application through the `dioxus::LaunchBuilder`, you can use the `with_cfg` method to pass in a configuration that enables streaming:

```rust
{{#include ../docs-router/src/doc_examples/streaming.rs:streaming_launch}}
```

or if you are using a custom axum server, you can pass the config into `serve_dioxus_application` directly:

```rust
{{#include ../docs-router/src/doc_examples/streaming.rs:streaming_axum}}
```

## Head elements with streaming

Head elements can only be rendered in the initial HTML chunk that contains the `<head>` tag. You should include all of your `document::Link`, `document::Meta`, and `document::Title` elements in the first part of your page if possible. If you have any head elements that are not included in the first chunk, they will be rendered by the client after hydration instead, which will not be visible to any search engines or users without JavaScript enabled.

The initial chunk of HTML is send after [commit_initial_chunk](https://docs.rs/dioxus-fullstack/0.7.0-alpha.1/dioxus_fullstack/prelude/fn.commit_initial_chunk.html) is called for the first time. If you are using the router, this will happen automatically when all suspense boundaries above the router are resolved. If you are not using the router, you can call `commit_initial_chunk` manually after all of your blocking head elements have been rendered.

```rust
{{#include ../docs-router/src/doc_examples/streaming.rs:head_elements}}
```
