# Web

Build single-page applications that run in the browser with Dioxus. To run on the Web, your app must be compiled to WebAssembly and depend on the `dioxus` and `dioxus-web` crates.

A build of Dioxus for the web will be roughly equivalent to the size of a React build (70kb vs 65kb) but it will load significantly faster because [WebAssembly can be compiled as it is streamed](https://hacks.mozilla.org/2018/01/making-webassembly-even-faster-firefoxs-new-streaming-and-tiering-compiler/).

Examples:

- [TodoMVC](https://github.com/DioxusLabs/example-projects/tree/master/todomvc)
- [ECommerce](https://github.com/DioxusLabs/example-projects/tree/master/ecommerce-site)

[![TodoMVC example](https://github.com/DioxusLabs/example-projects/raw/master/todomvc/example.png)](https://github.com/DioxusLabs/example-projects/blob/master/todomvc)

> Note: Because of the limitations of Wasm, [not every crate will work](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html) with your web apps, so you'll need to make sure that your crates work without native system calls (timers, IO, etc).

## Support

The Web is the best-supported target platform for Dioxus.

- Because your app will be compiled to WASM you have access to browser APIs through [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html).
- Dioxus provides hydration to resume apps that are rendered on the server. See the [fullstack](fullstack.md) getting started guide for more information.

## Tooling

To develop your Dioxus app for the web, you'll need a tool to build and serve your assets. We recommend using [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli) which includes a build system, Wasm optimization, a dev server, and support hot reloading:

```shell
cargo install dioxus-cli@0.5.0-alpha.0
```

Make sure the `wasm32-unknown-unknown` target for rust is installed:

```shell
rustup target add wasm32-unknown-unknown
```

## Creating a Project

Create a new crate:

```shell
cargo new --bin demo
cd demo
```

Add Dioxus with the web renderer feature as a dependency (this will edit your `Cargo.toml`):

```bash
cargo add dioxus@0.5.0-alpha.0 --features web
```

Edit your `main.rs`:

```rust
{{#include src/doc_examples/hello_world_web.rs}}
```

And to serve our app:

```bash
dx serve
```

If you open the browser and navigate to `127.0.0.1` you should see an app that looks like this:

```inject-dioxus
DemoFrame {
    hello_world::HelloWorldCounter {}
}
```


## Hot Reload

1. Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.

For the web renderer, you can use the dioxus cli to serve your application with hot reloading enabled.

### Setup

Install [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli).

### Usage

1. Run:

```bash
dx serve --hot-reload
```

2. Change some code within a rsx or render macro
3. Open your localhost in a browser
4. Save and watch the style change without recompiling


### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
