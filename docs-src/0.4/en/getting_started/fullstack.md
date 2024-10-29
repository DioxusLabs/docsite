# Fullstack

> This guide assumes you read the [Web](wasm.md) getting started guide and installed the [Dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)

# Getting Started

## Setup

For this guide, we're going to show how to use Dioxus with [Axum](https://docs.rs/axum/latest/axum/), but `dioxus-fullstack` also integrates with the [Warp](https://docs.rs/warp/latest/warp/) and [Salvo](https://docs.rs/salvo/latest/salvo/) web frameworks.

Make sure you have Rust and Cargo installed, and then create a new project:

```shell
cargo new --bin demo
cd demo
```

Add `dioxus` and `dioxus-fullstack` as dependencies:

```shell
cargo add dioxus
cargo add dioxus-fullstack
```

Next, set up features for the server (`ssr`) and the client (`web`):

```toml
[features]
default = []
ssr = ["dioxus-fullstack/axum"]
web = ["dioxus-fullstack/web"]
```

Your dependencies should look roughly like this:

```toml
[dependencies]
dioxus = { version = "*" }
dioxus-fullstack = { version = "*" }

[features]
default = []
ssr = ["dioxus-fullstack/axum"]
web = ["dioxus-fullstack/web"]
```

Now, set up your Axum app to serve the Dioxus app.

```rust
{{#include src/doc_examples/untested_04/server_basic.rs}}
```

Now, run your app with:

```
dx build --features web --release
cargo run --features ssr --release
```

Finally, open `http://localhost:8080` in your browser. You should see a server-side rendered page with a counter.

```inject-dioxus
SandBoxFrame {
	url: "https://codesandbox.io/p/sandbox/dioxus-fullstack-2nwsrz?file=%2Fsrc%2Fmain.rs%3A5%2C1"
}
```

## Hot Reload

1. Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.

### Usage

1. Run:

```bash
dx build --features web
dx serve --features ssr --hot-reload --platform desktop
```

2. Change some code within a rsx or render macro
3. Save and watch the style change without recompiling

### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
