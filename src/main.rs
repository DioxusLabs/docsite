#![allow(non_snake_case, non_upper_case_globals)]

use std::env;

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod doc_examples;
pub mod docs;
pub mod icons;
pub mod shortcut;
pub mod snippets;
pub use components::*;

#[component]
fn HeaderFooter() -> Element {
    let cb = use_callback(|_| *SHOW_SEARCH.write() = true);
    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || cb.call(())
    });

    rsx! {
        div { class: "bg-white dark:bg-ideblack",
            document::Title { "Dioxus | Fullstack crossplatform app framework for Rust" }
            document::Meta {
                name: "description",
                content: "A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
            }
            document::Link {
                rel: "icon shortcut",
                r#type: "image/png",
                href: asset!("/assets/static/favicon.png"),
            }
            document::Link { rel: "stylesheet", href: asset!("/assets/githubmarkdown.css") }
            document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
            document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
            document::Link { rel: "stylesheet", href: asset!("/assets/dxp.css") }
            document::Link { rel: "stylesheet", href: asset!("/assets/material.css") }
            document::Link { rel: "stylesheet", href: "https://rsms.me/inter/inter.css" }
            document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
            document::Link {
                href: "https://fonts.gstatic.com",
                rel: "preconnect",
                crossorigin: "false",
            }
            document::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/css2?family=Arimo:wght@100;400;600&display=swap",
            }
            document::Link {
                href: "https://fonts.googleapis.com/css2?family=Arimo:ital,wght@0,400..700;1,400..700&family=Lexend:wght@100;400&family=M+PLUS+1:wght@100..900&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
                rel: "stylesheet",
            }
            document::Script { src: asset!("/assets/ace/ace.js") }
            document::Script { src: asset!("/assets/ace/mode-rust.js") }
            document::Script { src: asset!("/assets/ace/theme-github.js") }
            document::Meta {
                property: "og:title",
                content: "Dioxus | Fullstack crossplatform app framework for Rust",
            }
            document::Meta { property: "og:type", content: "website" }
            document::Meta {
                property: "og:description",
                content: "A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
            }
            document::Meta { property: "og:url", content: "https://dioxuslabs.com" }
            document::Meta {
                property: "og:image",
                content: "https://dioxuslabs.com/static/opengraph.png",
            }
            document::Meta {
                name: "twitter:title",
                content: "Dioxus - Fullstack crossplatform app framework for Rust",
            }
            document::Meta {
                name: "twitter:description",
                content: "A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
            }
            document::Meta {
                name: "twitter:image",
                content: "https://dioxuslabs.com/static/opengraph.png",
            }
            document::Meta { name: "twitter:card", content: "summary_large_image" }
            document::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            }
            document::Script {
                r#async: true,
                src: "https://www.googletagmanager.com/gtag/js?id=G-EBE72MVZ1B",
            }
            document::Script {
                r#async: true,
                src: asset!("/assets/gtag.js"),
                r#type: "text/javascript",
            }

            Nav {}
            Outlet::<Route> {}
            Footer {}
        }
    }
}

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Homepage {},

        #[route("/play")]
        Playground {},

        #[route("/awesome")]
        Awesome {},

        #[route("/deploy")]
        Deploy {},

        #[nest("/blog")]
            #[route("/")]
            BlogList {},
            #[route("/release-050")]
            PostRelease050 {},
            #[route("/templates-diffing")]
            PostTemplate {},
            #[route("/going-fulltime")]
            PostFulltime {},
            #[route("/release-040")]
            PostRelease040 {},
            #[route("/release-030")]
            PostRelease030 {},
            #[route("/release-020")]
            PostRelease020 {},
            #[route("/introducing-dioxus")]
            PostRelease010 {},
        #[end_nest]

        #[layout(Learn)]
            #[nest("/learn")]
                #[redirect("/", || Route::Docs05 { child: crate::docs::router_05::BookRoute::Index {} })]

                #[route("/0.3/:..segments")]
                Docs03 {
                    segments: Vec<String>
                },

                #[route("/0.4/:..segments")]
                Docs04 {
                    segments: Vec<String>
                },

                #[child("/0.5")]
                Docs05 { child: crate::docs::router_05::BookRoute },

                #[child("/0.6")]
                Docs06 { child: crate::docs::router_06::BookRoute },

            #[end_nest]
        #[end_layout]
    #[end_nest]
    #[redirect("/docs/0.3/:..segments", |segments: Vec<String>| Route::Docs03 { segments })]
    #[redirect("/docs/:..segments", |segments: Vec<String>| {
        let joined = segments.join("/");
        let docs_route = format!("/docs/{}", joined);
        Route::from_str(&docs_route).unwrap_or_else(|_| Route::Docs06 { child: crate::docs::router_06::BookRoute::Index {} })
    })]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

pub fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        dioxus::launch(app);
    }

    #[cfg(feature = "prebuild")]
    {
        use log::LevelFilter;
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Error)
            .init()
            .unwrap();

        println!("CWD is {}", std::env::current_dir().unwrap().display());

        env::set_var("DIOXUS_OUT_DIR", "./target/dx/dioxus_docs_site/release/web");

        LaunchBuilder::new()
            .with_cfg(dioxus::static_site_generation::Config::new().github_pages())
            .launch(app);

        println!("prebuilt");

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    let mut path = std::path::PathBuf::default();
                    for (_i, segment) in route.split('/').enumerate() {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );

        println!("built search index");
    }
}
