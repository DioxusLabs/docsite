use super::{builder::Builder, BuildCommand, BuildError, BuildMessage, BuildRequest};
use crate::app::EnvVars;
use std::{
    collections::VecDeque,
    error::Error as _,
    sync::{atomic::AtomicBool, Arc},
};
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
};
use uuid::Uuid;

/// Start the build watcher.
///
/// The build watcher receives [`BuildCommand`]s through a channel and handles
/// the build queue, providing queue positions, and stopping/cancelling builds.
pub fn start_build_watcher(
    env: EnvVars,
    is_building: Arc<AtomicBool>,
) -> UnboundedSender<BuildCommand> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut builder = Builder::new(env, is_building);
        let mut pending_builds = VecDeque::new();

        loop {
            select! {
                // Handle incoming build commands.
                Some(command) = rx.recv() => {
                    match command {
                        BuildCommand::Start { request } => start_build(&mut builder, &mut pending_builds, request),
                        BuildCommand::Stop { id } => stop_build(&mut builder, &mut pending_builds, id),
                    }
                }
                // Handle finished build or make progress on current build.
                build_result = builder.finished() => handle_finished_build(&mut builder, &mut pending_builds, build_result),
            }
        }
    });

    tx
}

/// Start a build or add it to the queue.
fn start_build(
    builder: &mut Builder,
    pending_builds: &mut VecDeque<BuildRequest>,
    request: BuildRequest,
) {
    // If the builder has a build, add to queue, otherwise start the build.
    match builder.has_build() {
        false => builder.start(request),
        true => {
            let _ = request
                .ws_msg_tx
                .send(BuildMessage::QueuePosition(pending_builds.len() + 1));

            pending_builds.push_back(request);
        }
    };
}

/// Stop the current build by:
/// - Checking if it's the current build and if so, stop it, update queue positions, and return early.
/// - Iterate through queue looking for a matching id.
///   If matching id found, update queue positions *behind* matching queue and remove matched item.
fn stop_build(builder: &mut Builder, pending_builds: &mut VecDeque<BuildRequest>, id: Uuid) {
    // Check if the ongoing build is the cancelled build.
    let current_build_id = builder.current_build().map(|b| b.id);
    if let Some(current_build_id) = current_build_id {
        if id == current_build_id {
            builder.stop_current();

            // Start the next build request.
            let next_request = pending_builds.pop_front();
            if let Some(request) = next_request {
                builder.start(request);
            }

            update_queue_positions(pending_builds);
            return;
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
            let _ = build_request
                .ws_msg_tx
                .send(BuildMessage::QueuePosition(i - 1));
        }
    }

    // Remove the stopped build.
    if let Some(id) = matching_id {
        pending_builds.remove(id);
    }
}

/// Handle a finished build by:
/// - Finishing the current build, sending the BuildMessage::Finnished to the socket.
/// - Start the next build.
/// - Update queue positions.
fn handle_finished_build(
    builder: &mut Builder,
    pending_builds: &mut VecDeque<BuildRequest>,
    build_result: Result<BuildRequest, BuildError>,
) {
    // Tell the socket the result of their build.
    let _ = match build_result {
        Ok(request) => {
            dioxus::logger::tracing::trace!(request = ?request, "build finished");
            request
                .ws_msg_tx
                .send(BuildMessage::Finished(Ok(request.id)))
        }
        Err(e) => {
            dioxus::logger::tracing::warn!(err = ?e, src = ?e.source(), "build failed");
            match builder.current_build() {
                Some(request) => request
                    .ws_msg_tx
                    .send(BuildMessage::Finished(Err(e.to_string()))),
                None => Ok(()),
            }
        }
    };

    // Start the next build.
    let next_request = pending_builds.pop_front();
    if let Some(request) = next_request {
        builder.start(request);
    }

    update_queue_positions(pending_builds);
}

/// Iterate through the queue and alert each request with their current queue position.
fn update_queue_positions(pending_builds: &mut VecDeque<BuildRequest>) {
    for (i, build_request) in pending_builds.iter_mut().enumerate() {
        let _ = build_request
            .ws_msg_tx
            .send(BuildMessage::QueuePosition(i + 1));
    }
}
