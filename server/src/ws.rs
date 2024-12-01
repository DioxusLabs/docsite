use crate::{
    build::{
        watcher::{BuildCommand, BuildRequest},
        BuildMessage,
    },
    AppState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use dioxus_logger::tracing::error;
use futures::{stream::SplitSink, SinkExt, StreamExt as _};
use model::SocketMessage;
use tokio::{
    select,
    sync::mpsc::{self},
};
use uuid::Uuid;

/// Handle any pre-websocket processing.
pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

/// Handle the websocket connection.
///
/// We need to:
/// - Handle submitting build requests, allowing only one build per socket.
/// - Send any build messages to the client.
/// - Stop any ongoing builds if the connection closes.
async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut socket_tx, mut socket_rx) = socket.split();

    let (build_tx, mut build_rx) = mpsc::unbounded_channel();
    let mut current_build: Option<BuildRequest> = None;

    loop {
        select! {
            // Parse socket messages, performing the proper action.
            Some(Ok(socket_msg)) = socket_rx.next() => {
                let text = socket_msg.into_text().unwrap();
                let socket_msg = SocketMessage::from(text);

                match socket_msg {
                    // Start a new build, stopping any existing ones.
                    SocketMessage::CompileRequest(code) => {
                        if let Some(ref request) = current_build {
                            let result = state.build_queue_tx.send(BuildCommand::Stop { id: request.id });
                            if result.is_err() {
                                error!(build_id = ?request.id, "failed to send build stop signal for new build request");
                                continue;
                            }
                        }

                        let request = start_build(&state, &socket_tx, code);
                        current_build = Some(request);
                    }
                    // We don't care about any other message.
                    _ => {}
                }
            }
            // Handle sending build messages to client and closing the socket when finished.
            Some(build_msg) = build_rx.recv() => {
                match build_msg {
                    BuildMessage::Finished => {
                        current_build = None;
                        // TODO: send finished message
                        let _ = socket_tx.close();
                    },
                    _ => {}
                }
            }
            else => break,
        }
    }

    // The socket has closed for some reason. Make sure we cancel any active builds.
    if let Some(request) = current_build {
        let result = state
            .build_queue_tx
            .send(BuildCommand::Stop { id: request.id });

        if result.is_err() {
            error!(build_id = ?request.id, "failed to send build stop signal for closed websocket");
        }
    }
}

/// Assembles the build request and sends it off.
fn start_build(
    state: &AppState,
    socket_tx: &SplitSink<WebSocket, Message>,
    code: String,
) -> BuildRequest {
    let build_id = Uuid::new_v4();
    let (build_tx, build_rx) = mpsc::unbounded_channel();

    let request = BuildRequest {
        id: build_id.clone(),
        code,
        ws_msg_tx: build_tx,
    };

    state.build_queue_tx.send(BuildCommand::Start {
        request: request.clone(),
    });
    request
}
