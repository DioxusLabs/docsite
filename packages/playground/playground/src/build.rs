use std::time::Duration;

use crate::ws;
use dioxus::prelude::*;
use model::{AppError, CargoDiagnostic, Project, SocketMessage};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Store)]
pub(crate) enum BuildStage {
    NotStarted,
    Starting,
    Waiting(Duration),
    Queued(usize),
    Building(model::BuildStage),
    Finished(Result<Uuid, String>),
}

impl BuildStage {
    pub fn is_running(&self) -> bool {
        matches!(
            self,
            Self::Starting | Self::Building(..) | Self::Waiting(..) | Self::Queued(..)
        )
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished(..))
    }

    pub fn finished_id(&self) -> Option<Uuid> {
        if let Self::Finished(result) = self {
            return result.clone().ok();
        }

        None
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Self::Finished(Err(_)))
    }

    pub fn err_message(&self) -> Option<String> {
        if let Self::Finished(Err(e)) = self {
            return Some(e.to_string());
        }

        None
    }
}

#[derive(Debug, Clone, Store)]
pub(crate) struct BuildState {
    stage: BuildStage,
    diagnostics: Vec<CargoDiagnostic>,
    previous_build_id: Option<Uuid>,
}

impl BuildState {
    pub fn new(project: &Project) -> Self {
        Self {
            stage: if project.prebuilt {
                BuildStage::Finished(Ok(project.id()))
            } else {
                BuildStage::NotStarted
            },
            diagnostics: Vec::new(),
            previous_build_id: None,
        }
    }
}

#[store(pub)]
impl Store<BuildState> {
    /// Reset the build state to it's default.
    fn reset(&mut self) {
        self.stage().set(BuildStage::NotStarted);
        self.diagnostics().clear();
        self.previous_build_id().set(None);
    }

    /// Get the current stage.
    fn get_stage(&self) -> BuildStage {
        self.stage().cloned()
    }

    /// Set the build stage.
    fn set_stage(&mut self, stage: BuildStage) {
        self.stage().set(stage);
    }

    /// Add another diagnostic to the current list.
    fn push_diagnostic(&mut self, diagnostic: CargoDiagnostic) {
        self.diagnostics().push(diagnostic);
    }
}

/// Start a build and handle updating the build signals according to socket messages.
pub async fn start_build(
    mut build: Store<BuildState>,
    socket_url: String,
    code: String,
) -> Result<bool, AppError> {
    let stage = build.get_stage();
    // Reset build state
    if stage.is_running() {
        return Err(AppError::BuildIsAlreadyRunning);
    }
    build.reset();
    if let Some(build_id) = stage.finished_id() {
        build.previous_build_id().set(Some(build_id));
    }
    build.set_stage(BuildStage::Starting);

    // Send socket compile request
    let mut socket = ws::Socket::new(&socket_url)?;
    socket
        .send(SocketMessage::BuildRequest {
            code,
            previous_build_id: stage.finished_id(),
        })
        .await?;

    // Handle socket messages
    loop {
        let msg = socket.next().await;
        match msg {
            Err(e) => return Err(e),
            Ok(None) => break,
            Ok(Some(msg)) => {
                let finished = msg.is_finished();
                ws::handle_message(build, msg);
                if finished {
                    break;
                }
            }
        }
    }
    socket.close().await;

    let mut success = false;
    if let BuildStage::Finished(Ok(_)) = build.get_stage() {
        success = true;
    };

    Ok(success)
}
