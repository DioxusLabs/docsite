# Logging
Dioxus has a wide range of supported platforms, each with their own logging requirements. We'll discuss the different options available for your projects.

## Dioxus Logger

Dioxus provides a first-party logger as part of `launch`. This sets up a tracing subscriber that cleanly integrates with the Dioxus CLI and platforms like Web and Mobile. In development mode, the `Debug` tracing level is set, and in release only the `Info` level is set.

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        // Will only log in "dev" mode
        tracing::debug!("Rendering app!");

        // Will log in dev and release
        tracing::info!("Rendering app!");

        rsx! {}
    })
}
```

To override the default or initialize the logger before `launch`, you can use the `init` function yourself:

To use Dioxus Logger, call the `init()` function:
```rs
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    // Dioxus launch code
    dioxus::launch(|| rsx! {})
}
```


## The Tracing Crate
The [Tracing](https://crates.io/crates/tracing) crate is the logging interface that the dioxus-logger uses. It is not required to use the Tracing crate, but you will not receive logs from the Dioxus library.

The Tracing crate provides a variety of simple `println`-like macros with varying levels of severity.
The available macros are as follows with the highest severity on the bottom:
```rs
fn main() {
    tracing::trace!("trace");
    tracing::debug!("debug");
    tracing::info!("info");
    tracing::warn!("warn");
    tracing::error!("error");
}
```
All the loggers provided on this page are, besides configuration and initialization, interfaced using these macros. Often you will also utilize the Tracing crate's `Level` enum. This enum usually represents the maximum log severity you want your application to emit and can be loaded from a variety of sources such as configuration file, environment variable, and more.

For more information, visit the Tracing crate's [docs](https://docs.rs/tracing/latest/tracing/).

## Platform Intricacies
On web, Dioxus Logger will use [tracing-wasm](https://crates.io/crates/tracing-wasm). On Desktop and server-based targets, Dioxus Logger will use [tracing-subscriber](https://crates.io/crates/tracing-subscriber)'s `FmtSubscriber`.


## Viewing Logs
Android logs are sent to logcat. To use logcat through the Android debugger, run:
```cmd
adb -d logcat
```
Your Android device will need developer options/usb debugging enabled.

For more information, visit android_logger's [docs](https://docs.rs/android_logger/latest/android_logger/).

iOS logs are sent to oslog.

For more information, visit [oslog](https://crates.io/crates/oslog).

#### Final Notes
Dioxus Logger is the preferred logger to use with Dioxus if it suites your needs. There are more features to come. If there are any feature suggestions or issues with Dioxus Logger, feel free to reach out on the [Dioxus Discord Server](https://discord.gg/XgGxMSkvUM)!

For more information, visit Dioxus Logger's [docs](https://docs.rs/dioxus-logger/latest/dioxus_logger/).
