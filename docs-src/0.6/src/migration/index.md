# How to Upgrade to Dioxus 0.6

This guide will outline the API changes between the `0.5` and `0.6` releases. The `0.6` release contains a few breaking changes to:
- The `Element` type
- Prevent default
- Assets with Manganis

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
