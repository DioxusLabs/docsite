// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release

#[allow(unused)]

use dioxus_docs_site::*;

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    #[cfg(feature = "ssr")]
    {
        simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Trace).init().unwrap();
    }

    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        incremental,
    });
}
