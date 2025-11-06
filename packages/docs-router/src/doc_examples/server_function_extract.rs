#![allow(non_snake_case, unused)]
use dioxus::prelude::*;

// ANCHOR: server_function_extract
// let headers: http::HeaderMap = extract().await?; // todo! bring back extract
#[get("/log_headers", headers: http::HeaderMap)]
pub async fn log_headers() -> Result<(), ServerFnError> {
    tracing::info!("{:?}", headers[http::header::USER_AGENT]);
    Ok(())
}
// ANCHOR_END: server_function_extract
