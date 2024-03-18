# Fullstack

> This guide assumes you read the [Web](wasm.md) getting started guide and installed the [Dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)

# Getting Started

## Setup

Before we get started, make sure you have Rust and Cargo installed. Then create a new project:

```shell
cargo new --bin demo
cd demo
```

Add `dioxus` and `dioxus-fullstack` as dependencies:

```shell
cargo add dioxus@0.5.0-alpha.2 --features fullstack
```

Next, set up features for the server (`server`) and the client (`web`):

```toml
[features]
default = []
server = ["dioxus/axum"]
web = ["dioxus/web"]
```

Your dependencies should look roughly like this:

```toml
[dependencies]
dioxus = { version = "*", features = ["fullstack"] }

[features]
default = []
server = ["dioxus/axum"]
web = ["dioxus/web"]
```

Now, set up your fullstack app to serve the Dioxus app.

```rust
{{#include src/doc_examples/server_basic.rs}}
```

Now, run your app with:

```
dx serve --platform fullstack
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
dx serve --hot-reload --platform fullstack
```

2. Change some code within a rsx or render macro
3. Save and watch the style change without recompiling

### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
