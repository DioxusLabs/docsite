#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod docs;
pub mod icons;
#[cfg(feature = "server")]
pub mod og_image;
pub mod seo;
pub mod shortcut;
pub mod snippets;
pub use components::*;

fn main() {
    let _class = rsx! {
        div { class: "mx-2 mx-4" }
    };

    // If we are just building the search index, we don't need to launch the app
    #[cfg(feature = "server")]
    if std::env::args().any(|arg| arg == "--generate-search-index") {
        llms::generate_llms_txt();
        search::generate_search_index();
        og_image::generate_og_images();
        return;
    }

    create_sitemap();

    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder().incremental(
                dioxus::server::IncrementalRendererConfig::new()
                    .static_dir(static_dir())
                    .clear_cache(false)
            )
        })
        .launch(|| {
            rsx! {
                Router::<Route> {}
            }
        });
}

#[component]
fn HeaderLayout() -> Element {
    let cb = use_callback(|_| *SHOW_SEARCH.write() = true);

    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || cb.call(())
    });

    rsx! {
        div { class: "bg-white dark:bg-black min-h-screen",
            Nav {}
            div { Outlet::<Route> {} }
        }
    }
}

#[component]
fn FooterLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
        Footer {}
    }
}

fn HeadLayout() -> Element {
    rsx! {
        Head {}
        Outlet::<Route> {}
    }
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

    // Tell google to not index old documentation
    let current_doc_route = use_route::<Route>();
    let meta = seo::PageMeta::for_route(&current_doc_route);
    // Generated per-page images live at a stable unhashed path; the default
    // image goes through asset!() which serves it at a hashed path
    let og_image = if seo::has_generated_og_image(&current_doc_route) {
        meta.image.clone()
    } else {
        format!(
            "{}{}",
            seo::SITE_URL,
            asset!("/assets/static/opengraph.png")
        )
    };
    let og_type = match meta.kind {
        seo::PageKind::Article { .. } => "article",
        seo::PageKind::Website => "website",
    };

    rsx! {
        Title { "{meta.title}" }
        Meta {
            name: "description",
            content: "{meta.description}",
        }
        Link { rel: "canonical", href: "{meta.url}" }
        Link {
            rel: "icon shortcut",
            r#type: "image/png",
            href: asset!("/assets/static/favicon.png"),
        }
        Stylesheet { href: asset!("/assets/tailwind.css", CssAssetOptions::new().with_minify(false)) }
        Stylesheet { href: asset!("/assets/main.css") }
        // Stylesheet { href: "https://rsms.me/inter/inter.css" }

        // link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        // link {
        //     crossorigin: "false",
        //     href: "https://fonts.gstatic.com",
        //     rel: "preconnect",
        // }
        // link {
        //     href: "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300..700&display=swap",
        //     rel: "stylesheet",
        // }

        Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        Link {
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
            crossorigin: "false",
        }
        Meta {
            property: "og:title",
            content: "{meta.title}",
        }
        Meta { property: "og:type", content: og_type }
        Meta {
            property: "og:description",
            content: "{meta.description}",
        }
        Meta { property: "og:url", content: "{meta.url}" }
        Meta { property: "og:image", content: "{og_image}" }
        Meta { property: "og:image:width", content: "1200" }
        Meta { property: "og:image:height", content: "630" }
        Meta { property: "og:image:alt", content: "{meta.title}" }
        Meta { property: "og:site_name", content: "Dioxus" }
        Meta {
            name: "twitter:title",
            content: "{meta.title}",
        }
        Meta {
            name: "twitter:description",
            content: "{meta.description}",
        }
        Meta {
            name: "twitter:image",
            content: "{og_image}",
        }
        Meta { name: "twitter:card", content: "summary_large_image" }
        Meta { name: "twitter:site", content: "@dioxuslabs" }
        if let seo::PageKind::Article { published, author } = meta.kind {
            if let Some(published) = published {
                Meta {
                    property: "article:published_time",
                    content: "{published}",
                }
            }
            Meta {
                property: "article:author",
                content: author,
            }
        }
        Script {
            r#async: true,
            src: "https://www.googletagmanager.com/gtag/js?id=G-EBE72MVZ1B",
        }
        Script {
            r#async: true,
            src: asset!("/assets/gtag.js"),
            r#type: "text/javascript",
        }
        if meta.noindex {
            Meta { name: "robots", content: "noindex" }
        }
    }
}

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[rustfmt::skip]
pub enum Route {
    // #[layout(HeadLayout)]
    // #[layout(HeaderLayout)]
    // #[layout(FooterLayout)]
    #[layout(HeaderFooter)]
        #[route("/")]
        Homepage {},

