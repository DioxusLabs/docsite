use super::{BuildError, BuildRequest};
use crate::app::EnvVars;
use crate::build::{BuildMessage, CliMessage};
use dioxus_dx_wire_format::StructuredOutput;
use dioxus_logger::tracing;
use dioxus_logger::tracing::debug;
use fs_extra::dir::CopyOptions;
use model::{BuildStage, CargoDiagnostic};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::task::JoinHandle;
use tokio::{fs, select};

const BUILD_ID_ID: &str = "{BUILD_ID}";

// TODO: We need some way of cleaning up any stopped builds.
/// The builder provides a convenient interface for controlling builds running in another task.
pub struct Builder {
    template_path: PathBuf,
    built_path: PathBuf,
    is_building: Arc<AtomicBool>,
    current_build: Option<BuildRequest>,
    task: JoinHandle<Result<(), BuildError>>,
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
    pub async fn finished(&mut self) -> Result<BuildRequest, BuildError> {
        // Ensure we don't poll a completed task.
        if self.task.is_finished() {
            self.stop_current();
        }

        // Make progress on the build task.
        let task = &mut self.task;
        task.await??;

        self.current_build.take().ok_or(BuildError::NotStarted)
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
async fn build(
    template_path: PathBuf,
    built_path: PathBuf,
    request: BuildRequest,
) -> Result<(), BuildError> {
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
async fn setup_template(template_path: &Path, request: &BuildRequest) -> Result<(), BuildError> {
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
async fn dx_build(template_path: &PathBuf, request: &BuildRequest) -> Result<(), BuildError> {
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

                        return Err(BuildError::DxFailed(Some(code)));
                    }
                }
                return Err(BuildError::DxFailed(None));
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
) -> Result<(), BuildError> {
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
    tokio::task::spawn_blocking::<_, Result<(), BuildError>>(move || {
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
