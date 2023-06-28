#![allow(non_snake_case, non_upper_case_globals)]

use crate::components::blog::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

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

pub use components::*;
use fermi::{use_init_atom_root, use_set};
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

pub fn app(cx: Scope) -> Element {
    render! {
        Router {}
    }
}

#[inline_props]
fn HeaderFooter(cx: Scope) -> Element {
    use_init_atom_root(cx);

    render !{
        Nav {}
        Outlet {}
        Footer {}
    }
}
use docs::BookRoute;

mod docs {
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"./docs"}
}

#[derive(Clone, Routable, PartialEq, Eq)]
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

        #[route("/tutorials/:id")]
        Tutorial { id: usize },

        #[nest("/blog")]
            #[route("/")]
            BlogList {},
            #[route("/going-fulltime")]
            GoingFulltime {},
            #[route("/release-030")]
            Release030 {},
            #[route("/release-020")]
            Release020 {},
            #[route("/introducing-dioxus")]
            IntroducingDioxus {},
            #[route("/templates-diffing/")]
            TemplatesDiffing {},
        #[end_nest]
        
        #[route("/:...segments")]
        Err404 { segments: Vec<String> },
        #[layout(Learn)]
            #[child("/learn")]
            Docs {
                child: BookRoute
            },
}

pub fn use_url(cx: &ScopeState) -> String {
    use_route(cx).unwrap().to_string()
}
