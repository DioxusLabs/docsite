#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(App);
    #[cfg(not(target_arch = "wasm32"))]
    launch(App);
}

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
            #[route("/")]
            BlogList {},
            #[route("/post/:name")]
            BlogPost { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// ANCHOR_END: router

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::BlogList {}, "Blog" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { h1 { "Welcome to the Dioxus Blog!" } }
}

// ANCHOR: blog
#[component]
fn Blog() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}
// ANCHOR_END: blog

// ANCHOR: blog_list
#[component]
fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}
// ANCHOR_END: blog_list

// ANCHOR: blog_post
// The name prop comes from the /:name route segment
#[component]
fn BlogPost(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}
// ANCHOR_END: blog_post

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
