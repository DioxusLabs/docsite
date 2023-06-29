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
        use dioxus_fullstack::prelude::*;
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                pre_cache_static_routes_with_props(
                    &ServeConfigBuilder::new_with_router(
                        dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
                    )
                    .incremental(IncrementalRendererConfig::default())
                    .build(),
                )
                .await
                .unwrap();
            });
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap();
    }

    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        incremental,
    });
}
