use std::fmt::Display;

/// This represents a basic websocket message.
/// 
/// Messages have an identifier and content.
/// e.g. `error:compilation failed on line x`
/// or
/// `please_compile:#[component]...`
pub enum SocketMessage {
    CompileRequest(String),
    CompileFinished(String),
    SystemError(String),
}

impl TryFrom<String> for SocketMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split(":").collect();

        // TODO: Error handling. This will panic!
        let first = split[0];
        let last = split[1].to_string();

        match first {
            "please_compile" => Ok(Self::CompileRequest(last)),
            "compilation_finished" => Ok(Self::CompileFinished(last)),
            "error" => Ok(Self::SystemError(last)),
            _ => Err("unknown ws message".to_string()),
        }
    }
}

impl Display for SocketMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketMessage::CompileRequest(s) => write!(f, "please_compile:{}", s),
            SocketMessage::CompileFinished(s) => write!(f, "compilation_finished:{}", s),
            SocketMessage::SystemError(s) => write!(f, "errpr:{}", s),
        }
    }
}