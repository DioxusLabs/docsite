# Web

To run on the Web, your app must be compiled to WebAssembly and depend on the `dioxus` and `dioxus-web` crates.

A build of Dioxus for the web will be roughly equivalent to the size of a React build (70kb vs 65kb) but it will load significantly faster because [WebAssembly can be compiled as it is streamed](https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/).

Examples:

- [TodoMVC](https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs)
- [Tailwind App](https://github.com/DioxusLabs/dioxus/tree/main/examples/tailwind)

[![TodoMVC example](https://github.com/DioxusLabs/example-projects/raw/master/todomvc/example.png)](https://github.com/DioxusLabs/dioxus/blob/main/examples/todomvc.rs)

> Note: Because of the limitations of Wasm, [not every crate will work](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html) with your web apps, so you'll need to make sure that your crates work without native system calls (timers, IO, etc).

## Support

The Web is the best-supported target platform for Dioxus.

- Because your app will be compiled to WASM you have access to browser APIs through [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html).
- Dioxus provides hydration to resume apps that are rendered on the server. See the [fullstack](../fullstack/index.md) reference for more information.

## Running Javascript

Dioxus provides some ergonomic wrappers over the browser API, but in some cases you may need to access parts of the browser API Dioxus does not expose.

For these cases, Dioxus web exposes the use_eval hook that allows you to run raw Javascript in the webview:

```rust
{{#include src/doc_examples/eval.rs}}
```

If you are targeting web, but don't plan on targeting any other Dioxus renderer you can also use the generated wrappers in the [web-sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) and [gloo](https://gloo-rs.web.app/) crates.

## Customizing Index Template

Dioxus supports providing custom index.html templates. The index.html must include a `div` with the id `main` to be used. Hot Reload is still supported. An example
is provided in the [PWA-Example](https://github.com/DioxusLabs/dioxus/blob/main/examples/PWA-example/index.html).
