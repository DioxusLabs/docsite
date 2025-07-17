#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;

// ANCHOR: server_function
#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
// ANCHOR_END: server_function

use dioxus_fullstack::prelude::*;

#[tokio::main]
async fn main() {
    // ANCHOR: server_url
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    // ANCHOR_END: server_url

    // ANCHOR: function_registration
    let _ = GetServerData::register_explicit();
    // ANCHOR_END: function_registration

    axum::Server::bind(&addr)
        .serve(
            axum::Router::new()
                .register_server_fns("")
                .into_make_service(),
        )
        .await
        .unwrap();
}
