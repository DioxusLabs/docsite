#![allow(non_snake_case, non_upper_case_globals, unused)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
pub(crate) use docs::BookRoute;
use serde::{Deserialize, Serialize};

mod doc_examples;
pub(crate) mod icons;
pub(crate) mod shortcut;
pub(crate) mod sitemap;
mod snippets;

pub(crate) use components::*;
pub(crate) mod components {
    pub(crate) mod awesome;
    pub use awesome::*;
    pub(crate) mod blog;
    pub use blog::*;
    pub(crate) mod deploy;
    pub use deploy::*;
    pub(crate) mod desktop_dependencies;
    pub use desktop_dependencies::*;
    pub(crate) mod footer;
    pub use footer::*;
    pub(crate) mod homepage;
    pub use homepage::*;
    pub(crate) mod learn;
    pub use learn::*;
    pub(crate) mod nav;
    pub use nav::*;
    pub(crate) mod notfound;
    pub use notfound::*;
    pub(crate) mod tutorials;
    pub use tutorials::*;
}

#[component]
fn HeaderFooter() -> Element {
    let cb = use_callback(|_| {
        *SHOW_SEARCH.write() = true;
    });

    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || {
            cb.call(());
        }
    });

    rsx! {
        div { class: "bg-white dark:bg-ideblack pb-8",
            head::Link { rel: "stylesheet", href: "/githubmarkdown.css" }
            head::Link { rel: "stylesheet", href: "/tailwind.css" }
            head::Link { rel: "stylesheet", href: "/main.css" }
            head::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            }
            Nav {}
            Outlet::<Route> {}
        }
        Footer {}
    }
}

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        #[redirect("/platforms", || Route::Homepage {})]
        #[redirect("/platforms/web", || Route::Homepage {})]
        #[redirect("/platforms/desktop", || Route::Homepage {})]
        #[redirect("/platforms/liveview", || Route::Homepage {})]
        #[redirect("/platforms/mobile", || Route::Homepage {})]
        #[redirect("/platforms/ssr", || Route::Homepage {})]
        #[redirect("/platforms/tui", || Route::Homepage {})]
        Homepage {},

        #[route("/awesome")]
        Awesome {},

        #[route("/deploy")]
        Deploy {},

        #[route("/tutorials/:id")]
        Tutorial { id: usize },

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
                #[redirect("/", || Route::Docs { child: BookRoute::Index {} })]

                #[route("/0.3/:..segments")]
                DocsO3 {
                    segments: Vec<String>
                },
                #[route("/0.4/:..segments")]
                DocsO4 {
                    segments: Vec<String>
                },
                #[child("/0.5")]
                Docs { child: BookRoute },
            #[end_nest]
        #[end_layout]
    #[end_nest]
    #[redirect("/docs/0.3/:..segments", |segments: Vec<String>| Route::DocsO3 { segments })]
    #[redirect("/docs/:..segments", |segments: Vec<String>| {
        let joined = segments.join("/");
        let docs_route = format!("/docs/{}", joined);
        Route::from_str(&docs_route).unwrap_or_else(|_| Route::Docs { child: BookRoute::Index {} })
    })]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

pub(crate) fn use_url() -> String {
    use_route::<Route>().to_string()
}

pub(crate) fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};
mod docs {
    use crate::components::*;
    use crate::doc_examples::*;
    use dioxus::prelude::*;

    #[component]
    fn SandBoxFrame(url: String) -> Element {
        rsx! {
            iframe {
                style: "border: 1px solid rgba(0, 0, 0, 0.1);border-radius:2px;",
                width: "800",
                height: "450",
                src: "{url}?embed=1",
                "allowfullscreen": true,
            }
        }
    }

    #[component]
    fn DemoFrame(children: Element) -> Element {
        rsx! {
            div {
                class: "bg-white rounded-md shadow-md p-4 my-4 overflow-scroll text-black dioxus-demo",
                max_height: "50vh",
                {children}
            }
        }
    }

    fn LayoutsExplanation() -> Element {
        rsx! {
            pre {
                onmouseenter: move |_| {
                    *HIGHLIGHT_NAV_LAYOUT.write() = true;
                    *HIGHLIGHT_DOCS_LAYOUT.write() = true;
                    *HIGHLIGHT_DOCS_CONTENT.write() = true;
                },
                onmouseleave: move |_| {
                    *HIGHLIGHT_NAV_LAYOUT.write() = false;
                    *HIGHLIGHT_DOCS_LAYOUT.write() = false;
                    *HIGHLIGHT_DOCS_CONTENT.write() = false;
                },
                span {
                    "#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {{\n\t"
                }
                span {
                    onmouseenter: move |_| {
                        *HIGHLIGHT_NAV_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_LAYOUT.write() = false;
                        *HIGHLIGHT_DOCS_CONTENT.write() = false;
                    },
                    onmouseleave: move |_| {
                        *HIGHLIGHT_NAV_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_CONTENT.write() = true;
                    },
                    class: "border border-orange-600 rounded-md",
                    "#[layout(HeaderFooter)]"
                }
                span { "\n\t\t// ... other routes\n\t\t" }
                span {
                    onmouseenter: move |_| {
                        *HIGHLIGHT_DOCS_LAYOUT.write() = true;
                        *HIGHLIGHT_NAV_LAYOUT.write() = false;
                        *HIGHLIGHT_DOCS_CONTENT.write() = false;
                    },
                    onmouseleave: move |_| {
                        *HIGHLIGHT_NAV_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_CONTENT.write() = true;
                    },
                    class: "border border-green-600 rounded-md",
                    r##"#[layout(DocsSidebars)]"##
                }
                "\n\t\t\t"
                span {
                    onmouseenter: move |_| {
                        *HIGHLIGHT_NAV_LAYOUT.write() = false;
                        *HIGHLIGHT_DOCS_LAYOUT.write() = false;
                        *HIGHLIGHT_DOCS_CONTENT.write() = true;
                    },
                    onmouseleave: move |_| {
                        *HIGHLIGHT_NAV_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_LAYOUT.write() = true;
                        *HIGHLIGHT_DOCS_CONTENT.write() = true;
                    },
                    class: "border border-blue-600 rounded-md",
                    r##"#[route("/learn")]"##
                }
                span { "\n\t\t\tDocs {{}},\n}}" }
            }
        }
    }

    use_mdbook::mdbook_router! {"docs-src/0.5"}
}

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

        std::env::remove_var("DIOXUS_ACTIVE");
        std::env::remove_var("CARGO");

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
