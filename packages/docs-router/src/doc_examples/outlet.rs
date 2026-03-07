#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: outlet
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Index {},
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        header { "header" }
        // The index route will be rendered here
        Outlet::<Route> {}
        footer { "footer" }
    }
}

#[component]
fn Index() -> Element {
    rsx! { h1 { "Index" } }
}
// ANCHOR_END: outlet

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

fn main() {
    let mut vdom = VirtualDom::new(App);
    vdom.rebuild_in_place();
    let html = dioxus_ssr::render(&vdom);
    assert_eq!(
        html,
        "<header>header</header><h1>Index</h1><footer>footer</footer>"
    );
}

mod with_props {
    use dioxus::prelude::*;

    // ANCHOR: outlet_with_params
    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[nest("/:name")]
            #[layout(Wrapper)]
                #[route("/")]
                Index {
                    name: String,
                },
    }

    #[component]
    fn Wrapper(name: String) -> Element {
        rsx! {
            header { "Welcome {name}!" }
            // The index route will be rendered here
            Outlet::<Route> {}
            footer { "footer" }
        }
    }

    #[component]
    fn Index(name: String) -> Element {
        rsx! { h1 { "This is a homepage for {name}" } }
    }
    // ANCHOR_END: outlet_with_params

    fn App() -> Element {
        rsx! { Router::<Route> {} }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        vdom.rebuild_in_place();
        let html = dioxus_ssr::render(&vdom);
        assert_eq!(
            html,
            "<header></header><h1>Index</h1><footer>footer</footer>"
        );
    }
}

mod use_route {
    use dioxus::prelude::*;

    // ANCHOR: outlet_route
    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[layout(Wrapper)]
            #[route("/:name")]
            Index {
                name: String,
            },
    }

    #[component]
    fn Wrapper() -> Element {
        let full_route = use_route::<Route>();
        rsx! {
            header { "Welcome to {full_route}!" }
            // The index route will be rendered here
            Outlet::<Route> {}
            footer { "footer" }
        }
    }

    #[component]
    fn Index(name: String) -> Element {
        rsx! { h1 { "This is a homepage for {name}" } }
    }
    // ANCHOR_END: outlet_route

    fn App() -> Element {
        rsx! { Router::<Route> {} }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        vdom.rebuild_in_place();
        let html = dioxus_ssr::render(&vdom);
        assert_eq!(
            html,
            "<header></header><h1>Index</h1><footer>footer</footer>"
        );
    }
}

mod multiple_routes {
    use dioxus::prelude::*;

    // ANCHOR: multiple_routes
    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        #[layout(Wrapper)]
            #[route("/")]
            Home {},
            
            #[route("/about")]
            About {},
        #[end_layout]
    }

    #[component]
    fn Wrapper() -> Element {
        rsx! {
            header { "header" }
            Outlet::<Route> {}
            footer { "footer" }
        }
    }

    #[component]
    fn Home() -> Element {
        rsx! { h1 { "Home" } }
    }

    #[component]
    fn About() -> Element {
        rsx! { h1 { "About" } }
    }
    // ANCHOR_END: multiple_routes

    fn App() -> Element {
        rsx! { Router::<Route> {} }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        vdom.rebuild_in_place();
        let html = dioxus_ssr::render(&vdom);
        assert_eq!(
            html,
            "<header>header</header><h1>Home</h1><footer>footer</footer>"
        );
    }
}

mod wrong_pattern {
    use dioxus::prelude::*;

    // ANCHOR: wrong_pattern
    #[derive(Routable, Clone)]
    #[rustfmt::skip]
    enum Route {
        // ❌ WRONG: Creates nested layouts
        #[layout(Wrapper)]
        #[route("/")]
        Home {},

        #[layout(Wrapper)]  // This nests inside the first layout!
        #[route("/about")]
        About {},
    }
    // ANCHOR_END: wrong_pattern

    #[component]
    fn Wrapper() -> Element {
        rsx! {
            header { "header" }
            Outlet::<Route> {}
            footer { "footer" }
        }
    }

    #[component]
    fn Home() -> Element {
        rsx! { h1 { "Home" } }
    }

    #[component]
    fn About() -> Element {
        rsx! { h1 { "About" } }
    }

    fn App() -> Element {
        rsx! { Router::<Route> {} }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        vdom.rebuild_in_place();
        let html = dioxus_ssr::render(&vdom);
        // About page renders with DOUBLE layout (2 headers, 2 footers)
        assert_eq!(
            html,
            "<header>header</header><header>header</header><h1>About</h1><footer>footer</footer><footer>footer</footer>"
        );
    }
}
