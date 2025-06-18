use crate::ws;
use dioxus::prelude::*;
use model::{AppError, CargoDiagnostic, SocketMessage};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BuildStage {
    NotStarted,
    Starting,
    Building(model::BuildStage),
    Finished(Result<Uuid, String>),
}

impl BuildStage {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Starting | Self::Building(..))
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

    /// Extract the compiling stage info if available.
    pub fn get_compiling_stage(&self) -> Option<(usize, usize, String)> {
        if let Self::Building(model::BuildStage::Compiling {
            crates_compiled,
            total_crates,
            current_crate,
        }) = self
        {
            return Some((*crates_compiled, *total_crates, current_crate.to_string()));
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BuildState {
    stage: Signal<BuildStage>,
    queue_position: Signal<Option<usize>>,
    diagnostics: Signal<Vec<CargoDiagnostic>>,
}

impl BuildState {
    pub fn new() -> Self {
        Self {
            stage: Signal::new(BuildStage::NotStarted),
            queue_position: Signal::new(None),
            diagnostics: Signal::new(Vec::new()),
        }
    }

    /// Reset the build state to it's default.
    pub fn reset(&mut self) {
        self.stage.set(BuildStage::NotStarted);
        self.queue_position.set(None);
        self.diagnostics.clear();
    }

    /// Get the current stage.
    pub fn stage(&self) -> BuildStage {
        (self.stage)()
    }

    /// Set the build stage.
    pub fn set_stage(&mut self, stage: BuildStage) {
        self.stage.set(stage);
    }

    /// Get the current queue position.
    pub fn queue_position(&self) -> Option<usize> {
        (self.queue_position)()
    }

    /// Set the queue position.
    pub fn set_queue_position(&mut self, position: Option<usize>) {
        self.queue_position.set(position);
    }

    /// Get the diagnostics signal.
    pub fn diagnostics(&self) -> Signal<Vec<CargoDiagnostic>> {
        self.diagnostics
    }

    /// Add another diagnostic to the current list.
    pub fn push_diagnostic(&mut self, diagnostic: CargoDiagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

/// Start a build and handle updating the build signals according to socket messages.
pub async fn start_build(
    mut build: BuildState,
    socket_url: String,
    code: String,
) -> Result<bool, AppError> {
    // Reset build state
    if build.stage().is_running() {
        return Err(AppError::BuildIsAlreadyRunning);
    }
    build.reset();
    build.set_stage(BuildStage::Starting);

    // Send socket compile request
    let mut socket = ws::Socket::new(&socket_url)?;
    socket.send(SocketMessage::BuildRequest(code)).await?;

    // Handle socket messages
    loop {
        let msg = socket.next().await;
        match msg {
            Err(e) => return Err(e),
            Ok(None) => break,
            Ok(Some(msg)) => {
                let is_done = ws::handle_message(build, msg);
                if is_done {
                    break;
                }
            }
        }
    }
    socket.close().await;

    let mut success = false;
    if let BuildStage::Finished(Ok(_)) = build.stage() {
        success = true;
    };

    Ok(success)
}
