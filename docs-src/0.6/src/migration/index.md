# How to Upgrade to Dioxus 0.6

This guide will outline the API changes between the `0.5` and `0.6` releases. The `0.6` release contains a breaking changes to:

- The `Element` type
- Prevent default
- Assets with Manganis
- `dioxus_logger` integration with `dioxus`
- The `launch` function
- The `eval` function
- The `dioxus-fullstack` crate
- The router crate
- The `derive(Props)` macro
- The `dioxus-core` crate
- Custom renderer API
- Global state management

## Element

The element type has changed from `Option<VNode>` to `Result<VNode, RenderError>`. This makes it possible to bubble up errors while rendering with the `?` operator, but it does remove the ability to return `None` from a component. Instead of returning `None`, you can return `VNode::empty()` or an empty `rsx!` macro.

Dioxus 0.5:

```rust
use dioxus::prelude::*;

fn app() -> Element {
    let number = use_signal(|| -1);

    if number() < 0 {
        // ❌ In dioxus 0.6, the element type is a result, so None values cannot be returned directly
        return None;
    }

    rsx! {
        "Positive number: {number}"
    }
}
```

Dioxus 0.6:

```rust
{{#include src/doc_examples/migration.rs:element_fixed}}
```

## Prevent Default

Dioxus 0.1-0.5 used the `prevent_default` attribute to prevent default behavior of event handlers for every event. Dioxus 0.6 introduces more fine-grained control over preventing default behavior with the `prevent_default` function on the event type. Instead of setting the `prevent_default` attribute for all events you want to prevent, you can create event handlers that call `event.prevent_default()`.

Dioxus 0.5:

```rust
{{#include src/doc_examples/migration.rs:prevent_default_old}}
```

Dioxus 0.6:

```rust
{{#include src/doc_examples/migration.rs:prevent_default_new}}
```

> Note: Since event handlers run on the server in Liveview, events cannot be prevented quickly inside the event handler. Because of this, the new `prevent_default` method does not prevent default behavior in Liveview.
>
> Instead you can use javascript inside the `onclick` handler to prevent default behavior.
>
> ```rust
> {{#include src/doc_examples/migration.rs:prevent_default_new_liveview}}
> ```

## Assets

The syntax of the `asset!` macro has changed in Dioxus 0.6. Instead of accepting a single argument with both the path and the configuration for the asset, you can now pass in the path as the first argument and the configuration as a optional second argument.

The path the `asset!` macro accepts has also changed. Previously, the macro used to accept absolute and relative paths where relative paths were relative to the current crate directory. Now the macro only accepts absolute paths which are resolved relative to the root of the crate.

Dioxus 0.5:

```rust
use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        img {
            src: asset!(image("./assets/static/bundle.png").size(100, 100))
        }
    }
}
```

Dioxus 0.6:

```rust
{{#include src/doc_examples/migration.rs:assets_new}}
```

## Logging

Dioxus 0.6 brings the `dioxus-logger` crate directly into dioxus itself.

Previously, you needed to add `dioxus-logger` to your Cargo.toml and then call its init function:

```rs
// cargo.toml:
// dioxus-logger = "0.5"

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Dioxus launch code
    dioxus::launch(app)
}
```

Now, in Dioxus 0.6, the logger is implicit with `launch`. Simply call launch and the logger is initialized to a default log level. In development mode, the `Debug` tracing level is set, and in release only the `Info` level is set.

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}
```

If you still need to set the level manually or configure a custom subscriber, do that before `launch`. We expose the `initialize_default` function in case you need additional logging before your `launch` call:

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::logger::initialize_default();

    tracing::info!("Logs received!");

    dioxus::launch(app);
}
```

## Launch

The `launch` function was removed from the prelude. You must now import the launch method from `dioxus` or use it by its full path:

```rust
use dioxus::prelude::*;

fn main() {
    // ❌ launch(app);
    dioxus::launch(app); // ✅
}
```

See https://github.com/DioxusLabs/dioxus/pull/2967 for more details.

## Eval

- `eval` was moved from the prelude to the `document` module. You must now call it with `document::eval` instead of `eval`:

```rust
use dioxus::prelude::*;

fn app() -> Element {
    // ❌ use_effect(|| eval("console.log(1)"));
    use_effect(|| document::eval("console.log(1)")); // ✅

    rsx! {}
}
```

