# Streaming

For some sites, it is extremely important to optimize "time-to-first-byte". Users want to see results as soon as possible, even if not *all* results are ready immediately.

Dioxus supports this usecase with a technology called *"HTML Streaming"*. HTML streaming allows you to quickly send an initial skeleton of the page to the user and then fill in various components as their data loads.

## What is Streaming?

The default rendering mode in dioxus fullstack waits for all [suspense boundaries](../basics/suspense.md#suspense-with-fullstack) to resolve before sending the entire page as HTML to the client. If you have a page with multiple chunks of async data, the server will wait for all of them to complete before rendering the page.

When streaming is enabled, the server can send chunks of HTML to the client as soon as each suspense boundary resolves. You can start interacting with a page as soon as the first part of the HTML is sent, instead of waiting for the entire page to be ready. This can lead to a much faster initial load time.

Bellow is the same [hackernews example](https://github.com/DioxusLabs/dioxus/tree/main/examples/01-app-demos/hackernews) rendered with and without streaming enabled. While both pages take the same amount of time to load all the data, the page with streaming enabled on the left shows you the data as soon as it becomes available.

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

## SEO and No JS

When streaming is enabled, all of the contents of the page are still rendered into the html document, so search engines can still crawl and index the full content of the page. However, the content will not be visible to users unless they have JavaScript enabled. If you want to support users without JavaScript, you will need to disable streaming and use the default rendering mode.

## Do You Need Streaming?

HTML streaming is best suited for apps like e-commerce sites where much of the data is quick to render (the product image, description, etc) but some data takes much longer to resolve. In these cases, you don't want to make the user wait too long for the page to load, so you send down what you have as soon as possible.

Streaming adds some slight overhead and complexity to your app, so it's disabled by default.

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

## Response Headers with Streaming

When rendering an app with streaming enabled, Dioxus will wait for the app to commit its initial skeleton before sending a response to the user's request. This is done with the `commit_initial_chunk()` method.

Once the initial chunk is committed, you can no longer change the headers of the response nor change the HTTP status.

For example, you might have a server function that throws a 404 status code:

```rust
#[get("/api/post/{id}")]
async fn get_post(id: u32) -> Result<String, HttpError> {
    match id {
        1 => Ok("first post".to_string()),
        _ => HttpError::not_found("Post not found")?,
    }
}
```

With streaming disabled, if this status code is bubbled to the root component as an error, the user will get a `404 NOT FOUND` status in the response.

```rust
#[component]
fn Post(id: ReadSignal<u32>) -> Element {
    // If `get_post` returns a 404, then the user will also get a 404
    let post_data = use_loader(move || get_post(id()))?;

    rsx! {
        h1 { "Post {id}" }
        p { "{post_data}" }
    }
}
```

However, when streaming is *enabled*, the status code from this server function will only be propagated to the user *before* the call to `commit_initial_chunk()`.

Normally, you won't call `commit_initial_chunk()` yourself since the `Router` component calls it for you once the root suspense boundary is resolved.

This means that, when suspense is enabled, server functions won't set the HTTP status code if they are called from within a dedicated suspense boundary:

```rust
fn Home() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { "loading..." },

            // Errors here won't propagate to the response headers
            Post { id: 123 }
        }
    }
}
```

