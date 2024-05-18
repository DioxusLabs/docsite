use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt as _};
use tokio::sync::oneshot;

use crate::AppState;

/// Handle any pre-websocket processing.
pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

/// Handle the websocket.
async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut tx, mut rx) = socket.split();
    let mut next_is_code = false;

    while let Some(Ok(msg)) = rx.next().await {
        let Some(txt) = msg.into_text().ok() else {
            tx.send("error:failed to parse received data".into())
                .await
                .ok();
            continue;
        };

        match &*txt {
            "compile" => {
                next_is_code = true;
            }
            txt => {
                if !next_is_code {
                    continue;
                }

                // Verify no banned words were submitted
                if let Some(banned) = is_unsafe(txt) {
                    let formatted = format!("error: banned word:{}", banned);
                    tx.send(formatted.into()).await.ok();
                    continue;
                }

                let (res_tx, res_rx) = oneshot::channel();

                // Receive response from oneshot and parse it. 
                // TODO: Compilation error handling
                if state.build_queue_tx.send((res_tx, txt.to_string())).is_ok() {
                    let Ok(id) = res_rx.await else {
                        tx.send("error:failed to compile code".into()).await.ok();
                        continue;
                    };

                    let formatted = format!("built:{}", id.to_string());
                    tx.send(formatted.into()).await.ok();
                } else {
                    tx.send("error:failed to compile code".into()).await.ok();
                    continue;
                };
            }
        }
    }
}

fn is_unsafe(code: &str) -> Option<&str> {
    for word in crate::BANNED_WORDS {
        if code.find(word).is_some() {
            return Some(*word);
        }
    }
    None
}
