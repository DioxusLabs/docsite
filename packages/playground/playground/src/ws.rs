use crate::{build::BuildStage, BuildState};
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
        SocketMessage::QueuePosition(position) => build.set_queue_position(Some(position)),
        SocketMessage::BuildFinished(result) => {
            build.set_stage(BuildStage::Finished(result));
            return true;
        }
        SocketMessage::BuildDiagnostic(diagnostic) => build.push_diagnostic(diagnostic),
        _ => {}
    }

    false
}
