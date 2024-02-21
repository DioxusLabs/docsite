# Liveview

Liveview allows apps to *run* on the server and *render* in the browser. It uses WebSockets to communicate between the server and the browser.

Examples:
- [Axum Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/axum.rs)
- [Salvo Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/salvo.rs)
- [Warp Example](https://github.com/DioxusLabs/dioxus/tree/master/packages/liveview/examples/warp.rs)


## Support

Liveview is currently limited in capability when compared to the Web platform. Liveview apps run on the server in a native thread. This means that browser APIs are not available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs are accessible, so streaming, WebSockets, filesystem, etc are all viable APIs.


## Setup

For this guide, we're going to show how to use Dioxus Liveview with [Axum](https://docs.rs/axum/latest/axum/).

Make sure you have Rust and Cargo installed, and then create a new project:

```shell
cargo new --bin demo
cd demo
```

Add Dioxus and the liveview renderer with the Axum feature as dependencies:

```shell
cargo add dioxus --features liveview,axum
```

Your dependencies should look roughly like this:

```toml
[dependencies]
dioxus = { version = "*", features = ["liveview", "axum"] }
```

Next, set up your app:

```rust
{{#include src/doc_examples/hello_world_liveview.rs:all}}
```

Finnaly, run your app with:

```sh
dx serve --platform desktop
# or
cargo run
```

And that's it!


## Hot Reload

1. Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.

### Setup

Install [dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli).

### Usage

1. Run:

```bash
dx serve --hot-reload --platform desktop
```

2. Change some code within a rsx or render macro
3. Save and watch the style change without recompiling

### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
