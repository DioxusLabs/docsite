use crate::app::EnvVars;
use anyhow::{bail, Context, Result};
use dioxus_dx_wire_format::StructuredOutput;
use dioxus_logger::tracing;
use dioxus_logger::tracing::debug;
use fs_extra::dir::CopyOptions;
use model::Project;
use model::{BuildStage, CargoDiagnostic};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio::{fs, select};
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
    pub project: Project,
    pub ws_msg_tx: UnboundedSender<BuildMessage>,
}

/// A message from the playground build process.
#[derive(Debug, Clone, PartialEq)]
pub enum BuildMessage {
    Building(model::BuildStage),
    CargoDiagnostic(CargoDiagnostic),
    Finished(Result<Uuid, String>),
    QueuePosition(usize),
}

/// The DX CLI serves parseable JSON output with the regular tracing message and a parseable "json" field.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CliMessage {
    json: Option<String>,
}

const BUILD_ID_ID: &str = "{BUILD_ID}";

// TODO: We need some way of cleaning up any stopped builds.
/// The builder provides a convenient interface for controlling builds running in another task.
pub struct Builder {
    template_path: PathBuf,
    built_path: PathBuf,
    is_building: Arc<AtomicBool>,
    current_build: Option<BuildRequest>,
    task: JoinHandle<Result<()>>,
}

impl Builder {
    pub fn new(env: EnvVars, is_building: Arc<AtomicBool>) -> Self {
        Self {
            template_path: env.build_template_path,
            built_path: env.built_path,
            is_building,
            current_build: None,
            task: tokio::spawn(std::future::pending()),
        }
    }

    /// Start a new build, cancelling any ongoing builds.
    pub fn start(&mut self, request: BuildRequest) {
        let _ = request.ws_msg_tx.send(BuildMessage::QueuePosition(0));

        self.stop_current();
        self.is_building.store(true, Ordering::SeqCst);
        self.current_build = Some(request.clone());
        self.task = tokio::spawn(build(
            self.template_path.clone(),
            self.built_path.clone(),
            request,
        ));
    }

    /// Stop the current build.
    pub fn stop_current(&mut self) {
        self.task.abort();
        self.task = tokio::spawn(std::future::pending());
        self.current_build = None;
        self.is_building.store(false, Ordering::SeqCst);
    }

    /// Wait for the current build to finish.
    pub async fn finished(&mut self) -> Result<BuildRequest> {
        // Ensure we don't poll a completed task.
        if self.task.is_finished() {
            self.stop_current();
        }

        // Make progress on the build task.
        let task = &mut self.task;
        task.await??;

        self.current_build.take().context("no current build")
    }

    /// Check if the builder has an ongoing build.
    pub fn has_build(&self) -> bool {
        self.current_build.is_some()
    }

    /// Get the current ongoing build if it exists.
    pub fn current_build(&self) -> Option<BuildRequest> {
        self.current_build.clone()
    }
}

/// Run the steps to produce a build for a [`BuildRequest`]
async fn build(template_path: PathBuf, built_path: PathBuf, request: BuildRequest) -> Result<()> {
    // If the project already exists, don't build it again.
    if std::fs::exists(built_path.join(request.id.to_string())).unwrap_or_default() {
        tracing::trace!("Skipping build for {request:?} since it already exists");
        return Ok(());
    }

    setup_template(&template_path, &request).await?;
    dx_build(&template_path, &request).await?;
    tracing::trace!("Noving build from {template_path:?} to {built_path:?}");
    move_to_built(&template_path, &built_path, &request).await?;

    Ok(())
}

/// Resets the template with values for the next build.
async fn setup_template(template_path: &Path, request: &BuildRequest) -> Result<()> {
    let snippets_from_copy = [
        template_path.join("snippets/Cargo.toml"),
        template_path.join("snippets/Dioxus.toml"),
    ];

    // New locations
    let snippets_to_copy = [
        template_path.join("Cargo.toml"),
        template_path.join("Dioxus.toml"),
    ];

    // Enumerate over a list of paths to copy and copies them to the new location while modifying any template strings.
    for (i, path) in snippets_from_copy.iter().enumerate() {
        let new_path = &snippets_to_copy[i];
        let contents = fs::read_to_string(path).await?;
        let contents = contents.replace(BUILD_ID_ID, &request.id.to_string());
        fs::write(new_path, contents).await?;
    }

    // Write the user's code to main.rs
    fs::write(
        template_path.join("src/main.rs"),
        request.project.contents(),
    )
    .await?;

    Ok(())
}

