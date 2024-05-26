use std::fmt::Display;

/// This represents a basic websocket message.
///
/// Messages have an identifier and content.
/// e.g. `error~:~compilation failed on line x`
/// or
/// `please_compile~:~#[component]...`
pub enum SocketMessage {
    CompileRequest(String),
    CompileFinished(String),
    CompileFinishedWithError,
    CompileMessage(String),
    BannedWord(String),
    SystemError(String),
}

impl TryFrom<String> for SocketMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split("~:~").collect();

        let Some(first) = split.first() else {
            return Err("invalid message".to_string());
        };

        let last = split.get(1).unwrap_or(&"").to_string();

        match *first {
            "please_compile" => Ok(Self::CompileRequest(last)),
            "compilation_finished" => Ok(Self::CompileFinished(last)),
            "compilation_finished_err" => Ok(Self::CompileFinishedWithError),
            "compile_msg" => Ok(Self::CompileMessage(last)),
            "banned_word" => Ok(Self::BannedWord(last)),
            "error" => Ok(Self::SystemError(last)),
            _ => Err("unknown ws message".to_string()),
        }
    }
}

impl Display for SocketMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompileRequest(s) => write!(f, "please_compile~:~{}", s),
            Self::CompileFinished(s) => write!(f, "compilation_finished~:~{}", s),
            Self::CompileFinishedWithError => write!(f, "compilation_finished_err~:~"),
            Self::CompileMessage(s) => write!(f, "compile_msg~:~{}", s),
            Self::BannedWord(s) => write!(f, "banned_word~:~{}", s),
            Self::SystemError(s) => write!(f, "error~:~{}", s),
        }
    }
}

#[cfg(feature = "web")]
use gloo_net::websocket::Message;

#[cfg(feature = "web")]
impl From<SocketMessage> for Message {
    fn from(value: SocketMessage) -> Self {
        let msg = value.to_string();
        Message::Text(msg)
    }
}

#[cfg(feature = "web")]
impl TryFrom<Message> for SocketMessage {
    type Error = String;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let Message::Text(txt) = value else {
            return Err("unable to `TryInto` binary messages".to_string());
        };

        SocketMessage::try_from(txt)
    }
}
