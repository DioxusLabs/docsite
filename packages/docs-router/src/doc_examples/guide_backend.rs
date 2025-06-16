mod save_dog_v1 {
    use dioxus::prelude::*;

    // ANCHOR: save_dog_v1
    #[server]
    async fn save_dog(image: String) -> Result<(), ServerFnError> {
        Ok(())
    }
    // ANCHOR_END: save_dog_v1
}

mod save_dog_client {
    use dioxus::prelude::*;

    // ANCHOR: save_dog_client
    // on the client:
    async fn save_dog(image: String) -> Result<(), ServerFnError> {
        reqwest::Client::new()
            .post("http://localhost:8080/api/save_dog")
            .json(&image)
            .send()
            .await?;
        Ok(())
    }
    // ANCHOR_END: save_dog_client
}

mod save_dog_server {
    use axum::Json;
    use dioxus::prelude::*;

    // ANCHOR: save_dog_server
    // on the server:
    struct SaveDogArgs {
        image: String,
    }

    async fn save_dog(Json(args): Json<SaveDogArgs>) -> Result<(), ServerFnError> {
        Ok(())
    }
    // ANCHOR_END: save_dog_server
}

mod save_dog_launch {
    use dioxus::prelude::*;

    // ANCHOR: save_dog_launch
    async fn launch(config: ServeConfig, app: fn() -> Element) {
        // register server functions
        let router = axum::Router::new().serve_dioxus_application(config, app);

        // start server
        let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();
        let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
    // ANCHOR_END: save_dog_launch
}

mod separate_server_launch {
    use dioxus::prelude::*;

    fn App() -> Element {
        rsx! { "hello world" }
    }

    // ANCHOR: separate_server_launch
    fn main() {
        #[cfg(feature = "server")]
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(launch_server());
        #[cfg(not(feature = "server"))]
        dioxus::launch(App);
    }

    #[cfg(feature = "server")]
    async fn launch_server() {
        // Connect to dioxus' logging infrastructure
        dioxus::logger::initialize_default();

        // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
        let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

        // Build a custom axum router
        let router = axum::Router::new()
            .serve_dioxus_application(ServeConfigBuilder::new(), App)
            .into_make_service();

        // And launch it!
        let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
    // ANCHOR_END: separate_server_launch
}

mod server_client_split_broken {
    use dioxus::prelude::*;

    async fn connect_to_db(password: &str) -> Result<(), ServerFnError> {
        Ok(())
    }

    // ANCHOR: server_client_split_broken
    // ❌ this will leak your DB_PASSWORD to your client app!
    static DB_PASSWORD: &str = "1234";

    #[server]
    async fn DoThing() -> Result<(), ServerFnError> {
        connect_to_db(DB_PASSWORD).await
        // ...
    }
    // ANCHOR_END: server_client_split_broken
}

mod server_client_split_fixed {
    // ANCHOR: server_client_split_fixed
    // ✅ code in this module can only be accessed on the server
    #[cfg(feature = "server")]
    mod server_utils {
        pub static DB_PASSWORD: &str = "1234";
    }
    // ANCHOR_END: server_client_split_fixed
}

mod server_client_split_client_broken {
    use dioxus::prelude::*;

    fn App() -> Element {
        rsx! { "hello world" }
    }

    // ANCHOR: server_client_split_broken_client_broken
    fn main() {
        // ❌ attempting to use web_sys on the server will panic!
        let window = web_sys::window();

        // ..

        dioxus::launch(App);
    }
    // ANCHOR_END: server_client_split_broken_client_broken
}

mod save_dog_v2 {
    use dioxus::prelude::*;

    // ANCHOR: save_dog_v2
    // Expose a `save_dog` endpoint on our server that takes an "image" parameter
    #[server]
    async fn save_dog(image: String) -> Result<(), ServerFnError> {
        use std::io::Write;

        // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("dogs.txt")
            .unwrap();

        // And then write a newline to it with the image url
        file.write_fmt(format_args!("{image}\n"));

        Ok(())
    }
    // ANCHOR_END: save_dog_v2
}

mod save_dog_call {
    use dioxus::prelude::*;

    async fn save_dog(image: String) -> Result<(), ServerFnError> {
        todo!()
    }

    macro_rules! snipped {
        () => {
            || async move { todo!() }
        };
    }

    // ANCHOR: save_dog_call
    fn DogView() -> Element {
        let mut img_src = use_resource(snipped!());

        // ...
        rsx! {
            // ...
            div { id: "buttons",
                // ...
                button {
                    id: "save",
                    onclick: move |_| async move {
                        let current = img_src.cloned().unwrap();
                        img_src.restart();
                        _ = save_dog(current).await;
                    },

                    "save!"
                }
            }
        }
    }
    // ANCHOR_END: save_dog_client
}
