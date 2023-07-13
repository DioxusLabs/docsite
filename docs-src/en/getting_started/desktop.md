# Desktop Overview

Build a standalone native desktop app that looks and feels the same across operating systems.

Apps built with Dioxus Desktop use the system webview to render the page. This makes the final size of application much smaller than other webview renderers (typically under 5mb).

Although desktop apps are rendered in a webview, your Rust code runs natively. This means that browser APIs are _not_ available, so rendering WebGL, Canvas, etc is not as easy as the Web. However, native system APIs _are_ accessible, so streaming, WebSockets, filesystem, etc are all easly accessable though system APIs.

Dioxus Desktop is built off [Tauri](https://tauri.app/). Right now there are limited Dioxus abstractions over the menubar, event handling, etc. In some places you may need to leverage Tauri dirrectly – through [Wry](http://github.com/tauri-apps/wry/) and [Tao](http://github.com/tauri-apps/tao).

> In the future, we plan to move to a custom web renderer-based DOM renderer with WGPU integrations ([Blitz](https://github.com/DioxusLabs/blitz)).

## Examples

- [File Explorer](https://github.com/DioxusLabs/example-projects/blob/master/file-explorer)
- [WiFi Scanner](https://github.com/DioxusLabs/example-projects/blob/master/wifi-scanner)

[![File ExplorerExample](https://raw.githubusercontent.com/DioxusLabs/example-projects/master/file-explorer/image.png)](https://github.com/DioxusLabs/example-projects/tree/master/file-explorer)

# Getting started

## Platform-Specific Dependencies

Dioxus desktop renders through a web view. Depending on your platform, you might need to install some dependancies.

### Windows

Windows Desktop apps depend on WebView2 – a library that should be installed in all modern Windows distributions. If you have Edge installed, then Dioxus will work fine. If you _don't_ have Webview2, [then you can install it through Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/). MS provides 3 options:

1. A tiny "evergreen" _bootstrapper_ that fetches an installer from Microsoft's CDN
2. A tiny _installer_ that fetches Webview2 from Microsoft's CDN
3. A statically linked version of Webview2 in your final binary for offline users

For development purposes, use Option 1.

### Linux

Webview Linux apps require WebkitGtk. When distributing, this can be part of your dependency tree in your `.rpm` or `.deb`. However, likely, your users will already have WebkitGtk.

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

When using Debian/bullseye `libappindicator3-dev` is no longer available but replaced by `libayatana-appindicator3-dev`.

```bash
# on Debian/bullseye use:
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev
```

If you run into issues, make sure you have all the basics installed, as outlined in the [Tauri docs](https://tauri.studio/v1/guides/getting-started/prerequisites#setting-up-linux).

### MacOS

Currently – everything for macOS is built right in! However, you might run into an issue if you're using nightly Rust due to some permissions issues in our Tao dependency (which have been resolved but not published).

## Creating a Project

Create a new crate:

```shell
cargo new --bin demo
cd demo
```

Add Dioxus and the desktop renderer as dependencies (this will edit your `Cargo.toml`):

```shell
cargo add dioxus
cargo add dioxus-desktop
```

Edit your `main.rs`:

```rust
{{#include src/doc_examples/hello_world_desktop.rs:all}}
```

## Hot Reload

1. Hot reloading allows much faster iteration times inside of rsx calls by interpreting them and streaming the edits.
2. It is useful when changing the styling/layout of a program, but will not help with changing the logic of a program.

You can place the hot reload macro at the top of your main function to enable hot reloading on debug builds.

For more information about hot reloading on native platforms and configuration options see the [dioxus-hot-reload](https://crates.io/crates/dioxus-hot-reload) crate.

### Setup

Add the following to your main function:

```rust
fn main() {
	hot_reload_init!();
	// launch your application
}
```

### Usage

1. Run:

```bash
cargo run
```

2. Change some code within a rsx or render macro
3. Save and watch the style change without recompiling

### Limitations

1. The interpreter can only use expressions that existed on the last full recompile. If you introduce a new variable or expression to the rsx call, it will require a full recompile to capture the expression.
2. Components, Iterators, and some attributes can contain arbitrary rust code and will trigger a full recompile when changed.
