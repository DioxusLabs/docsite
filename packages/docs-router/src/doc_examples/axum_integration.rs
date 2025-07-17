#![allow(non_snake_case)]
use dioxus::prelude::*;

// ANCHOR: main
fn main() {
    #[cfg(feature = "web")]
    // Hydrate the application on the client
    dioxus::launch(App);

    // Launch axum on the server
    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                launch_server(App).await;
            });
    }
}
// ANCHOR_END: main

// ANCHOR: server
#[cfg(feature = "server")]
async fn launch_server(component: fn() -> Element) {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us
    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let router = axum::Router::new()
        // serve_dioxus_application adds routes to server side render the application, serve static assets, and register server functions
        .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .into_make_service();
    axum::serve(listener, router).await.unwrap();
}
// ANCHOR_END: server

#[component]
fn App() -> Element {
    todo!()
}
