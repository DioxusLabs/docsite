# Desktop

This guide will cover concepts specific to the Dioxus desktop renderer.

Apps built with Dioxus desktop use the system WebView to render the page. This makes the final size of application much smaller than other WebView renderers (typically under 5MB).

Although desktop apps are rendered in a WebView, your Rust code runs natively. This means that browser APIs are _not_ available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs _are_ accessible, so streaming, WebSockets, filesystem, etc are all easily accessible though system APIs.

Dioxus desktop is built off [Tauri](https://tauri.app/). Right now there are limited Dioxus abstractions over the menubar, event handling, etc. In some places you may need to leverage Tauri directly – through [Wry](http://github.com/tauri-apps/wry/) and [Tao](http://github.com/tauri-apps/tao).

> In the future, we plan to move to a custom web renderer-based DOM renderer with WGPU integrations ([Blitz](https://github.com/DioxusLabs/blitz)).

## Examples

- [File Explorer](https://github.com/DioxusLabs/dioxus/blob/main/examples/file_explorer.rs)
- [Tailwind App](https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind)

[![Tailwind App screenshot](./public/static/tailwind_desktop_app.png)](https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind)

## Running Javascript

Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose. 


For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:

```rust
{{#include src/doc_examples/eval.rs}}
```

## Custom Assets

You can link to local assets in dioxus desktop instead of using a url:

```rust
{{#include src/doc_examples/custom_assets.rs}}
```

You can read more about assets in the [assets](./assets.md) reference.

## Integrating with Wry

In cases where you need more low level control over your window, you can use wry APIs exposed through the [Desktop Config](https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/struct.Config.html) and the [use_window hook](https://docs.rs/dioxus-desktop/0.5.0/dioxus_desktop/fn.use_window.html)
