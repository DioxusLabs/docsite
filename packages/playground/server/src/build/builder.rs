use super::{BuildError, BuildRequest};
use crate::app::EnvVars;
use crate::build::{BuildMessage, CliMessage};
use dioxus::subsecond::JumpTable;
use dioxus_dx_wire_format::StructuredOutput;
use dioxus_logger::tracing;
use dioxus_logger::tracing::debug;
use fs_extra::dir::CopyOptions;
use model::{BuildStage, CargoDiagnostic};
use std::future::pending;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Stdio;
use std::process::{Child, Command};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::fs;
use tokio::task::JoinHandle;
use tracing::{trace, warn};

const BUILD_ID_ID: &str = "{BUILD_ID}";

// TODO: We need some way of cleaning up any stopped builds.
/// The builder provides a convenient interface for controlling builds running in another task.
pub struct Builder {
    env: EnvVars,
    is_building: Arc<AtomicBool>,
    current_build: Option<BuildRequest>,
    task: Option<JoinHandle<Result<Option<JumpTable>, BuildError>>>,
}

impl Builder {
    pub fn new(env: EnvVars, is_building: Arc<AtomicBool>) -> Self {
        Self {
            env,
            is_building,
            current_build: None,
            task: None,
        }
    }

    /// Make sure the components are initilized
    pub fn update_component_library(&mut self) -> Result<(), BuildError> {
        let update_status = Command::new("dx")
            .arg("component")
            .arg("update")
            .current_dir(&self.env.build_template_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if !update_status.success() {
            return Err(BuildError::DxFailed(update_status.code()));
        }
        let status = Command::new("dx")
            .arg("component")
            .arg("add")
            .arg("--all")
            .arg("--force")
            .current_dir(&self.env.build_template_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        if !status.success() {
            return Err(BuildError::DxFailed(status.code()));
        }

        Ok(())
    }

    /// Start a new build, cancelling any ongoing builds.
    pub fn start(&mut self, request: BuildRequest) {
        let _ = request.ws_msg_tx.send(BuildMessage::QueuePosition(0));

        self.stop_current();
        self.is_building.store(true, Ordering::SeqCst);
        self.current_build = Some(request.clone());
        self.task = Some(tokio::spawn(build(self.env.clone(), request)));
    }

    /// Stop the current build.
    pub fn stop_current(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
        self.current_build = None;
        self.is_building.store(false, Ordering::SeqCst);
    }

    /// Wait for the current build to finish.
    pub async fn finished(&mut self) -> Result<(BuildRequest, Option<JumpTable>), BuildError> {
        // Ensure we don't poll a completed task.
        if let Some(task) = &mut self.task {
            if task.is_finished() {
                self.stop_current();
            } else {
                // Make progress on the build task.
                let response = task.await??;
                let request = self.current_build.take().ok_or(BuildError::NotStarted)?;
                return Ok((request, response));
            }
        }
        pending().await
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
async fn build(env: EnvVars, request: BuildRequest) -> Result<Option<JumpTable>, BuildError> {
    let built_path = &env.built_path;
    let template_path = &env.build_template_path;

    // If the project already exists, don't build it again.
    if std::fs::exists(built_path.join(request.id.to_string())).unwrap_or_default() {
        tracing::trace!("Skipping build for {request:?} since it already exists");
        return Ok(None);
    }

    // Check if we need to clean up old builds before starting a new one.
    if let Err(e) = super::cleanup::check_cleanup(&env).await {
        warn!("failed to clean built projects: {e}");
    }

    let patch = dx_build(&request, &env).await?;
    move_to_built(&template_path, &built_path, &request, patch.is_some()).await?;

    Ok(patch)
}

/// Resets the template with values for the next build.
async fn setup_template(
    template_path: &Path,
    request: &BuildRequest,
    patch: bool,
) -> Result<(), BuildError> {
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
        let contents = contents.replace(
            BUILD_ID_ID,
            &request
                .previous_build_id
                .filter(|_| patch)
                .unwrap_or(request.id)
                .to_string(),
        );
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

/// Start a process with limited access to the environment and resources
fn start_limited_process(mut command: Command, env: &EnvVars) -> Result<Child, BuildError> {
    let filtered_vars = std::env::vars().filter(|(k, _)| {
        let allowed = ["RUST_VERSION", "RUSTUP_HOME", "CARGO_HOME", "PATH", "HOME"];
        allowed.contains(&k.as_str())
    });

    let child = command
        .env_clear()
        .envs(filtered_vars)
        .current_dir(&env.build_template_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    set_dx_limits(&child, env);

    Ok(child)
}

/// Run the build command provided by the DX CLI.
/// Returns if DX built the project successfully.
async fn dx_build(request: &BuildRequest, env: &EnvVars) -> Result<Option<JumpTable>, BuildError> {
    let previous_build_cache_dir = request
        .previous_build_id
        .map(|id| env.built_template_hotpatch_cache(&id))
        .filter(|p| p.exists());

    // if there is a previous cache, try to use that to hot patch the build
    if let Some(cache_dir) = previous_build_cache_dir {
        let cache_dir = cache_dir.canonicalize()?;
        let result = dx_patch_build(&request, &env, &cache_dir).await;
        if let Ok(Some(patch)) = result {
            return Ok(Some(patch));
        } else {
            trace!("hotpatch build failed, falling back to full build");
        }
    }

    let cache_dir = env.built_template_hotpatch_cache(&request.id);

    // fallback to a full build
    if let Err(err) = std::fs::create_dir_all(&cache_dir) {
        warn!("failed to create hotpatch cache dir {cache_dir:?}: {err}");
    }
    let cache_dir = cache_dir.canonicalize()?;
    dx_full_build(request, env, &cache_dir).await.map(|_| None)
}

async fn dx_patch_build(
    request: &BuildRequest,
    env: &EnvVars,
    cache_dir: &Path,
) -> Result<Option<JumpTable>, BuildError> {
    setup_template(&env.build_template_path, &request, true).await?;
    let mut command = Command::new("dx");
    command
        .arg("tools")
        .arg("hotpatch")
        .arg("--web")
        .arg("--session-cache-dir")
        .arg(cache_dir)
        .arg("--aslr-reference")
        .arg("0")
        .arg("--json-output");
    tracing::info!("running dx command: {:?}", command);

    let mut child = start_limited_process(command, env)?;

    // If there is a artifacts cache, we can pipe it to dx for hot patching.
    let artifacts_cache = cache_dir.join("artifacts.json");
    let artifacts_cache = std::fs::read_to_string(artifacts_cache)?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(artifacts_cache.as_bytes())?;
        stdin.flush()?;
    }

    let (logs, patch) = process_build_messages(&mut child, env, request);

    let status = tokio::task::spawn_blocking(move || child.wait()).await;

    // Check if the build was successful.
    let exit_code = status.map(|r| r.map(|c| c.code()));
    if let Ok(Ok(Some(code))) = exit_code {
        if code == 0 {
            return Ok(patch);
        } else {
            // Dump logs in debug.
            for log in logs {
                debug!("{log}");
            }
            return Err(BuildError::DxFailed(Some(code)));
        }
    }

    // Dump logs in debug.
    for log in logs {
        debug!("{log}");
    }

    Err(BuildError::DxFailed(None))
}

async fn dx_full_build(
    request: &BuildRequest,
    env: &EnvVars,
    cache_dir: &Path,
) -> Result<(), BuildError> {
    setup_template(&env.build_template_path, &request, false).await?;

    let mut command = Command::new("dx");
    command
        .arg("build")
        .arg("--web")
        .arg("--fat-binary")
        .arg("--session-cache-dir")
        .arg(cache_dir)
        .arg("--json-output");
    tracing::info!("running dx command: {:?}", command);
    let mut child = start_limited_process(command, env)?;

    let (logs, _patch) = process_build_messages(&mut child, env, request);

    let status = tokio::task::spawn_blocking(move || child.wait()).await;

    // Check if the build was successful.
    let exit_code = status.map(|r| r.map(|c| c.code()));
    if let Ok(Ok(Some(code))) = exit_code {
        if code == 0 {
            return Ok(());
        } else {
            // Dump logs in debug.
            for log in logs {
                debug!("{log}");
            }
            return Err(BuildError::DxFailed(Some(code)));
        }
    }
    Err(BuildError::DxFailed(None))
}

fn process_build_messages(
    child: &mut Child,
    env: &EnvVars,
    request: &BuildRequest,
) -> (Vec<String>, Option<JumpTable>) {
    let stderr = child.stderr.take().expect("dx stdout should exist");
    let stdout = child.stdout.take().expect("dx stdout should exist");
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let mut logs = Vec::new();
    let mut patch = None;

    while let Some(Ok(line)) = stdout_reader.next() {
        logs.push(line.clone());
        patch = patch.or(process_dx_message(env, request, line));
    }

    while let Some(Ok(line)) = stderr_reader.next() {
        logs.push(line.clone());
        warn!("dx stderr: {line}");
    }

    (logs, patch)
}

#[allow(unused)]
fn set_dx_limits(process: &Child, env: &EnvVars) {
    // TODO this isn't working, not sure why
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use rustix::process::{Resource, Rlimit};
        let id = rustix::process::Pid::from_child(process);
        let memory_limit = Rlimit {
            current: Some(env.dx_memory_limit),
            maximum: Some(env.dx_memory_limit),
        };

        if let Err(err) = rustix::process::prlimit(Some(id), Resource::As, memory_limit) {
            warn!("failed to set memory limit for dx process {id}: {err}");
        }

        let cpu_limit = Rlimit {
            current: Some(env.dx_build_timeout),
            maximum: Some(env.dx_build_timeout),
        };

        if let Err(err) = rustix::process::prlimit(Some(id), Resource::Cpu, cpu_limit) {
            warn!("failed to set cpu time limit for dx process {id}: {err}");
        }
    }
}

/// Process a JSON-formatted message from the DX CLI, returning nothing on error.
///
/// We don't care if this errors as it is human-readable output which the playground doesn't depend on for build status.
fn process_dx_message(env: &EnvVars, request: &BuildRequest, message: String) -> Option<JumpTable> {
    // We parse the tracing json log and if it contains a json field, we parse that as StructuredOutput.
    let result = serde_json::from_str::<CliMessage>(&message)
        .and_then(|m| serde_json::from_str::<StructuredOutput>(&m.json));

    let Ok(output) = result else {
        return None;
    };

    let _ = match output {
        StructuredOutput::BuildUpdate { stage } => {
            let stage = BuildStage::from(stage);
            request.ws_msg_tx.send(BuildMessage::Building(stage))
        }
        StructuredOutput::CargoOutput { message } => {
            let Ok(diagnostic) = CargoDiagnostic::try_from(message) else {
                return None;
            };

            // Don't send any diagnostics for dependencies.
            if diagnostic.target_crate != Some(format!("play-{}", request.id)) {
                return None;
            }

            request
                .ws_msg_tx
                .send(BuildMessage::CargoDiagnostic(diagnostic))
        }
        StructuredOutput::RustcOutput { message } => {
            let Ok(diagnostic) = CargoDiagnostic::try_from(message) else {
                return None;
            };

            request
                .ws_msg_tx
                .send(BuildMessage::CargoDiagnostic(diagnostic))
        }
        StructuredOutput::BuildsFinished { client, .. } => {
            let cache_dir = env.built_template_hotpatch_cache(&request.id);
            let artifacts_cache = cache_dir.join("artifacts.json");
            if let Err(err) = std::fs::write(
                &artifacts_cache,
                serde_json::to_string(&client).unwrap_or_default(),
            ) {
                warn!("failed to write artifacts cache {artifacts_cache:?}: {err}");
            }
            Ok(())
        }
        StructuredOutput::Hotpatch { jump_table, .. } => {
            return Some(jump_table);
        }
        _ => Ok(()),
    };

    None
}

/// Moves the project built by `dx` to the final location for serving.
async fn move_to_built(
    template_path: &Path,
    built_path: &Path,
    request: &BuildRequest,
    patched: bool,
) -> Result<(), BuildError> {
    let id_string = request
        .previous_build_id
        .filter(|_| patched)
        .unwrap_or(request.id)
        .to_string();

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
        _ = std::fs::create_dir_all(&built_path);
        // Copy to the built project folder for serving.
        let options = CopyOptions::new().overwrite(true);
        fs_extra::dir::copy(&public_folder, &built_path, &options)?;
        let out_dir = built_path.join(id_string);
        // Remove the old output dir if it exists
        _ = std::fs::remove_dir_all(&out_dir);
        // rename to the build id
        std::fs::rename(built_path.join("public"), out_dir)?;
        Ok(())
    })
    .await??;

    Ok(())
}
