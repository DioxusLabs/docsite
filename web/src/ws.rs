use crate::{error::AppError, BuildState};
use dioxus::signals::{Writable as _, WritableVecExt as _};
use dioxus_logger::tracing::warn;
use futures::{SinkExt as _, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
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
            .map_err(|e| SocketError::from(e))?;

        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<SocketMessage>, AppError> {
        match self.socket.next().await {
            Some(Ok(msg)) => Ok(Some(SocketMessage::try_from(msg)?)),
            Some(Err(e)) => Err(SocketError::from(e))?,
            _ => Ok(None),
        }
    }

    pub async fn close(self) {
        self.socket.close(None, None).ok();
    }
}

/// Handles a websocket message, returning true if further messages shouldn't be handled.
pub fn handle_message(signals: &mut BuildState, msg: SocketMessage) -> bool {
    let BuildState {
        is_compiling,
        queue_position,
        built_page_id,
        compiler_messages,
    } = signals;

    match msg {
        // Handle a finished compilation of either success or error.
        SocketMessage::BuildFinished(result) => {
            match result {
                Ok(id) => {
                    built_page_id.set(Some(id.to_string()));
                }
                Err(e) => {
                    built_page_id.set(None);
                    compiler_messages.push(format!("Build Error: {e}"));
                }
            }

            is_compiling.set(false);
            queue_position.set(None);

            true
        }
        // Handle adding compile messages to the log.
        SocketMessage::BuildMessage(msg) => {
            compiler_messages.push(msg);
            false
        }
        // Handle displaying the current queue position to the user.
        SocketMessage::QueuePosition(action) => {
            match action {
                QueueAction::Set(pos) => queue_position.set(Some(pos)),
                QueueAction::Sub => {
                    if let Some(pos) = queue_position() {
                        queue_position.set(Some(pos - 1));
                    }
                }
            }
            false
        }
        SocketMessage::BuildRequest(_) => unimplemented!(),
        SocketMessage::Unknown => {
            warn!("encountered an unknown socket message");
            false
        }
    }
}