- The `eval` feature flag was removed from the `dioxus-html` crate and the functionality of `EvalProvider` was moved to the new `dioxus-document` crate. Custom renderers must now provide a `Rc<dyn Document>` context to the application to make `eval` and head elements work correctly. See https://github.com/DioxusLabs/dioxus/pull/2635 for more details.
- `Eval::recv` and `Eval::join` now returns any value that implements `DeserializeOwned` instead of `serde_json::Value`. `Eval::send` now accepts any value that implements `Serialize`. See https://github.com/DioxusLabs/dioxus/pull/3035 for more details

## Fullstack

- The feature `dioxus/axum` was renamed to `dioxus/server`

```toml
[features]
default = []
# ❌ server = ["dioxus/axum"]
server = ["dioxus/server"] # ✅
web = ["dioxus/web"]
```

See https://github.com/DioxusLabs/dioxus/pull/3186 for more details

- The `fullstack::Config` item was removed. You can now pass the platform configs into the `LaunchBuilder` directly. For example, if you want to set the rootname on each platform, you can set the root name in each config:

```rust
LaunchBuilder::new()
    // Only set the server config if the server feature is enabled
    .with_cfg(server_only! {
        ServeConfigBuilder::default().root_id("app")
    })
    // You also need to set the root id in your web config
    .with_cfg(web! {
        dioxus::web::Config::default().rootname("app")
    })
    // And desktop config
    .with_cfg(desktop! {
        dioxus::desktop::Config::default().with_root_name("app")
    })
    .launch(app);
```

See https://github.com/DioxusLabs/dioxus/pull/2967 for more details.

- The dioxus-cli now proxies fullstack applications at a port behind a reverse proxy. If you have a custom axum server, you must serve your application at the port returned by `dioxus_cli_config::server_port` and the address returned by `dioxus_cli_config::server_ip` or the complete address returned by `dioxus_cli_config::fullstack_address_or_localhost` during development:

```rust
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let address = dioxus_cli_config::fullstack_address_or_localhost();

    // Launch the fullstack application on the address the CLI is proxying
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), App);

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```

See https://github.com/DioxusLabs/dioxus/pull/2258 for more details.

- `serve_dioxus_application` was changed to accept a component directly instead of a virtual dom factory. See https://github.com/DioxusLabs/dioxus/pull/2515 for more details.
- `register_server_fns` was renamed to `register_server_functions`. See https://github.com/DioxusLabs/dioxus/pull/2515 for more details.
- `RenderHandleState::new` accepts a new `ServeConfig` argument. See https://github.com/DioxusLabs/dioxus/pull/2862 for more details.
- `ServeConfigBuilder::build` returns a result. It may fail during desktop builds if no `index.html` file is found. This error is fine to ignore in desktop builds. You can pass the builder directly to `serve_dioxus_application` to only serve the index.html file if it exists. See https://github.com/DioxusLabs/dioxus/pull/2862 for more details.
- `dioxus_fullstack::Config::addr` was removed. You can now export the `PORT` and `IP` environment variables to set the address the `launch` method uses for the server.

## Router

- The `Routable` derive macro no longer accepts fields that are not present in the `route("/route")` if the web feature is enabled. See https://github.com/DioxusLabs/dioxus/pull/2159 for more details.
- The `ToRouteSegments` trait in the router was changed from accepting `self` to accepting `&self`. This means you can now implement it for `T` directly instead of `&T`. See https://github.com/DioxusLabs/dioxus/pull/2283 for more details.

## derive(Props)

- `#[props(into)]` is ignore on any String props. String props already accept `impl ToString` which is implemented for many of the same types, but if you implement `Into<String>` for a specific type, your code may require some changes. See https://github.com/DioxusLabs/dioxus/pull/2501 for more details
- Properties that start with an uppercase letter are no longer accepted. This allows us to autocomplete Components. See https://github.com/DioxusLabs/dioxus/pull/2652 for more details.

## State Management

