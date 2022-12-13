#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};

pub mod icons;
pub mod sitemap;
pub mod components {
    pub mod blog;
    pub mod footer;
    pub mod homepage;
    pub mod learn;
    pub mod nav;
    pub mod notfound;
}

use components::blog::*;
use components::homepage::Homepage;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            components::nav::Nav {}
            Route { to: "/", Homepage {} }
            Route { to: "/index.html", Homepage {} }
            Route { to: "/learn", components::learn::Learn {} }
            Route { to: "/platforms/", Homepage {} }
            Route { to: "/platforms/web", Homepage {} }
            Route { to: "/platforms/desktop", Homepage {} }
            Route { to: "/platforms/liveview", Homepage {} }
            Route { to: "/platforms/mobile", Homepage {} }
            Route { to: "/platforms/ssr", Homepage {} }
            Route { to: "/platforms/tui", Homepage {} }

            Route { to: "/blog", BlogList {} }
            Route { to: "/blog/", BlogList {} }
            Route { to: "/blog/introducing-dioxus", components::blog::SinglePost { post: components::blog::POST_RELEASE_010 } }
            Route { to: "/blog/release-020", components::blog::SinglePost { post: components::blog::POST_RELEASE_020 } }
            Route { to: "/blog/templates-diffing/", components::blog::SinglePost { post: components::blog::POST_TEMPLATE } }
            Route { to: "", components::notfound::Err404 {} }
        }
        crate::components::footer::Footer {}
    })
}