/// Run the build command provided by the DX CLI.
/// Returns if DX built the project successfully.
async fn dx_build(template_path: &PathBuf, request: &BuildRequest) -> Result<()> {
    let mut child = Command::new("dx")
        .arg("build")
        .arg("--platform")
        .arg("web")
        .arg("--json-output")
        .arg("--verbose")
        .arg("--trace")
        .current_dir(template_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("dx stdout should exist");
    let mut stdout_reader = BufReader::new(stdout).lines();

    let mut logs = Vec::new();

    loop {
        select! {
            // Read stdout lines from DX.
            result = stdout_reader.next_line() => {
                let Ok(Some(line)) = result else {
                    continue;
                };

                logs.push(line.clone());
                process_dx_message(request, line);
            }
            // Wait for the DX process to exit.
            status = child.wait() => {
                // Check if the build was successful.
                let exit_code = status.map(|c| c.code());
                if let Ok(Some(code)) = exit_code {
                    if code == 0 {
                        break;
                    } else {
                        // Dump logs in debug.
                        for log in logs {
                            debug!("{log}");
                        }

                        bail!("DX build failed: {code}");
                    }
                }
                bail!("DX build failed");
            }
        }
    }

    Ok(())
}

/// Process a JSON-formatted message from the DX CLI, returning nothing on error.
///
/// We don't care if this errors as it is human-readable output which the playground doesn't depend on for build status.
fn process_dx_message(request: &BuildRequest, message: String) {
    // We parse the tracing json log and if it contains a json field, we parse that as StructuredOutput.
    let result = serde_json::from_str::<CliMessage>(&message)
        .ok()
        .and_then(|v| v.json)
        .and_then(|json| serde_json::from_str::<StructuredOutput>(&json).ok());

    let Some(output) = result else {
        return;
    };

    let _ = match output {
        StructuredOutput::BuildUpdate { stage } => {
            let stage = BuildStage::from(stage);
            request.ws_msg_tx.send(BuildMessage::Building(stage))
        }
        StructuredOutput::CargoOutput { message } => {
            let Ok(diagnostic) = CargoDiagnostic::try_from(message) else {
                return;
            };

            // Don't send any diagnostics for dependencies.
            if diagnostic.target_crate != format!("play-{}", request.id) {
                return;
            }

            request
                .ws_msg_tx
                .send(BuildMessage::CargoDiagnostic(diagnostic))
        }
        _ => Ok(()),
    };
}

/// Moves the project built by `dx` to the final location for serving.
async fn move_to_built(
    template_path: &Path,
    built_path: &Path,
    request: &BuildRequest,
) -> Result<()> {
    let id_string = request.id.to_string();

    // The path to the built project from DX
    let play_build_id = format!("play-{}", &id_string);
    let built_project_parent = template_path.join("target/dx").join(play_build_id);

    // The public folder of the built project (what we want).
    let debug_web = built_project_parent.join("debug/web/");
    let public_folder = debug_web.join("public");

    let built_path = built_path.to_path_buf();

    // Rename the built project public folder to the build id.
    // Move the built project to the built projects folder to be served.
    // Delete the built project in the target directory to prevent a storage leak.
    // We use `spawn_blocking` to batch call `std::fs` as recommended by Tokio.
    tokio::task::spawn_blocking::<_, Result<()>>(move || {
        // Rename to be the build id
        let built_project = debug_web.join(&id_string);
        std::fs::rename(&public_folder, &built_project)?;

        // Copy to the built project folder for serving.
        let options = CopyOptions::new().overwrite(true);
        fs_extra::dir::move_dir(&built_project, &built_path, &options)?;
        std::fs::remove_dir_all(&built_project_parent)?;
        Ok(())
    })
    .await??;

    Ok(())
}

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
    build_result: Result<BuildRequest>,
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