- `use_coroutine` now accepts `impl FnMut` instead of `impl FnOnce`. This was required to support restarting the coroutine without rerunning the component. See https://github.com/DioxusLabs/dioxus/pull/3005 for more details.
- `Signal::global_memo` now requires `T: PartialEq` just like `use_memo`. See https://github.com/DioxusLabs/dioxus/pull/2851 for more details.
- `GlobalMemo<T>` is now a trait alias for `Global<Memo<T>, T>` and `GlobalSignal<T>` is now a trait alias for `Global<Signal<T>, T>`. To get the underlying `Memo` or `Signal`, you can now use the `resolve` method instead of `signal` or `memo`. See https://github.com/DioxusLabs/dioxus/pull/2851 for more details.
- The `Readable` trait in dioxus signals now requires a `try_peek_unchecked` method instead of `peek_unchecked`. See https://github.com/DioxusLabs/dioxus/pull/2714 for more details.
- The `check_generation` feature flag was removed from the `generational-box` crate. See https://github.com/DioxusLabs/dioxus/pull/2638 for more details.

## Core changes

- The `Template::name` field was removed. See https://github.com/DioxusLabs/dioxus/pull/2799 for more details.
- `Properties::into_vcomponent` now accepts only the `render_fn` instead of the `render_fn` and `component_name`. This change fixes the name of re-exported components. Fixes https://github.com/DioxusLabs/dioxus/pull/2744
- The field `VNode::template` is now `Template` instead of `Cell<Template>`. See https://github.com/DioxusLabs/dioxus/pull/2705 for more details
- `Mutations::santize` was renamed to `Mutations::sanitize`. See https://github.com/DioxusLabs/dioxus/pull/2653 for more details.
- The variant `AttributeValue::Any` now contains `Rc<dyn AnyValue>` instead of `Box<dyn AnyValue>` to make `AttributeValue` `Clone`. See https://github.com/DioxusLabs/dioxus/pull/2705 for more details

## Custom Renderers

If you are building a custom renderer, there were some breaking changes to hot reloading and rsx that you should be aware of:

- The CLI hot reloading format changed significantly. Custom renderers must switch from `dioxus-hot-reload` to `dioxus_devtools`. Renderers can connect to the hot reloading engine with the [connect](https://docs.rs/dioxus-devtools/0.6.0/dioxus_devtools/fn.connect.html) function. See https://github.com/DioxusLabs/dioxus/pull/2258 for more details.
- The format of custom elements was changed to improve autocomplete. The `dioxus_elements` namespace must now contain each element as a module with a TAG_NAME and NAME_SPACE constant inside that module. Each attribute should be another constant in that module. The top-level `dioxus_elements` module should contain a `completions` module with a `CompleteWithBraces` enum that re-exports each element the namespace supports for braces autocomplete. See https://github.com/DioxusLabs/dioxus/pull/2421 for more details.
- The format for custom event handlers changed include `eventname::call_with_explicit_closure` to provide better type inference for inline closures. See https://github.com/DioxusLabs/dioxus/pull/2437 for more details

If you are also using dioxus-html, there are a few more breaking changes:

- A `file_size` method was added to the `FileEngine` trait. Any custom renderers must implement this method. See https://github.com/DioxusLabs/dioxus/pull/2323/files for more details.
- `HtmlEventConverter` has a new `convert_resize_data` method which must be implemented by any custom renderers that use dioxus-html. See https://github.com/DioxusLabs/dioxus/pull/2479 for more details
- The web and native features were removed from the `dioxus-html` crate. See https://github.com/DioxusLabs/dioxus/pull/3006 for more details.
- `dioxus_html::AttributeDiscription ` was renamed to `dioxus_html::AttributeDescription`. See https://github.com/DioxusLabs/dioxus/pull/2653 for more details.

## Minor Breaking Changes

There were several more minor breaking changes in Dioxus 0.6:

- Many implicit features from dioxus crates were removed. These features were automatically generated by cargo and generally not functional. See https://github.com/DioxusLabs/dioxus/pull/2512 for more details.
- `dioxus_autofmt::write_block_out` accepts `&CallBody` instead of `CallBody`. See https://github.com/DioxusLabs/dioxus/pull/2573 for more details.
- The `panic_hook` feature which provides a console panic message for wasm panics was moved from the `dioxus-web` crate to the `dioxus-logger` crate. The handler is still provided by default. See https://github.com/DioxusLabs/dioxus/pull/3302 for more details.
