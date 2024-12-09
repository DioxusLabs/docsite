#![allow(non_snake_case, non_upper_case_globals)]

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

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            // Only in release do we SSG
            let mut cfg = ServeConfig::builder();

            if !cfg!(debug_assertions) {
                cfg = cfg.incremental(
                    IncrementalRendererConfig::new()
                        .static_dir(static_dir())
                        .clear_cache(false)
                );
            }

            cfg.build().expect("Unable to build ServeConfig")
        })
        .launch(|| {
            rsx! {
                Router::<Route> {}
            }
        });
}

#[component]
fn HeaderFooter() -> Element {
    let cb = use_callback(|_| *SHOW_SEARCH.write() = true);

    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || cb.call(())
    });

    rsx! {
        Head {}
        div { class: "bg-white dark:bg-black min-h-screen",
            Nav {}
            div {
                Outlet::<Route> {}
                Footer {}
            }
        }
    }
}

fn Head() -> Element {
    use document::{Link, Meta, Script, Stylesheet, Title};

    rsx! {
        Title { "Dioxus | Fullstack crossplatform app framework for Rust" }
        Meta {
            name: "description",
            content: "Dioxus | A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
        }
        Link {
            rel: "icon shortcut",
            r#type: "image/png",
            href: asset!("/assets/static/favicon.png"),
        }
        Stylesheet { href: asset!("/assets/githubmarkdown.css") }
        Stylesheet { href: asset!("/assets/tailwind.css", CssAssetOptions::new().with_minify(false)) }
        Stylesheet { href: asset!("/assets/main.css") }
        Stylesheet { href: asset!("/assets/dxp.css") }
        Stylesheet { href: asset!("/assets/material.css") }
        Stylesheet { href: "https://rsms.me/inter/inter.css" }
        Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        Link {
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
            crossorigin: "false",
        }
        Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Arimo:wght@100;400;600&display=swap",
        }
        Link {
            href: "https://fonts.googleapis.com/css2?family=Arimo:ital,wght@0,400..700;1,400..700&family=Lexend:wght@100;400&family=M+PLUS+1:wght@100..900&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
            rel: "stylesheet",
        }
        Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
        }
        Script { src: asset!("/assets/ace/ace.js") }
        Script { src: asset!("/assets/ace/mode-rust.js") }
        Script { src: asset!("/assets/ace/theme-github.js") }
        Meta {
            property: "og:title",
            content: "Dioxus | Fullstack crossplatform app framework for Rust",
        }
        Meta { property: "og:type", content: "website" }
        Meta {
            property: "og:description",
            content: "A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
        }
        Meta { property: "og:url", content: "https://dioxuslabs.com" }
        Meta {
            property: "og:image",
            content: "https://dioxuslabs.com/static/opengraph.png",
        }
        Meta {
            name: "twitter:title",
            content: "Dioxus - Fullstack crossplatform app framework for Rust",
        }
        Meta {
            name: "twitter:description",
            content: "Dioxus | A fullstack crossplatform app framework for Rust. Supports Web, Desktop, SSR, Liveview, and Mobile.",
        }
        Meta {
            name: "twitter:image",
            content: "https://dioxuslabs.com/static/opengraph.png",
        }
        Meta { name: "twitter:card", content: "summary_large_image" }
        Script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id=G-EBE72MVZ1B",
        }
        Script {
            r#async: true,
            src: asset!("/assets/gtag.js"),
            r#type: "text/javascript",
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
            #[layout(BlogPost)]
                #[child("")]
                BlogPost { child: crate::docs::router_blog::BookRoute },
            #[end_layout]
        #[end_nest]

        #[layout(Learn)]
            #[nest("/learn")]
                #[redirect("/", || Route::Docs05 { child: crate::docs::router_05::BookRoute::Index {} })]
                #[child("/0.6")]
                Docs06 { child: crate::docs::router_06::BookRoute },

                #[child("/0.5")]
                Docs05 { child: crate::docs::router_05::BookRoute },

                #[child("/0.4")]
                Docs04 { child: crate::docs::router_04::BookRoute },

                #[child("/0.3")]
                Docs03 { child: crate::docs::router_03::BookRoute },
            #[end_nest]
        #[end_layout]
    #[end_nest]

    // once all the routes above have been generated, we build the search index
    // a bit of a hack, sorry
    #[route("/search")]
    Search {},

    #[redirect("/docs/:..segments", |segments: Vec<String>| {
        let joined = segments.join("/");
        let docs_route = format!("/docs/{}", joined);
        Route::from_str(&docs_route).unwrap_or_else(|_| Route::Docs06 { child: crate::docs::router_06::BookRoute::Index {} })
    })]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

impl Route {
    fn is_docs(&self) -> bool {
        matches!(
            self,
            Route::Docs06 { .. }
                | Route::Docs05 { .. }
                | Route::Docs04 { .. }
                | Route::Docs03 { .. }
        )
    }
}

// todo - when we update to 0.6.0 we need to change this to return a Vec<String>
#[cfg(feature = "fullstack")]
#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}
