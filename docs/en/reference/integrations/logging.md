# Logging
Dioxus has a wide range of supported platforms, each with their own logging requirements. We'll discuss the different options available to you.

#### The Log Crate
The [Log](https://crates.io/crates/log) crate is the most universally recognized logging facade in Rust. 
It is also the easiest to work with in Dioxus; therefore we will be focusing on loggers that work with this crate.

The log crate provides a variety of simple ``println``-like macros with varying levels of severity. 
The available macros are as follows with the highest severity on the bottom:
```rs
fn main() {
    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");
}
```
All the loggers provided on this page are, besides configuration and initialization, interfaced using these macros. 
Often you will also utilize the log crate's ``LevelFilter`` enum. This enum usually represents the lowest log severity you want your application to emit and can be loaded from a configuration file, environment variable, or other.

For more information, visit log crate's [docs](https://docs.rs/log/latest/log/).

## dioxus-logger
[Dioxus Logger](https://crates.io/crates/dioxus-logger) is a planned-to-be feature-rich logger that supports all of Dioxus' platforms. Currently only Desktop, Web, and any server-based targets work with Dioxus Logger.

The easiest way to use Dioxus Logger is by calling the ``init()`` function:
```rs
use log::LevelFilter;

fn main() {
    // Init logger
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    // Dioxus code
}
```
The ``dioxus_logger::init()`` function initializes Dioxus Logger with the log crate using the default configuration and provided ``LevelFilter``.

#### Custom Format
Dioxus Logger allows you more control with the ability to set a custom format using the ``new`` function on the ``DioxusLogger`` struct:
```rs
use log::LevelFilter;

fn main() {
    // Init logger
    dioxus_logger::DioxusLogger::new(LevelFilter::Info)
        .use_format("[{LEVEL}] {PATH} - {ARGS}")
        .build()
        .expect("failed to init logger");

    // Dioxus code
}
```
In this example, we are building a new ``DioxusLogger`` struct, providing the ``LevelFilter``, calling the ``use_format()`` function, and initializing the logger with the ``build()`` function (acts as ``init()`` in the previous example).

The key function call in this example is ``use_format()``. This function takes a ``&str`` that specifies how you want your logs to be formatted. To specify a variable in the format, you wrap it's name in ``{}``.

The available variables are:
- LEVEL     - The ``LevelFilter`` of the emitted log.
- PATH      - The file path of where the log was emitted, or the crate name.
- ARGS      - The arguments passed through the log macro.
- TIMESTAMP - A timestamp of when the log was emitted. (Requires ``timestamp`` feature)

#### Timestamps

Another feature of Dioxus Logger is the ability to include timestamps with your logs. By default, this feature is disabled and has to be enabled by adding ``timestamp`` to your features section of the ``dioxus-logger`` dependency:
```toml
dioxus-logger = { version = "*", features = ["timestamp"] }
```

By enabling this feature, you gain access to the ``TIMESTAMP`` format variable.


#### Platform Intricacies
On web, Dioxus Logger will use [web-sys](https://crates.io/crates/web-sys) to interact with ``console.log()`` to output your logs to the browser's console. On Desktop and server-based targets, Dioxus Logger will output using ``println()``.

#### Final Notes
Dioxus Logger is the preferred logger to use with Dioxus if it suites your needs. There are many more features to come and Dioxus Logger is planned to become an integral part of Dioxus. If there are any feature suggestions or issues with Dioxus Logger, feel free to reach out on the [Dioxus Discord Server](https://discord.gg/XgGxMSkvUM)!

For more information, visit Dioxus Logger's [docs](https://docs.rs/dioxus-logger/latest/dioxus_logger/).

## wasm-logger
[WASM Logger](https://crates.io/crates/wasm-logger) is a logging interface that can be used with Dioxus' web platform.


For more information, visit wasm-logger's [docs](https://docs.rs/wasm-logger/latest/wasm_logger/).