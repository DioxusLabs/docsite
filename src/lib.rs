#![allow(non_snake_case, non_upper_case_globals)]

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
use fermi::use_init_atom_root;
pub mod components {
    export_items! {
        pub mod blog;
        pub mod footer;
        pub mod homepage;
        pub mod learn;
        pub mod nav;
        pub mod notfound;
        pub mod tutorials;
    }
}

#[inline_props]
fn HeaderFooter(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let set_search = fermi::use_set(cx, &SHOW_SEARCH);
    shortcut::use_shortcut(cx, Key::Character("k".to_string()), Modifiers::CONTROL, {
        to_owned![set_search];
        move || {
            set_search(components::nav::ShowSearch(true));
        }
    });

    render! {
        div {
            Nav {}
            Outlet {}
            Footer {}
        }
    }
}

mod docs {
    use crate::components::*;
    use crate::doc_examples::*;
    use dioxus::prelude::*;
    use dioxus_router::prelude::*;
    use fermi::use_atom_state;

    use_mdbook::mdbook_router! {"./docs-src"}

    #[inline_props]
    fn DemoFrame<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
        render! {
            div {
                class: "bg-white rounded-md shadow-md p-4 my-4 overflow-scroll text-black",
                max_height: "50vh",
                children
            }
        }
    }

    fn LayoutsExplanation(cx: Scope) -> Element {
        let highlight_nav = use_atom_state(cx, &HIGHLIGHT_NAV_LAYOUT);
        let highlight_docs_nav = use_atom_state(cx, &HIGHLIGHT_DOCS_LAYOUT);
        let highlight_docs_content = use_atom_state(cx, &HIGHLIGHT_DOCS_CONTENT);

        render! {
            pre {
                onmouseenter: move |_| {
                    highlight_nav.set(NavLayoutHighlighted(true));
                    highlight_docs_nav.set(DocsLayoutHighlighted(true));
                    highlight_docs_content.set(DocsContentHighlighted(true));
                },
                onmouseleave: move |_| {
                    highlight_nav.set(NavLayoutHighlighted(false));
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
                        highlight_nav.set(NavLayoutHighlighted(true));
                        highlight_docs_nav.set(DocsLayoutHighlighted(false));
                        highlight_docs_content.set(DocsContentHighlighted(false));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(NavLayoutHighlighted(true));
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-orange-600 rounded-md",
                    "#[layout(HeaderFooter)]"
                }
                span {
                    "\n\t\t// ... other routes\n\t\t"
                }
                span {
                    onmouseenter: move |_| {
                        highlight_nav.set(NavLayoutHighlighted(false));
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(false));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(NavLayoutHighlighted(true));
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-green-600 rounded-md",
                    r##"#[layout(DocsSidebars)]"##
                }
                "\n\t\t\t"
                span {
                    onmouseenter: move |_| {
                        highlight_nav.set(NavLayoutHighlighted(false));
                        highlight_docs_nav.set(DocsLayoutHighlighted(false));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    onmouseleave: move |_| {
                        highlight_nav.set(NavLayoutHighlighted(true));
                        highlight_docs_nav.set(DocsLayoutHighlighted(true));
                        highlight_docs_content.set(DocsContentHighlighted(true));
                    },
                    class: "border border-blue-600 rounded-md",
                    r##"#[route("/learn")]"##
                }
                span {
                    "\n\t\t\tDocs {{}},\n}}"
                }
            }
        }
    }
}

mod blog_posts {
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"./blog"}
}

pub use blog_posts::BookRoute as BlogRoute;

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(HeaderFooter)]
        #[nest("/docsite")]
            #[route("/")]
            #[redirect("/platforms", || Route::Homepage {})]
            #[redirect("/platforms/web", || Route::Homepage {})]
            #[redirect("/platforms/desktop", || Route::Homepage {})]
            #[redirect("/platforms/liveview", || Route::Homepage {})]
            #[redirect("/platforms/mobile", || Route::Homepage {})]
            #[redirect("/platforms/ssr", || Route::Homepage {})]
            #[redirect("/platforms/tui", || Route::Homepage {})]
            Homepage {},

            #[route("/tutorials/:id")]
            Tutorial { id: usize },

            #[nest("/blog")]
                #[route("/")]
                BlogList {},
                #[layout(BlogPost)]
                    #[child("/")]
                    Blog { child: BlogRoute },
                #[end_layout]
            #[end_nest]
            #[layout(Learn)]
                #[child("/learn")]
                Docs { child: BookRoute },
            #[end_layout]
        #[end_nest]
        #[route("/:...segments")]
        Err404 { segments: Vec<String> },
}

pub fn use_url(cx: &ScopeState) -> String {
    use_route(cx).unwrap().to_string()
}

pub fn app(cx: Scope) -> Element {
    render! {
        Router {}
    }
}
