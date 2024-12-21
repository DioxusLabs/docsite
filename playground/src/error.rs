#![allow(unused)]

use dioxus::prelude::document::EvalError;
use gloo_utils::errors::JsError;
use model::SocketError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Socket(#[from] SocketError),

    #[error("json parse failed: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("share code failed: {0}")]
    ShareCode(ShareError),

    #[error("js eval failed: {0}")]
    JsEvalError(#[from] EvalError),

    #[error("js failed: {0}")]
    JsError(#[from] JsError),

    #[error("build is already running")]
    BuildIsAlreadyRunning,
}

#[derive(Debug, Error)]
pub enum ShareError {
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),

    #[error(transparent)]
    Decompression(#[from] miniz_oxide::inflate::DecompressError),

    #[error(transparent)]
    Utf8Decode(#[from] FromUtf8Error),
}
