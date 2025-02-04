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
cd app
```

Add Dioxus and the liveview renderer with the Axum feature as dependencies:

```shell
cargo add dioxus
cargo add dioxus-liveview --features axum
```

Next, add all the Axum dependencies. This will be different if you're using a different Web Framework

```
cargo add tokio --features full
cargo add axum
```

Your dependencies should look roughly like this:

```toml
[dependencies]
axum = "0.4.5"
dioxus = { version = "*" }
dioxus-liveview = { version = "*", features = ["axum"] }
tokio = { version = "1.15.0", features = ["full"] }
```

```rust
{{#include src/included_example.rs}}
```

```sh
{"timestamp":"   9.927s","level":"INFO","message":"Bundled app successfully!","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"App produced 2 outputs:","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"app - [target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"INFO","message":"dmg - [target/dx/hot_dog/bundle/macos/bundle/dmg/HotDog_0.1.0_aarch64.dmg]","target":"dx::cli::bundle"}
{"timestamp":"   9.927s","level":"DEBUG","json":"{\"BundleOutput\":{\"bundles\":[\"target/dx/hot_dog/bundle/macos/bundle/macos/HotDog.app\"]}}"}
```
