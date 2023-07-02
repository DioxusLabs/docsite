#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};

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
        pub mod awesome;
    }
}

pub fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let show_nav = use_set(cx, SHOW_NAV);

    cx.render(rsx! {
        Router { onchange: move |_| show_nav(false),
            Nav {}
            Route { to: "/", Homepage {} }
            Route { to: "/index.html", Homepage {} }
            Route { to: "/awesome", Awesome {}}
            Route { to: "/learn", Learn {} }
            Route { to: "/platforms/", Homepage {} }
            Route { to: "/platforms/web", Homepage {} }
            Route { to: "/platforms/desktop", Homepage {} }
            Route { to: "/platforms/liveview", Homepage {} }
            Route { to: "/platforms/mobile", Homepage {} }
            Route { to: "/platforms/ssr", Homepage {} }
            Route { to: "/platforms/tui", Homepage {} }
            Route { to: "/tutorials/:id", Tutorial {} }
            // Route { to: "/tutorials/", Tutorials {} }

            Route { to: "/blog", BlogList {} }
            Route { to: "/blog/", BlogList {} }
            Route { to: "/blog/going-fulltime", SinglePost { post: POST_FULLTINME } }
            Route { to: "/blog/release-030", SinglePost { post: POST_RELEASE_030 } }
            Route { to: "/blog/introducing-dioxus", SinglePost { post: POST_RELEASE_010 } }
            Route { to: "/blog/release-020", SinglePost { post: POST_RELEASE_020 } }
            Route { to: "/blog/templates-diffing/", SinglePost { post: POST_TEMPLATE } }
            Route { to: "", Err404 {} }
        }
        Footer {}
    })
}
