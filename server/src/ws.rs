use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use futures::{stream::SplitSink, SinkExt, StreamExt as _};
use model::{QueueAction, SocketMessage};
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

use crate::{
    build::{
        watcher::{BuildCommand, BuildRequest},
        BuildMessage,
    },
    AppState,
};

/// Handle any pre-websocket processing.
pub async fn ws_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

/// Handle the websocket.
async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut socket_tx, mut socket_rx) = socket.split();

    // Store common errors
    // let failed_to_parse =
    //     SocketMessage::CompileFinished(Err("failed to parse received data".to_string()))
    //         .as_json_string()
    //         .expect("failed to convert error message to json");

    // let failed_to_compile =
    //     SocketMessage::CompileFinished(Err("failed to compile code".to_string()))
    //         .as_json_string()
    //         .expect("failed to convert error message to json");

    while let Some(Ok(msg)) = socket_rx.next().await {
        // Get websocket message and try converting to text.
        let Some(txt) = msg.into_text().ok() else {
            //tx.send(failed_to_parse.clone().into()).await.ok();
            continue;
        };

        // Convert websocket message from text to `SocketMessage`
        let msg = SocketMessage::from(txt);

        match msg {
            SocketMessage::CompileRequest(code) => {
                handle_compile_request(&state, &socket_tx, code).await
            }
            _ => {}
        };

        if let SocketMessage::CompileRequest(code) = msg {
            let (res_tx, mut res_rx) = mpsc::unbounded_channel();

            // Send build request and receive any response from builder.
            if state.build_queue_tx.send((res_tx, code)).is_ok() {
                while let Some(msg) = res_rx.recv().await {
                    let socket_msg = SocketMessage::from(msg);
                    if let Ok(as_json) = socket_msg.as_json_string() {
                        tx.send(as_json.into()).await.ok();
                    }
                }
            } else {
                tx.send(failed_to_compile.clone().into()).await.ok();
                continue;
            };
        }
    }
}

async fn handle_compile_request(
    state: &AppState,
    socket_tx: &SplitSink<WebSocket, Message>,
    code: String,
) {
    let build_id = Uuid::new_v4();
    let (build_tx, build_rx) = mpsc::unbounded_channel();

    let request = BuildRequest {
        id: build_id.clone(),
        code,
        ws_msg_tx: build_tx,
    };

    state.build_queue_tx.send(BuildCommand::Start { request });
    while let Some(build_msg) = build_rx.recv().await {
        match build_msg {
            BuildMessage::Finished => break,
            _ => {}
        }
    }

    
}
