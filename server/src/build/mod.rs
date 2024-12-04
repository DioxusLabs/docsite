use model::CargoDiagnostic;
use std::{error::Error, fmt::Display, io};
use tokio::task::JoinError;
use uuid::Uuid;

pub mod builder;
pub mod watcher;

/// A message from the playground build process.
#[derive(Debug, Clone, PartialEq)]
pub enum BuildMessage {
    Building(model::BuildStage),
    CargoDiagnostic(CargoDiagnostic),
    Finished(Result<Uuid, String>),
    QueuePosition(usize),
}

/// The DX CLI serves parseable JSON output with the regular tracing message and a parseable "json" field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliMessage {
    json: Option<String>,
}

/// Build failed to complete.
#[derive(Debug)]
pub enum BuildError {
    /// DX ClI returned a non-success status code. This could be caused by invalid user-submitted code.
    DxFailed(Option<i32>),
    /// The builder doesn't have a build to finish.
    NotStarted,

    Panicked(JoinError),
    Aborted(JoinError),

    Io(io::Error),
    FsExtra(fs_extra::error::Error),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DxFailed(code) => write!(f, "dx returned a non-success status code: {code:?}"),
            Self::NotStarted => write!(f, "the builder doesn't have a build to finish"),
            Self::Panicked(e) => write!(f, "build panicked: {e}"),
            Self::Aborted(e) => write!(f, "build aborted: {e}"),
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::FsExtra(e) => write!(f, "file system error: {e}"),
        }
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DxFailed(_) => None,
            Self::NotStarted => None,
            Self::Panicked(e) => e.source(),
            Self::Aborted(e) => e.source(),
            Self::Io(e) => e.source(),
            Self::FsExtra(e) => e.source(),
        }
    }
}

// Tokio task JoinError.
impl From<JoinError> for BuildError {
    fn from(value: JoinError) -> Self {
        if value.is_cancelled() {
            Self::Aborted(value)
        } else {
            Self::Panicked(value)
        }
    }
}

// Any io error.
impl From<io::Error> for BuildError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

// Fs extra error.
// Is there a way to convert this to an io::Error?
impl From<fs_extra::error::Error> for BuildError {
    fn from(value: fs_extra::error::Error) -> Self {
        Self::FsExtra(value)
    }
}
