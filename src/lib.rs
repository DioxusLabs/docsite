#![allow(non_snake_case, non_upper_case_globals, unused)]

use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
pub use docs::BookRoute;
use serde::{Deserialize, Serialize};

macro_rules! export_items {
    (
        $(
            pub mod $item:ident;
        )*
    ) => {
        $(
            pub mod $item;
            pub use $item::*;
        )*
    };
}

pub mod icons;
pub mod sitemap;

pub mod shortcut;

mod doc_examples;

pub use components::*;
pub mod components {
    export_items! {
        pub mod blog;
        pub mod footer;
        pub mod homepage;
        pub mod learn;
        pub mod nav;
        pub mod notfound;
        pub mod tutorials;
        pub mod awesome;
        pub mod deploy;
    }
}

#[component]
fn HeaderFooter() -> Element {
    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || {
            SHOW_SEARCH.set(components::nav::true);
        }
    });

    rsx! {
        div {
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
                #[child("/0.4")]
                Docs { child: BookRoute },
            #[end_nest]
        #[end_layout]
    #[end_nest]
    #[redirect("/docs/0.3/:..segments", |segments: Vec<String>| Route::DocsO3 { segments })]
    #[redirect("/docs/:.._segments", |_segments: Vec<String>| Route::Docs { child: BookRoute::Index {} })]
    #[route("/:..segments")]
    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}

pub fn use_url() -> String {
    use_route::<Route>().unwrap().to_string()
}

pub fn app() -> Element {
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
                style {
                    ".dioxus-demo div {{ all: revert; }}"
                    ".dioxus-demo input {{ all: revert; }}"
                    ".dioxus-demo form {{ all: revert; }}"
                }
                {children}
            }
        }
    }

    fn LayoutsExplanation() -> Element {
        let highlight_nav = use_atom_state(&HIGHLIGHT_NAV_LAYOUT);
        let highlight_docs_nav = use_atom_state(&HIGHLIGHT_DOCS_LAYOUT);
        let highlight_docs_content = use_atom_state(&HIGHLIGHT_DOCS_CONTENT);

        rsx! {
            pre {
                onmouseenter: move |_| {
                    highlight_nav.set(true);
                    highlight_docs_nav.set(DocsLayoutHighlighted(true));
                    highlight_docs_content.set(DocsContentHighlighted(true));
                },
                onmouseleave: move |_| {
                    highlight_nav.set(false);
                    highlight_docs_nav.set(DocsLayoutHighlighted(false));
                    highlight_docs_content.set(DocsContentHighlighted(false));
                },
                span {
                    "#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {{\n\t"
                }
                span {
                    onmouseenter: move |_| {
                        highlight_nav.set(true);
                        highlight_docs_nav.set(DocsLayoutHighlighted(false));
                        highlight_docs_content.set(DocsContentHighlighted(false));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(true);
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-orange-600 rounded-md",
                    "#[layout(HeaderFooter)]"
                }
                span { "\n\t\t// ... other routes\n\t\t" }
                span {
                    onmouseenter: move |_| {
                        highlight_nav.set(false);
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(false));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(true);
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-green-600 rounded-md",
                    r##"#[layout(DocsSidebars)]"##
                }
                "\n\t\t\t"
                span {
                    onmouseenter: move |_| {
                        highlight_nav.set(false);
                        highlight_docs_nav.set(DocsLayoutHighlighted(false));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(true);
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-blue-600 rounded-md",
                    r##"#[route("/learn")]"##
                }
                span { "\n\t\t\tDocs {{}},\n}}" }
            }
        }
    }

    use_mdbook::mdbook_router! {"docs-src/0.4"}
}
