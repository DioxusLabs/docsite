use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, string::FromUtf8Error};
use uuid::Uuid;

#[cfg(feature = "server")]
mod server;

#[cfg(feature = "web")]
mod web;

#[derive(Debug, Serialize, Deserialize)]
pub enum SocketMessage {
    BuildRequest(String),
    BuildFinished(Result<Uuid, String>),
    BuildStage(BuildStage),
    BuildDiagnostic(CargoDiagnostic),
    QueuePosition(usize),
    AlreadyConnected,
}

/// A stage of building from the playground.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuildStage {
    Compiling {
        crates_compiled: usize,
        total_crates: usize,
        current_crate: String,
    },
    RunningBindgen,
    Other,
}

impl SocketMessage {
    pub fn as_json_string(&self) -> Result<String, SocketError> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TryFrom<String> for SocketMessage {
    type Error = SocketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}

/// A cargo diagnostic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CargoDiagnostic {
    level: CargoLevel,
    message: String,
    spans: Vec<CargoDiagnosticSpan>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CargoLevel {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CargoDiagnosticSpan {
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
    label: Option<String>,
}

/// Any socket error.
#[derive(Debug)]
pub enum SocketError {
    Parse(Box<dyn Error>),
}

impl Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(..) => write!(f, "the socket message failed to parse"),
        }
    }
}

impl Error for SocketError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(e) => e.source(),
        }
    }
}

impl From<serde_json::Error> for SocketError {
    fn from(value: serde_json::Error) -> Self {
        Self::Parse(Box::new(value))
    }
}

impl From<FromUtf8Error> for SocketError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Parse(Box::new(value))
    }
}
