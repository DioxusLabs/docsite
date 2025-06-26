//! Web-specific implementations

use crate::{AppError, SocketError, SocketMessage};
use gloo_net::websocket::Message as GlooMessage;

impl SocketMessage {
    pub fn into_gloo(self) -> Result<GlooMessage, SocketError> {
        let val = self.as_json_string()?;
        Ok(GlooMessage::Text(val))
    }
}

// Gloo message to SocketMessage conversion.
impl TryFrom<GlooMessage> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: GlooMessage) -> Result<Self, Self::Error> {
        Ok(match value {
            GlooMessage::Bytes(bytes) => {
                let as_string = String::from_utf8(bytes)?;
                Self::try_from(as_string)?
            }
            GlooMessage::Text(txt) => Self::try_from(txt)?,
        })
    }
}

// Web AppError impls
use gloo_utils::errors::JsError;
impl From<JsError> for AppError {
    fn from(value: JsError) -> Self {
        Self::Js(Box::new(value))
    }
}

use dioxus_document::EvalError;
impl From<EvalError> for AppError {
    fn from(value: EvalError) -> Self {
        Self::Js(Box::new(value))
    }
}
