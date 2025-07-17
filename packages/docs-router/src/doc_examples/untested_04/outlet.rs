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
fn Wrapper(cx: Scope) -> Element {
    render! {
        header { "header" }
        // The index route will be rendered here
        Outlet::<Route> { }
        footer { "footer" }
    }
}

#[component]
fn Index(cx: Scope) -> Element {
    render! {
        h1 { "Index" }
    }
}
// ANCHOR_END: outlet

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

fn main() {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();
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
    fn Wrapper(cx: Scope, name: String) -> Element {
        render! {
            header { "Welcome {name}!" }
            // The index route will be rendered here
            Outlet::<Route> { }
            footer { "footer" }
        }
    }

    #[component]
    fn Index(cx: Scope, name: String) -> Element {
        render! {
            h1 { "This is a homepage for {name}" }
        }
    }
    // ANCHOR_END: outlet_with_params

    fn App(cx: Scope) -> Element {
        render! {
            Router::<Route> {}
        }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        let _ = vdom.rebuild();
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
    fn Wrapper(cx: Scope) -> Element {
        let full_route = use_route::<Route>(cx).unwrap();
        render! {
            header { "Welcome to {full_route}!" }
            // The index route will be rendered here
            Outlet::<Route> { }
            footer { "footer" }
        }
    }

    #[component]
    fn Index(cx: Scope, name: String) -> Element {
        render! {
            h1 { "This is a homepage for {name}" }
        }
    }
    // ANCHOR_END: outlet_route

    fn App(cx: Scope) -> Element {
        render! {
            Router::<Route> {}
        }
    }

    fn main() {
        let mut vdom = VirtualDom::new(App);
        let _ = vdom.rebuild();
        let html = dioxus_ssr::render(&vdom);
        assert_eq!(
            html,
            "<header></header><h1>Index</h1><footer>footer</footer>"
        );
    }
}
