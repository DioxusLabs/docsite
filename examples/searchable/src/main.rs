// Run with:
// dx build --release --features web
// cargo run --release --features ssr
//
// Note: The first time you run the build, the search index will be empty. You need to rebuild the build again to fill the search index.

use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "ssr")]
    {
        use log::LevelFilter;
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .with_module_level("dioxus_search_macro", LevelFilter::Trace)
            .with_module_level("dioxus_search_shared", LevelFilter::Trace)
            .init()
            .unwrap();
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
        println!("finished prebuilding static routes");

        dioxus_search::SearchIndex::<Route>::create(
            "searchable",
            dioxus_search::BaseDirectoryMapping::new("./static"),
        );
        println!("finished creating search index");
    }

    launch(|| rsx! { Router::<Route> {} });
}

#[derive(Clone, Routable, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Homepage {},

    #[route("/blog")]
    BlogPost {},

    #[route("/other")]
    OtherPost {},
}

fn Homepage() -> Element {
    let mut search_text = use_signal(String::new);
    let results = SEARCH_INDEX.search(&search_text());

    rsx! {
        input {
            oninput: move |e| {
                search_text.set(e.value());
            },
            value: "{search_text}",
        }
        ul {
            for result in results.map(|i| i.into_iter()).ok().into_iter().flatten() {
                li {
                    Link {
                        to: result.route.clone(),
                        "{result:#?}"
                    }
                }
            }
        }
    }
}

fn BlogPost() -> Element {
    rsx! {
        div {
            h1 { "Hello World" }
            p { "This is a blog post" }
        }
    }
}

fn OtherPost() -> Element {
    rsx! {
        div {
            h1 { "Goodbye" }
            p { "This is another blog post" }
        }
    }
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "searchable"
};
