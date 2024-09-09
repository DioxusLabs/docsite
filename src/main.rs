#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
pub use docs::BookRoute;
use serde::{Deserialize, Serialize};

pub mod components;
pub mod doc_examples;
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
        div { class: "bg-white dark:bg-ideblack pb-8",
            head::Link { rel: "stylesheet", href: "/githubmarkdown.css" }
            head::Link { rel: "stylesheet", href: "/tailwind.css" }
            head::Link { rel: "stylesheet", href: "/main.css" }
            head::Link { rel: "stylesheet", href: "/dxp.css" }
            head::Link { rel: "stylesheet", href: "https://rsms.me/inter/inter.css" }
            head::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            }
            head::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
            head::Link {
                href: "https://fonts.gstatic.com",
                rel: "preconnect",
                crossorigin: "false",
            }
            head::Link {
                rel: "stylesheet",
                href: "https://fonts.googleapis.com/css2?family=Arimo:wght@100;400;600&display=swap",
            }
            head::Link {
                href: "https://fonts.googleapis.com/css2?family=Arimo:ital,wght@0,400..700;1,400..700&family=Lexend:wght@100;400&family=M+PLUS+1:wght@100..900&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap",
                rel: "stylesheet",
            }
            head::Script { src: "/ace/ace.js" }
            head::Script { src: "/ace/mode-rust.js" }
            head::Script { src: "/ace/theme-github.js" }

            Nav {}
            Outlet::<Route> {}
        }
        Footer {}
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

pub fn app() -> Element {
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
    fn CodeBlock(contents: String) -> Element {
        rsx! {
            div {
                style: "position: relative;",
                div {
                    dangerous_inner_html: contents
                }
                button {
                    style: "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                    "onclick": "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                    "Copy"
                }
            }
        }
    }

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
            pre { onmouseenter: move |_| {}, onmouseleave: move |_| {},
                span {
                    "#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {{\n\t"
                }
                span {
                    onmouseenter: move |_| {},
                    onmouseleave: move |_| {},
                    class: "border border-orange-600 rounded-md",
                    "#[layout(HeaderFooter)]"
                }
                span { "\n\t\t// ... other routes\n\t\t" }
                span {
                    onmouseenter: move |_| {},
                    onmouseleave: move |_| {},
                    class: "border border-green-600 rounded-md",
                    r##"#[layout(DocsSidebars)]"##
                }
                "\n\t\t\t"
                span {
                    onmouseenter: move |_| {},
                    onmouseleave: move |_| {},
                    class: "border border-blue-600 rounded-md",
                    r##"#[route("/learn")]"##
                }
                span { "\n\t\t\tDocs {{}},\n}}" }
            }
        }
    }

    mod router;
    pub use router::*;
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

    #[allow(deprecated)]
    #[cfg(not(feature = "prebuild"))]
    launch(app);
}
