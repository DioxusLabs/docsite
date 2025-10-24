mod first_router {
    // ANCHOR: first_router
    use dioxus::prelude::*;

    #[derive(Clone, Debug, PartialEq, Routable)]
    enum Route {
        #[route("/")]
        Home,

        #[route("/about")]
        About,

        #[route("/user/:id")]
        User { id: u32 },
    }

    #[component]
    fn Home() -> Element {
        rsx! { "Welcome to the home page!" }
    }

    #[component]
    fn About() -> Element {
        rsx! { "This is the about page." }
    }

    #[component]
    fn User(id: u32) -> Element {
        rsx! { "User page for user with id: {id}" }
    }
    // ANCHOR_END: first_router
    // ANCHOR: launch_router
    fn main() {
        dioxus::launch(|| rsx! { Router::<Route> {} });
    }
    // ANCHOR_END: launch_router

    mod first_link {
        use super::Route;
        use dioxus::prelude::*;
        // ANCHOR: first_link
        #[component]
        fn Home() -> Element {
            rsx! {
                div {
                    "Welcome to the home page!"
                    Link { to: Route::About, "Go to About Page" }
                }
            }
        }
        // ANCHOR_END: first_link
    }
}
