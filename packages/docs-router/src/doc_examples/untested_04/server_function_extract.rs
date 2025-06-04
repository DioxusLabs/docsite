#![allow(non_snake_case, unused)]
use dioxus_fullstack::prelude::*;

// ANCHOR: server_function_extract
#[server]
pub async fn log_user_agent() -> Result<(), ServerFnError> {
    let axum::TypedHeader(user_agent): axum::TypedHeader<axum::headers::UserAgent> =
        extract().await?;
    log::info!("{:?}", user_agent);
    Ok(())
}
// ANCHOR_END: server_function_extract
