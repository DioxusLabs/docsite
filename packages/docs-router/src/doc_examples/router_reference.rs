use dioxus::prelude::*;

mod simple_router {
    use super::*;

    // ANCHOR: router_definition
    // All of our routes will be a variant of this Route enum
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        // if the current location is "/home", render the Home component
        #[route("/home")]
        Home {},
        // if the current location is "/blog", render the Blog component
        #[route("/blog")]
        Blog {},
    }

    fn Home() -> Element {
        todo!()
    }

    fn Blog() -> Element {
        todo!()
    }
    // ANCHOR_END: router_definition
}

mod router_404 {
    use dioxus::prelude::*;

    // ANCHOR: router_definition_404
    // All of our routes will be a variant of this Route enum
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        #[route("/home")]
        Home {},
        #[route("/blog")]
        Blog {},
        //  if the current location doesn't match any of the above routes, render the NotFound component
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
    }

    fn Home() -> Element {
        todo!()
    }

    fn Blog() -> Element {
        todo!()
    }

    #[component]
    fn NotFound(segments: Vec<String>) -> Element {
        todo!()
    }
    // ANCHOR_END: router_definition_404
}

mod router_404_redirect {
    use super::*;

    // ANCHOR: router_404_redirect
    // All of our routes will be a variant of this Route enum
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        #[route("/home")]
        //  if the current location doesn't match any of the other routes, redirect to "/home"
        #[redirect("/:..segments", |segments: Vec<String>| Route::Home {})]
        Home {},
        #[route("/blog")]
        Blog {},
    }
    // ANCHOR_END: router_404_redirect
}

fn Home() -> Element {
    todo!()
}

fn Blog() -> Element {
    todo!()
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    todo!()
}

fn App() -> Element {
    #[derive(Routable, PartialEq, Clone)]
    enum Route {
        #[route("/home")]
        Home {},
    }
    // ANCHOR: links
    rsx! {
        Link { to: Route::Home {}, "Go home!" }
    }
    // ANCHOR_END: links
}
