#![allow(non_snake_case)]
use dioxus::prelude::*;

fn app() -> Element {
    todo!()
}

mod launch {
    use super::app;
    use dioxus::prelude::*;

    // ANCHOR: streaming_launch
    pub fn main() {
        dioxus::LaunchBuilder::new()
            .with_cfg(server_only! {
                dioxus::fullstack::ServeConfig::builder().enable_out_of_order_streaming()
            })
            .launch(app);
    }
    // ANCHOR_END: streaming_launch
}

mod axum_entry_point {
    use super::app;
    use dioxus::prelude::*;

    // ANCHOR: streaming_axum
    #[cfg(feature = "server")]
    #[tokio::main]
    async fn main() {
        let addr = dioxus::cli_config::fullstack_address_or_localhost();
        let router = axum::Router::new()
            // Server side render the application, serve static assets, and register server functions
            .serve_dioxus_application(
                dioxus::fullstack::ServeConfig::builder()
                    .enable_out_of_order_streaming()
                    .build()
                    .unwrap(),
                app,
            )
            .into_make_service();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
    // ANCHOR_END: streaming_axum
}

mod head_elements {
    use dioxus::prelude::*;
    fn app() -> Element {
        rsx! {
            Router::<Route> {}
        }
    }

    async fn get_title() -> String {
        // Simulate a server function that fetches the title
        "My Dioxus App".to_string()
    }

    async fn get_description() -> String {
        // Simulate a server function that fetches the description
        "This is a Dioxus app with streaming support.".to_string()
    }

    // ANCHOR: head_elements
    /// An enum of all of the possible routes in the app.
    #[derive(Routable, Clone)]
    enum Route {
        // The home page is at the / route
        #[route("/")]
        Home,
    }

    fn Home() -> Element {
        let title = use_server_future(get_title)?;
        let description = use_server_future(get_description)?;

        rsx! {
            // This will be rendered on the server because it is inside the same (root)
            // suspense boundary as the `Router` component.
            document::Title { {title} }
            SuspenseBoundary {
                fallback: |_| {
                    rsx! { "Loading..." }
                },
                AsyncHead {}
            }
        }
    }

    fn AsyncHead() -> Element {
        let description = use_server_future(get_description)?;
        // The resource should always be resolved at this point because the `?` above bubbles
        // up the async case if it is pending
        let current_description = description.read_unchecked();
        let current_description = current_description.as_ref().unwrap();

        rsx! {
            // This will be rendered on the client because it is in a
            // suspense boundary below the `Router` component.
            document::Meta { name: "description", content: "{current_description}" }
        }
    }
    // ANCHOR_END: head_elements
}
