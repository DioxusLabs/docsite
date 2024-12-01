use super::{builder::Builder, BuildMessage};
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use uuid::Uuid;

/// A build command which allows consumers of the builder api to submit and stop builds.
#[derive(Debug, Clone)]
pub enum BuildCommand {
    Start { request: BuildRequest },
    Stop { id: Uuid },
}

/// A build request which contains the id of the build, the code to be built, and a socket to send build updates.
#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub id: Uuid,
    pub code: String,
    pub ws_msg_tx: UnboundedSender<BuildMessage>,
}

/// Start the build watcher.
///
/// The build watcher receives [`BuildCommand`]s through a channel and handles
/// the build queue, providing queue positions, and stopping/cancelling builds.
pub async fn start_build_watcher(
    is_building: Arc<AtomicBool>,
    template_path: PathBuf,
) -> UnboundedSender<BuildCommand> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut builder = Builder::new(template_path, is_building);
        let mut pending_builds = VecDeque::new();

        loop {
            select! {
                // Handle any build commands from connected sockets.
                Some(command) = rx.recv() => {
                    match command {
                        // Add new request to queue and respond with queue position.
                        BuildCommand::Start { request } => {
                            // If the builder has a build, add to queue, otherwise start the build.
                            if builder.has_build() {
                                let _ = request.ws_msg_tx.send(BuildMessage::QueuePosition(pending_builds.len() + 1));
                                pending_builds.push_back(request);
                            } else {
                                builder.start(request);
                            }
                        }
                        // Loop through queue and:
                        // - Remove the request with matching id
                        // - Alert all requests *after* the removed one that their position has moved.
                        BuildCommand::Stop { id } => {
                            // Check if the cancelled build is the ongoing build.
                            let current_build_id = builder.current_build().map(|b| b.id);
                            if let Some(current_build_id) = current_build_id {
                                if id == current_build_id {
                                    builder.stop_current();

                                    // Start the next build request.
                                    let next_request = pending_builds.pop_front();
                                    if let Some(request) = next_request {
                                        builder.start(request);
                                    }
                                    continue;
                                }
                            }

                            // Try finding the build in the queue
                            let mut matching_id = None;

                            for (i, build_request) in pending_builds.iter_mut().enumerate() {
                                if build_request.id == id {
                                    matching_id = Some(i);
                                    continue;
                                }

                                // Tell any other requests behind the removed that they're moving up.
                                if matching_id.is_some() {
                                    let _ = build_request.ws_msg_tx.send(BuildMessage::QueuePosition(i - 1));
                                }
                            }

                            // Remove the stopped build.
                            if let Some(id) = matching_id {
                                pending_builds.remove(id);
                            }
                        }
                    }
                }

                // The current build finished.
                _ = builder.finished() => {
                    let next_request = pending_builds.pop_front();
                    if let Some(request) = next_request {
                        builder.start(request);
                    }
                }
            }
        }
    });

    tx
}
