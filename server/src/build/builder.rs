use crate::build::{BuildMessage, CliMessage};

use super::watcher::BuildRequest;
use dioxus_dx_wire_format::{BuildStage, StructuredOutput};
use fs_extra::dir::CopyOptions;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::task::JoinHandle;
use tokio::{fs, select};

// The in-code templates that get replaced.
const USER_CODE_ID: &str = "{USER_CODE}";
// The build id template.
const BUILD_ID_ID: &str = "{BUILD_ID}";

// TODO: We need some way of cleaning up any stopped builds.
/// The builder provides a convenient interface for controlling builds running in another task.
pub struct Builder {
    template_path: PathBuf,
    is_building: Arc<AtomicBool>,
    current_build: Option<BuildRequest>,
    task: JoinHandle<()>,
}

impl Builder {
    pub fn new(template_path: PathBuf, is_building: Arc<AtomicBool>) -> Self {
        Self {
            template_path,
            is_building,
            current_build: None,
            task: tokio::spawn(std::future::pending()),
        }
    }

    /// Start a new build, cancelling any ongoing builds.
    pub fn start(&mut self, request: BuildRequest) {
        self.stop_current();
        self.is_building.store(true, Ordering::SeqCst);
        self.current_build = Some(request.clone());
        self.task = tokio::spawn(build(self.template_path.clone(), request));
    }

    /// Stop the current build.
    pub fn stop_current(&mut self) {
        self.task.abort();
        self.task = tokio::spawn(std::future::pending());
        self.current_build = None;
        self.is_building.store(false, Ordering::SeqCst);
    }

    /// Wait for the current build to finish.
    pub async fn finished(&mut self) {
        let task = &mut self.task;
        task.await.unwrap();
        self.task = tokio::spawn(std::future::pending());
        self.current_build = None;
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
async fn build(template_path: PathBuf, request: BuildRequest) {
    setup_template(&template_path, &request).await;
    dx_build(&template_path, &request).await;
    move_to_built(&template_path, &request).await;
}

/// Resets the template with values for the next build.
async fn setup_template(template_path: &PathBuf, request: &BuildRequest) {
    let snippets_from_copy = [
        template_path.join("snippets/main.rs"),
        template_path.join("snippets/Cargo.toml"),
        template_path.join("snippets/Dioxus.toml"),
    ];

    // New locations
    let snippets_to_copy = [
        template_path.join("src/main.rs"),
        template_path.join("Cargo.toml"),
        template_path.join("Dioxus.toml"),
    ];

    // Enumerate over a list of paths to copy and copies them to the new location while modifying any template strings.
    for (i, path) in snippets_from_copy.iter().enumerate() {
        let new_path = &snippets_to_copy[i];
        let contents = fs::read_to_string(path).await.unwrap();

        let contents = contents
            .replace(USER_CODE_ID, &request.code)
            .replace(BUILD_ID_ID, &request.id.to_string());

        fs::write(new_path, contents).await.unwrap();
    }
}

/// Run the build command provided by the DX CLI.
/// Returns if DX built the project successfully.
async fn dx_build(template_path: &PathBuf, request: &BuildRequest) -> bool {
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
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();
    let mut stdout_reader = BufReader::new(stdout).lines();

    loop {
        select! {
            result = stdout_reader.next_line() => {
                let Ok(Some(line)) = result else {
                    continue;
                };

                println!("{:?}", line);

                let cli_message = serde_json::from_str::<CliMessage>(&line).unwrap();
                let Some(json) = cli_message.json else {
                    continue;
                };

                let cli_message = serde_json::from_str::<StructuredOutput>(&json).unwrap();

                match cli_message {
                    StructuredOutput::BuildUpdate { stage } => {
                        match stage {
                            BuildStage::Compiling { current, total, krate, .. } => {
                                println!("compiling");
                                let _ = request.ws_msg_tx.send(BuildMessage::Compiling { current_crate: current, total_crates: total, krate, });
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

            }
            status = child.wait() => {
                println!("DX EXITED WTIH: {:?}", status);

                // Check if the build was successful.
                let exit_code = status.map(|c| c.code());
                if let Ok(Some(code)) = exit_code {
                    if code == 0 {
                        break true;
                    }
                }
                break false;
            }
        }
    }
}

/// Moves the project built by `dx` to the final location for serving.
async fn move_to_built(template_path: &PathBuf, request: &BuildRequest) {
    let id_string = request.id.to_string();

    // The path to the built project from DX
    let play_build_id = format!("play-{}", &id_string);
    let built_project_parent = template_path.join("target/dx").join(play_build_id);

    // The public folder of the built project (what we want).
    let debug_web = built_project_parent.join("debug/web/");
    let public_folder = debug_web.join("public");

    let built_path = PathBuf::from(crate::TEMP_PATH);

    // Rename the built project public folder to the build id.
    // Move the built project to the built projects folder to be served.
    // Delete the built project in the target directory to prevent a storage leak.
    // We use `spawn_blocking` to batch call `std::fs` as recommended by Tokio.
    tokio::task::spawn_blocking(move || {
        // Rename to be the build id
        let built_project = debug_web.join(&id_string);
        std::fs::rename(&public_folder, &built_project).unwrap();

        // Copy to the built project folder for serving.
        let options = CopyOptions::new().overwrite(true);
        fs_extra::dir::move_dir(&built_project, &built_path, &options).unwrap();
        std::fs::remove_dir_all(&built_project_parent).unwrap();
    })
    .await
    .unwrap();
}
