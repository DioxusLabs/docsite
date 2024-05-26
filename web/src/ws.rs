use dioxus::signals::{Signal, Writable as _, WritableVecExt as _};
use futures::{SinkExt as _, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use model::*;

use crate::error::AppError;

pub struct Socket {
    socket: WebSocket,
}

impl Socket {
    pub fn new(url: &str) -> Result<Self, AppError> {
        let socket = WebSocket::open(url).map_err(|e| AppError::Socket(e.to_string()))?;

        Ok(Self { socket })
    }

    pub async fn compile(&mut self, code: String) -> Result<(), AppError> {
        self.socket
            .send(SocketMessage::CompileRequest(code).into())
            .await
            .map_err(|e| AppError::Socket(e.to_string()))
    }

    pub async fn next(&mut self) -> Option<SocketMessage> {
        match self.socket.next().await {
            Some(Ok(msg)) => SocketMessage::try_from(msg).ok(),
            _ => None,
        }
    }

    pub async fn close(self) {
        self.socket.close(None, None).ok();
    }
}

pub fn handle_message(
    mut is_compiling: Signal<bool>,
    mut built_page_id: Signal<Option<String>>,
    mut compiler_messages: Signal<Vec<String>>,
    msg: SocketMessage,
) -> bool {
    match msg {
        SocketMessage::CompileFinished(id) => {
            is_compiling.set(false);
            built_page_id.set(Some(id));
            true
        }
        SocketMessage::CompileMessage(msg) => {
            compiler_messages.push(msg);
            false
        }
        // TODO: Handle banned words on both client and server
        // This would avoid unnescessary requests.
        SocketMessage::BannedWord(word) => {
            compiler_messages.push("Error:".to_string());
            compiler_messages.push(format!("A banned word was used: {word}"));
            compiler_messages
                .push("Please remove any instances of that word and run again.".to_string());
            compiler_messages.push("Using that word inside of another word is not allowed either. e.g. `move` in `remove`".to_string());
            is_compiling.set(false);
            built_page_id.set(None);
            true
        }
        SocketMessage::CompileFinishedWithError => {
            is_compiling.set(false);
            true
        }
        SocketMessage::SystemError(s) => {
            is_compiling.set(false);
            built_page_id.set(None);
            compiler_messages.push(format!("Server Error: {s}"));
            true
        }
        _ => false,
    }
}
