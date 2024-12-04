use crate::{
    build::{BuildCommand, BuildMessage, BuildRequest},
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
use futures::{SinkExt, StreamExt as _};
use model::SocketMessage;
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
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
                let socket_msg = SocketMessage::try_from(socket_msg).unwrap();

                // Start a new build, stopping any existing ones.
                if let SocketMessage::BuildRequest(code) = socket_msg {
                    if let Some(ref request) = current_build {
                        let result = state.build_queue_tx.send(BuildCommand::Stop { id: request.id });
                        if result.is_err() {
                            error!(build_id = ?request.id, "failed to send build stop signal for new build request");
                            continue;
                        }
                    }

                    let request = start_build(&state, build_tx.clone(), code);
                    current_build = Some(request);
                }
            }
            // Handle sending build messages to client and closing the socket when finished.
            Some(build_msg) = build_rx.recv() => {
                let socket_msg = SocketMessage::from(build_msg.clone());
                let _ = socket_tx.send(Message::Text(socket_msg.as_json_string().unwrap())).await;

                // If the build finished, let's close this socket.
                if let BuildMessage::Finished(_) = build_msg {
                    current_build = None;
                    let _ = socket_tx.close().await;
                    break;
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

/// Assembles the build request and sends it to the queue.
fn start_build(
    state: &AppState,
    build_tx: UnboundedSender<BuildMessage>,
    code: String,
) -> BuildRequest {
    let build_id = Uuid::new_v4();
    let request = BuildRequest {
        id: build_id,
        code,
        ws_msg_tx: build_tx,
    };

    state
        .build_queue_tx
        .send(BuildCommand::Start {
            request: request.clone(),
        })
        .expect("the build queue channel should never close");

    request
}

impl From<BuildMessage> for SocketMessage {
    fn from(value: BuildMessage) -> Self {
        match value {
            BuildMessage::Building(stage) => Self::BuildStage(stage),
            BuildMessage::Finished(result) => Self::BuildFinished(result),
            BuildMessage::QueuePosition(i) => Self::QueuePosition(i),
            BuildMessage::CargoDiagnostic(diagnostic) => Self::BuildDiagnostic(diagnostic),
        }
    }
}
