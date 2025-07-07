use dioxus::prelude::*;

// ANCHOR: new_router
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    DogView,
}
// ANCHOR_END: new_router

fn DogView() -> Element {
    rsx! { "dogs!" }
}

mod new_router_with_component {
    // ANCHOR: new_router_with_component
    use dioxus::prelude::*;

    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        #[route("/")]
        DogView, // <---- a DogView component must be in scope
    }

    fn DogView() -> Element {
        todo!()
    }
    // ANCHOR_END: new_router_with_component
}

mod rendering_the_route {
    use super::Route;
    use dioxus::prelude::*;

    // ANCHOR: rendering_the_route
    fn app() -> Element {
        rsx! {
            document::Stylesheet { href: asset!("/assets/main.css") }

            // 📣 delete Title and DogView and replace it with the Router component.
            Router::<Route> {}
        }
    }
    // ANCHOR_END: rendering_the_route
}

mod catch_all {
    use dioxus::prelude::*;

    #[component]
    fn PageNotFound(segments: Vec<String>) -> Element {
        todo!()
    }

    // ANCHOR: catch_all
    #[derive(Routable, Clone, PartialEq)]
    enum Route {
        // ...
        // We can collect the segments of the URL into a Vec<String>
        #[route("/:..segments")]
        PageNotFound { segments: Vec<String> },
    }
    // ANCHOR_END: catch_all
}

mod nav_bar {
    use super::Route;

    // ANCHOR: nav_bar
    use dioxus::prelude::*;

    #[component]
    pub fn NavBar() -> Element {
        rsx! {
            div { id: "title",
                Link { to: Route::DogView,
                    h1 { "🌭 HotDog! " }
                }
            }
            Outlet::<Route> {}
        }
    }
    // ANCHOR_END: nav_bar
}

mod link {
    use super::Route;
    use dioxus::prelude::*;

    fn LinkExamples() -> Element {
        rsx! {
            // ANCHOR: link
            // Using the Link with Route
            Link { to: Route::DogView }

            // Or passing in a "/" route directly
            Link { to: "/" }
            // ANCHOR_END: link
        }
    }
}

mod nav_bar_router {
    use dioxus::prelude::*;
    fn NavBar() -> Element {
        todo!()
    }
    fn DogView() -> Element {
        todo!()
    }

    // ANCHOR: nav_bar_router
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        #[layout(NavBar)] // <---- add the #[layout] attribute
        #[route("/")]
        DogView,
    }
    // ANCHOR_END: nav_bar_router
}

mod favorites {
    // ANCHOR: favorites
    use dioxus::prelude::*;

    #[component]
    pub fn Favorites() -> Element {
        rsx! { "favorites!" }
    }
    // ANCHOR_END: favorites

    // ANCHOR: favorites_router
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        #[layout(NavBar)]
        #[route("/")]
        DogView,

        #[route("/favorites")]
        Favorites, // <------ add this new variant
    }
    // ANCHOR_END: favorites_router

    fn DogView() -> Element {
        todo!()
    }

    use nav_bar_favorites_link::NavBar;
    mod nav_bar_favorites_link {
        use super::Route;
        // ANCHOR: nav_bar_favorites_link
        use dioxus::prelude::*;

        #[component]
        pub fn NavBar() -> Element {
            rsx! {
                div { id: "title",
                    Link { to: Route::DogView,
                        h1 { "🌭 HotDog! " }
                    }
                    Link { to: Route::Favorites, id: "heart", "♥️" } // <------- add this Link
                }
                Outlet::<Route> {}
            }
        }
        // ANCHOR_END: nav_bar_favorites_link
    }
}

mod list_dogs {
    mod backend {
        use crate::doc_examples::guide_databases::DB;
        use dioxus::prelude::*;
        // ANCHOR: list_dogs
        // Query the database and return the last 10 dogs and their url
        #[server]
        pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
            let dogs = DB.with(|f| {
                f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
                    .unwrap()
                    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
                    .unwrap()
                    .map(|r| r.unwrap())
                    .collect()
            });

            Ok(dogs)
        }
        // ANCHOR_END: list_dogs
    }

    mod favorites {

        // ANCHOR: favorites_list_dogs
        use dioxus::prelude::*;

        #[component]
        pub fn Favorites() -> Element {
            // Create a pending resource that resolves to the list of dogs from the backend
            // Wait for the favorites list to resolve with `.suspend()`
            let mut favorites = use_resource(super::backend::list_dogs).suspend()?;

            rsx! {
                div { id: "favorites",
                    div { id: "favorites-container",
                        for (id, url) in favorites().unwrap() {
                            // Render a div for each photo using the dog's ID as the list key
                            div {
                                key: id,
                                class: "favorite-dog",
                                img { src: "{url}" }
                            }
                        }
                    }
                }
            }
        }
        // ANCHOR_END: favorites_list_dogs
    }
}
