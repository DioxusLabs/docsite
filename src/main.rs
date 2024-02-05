// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(unused)]

use dioxus::prelude::*;
use dioxus_docs_site::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::routable::Routable;

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    #[cfg(feature = "prebuild")]
    {
        use dioxus_fullstack::prelude::*;
        use dioxus_router::prelude::*;
        use log::LevelFilter;
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                pre_cache_static_routes_with_props(
                    &ServeConfigBuilder::new_with_router(
                        dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
                    )
                    .assets_path("docs")
                    .incremental(
                        IncrementalRendererConfig::default()
                            .static_dir("docs")
                            .map_path(|route| {
                                let mut path = std::env::current_dir().unwrap();
                                path.push("docs");
                                for (i, segment) in route.split('/').enumerate() {
                                    path.push(segment);
                                }
                                println!("build: {path:?}");
                                path
                            }),
                    )
                    .build(),
                )
                .await
                .unwrap();
            });
        println!("prebuilt");

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    let mut path = std::path::PathBuf::new();
                    for (i, segment) in route.split('/').enumerate() {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );
        return;
    }

    #[cfg(not(feature = "prebuild"))]
    launch(app);
}
