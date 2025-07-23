# Desktop

This guide will cover concepts specific to the Dioxus desktop renderer.

## Running Javascript

Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose.


For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:

```rust
{{#include ../docs-router/src/doc_examples/untested_04/eval.rs}}
```

## Custom Assets

You can link to local assets in dioxus desktop instead of using a url:

```rust
{{#include ../docs-router/src/doc_examples/untested_04/custom_assets.rs}}
```

## Integrating with Wry

In cases where you need more low level control over your window, you can use wry APIs exposed through the [Desktop Config](https://docs.rs/dioxus-desktop/0.3.0/dioxus_desktop/struct.Config.html) and the [use_window hook](https://docs.rs/dioxus-desktop/0.4.0/dioxus_desktop/fn.use_window.html)
