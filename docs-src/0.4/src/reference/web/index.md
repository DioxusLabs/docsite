# Web

This guide will cover concepts specific to the Dioxus web renderer.

## Running Javascript

Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose.


For these cases, Dioxus desktop exposes the use_eval hook that allows you to run raw Javascript in the webview:

```rust
{{#include ../docs-router/src/doc_examples/untested_04/eval.rs}}
```

If you are targeting web, but don't plan on targeting any other Dioxus renderer you can also use the generated wrappers in the [web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) and [gloo](https://gloo-rs.web.app/) crates.

## Customizing Index Template

Dioxus supports providing custom index.html templates. The index.html must include a `div` with the id `main` to be used. Hot Reload is still supported. An example
is provided in the [PWA-Example](https://github.com/DioxusLabs/Dioxus/examples/PWA-example/index.html).
