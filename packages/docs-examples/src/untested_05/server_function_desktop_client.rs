#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: server_function
#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
// ANCHOR_END: server_function

use dioxus::prelude::*;
use server_fn::axum::register_explicit;

#[tokio::main]
async fn main() {
    // ANCHOR: server_url
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    // ANCHOR_END: server_url

    // ANCHOR: function_registration
    register_explicit::<GetServerData>();
    // ANCHOR_END: function_registration

    axum::serve(
        listener,
        axum::Router::new()
            .register_server_fns()
            .into_make_service(),
    )
    .await
    .unwrap();
}
