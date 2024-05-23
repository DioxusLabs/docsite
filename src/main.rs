#![allow(non_snake_case, non_upper_case_globals, unused)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
pub(crate) use docs::BookRoute;
use serde::{Deserialize, Serialize};

macro_rules! export_items {
    (
        $(
            pub(crate) mod $item:ident;
        )*
    ) => {
        $(
            pub(crate) mod $item;
            pub(crate) use $item::*;
        )*
    };
}

pub(crate) mod icons;
pub(crate) mod sitemap;

pub(crate) mod shortcut;

mod doc_examples;
mod snippets;

pub(crate) use components::*;
pub(crate) mod components {
    export_items! {
        pub(crate) mod blog;
        pub(crate) mod footer;
        pub(crate) mod homepage;
        pub(crate) mod learn;
        pub(crate) mod nav;
        pub(crate) mod notfound;
        pub(crate) mod tutorials;
        pub(crate) mod awesome;
        pub(crate) mod deploy;
        pub(crate) mod desktop_dependencies;
    }
}

#[component]
fn HeaderFooter() -> Element {
    let cb = use_callback(|| {
        *SHOW_SEARCH.write() = true;
    });

    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || {
            cb.call();
        }
    });

    rsx! {
        div { class: "bg-white dark:bg-ideblack pb-8",
            link { rel: "stylesheet", href: "/githubmarkdown.css" }
            link { rel: "stylesheet", href: "/tailwind.css" }
            link { rel: "stylesheet", href: "/main.css" }
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
        let docs_route = format!("/learn/0.5/{}", joined);
        Route::from_str(&docs_route).unwrap_or_else(|_| Route::Docs { child: BookRoute::Index {} })
    })]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

pub(crate) fn use_url() -> String {
    use_route::<Route>().to_string()
}

pub(crate) fn app() -> Element {
    rsx! { Router::<Route> {} }
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
                "allowfullscreen": true
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
    #[cfg(feature = "canonicalize")]
    {
        canonicalize();
    }

    #[cfg(not(any(feature = "prebuild", feature = "canonicalize")))]
    launch(app);
}

#[cfg(feature = "canonicalize")]
fn canonicalize() {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;

    // Look through all the old docs and add links to the new docs. This helps with SEO.
    // Walk the /docs/* and /learn/* directories and try to add a canonical link to the new docs
    let prefixes = [
        PathBuf::from("./docs/0.3"),
        PathBuf::from("./docs/nightly"),
        PathBuf::from("./learn/0.3"),
        PathBuf::from("./learn/0.4"),
    ];
    let prefixes = prefixes
        .iter()
        .filter_map(|p| p.canonicalize().ok())
        .collect::<Vec<_>>();
    let mut queued_paths = prefixes.clone();

    while let Some(path) = queued_paths.pop() {
        let read = std::fs::read_dir(path).into_iter().flatten().flatten();
        for entry in read {
            let path = entry.path();
            let path = path.canonicalize().unwrap();
            if path.is_dir() {
                queued_paths.push(path);
            } else {
                // First remove any old canonical links
                let mut file = File::open(&path).unwrap();
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    // We only care about paths that are relative to one of the old doc directories
                    if let Some(relative_path) = prefixes
                        .iter()
                        .find_map(|p| path.strip_prefix(p).map(|p| p.to_path_buf()).ok())
                    {
                        // Remove anything between <!-- CANONICAL --> and <!-- END CANONICAL -->
                        let end_needle = "<!-- END CANONICAL -->";
                        if let (Some(start), Some(end)) = (
                            contents.find("<!-- CANONICAL -->"),
                            contents.find(end_needle),
                        ) {
                            contents.replace_range(start..(end + end_needle.len()), "");
                        }

                        // Try to parse the path as a path to the new docs
                        let mut path_as_string = String::from("/docs/");
                        for component in relative_path.components() {
                            if let std::path::Component::Normal(component) = component {
                                if component.to_string_lossy() == "index.html" {
                                    continue;
                                }
                                path_as_string.push_str(&component.to_string_lossy());
                                path_as_string.push('/');
                            }
                        }

                        if let Ok(route @ Route::Docs { .. }) = path_as_string.parse::<Route>() {
                            // Then add the canonical link to the head
                            let needle = "</head>";
                            let start = contents.find(needle).unwrap();
                            contents.insert_str(start, format!("<!-- CANONICAL --><link rel=\"canonical\" href=\"https://dioxuslabs.com{}\" /><!-- END CANONICAL -->", route.to_string()).as_str());
                        }

                        // Then write the file with the new canonical link
                        let mut file = File::create(path).unwrap();
                        file.write_all(contents.as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}
