use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use dioxus_logger::tracing::warn;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;
use tower_http::services::{ServeDir, ServeFile};
use tower_util::ServiceExt;
use uuid::Uuid;

use crate::app::AppState;

/// Handle providing temporary built wasm assets.
/// This should delete temporary projects after 30 seconds.
pub async fn serve_built_index(
    State(state): State<AppState>,
    Path(build_id): Path<Uuid>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let path = state.env.built_path.join(build_id.to_string());

    let index_path = path.join("index.html");

    tower_http::services::ServeFile::new(index_path)
        .oneshot(request)
        .await
        .map_err(|e| {
            warn!(err = ?e, build_id = ?build_id, "failed to serve built project file:");
            (StatusCode::NOT_FOUND, "not found")
        })
}

pub async fn serve_other_built(
    State(state): State<AppState>,
    Path((build_id, file_path)): Path<(Uuid, PathBuf)>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let path = state
        .env
        .built_path
        .join(build_id.to_string())
        .join(file_path);

    tower_http::services::ServeFile::new(path)
        .oneshot(request)
        .await
        .map_err(|e| {
            warn!(err = ?e, build_id = ?build_id, "failed to serve built project file:");
            (StatusCode::NOT_FOUND, "not found")
        })
}
