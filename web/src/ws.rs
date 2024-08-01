use crate::{error::AppError, BuildSignals};
use dioxus::signals::{Writable as _, WritableVecExt as _};
use dioxus_logger::tracing::warn;
use futures::{SinkExt as _, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use model::*;

pub struct Socket {
    socket: WebSocket,
}

impl Socket {
    pub fn new(url: &str) -> Result<Self, AppError> {
        let socket = WebSocket::open(url)?;
        Ok(Self { socket })
    }

    pub async fn compile(&mut self, code: String) -> Result<(), AppError> {
        let as_json = SocketMessage::CompileRequest(code).as_json_string()?;

        self.socket.send(Message::Text(as_json)).await?;

        Ok(())
    }

    pub async fn next(&mut self) -> Option<Result<SocketMessage, AppError>> {
        match self.socket.next().await {
            Some(Ok(msg)) => Some(Ok(SocketMessage::from(msg))),
            Some(Err(e)) => Some(Err(e.into())),
            _ => None,
        }
    }

    pub async fn close(self) {
        self.socket.close(None, None).ok();
    }
}

/// Handles a websocket message, returning true if further messages shouldn't be handled.
pub fn handle_message(signals: &mut BuildSignals, msg: SocketMessage) -> bool {
    let BuildSignals {
        is_compiling,
        queue_position,
        built_page_id,
        compiler_messages,
    } = signals;

    match msg {
        // Handle a finished compilation of either success or error.
        SocketMessage::CompileFinished(result) => {
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
        SocketMessage::CompileMessage(msg) => {
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
        SocketMessage::CompileRequest(_) => unimplemented!(),
        SocketMessage::Unknown => {
            warn!("encountered an unknown socket message");
            false
        }
    }
}
