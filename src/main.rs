// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(unused)]

use dioxus_docs_site::*;

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
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
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
                                let mut path = std::path::PathBuf::from("./docs");
                                for (i, segment) in route.split('/').enumerate() {
                                    if (1, "docsite") == (i, segment) {
                                        continue;
                                    }
                                    path.push(segment);
                                }
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
                        if (1, "docsite") == (i, segment) {
                            continue;
                        }
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );
        return;
    }

    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        serve_cfg: {
            dioxus_fullstack::prelude::ServeConfigBuilder::new_with_router(
                dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
            )
            .assets_path("docs")
            .incremental(dioxus_fullstack::prelude::IncrementalRendererConfig::default())
        },
    });
}
