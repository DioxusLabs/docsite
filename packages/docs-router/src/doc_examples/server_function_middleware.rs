#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: server_function_middleware
#[cfg(feature = "server")]
use {std::time::Duration, tower_http::timeout::TimeoutLayer};

// Add a timeout middleware to the server function that will return an error if the function takes longer than 1 second to execute
#[post("/api/timeout")]
#[middleware(TimeoutLayer::new(Duration::from_secs(1)))]
pub async fn timeout() -> Result<(), ServerFnError> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(())
}
// ANCHOR_END: server_function_middleware
