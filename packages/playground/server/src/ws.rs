use std::net::IpAddr;

use crate::{
    AppState,
    build::{BuildCommand, BuildMessage, BuildRequest},
};
use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::IntoResponse,
};
use axum_client_ip::ClientIp;
use dioxus_logger::tracing::error;
use futures::{SinkExt, StreamExt as _};
use governor::clock::{Clock, QuantaClock};
use model::{Project, SocketMessage};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use uuid::Uuid;

/// Handle any pre-websocket processing.
pub async fn ws_handler(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, ip, socket))
}

/// Handle the websocket connection.
///
/// We need to:
/// - Handle submitting build requests, allowing only one build per socket.
/// - Send any build messages to the client.
/// - Stop any ongoing builds if the connection closes.
async fn handle_socket(state: AppState, ip: IpAddr, socket: WebSocket) {
    let (mut socket_tx, mut socket_rx) = socket.split();

    // Start our build loop.
    let (build_tx, mut build_rx) = mpsc::unbounded_channel();
    let mut current_build: Option<BuildRequest> = None;

    loop {
        select! {
            // Parse socket messages, performing the proper action.
            Some(Ok(socket_msg)) = socket_rx.next() => {
                // Try decoding the socket messages into our own type. If invalid, just ignore it.
                let Ok(socket_msg) = SocketMessage::try_from(socket_msg) else {
                    continue;
                };

                // Start a new build, stopping any existing ones.
                if let SocketMessage::BuildRequest { code, previous_build_id } = socket_msg {
                    if let Some(ref request) = current_build {
                        let result = state.build_queue_tx.send(BuildCommand::Stop { id: request.id });
                        if result.is_err() {
                            error!(build_id = ?request.id, "failed to send build stop signal for new build request");
                            continue;
                        }
                    }

                    // Rate limit the build requests by ip
                    if let Err(n) = state.build_govener.check_key(&ip) {
                        let wait_time = n.wait_time_from(QuantaClock::default().now());
                        let socket_msg = SocketMessage::RateLimited(wait_time);
                        let socket_result = socket_tx.send(socket_msg.into_axum()).await;
                        if socket_result.is_err() {
                            break;
                        }
                        tokio::time::sleep(wait_time).await;
                    }

                    let request = start_build(&state, build_tx.clone(), code, previous_build_id);
                    current_build = Some(request);
                }
            }
            // Handle sending build messages to client and closing the socket when finished.
            Some(build_msg) = build_rx.recv() => {
                // Send the build message.
                let socket_msg = SocketMessage::from(build_msg.clone());
                let socket_result = socket_tx.send(socket_msg.into_axum()).await;
                if socket_result.is_err() {
                    break;
                }

                // If the build finished, let's close this socket.
                if   build_msg.is_done() {
                    current_build = None;
                    let _ = socket_tx.close().await;
                    break;
                }
            }
            else => break,
        }
    }

    // The socket has closed. Make sure we cancel any active builds.
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
    previous_build_id: Option<Uuid>,
) -> BuildRequest {
    let project = Project::new(code, None, None);
    let request = BuildRequest {
        id: project.id(),
        previous_build_id,
        project,
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
