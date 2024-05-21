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
    CompileFinishedWithError,
    CompileMessage(String),
    BannedWord(String),
    SystemError(String),
}

impl TryFrom<String> for SocketMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split("~:~").collect();

        // TODO: Error handling. This will panic!
        let first = split[0];
        let last = split[1].to_string();

        match first {
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
