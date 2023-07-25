# Fullstack

> This guide assumes you read the [Web](web.md) getting started guide and installed the [Dioxus-cli](https://github.com/DioxusLabs/dioxus/tree/master/packages/cli)

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
cargo add dioxus-fullstack --features axum
```

Next, add all the Axum dependencies. This will be different if you're using a different Web Framework

```shell
cargo add tokio --features full
cargo add axum
```

Your dependencies should look roughly like this:

```toml
[dependencies]
axum = "*"
dioxus = { version = "*" }
dioxus-fullstack = { version = "*", features = ["axum", "ssr"] }
tokio = { version = "*", features = ["full"] }
```

Now, set up your Axum app to serve the Dioxus app.

```rust
{{#include src/doc_examples/server_basic.rs}}
```

Now, run your app with `cargo run` and open `http://localhost:8080` in your browser. You should see a server-side rendered page with a counter.

## Hot Reload

1. Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.

You can place the hot reload macro at the top of your main function in ssr mode to enable hot reloading on debug builds.

For more information about hot reloading on native platforms and configuration options see the [dioxus-hot-reload](https://crates.io/crates/dioxus-hot-reload) crate.

### Setup

Add the following to your main function:

```rust
fn main() {
	// launch your application
}
```

### Usage

1. Run:

```bash
dioxus build --features web
dioxus run --features ssr --hot-reload
```

2. Change some code within a rsx or render macro
3. Save and watch the style change without recompiling

### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
