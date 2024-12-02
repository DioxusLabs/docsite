use serde::{Deserialize, Serialize};
use std::{error::Error, string::FromUtf8Error};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    CompileRequest(String),
    CompileFinished(Result<Uuid, String>),
    CompileMessage(String),
    Compiling {
        current_crate: usize,
        total_crates: usize,
        krate: String,
    },
    QueuePosition(usize),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueueAction {
    Set(u32),
    Sub,
}

impl SocketMessage {
    pub fn as_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl From<String> for SocketMessage {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or(SocketMessage::Unknown)
    }
}

// Automatic SocketMessage conversion from gloo_net
#[cfg(feature = "web")]
use gloo_net::websocket::Message;

#[cfg(feature = "web")]
impl TryFrom<SocketMessage> for Message {
    type Error = serde_json::Error;

    fn try_from(value: SocketMessage) -> Result<Self, Self::Error> {
        let val = value.as_json_string()?;
        Ok(Self::Text(val))
    }
}

#[cfg(feature = "web")]
impl From<Message> for SocketMessage {
    fn from(value: Message) -> Self {
        match value {
            Message::Bytes(bytes) => {
                let as_string = String::from_utf8(bytes).unwrap_or("unknown".to_string());
                Self::from(as_string)
            }
            Message::Text(txt) => Self::from(txt),
        }
    }
}

/// Represents a generic socket error.
pub struct SocketError(pub Box<dyn Error>);

impl From<serde_json::Error> for SocketError {
    fn from(value: serde_json::Error) -> Self {
        Self(Box::new(value))
    }
}

impl From<FromUtf8Error> for SocketError {
    fn from(value: FromUtf8Error) -> Self {
        Self(Box::new(value))
    }
}
