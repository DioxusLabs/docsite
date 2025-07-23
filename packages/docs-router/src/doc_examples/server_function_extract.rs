#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: server_function_extract
#[server]
pub async fn log_headers() -> Result<(), ServerFnError> {
    let headers: http::HeaderMap = extract().await?;
    tracing::info!("{:?}", headers[http::header::USER_AGENT]);
    Ok(())
}
// ANCHOR_END: server_function_extract
