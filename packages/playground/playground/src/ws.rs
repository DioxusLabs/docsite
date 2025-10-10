use crate::{build::BuildStage, hotreload::send_hot_reload, BuildState};
use dioxus::signals::ReadableExt;
use dioxus_devtools::HotReloadMsg;
use futures::{SinkExt as _, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use model::*;

/// A socket connection.
pub struct Socket {
    socket: WebSocket,
}

impl Socket {
    /// Create a new socket connection at the url.
    pub fn new(url: &str) -> Result<Self, AppError> {
        let socket = WebSocket::open(url)?;
        Ok(Self { socket })
    }

    pub async fn send(&mut self, message: SocketMessage) -> Result<(), AppError> {
        self.socket
            .send(message.into_gloo()?)
            .await
            .map_err(SocketError::from)?;

        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<SocketMessage>, AppError> {
        match self.socket.next().await {
            Some(Ok(message)) => Ok(Some(SocketMessage::try_from(message)?)),
            Some(Err(e)) => Err(SocketError::from(e))?,
            _ => Ok(None),
        }
    }

    pub async fn close(self) {
        self.socket.close(None, None).ok();
    }
}

/// Handles a websocket message, returning true if further messages shouldn't be handled.
pub fn handle_message(mut build: BuildState, message: SocketMessage) -> bool {
    match message {
        SocketMessage::BuildStage(stage) => build.set_stage(BuildStage::Building(stage)),
        SocketMessage::QueuePosition(position) => build.set_stage(BuildStage::Queued(position)),
        SocketMessage::BuildFinished(BuildResult::Failed(failure)) => {
            build.set_stage(BuildStage::Finished(Err(failure)));
            return true;
        }
        SocketMessage::BuildFinished(BuildResult::Built(id)) => {
            build.set_stage(BuildStage::Finished(Ok(id)));
            return true;
        }
        SocketMessage::BuildFinished(BuildResult::HotPatched(patch)) => {
            // Get the iframe to apply the patch to.
            send_hot_reload(HotReloadMsg {
                templates: Default::default(),
                assets: Default::default(),
                ms_elapsed: Default::default(),
                for_pid: Default::default(),
                for_build_id: Some(0),
                jump_table: Some(patch),
            });
            if let Some(id) = build.previous_build_id().cloned() {
                build.set_stage(BuildStage::Finished(Ok(id)));
            }
            return true;
        }
        SocketMessage::BuildDiagnostic(diagnostic) => build.push_diagnostic(diagnostic),
        SocketMessage::RateLimited(time) => build.set_stage(BuildStage::Waiting(time)),
        _ => {}
    }

    false
}
