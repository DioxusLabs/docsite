// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(unused)]

use dioxus::prelude::*;
use dioxus_docs_site::*;

use dioxus_router::routable::Routable;

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    #[cfg(feature = "prebuild")]
    {
        use dioxus_router::prelude::*;
        use log::LevelFilter;
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Error)
            .init()
            .unwrap();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let index_html = std::fs::read_to_string("docs/index.html").unwrap();
                let main_tag = r#"<div id="main">"#;
                let (before_body, after_body) =
                    index_html.split_once(main_tag).expect("main id not found");
                let after_body = after_body
                    .split_once("</div>")
                    .expect("main id not found")
                    .1;
                let wrapper = DefaultRenderer {
                    before_body: before_body.to_string() + main_tag,
                    after_body: "</div>".to_string() + after_body,
                };
                let mut renderer = IncrementalRenderer::builder()
                    .static_dir("docs_static")
                    .map_path(|route| {
                        let mut path = std::env::current_dir().unwrap();
                        path.push("docs_static");
                        for segment in route.split('/') {
                            path.push(segment);
                        }
                        println!("built: {}", path.display());
                        path
                    })
                    .build();
                renderer.renderer_mut().pre_render = true;
                pre_cache_static_routes::<Route, _>(&mut renderer, &wrapper)
                    .await
                    .unwrap();

                // Copy everything from docs_static to docs
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.content_only = true;
                options.copy_inside = true;
                std::fs::create_dir_all(format!("./docs")).unwrap();
                fs_extra::dir::move_dir("./docs_static", &format!("./docs"), &options).unwrap();
            });
        println!("prebuilt");

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    let mut path = std::path::PathBuf::default();
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
