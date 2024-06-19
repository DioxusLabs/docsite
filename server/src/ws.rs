use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt as _};
use model::SocketMessage;
use tokio::sync::mpsc;

use crate::{build::BuildMessage, AppState};

/// Handle any pre-websocket processing.
pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

/// Handle the websocket.
async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut tx, mut rx) = socket.split();

    // Store common errors
    let failed_to_parse =
        SocketMessage::SystemError("failed to parse received data".to_string()).to_string();

    let failed_to_compile =
        SocketMessage::SystemError("failed to compile code".to_string()).to_string();

    while let Some(Ok(msg)) = rx.next().await {
        // Get websocket message and try converting to text.
        let Some(txt) = msg.into_text().ok() else {
            tx.send(failed_to_parse.clone().into()).await.ok();
            continue;
        };

        // Try converting websocket message from text to `SocketMessage`
        let Ok(msg) = SocketMessage::try_from(txt) else {
            tx.send(failed_to_parse.clone().into()).await.ok();
            continue;
        };

        if let SocketMessage::CompileRequest(code) = msg {
            // Verify no banned words were submitted
            if let Some(banned) = is_unsafe(&code) {
                let banned_msg = SocketMessage::BannedWord(banned).to_string();
                tx.send(banned_msg.into()).await.ok();
                continue;
            }

            let (res_tx, mut res_rx) = mpsc::unbounded_channel();

            // Receive response from oneshot and parse it.
            if state.build_queue_tx.send((res_tx, code)).is_ok() {
                while let Some(msg) = res_rx.recv().await {
                    let as_socket_msg: SocketMessage = msg.into();
                    tx.send(as_socket_msg.to_string().into()).await.ok();
                }
            } else {
                tx.send(failed_to_compile.clone().into()).await.ok();
                continue;
            };
        }
    }
}

fn is_unsafe(code: &str) -> Option<String> {
    for word in crate::BANNED_WORDS {
        if code.contains(word) {
            return Some(word.to_string());
        }
    }
    None
}

impl From<BuildMessage> for SocketMessage {
    fn from(value: BuildMessage) -> Self {
        match value {
            BuildMessage::Message(msg) => SocketMessage::CompileMessage(msg),
            BuildMessage::Finished(id) => SocketMessage::CompileFinished(id.to_string()),
            BuildMessage::BuildError(e) => SocketMessage::SystemError(e),
            BuildMessage::FinishedWithError => SocketMessage::CompileFinishedWithError,
        }
    }
}
