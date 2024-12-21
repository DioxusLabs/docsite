use crate::{error::AppError, ws};
use dioxus::prelude::*;
use model::SocketMessage;
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
        match self {
            Self::Starting => true,
            Self::Building(..) => true,
            _ => false,
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Self::Finished(..) => true,
            _ => false,
        }
    }

    pub fn finished_id(&self) -> Option<Uuid> {
        if let Self::Finished(result) = self {
            return result.clone().ok();
        }

        None
    }

    /// Extract the compiling stage info if available.
    pub fn get_compiling_stage(&self) -> Option<(usize, usize, String)> {
        if let Self::Building(stage) = self {
            if let model::BuildStage::Compiling {
                crates_compiled,
                total_crates,
                current_crate,
            } = stage
            {
                return Some((*crates_compiled, *total_crates, current_crate.to_string()));
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BuildState {
    stage: Signal<BuildStage>,
    queue_position: Signal<Option<usize>>,
}

impl BuildState {
    pub fn new() -> Self {
        Self {
            stage: Signal::new(BuildStage::NotStarted),
            queue_position: Signal::new(None),
        }
    }

    /// Reset the build state to it's default.
    pub fn reset(&mut self) {
        self.stage.set(BuildStage::NotStarted);
        self.queue_position.set(None);
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
}

/// Start a build and handle updating the build signals according to socket messages.
pub async fn start_build(
    build: &mut BuildState,
    socket_url: String,
    code: String,
) -> Result<(), AppError> {
    // Reset build state
    if build.stage().is_running() {
        return Ok(());
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
    Ok(())
}
