#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod icons;
pub mod sitemap;
pub mod components {
    pub mod blog;
    pub mod footer;
    pub mod homepage;
    pub mod nav;
    pub mod notfound;
    pub mod snippets;
}

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            components::nav::Nav {},

            // Variosu homepage entrypoints
            Route { to: "/", components::homepage::Homepage {} }
            Route { to: "/index.html", components::homepage::Homepage {} }

            // Showcase all our platforms
            Route { to: "/platforms/", components::homepage::Homepage {} }
            Route { to: "/platforms/web", components::homepage::Homepage {} }
            Route { to: "/platforms/desktop", components::homepage::Homepage {} }
            Route { to: "/platforms/liveview", components::homepage::Homepage {} }
            Route { to: "/platforms/mobile", components::homepage::Homepage {} }
            Route { to: "/platforms/ssr", components::homepage::Homepage {} }
            Route { to: "/platforms/tui", components::homepage::Homepage {} }

            // Blog is done manually for now :)
            Route { to: "/blog", components::blog::BlogList {} }
            Route { to: "/blog/", components::blog::BlogList {} }
            Route { to: "/blog/introducing-dioxus/", components::blog::single_blog_post() }

            Route { to: "", components::notfound::Err404 {} }
        }
        crate::components::footer::Footer {}
    })
}
