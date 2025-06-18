use model::CargoDiagnostic;
use model::Project;
use std::io;
use thiserror::Error;
use tokio::{sync::mpsc::UnboundedSender, task::JoinError};
use uuid::Uuid;

pub mod builder;
pub mod watcher;

/// A build command which allows consumers of the builder api to submit and stop builds.
#[derive(Debug, Clone)]
pub enum BuildCommand {
    Start { request: BuildRequest },
    Stop { id: Uuid },
}

/// A build request which contains the id of the build, the code to be built, and a socket to send build updates.
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub id: Uuid,
    pub project: Project,
    pub ws_msg_tx: UnboundedSender<BuildMessage>,
}

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
#[derive(Error, Debug)]
pub enum BuildError {
    /// DX ClI returned a non-success status code. This could be caused by invalid user-submitted code.
    #[error("dx returned a non-success status code: {0:?}")]
    DxFailed(Option<i32>),
    /// The builder doesn't have a build to finish.
    #[error("the builder doesn't have a build to finish")]
    NotStarted,

    #[error("build panicked: {0}")]
    Panicked(JoinError),

    #[error("build aborted: {0}")]
    Aborted(JoinError),

    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("io error: {0}")]
    FsExtra(#[from] fs_extra::error::Error),
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
