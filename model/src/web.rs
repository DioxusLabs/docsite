//! Web-specific implementations

use crate::{SocketError, SocketMessage};
use gloo_net::websocket::Message as GlooMessage;

// SocketMessage to Gloo conversion.
impl TryFrom<SocketMessage> for GlooMessage {
    type Error = SocketError;

    fn try_from(value: SocketMessage) -> Result<Self, Self::Error> {
        let val = value.as_json_string()?;
        Ok(Self::Text(val))
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
