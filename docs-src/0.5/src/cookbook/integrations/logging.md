# Logging
Dioxus has a wide range of supported platforms, each with their own logging requirements. We'll discuss the different options available for your projects.

#### The Tracing Crate
The [Tracing](https://crates.io/crates/tracing) crate is the logging interface that the Dioxus library uses. It is not required to use the Tracing crate, but you will not recieve logs from the Dioxus library.

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

## Dioxus Logger
[Dioxus Logger](https://crates.io/crates/dioxus-logger) is a logging utility that will start the appropriate logger for the platform. Currently every platform except mobile is supported.

To use Dioxus Logger, call the `init()` function:
```rs
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    // Dioxus launch code
}
```
The `dioxus_logger::init()` function initializes Dioxus Logger with the appropriate tracing logger using the default configuration and provided `Level`.

#### Platform Intricacies
On web, Dioxus Logger will use [tracing-wasm](https://crates.io/crates/tracing-wasm). On Desktop and server-based targets, Dioxus Logger will use [tracing-subscriber](https://crates.io/crates/tracing-subscriber)'s `FmtSubscriber`.

#### Final Notes
Dioxus Logger is the preferred logger to use with Dioxus if it suites your needs. There are more features to come and Dioxus Logger is planned to become an integral part of Dioxus. If there are any feature suggestions or issues with Dioxus Logger, feel free to reach out on the [Dioxus Discord Server](https://discord.gg/XgGxMSkvUM)!

For more information, visit Dioxus Logger's [docs](https://docs.rs/dioxus-logger/latest/dioxus_logger/).

## Desktop and Server
For Dioxus' desktop and server targets, you can generally use the logger of your choice.


Some popular options are:
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber)'s `FmtSubscriber` for console output.
- [tracing-appender](https://crates.io/crates/tracing-appender) for logging to files.
- [tracing-bunyan-formatter](https://crates.io/crates/tracing-bunyan-formatter) for the Bunyan format.

To keep this guide short, we will not be covering the usage of these crates.

For a full list of popular tracing-based logging crates, visit [this](https://docs.rs/tracing/latest/tracing/#related-crates) list in the Tracing crate's docs.

## Web
[tracing-wasm](https://crates.io/crates/tracing-wasm) is a logging interface that can be used with Dioxus' web platform.

The easiest way to use WASM Logger is with the `set_as_global_default` function:
```rs
fn main() {
    // Init logger
    tracing_wasm::set_as_global_default();
    // Dioxus code
}
```
This starts tracing with a `Level` of `Trace`. 

Using a custom `level` is a little trickier. We need to use the `WasmLayerConfigBuilder` and start the logger with `set_as_global_default_with_config()`:
```rs
use tracing::Level;

fn main() {
    // Init logger
    let tracing_config = tracing_wasm::WASMLayerConfigBuilder::new().set_max_level(Level::INFO).build();
    tracing_wasm::set_as_global_default_with_config(tracing_config);
    // Dioxus code
}
```

# Mobile
Unfortunately there are no tracing crates that work with mobile targets. As an alternative you can use the [log](https://crates.io/crates/log) crate.

## Android
[Android Logger](https://crates.io/crates/android_logger) is a logging interface that can be used when targeting Android. Android Logger runs whenever an event `native_activity_create` is called by the Android system:
```rs
use log::LevelFilter;
use android_logger::Config;

fn native_activity_create() {
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Info)
            .with_tag("myapp");
    );
}
```
The `with_tag()` is what your app's logs will show as.

#### Viewing Logs
Android logs are sent to logcat. To use logcat through the Android debugger, run:
```cmd
adb -d logcat
```
Your Android device will need developer options/usb debugging enabled.

For more information, visit android_logger's [docs](https://docs.rs/android_logger/latest/android_logger/).

## iOS
The current option for iOS is the [oslog](https://crates.io/crates/oslog) crate. 

```rs
fn main() {
    // Init logger
    OsLogger::new("com.example.test")
        .level_filter(LevelFilter::Debug)
        .init()
        .expect("failed to init logger");
    // Dioxus code
}
```

#### Viewing Logs
You can view the emitted logs in Xcode. 

For more information, visit [oslog](https://crates.io/crates/oslog). 
