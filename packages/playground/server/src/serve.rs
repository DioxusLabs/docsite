use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use dioxus_logger::tracing::warn;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::app::AppState;

/// Handle providing temporary built wasm assets.
/// This should delete temporary projects after 30 seconds.
pub async fn serve_built_index(
    State(state): State<AppState>,
    Path(build_id): Path<Uuid>,
) -> impl IntoResponse {
    let path = state.env.built_path.join(build_id.to_string());

    let index_path = path.join("index.html");
    let file = match tokio::fs::File::open(index_path.clone()).await {
        Ok(f) => f,
        Err(e) => {
            warn!(err = ?e, path = ?index_path, "failed to read built project:");
            return Err((StatusCode::NOT_FOUND, "not found"));
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [(header::CONTENT_TYPE, "text/html")];

    Ok((headers, body))
}

pub async fn serve_other_built(
    State(state): State<AppState>,
    Path((build_id, file_path)): Path<(Uuid, PathBuf)>,
) -> impl IntoResponse {
    let path = state
        .env
        .built_path
        .join(build_id.to_string())
        .join(file_path);

    let file = match tokio::fs::File::open(path.clone()).await {
        Ok(f) => f,
        Err(e) => {
            warn!(err = ?e, path = ?path, "failed to read built project:");
            return Err((StatusCode::NOT_FOUND, "read failure"));
        }
    };

    let Some(file_ext) = path.extension() else {
        warn!(build_id = ?build_id, path = ?path, "failed to get file extension");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "read failure"));
    };

    let content_type = match file_ext.to_str() {
        Some("wasm") => "application/wasm",
        Some("js") => "application/javascript",
        Some(_) => {
            warn!(build_id = ?build_id, path = ?path, "project tried accessing denied file");
            return Err((StatusCode::NOT_FOUND, "not found"));
        }
        None => {
            warn!(build_id = ?build_id, path = ?path, "failed to get file extension");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "read failure"));
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [(header::CONTENT_TYPE, content_type)];

    Ok((headers, body))
}
