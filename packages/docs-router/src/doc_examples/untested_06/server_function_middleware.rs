#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: server_function_middleware
#[server]
// Add a timeout middleware to the server function that will return an error if the function takes longer than 1 second to execute
#[middleware(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(1)))]
pub async fn timeout() -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(())
}
// ANCHOR_END: server_function_middleware
