use std::error::Error;

use dioxus::prelude::document::EvalError;
use gloo_net::websocket::WebSocketError;
use gloo_utils::errors::JsError;
use model::SocketError;

#[allow(unused)]
#[derive(Debug)]
pub enum AppError {
    Socket(Box<dyn Error>),
    JsError(Box<dyn Error>),
}

impl From<SocketError> for AppError {
    fn from(value: SocketError) -> Self {
        Self::Socket(value.0)
    }
}

impl From<WebSocketError> for AppError {
    fn from(value: WebSocketError) -> Self {
        Self::Socket(Box::new(value))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Socket(Box::new(value))
    }
}

impl From<EvalError> for AppError {
    fn from(value: EvalError) -> Self {
        // TODO: Put _value in Self::Eval once EvalError implements Error
        Self::JsError(Box::new(value))
    }
}

impl From<JsError> for AppError {
    fn from(value: JsError) -> Self {
        Self::JsError(Box::new(value))
    }
}
