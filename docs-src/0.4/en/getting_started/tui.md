# Terminal UI

You can build a text-based interface that will run in the terminal using Dioxus.

![Hello World screenshot](https://github.com/DioxusLabs/rink/raw/master/examples/example.png)

> Note: this book was written with HTML-based platforms in mind. You might be able to follow along with TUI, but you'll have to adapt a bit.

## Support

TUI support is currently quite experimental. But, if you're willing to venture into the realm of the unknown, this guide will get you started.

- It uses flexbox for the layout
- It only supports a subset of the attributes and elements
- Regular widgets will not work in the tui render, but the tui renderer has its own widget components that start with a capital letter. See the [widgets example](https://github.com/DioxusLabs/dioxus/blob/master/packages/dioxus-tui/examples/widgets.rs)
- 1px is one character line height. Your regular CSS px does not translate
- If your app panics, your terminal is wrecked. This will be fixed eventually

## Getting Set up

Start by making a new package and adding Dioxus and the TUI renderer as dependencies.

```shell
cargo new --bin demo
cd demo
cargo add dioxus
cargo add dioxus-tui
```

Then, edit your `main.rs` with the basic template.

```rust
{{#include ../docs-router/src/doc_examples/untested_04/hello_world_tui.rs}}
```

To run our app:

```shell
cargo run
```

Press "ctrl-c" to close the app. To switch from "ctrl-c" to just "q" to quit you can launch the app with a configuration to disable the default quit and use the root TuiContext to quit on your own.

```rust
{{#include ../docs-router/src/doc_examples/untested_04/hello_world_tui_no_ctrl_c.rs}}
```

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