        // #[route("/playground")]
        // Playground {},

        // #[route("/playground/shared/:share_code")]
        // SharePlayground { share_code: String },



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
                #[redirect("/", || Route::Docs07 { child: crate::docs::router_07::BookRoute::Index { section: Default::default() } })]
                #[child("/0.7")]
                Docs07 { child: crate::docs::router_07::BookRoute },

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

        // #[layout(!FooterLayout)]
    #[route("/components")]
    Components { },

    // #[redirect("/docs/:..segments", |segments: Vec<String>| {
    //     let joined = segments.join("/");
    //     let child = crate::docs::router_06::BookRoute::from_str(&joined).unwrap_or_else(|_| crate::docs::router_06::BookRoute::Index { section: Default::default() });
    //     Route::Docs06 { child }
    // })]
    // #[redirect("/docs/:..segments", |segments: Vec<String>| {
    //     let joined = segments.join("/");
    //     let docs_route = format!("/{}", joined.trim_matches('/'));
    //     let child = crate::docs::router_06::BookRoute::from_str(&docs_route).unwrap_or_else(|_| crate::docs::router_06::BookRoute::Index { section: Default::default() });
    //     Route::Docs06 { child }
    // })]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

impl Route {
    fn is_docs(&self) -> bool {
        matches!(
            self,
            Route::Docs07 { .. }
                | Route::Docs06 { .. }
                | Route::Docs05 { .. }
                | Route::Docs04 { .. }
                | Route::Docs03 { .. }
        )
    }

    fn is_latest_docs(&self) -> bool {
        matches!(self, Route::Docs07 { .. })
    }
}

/// The active theme for the site.
pub(crate) static DARK_MODE: GlobalSignal<Option<bool>> = Signal::global(|| None);
pub(crate) fn dark_mode() -> bool {
    DARK_MODE().unwrap_or_default()
}

#[cfg(feature = "fullstack")]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut static_routes = Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>();

    // Add the components preview routes manually
    static_routes.push("/components/".to_string());
    static_routes.push("/components/component/".to_string());

    Ok(static_routes)
}

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

fn create_sitemap() {
    #[cfg(feature = "production")]
    server_only! {
        use std::io::Write;

        // Write a sitemap file on the server
        // The sitemap helps with SEO because google will deprioritize pages it finds that are not in the sitemap
        let all_routes = Route::static_routes();
        _ = std::fs::create_dir_all(static_dir());
        let output_path = static_dir().join("sitemap.xml");
        let Ok(file) = std::fs::File::create(output_path) else {
            eprintln!("Failed to create sitemap file");
            return;
        };
        let mut writer = std::io::BufWriter::new(file);
        _ = writeln!(writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        _ = writeln!(writer, r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        for route in all_routes {
            // If the documentation is out of date, don't include it in the sitemap
            if route.is_docs() && !route.is_latest_docs() {
                continue;
            }
            _ = writeln!(writer, r#"<url>"#);
            let url = format!("https://dioxuslabs.com{}", route);
            let escaped_url = askama_escape::escape(&url, askama_escape::Html);
            _ = writeln!(writer, r#"    <loc>{}</loc>"#, escaped_url);
            _ = writeln!(writer, r#"</url>"#);
        }
        _ = writeln!(writer, r#"</urlset>"#);

        // Point to the sitemap file in the robots.txt
        _ = std::fs::write(static_dir().join("robots.txt"), format!("Sitemap: https://dioxuslabs.com/sitemap.xml"));
    }
}
