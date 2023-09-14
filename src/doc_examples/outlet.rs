use dioxus::prelude::*;
use dioxus_router::prelude::*;

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

#[component]
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
