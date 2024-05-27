use crate::REMOVAL_DELAY;
use dioxus_logger::tracing::error;
use fs_extra::{dir, file};
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc::{self, UnboundedSender},
};
use uuid::Uuid;

// The in-code templates that get replaced.
const USER_CODE_ID: &str = "{USER_CODE}";
// Cargo complains about `{}` inside of package name.
const BUILD_ID_ID: &str = "BUILD_ID";

pub type QueueType = (UnboundedSender<BuildMessage>, String);

/// Represents a message from the build process.
pub enum BuildMessage {
    Message(String),
    Finished(Uuid),
    FinishedWithError,
    BuildError(String),
}

pub async fn start_build_watcher(build_template_path: String) -> UnboundedSender<QueueType> {
    let (tx, mut rx) = mpsc::unbounded_channel::<QueueType>();

    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            build(&build_template_path, item.0, item.1).await;
        }
    });

    tx
}

async fn build(build_template_path: &str, tx: UnboundedSender<BuildMessage>, code: String) {
    let id = Uuid::new_v4();
    let template = PathBuf::from(build_template_path);
    let dist = template.join("dist");

    // Delete template/dist if it exists (clean slate)
    fs::remove_dir_all(&dist).await.ok();

    let snippets_from_copy = [
        template.join("snippets/main.rs"),
        template.join("snippets/Cargo.toml"),
        template.join("snippets/Dioxus.toml"),
    ];

    // New locations
    let snippets_to_copy = [
        template.join("src/main.rs"),
        template.join("Cargo.toml"),
        template.join("Dioxus.toml"),
    ];

    let options = file::CopyOptions::new().overwrite(true);

    // Enumerates over a list of paths to copy and copies them, replacing the existing ones.
    // Allows us to reset the template for the new build.
    for (i, path) in snippets_from_copy.iter().enumerate() {
        let new_path = &snippets_to_copy[i];
        if let Err(e) = fs_extra::file::copy(path, &new_path, &options) {
            error!(?path, ?new_path, error = ?e, "failed to copy snippets to destination path");
            tx.send(BuildMessage::BuildError(
                "Server failed to copy snippets.".to_string(),
            ))
            .ok();
            return;
        }
    }

    // Modify template with new id and code.
    // (Dioxus.toml, Cargo.toml, main.rs)
    for path in snippets_to_copy {
        let text = match fs::read_to_string(&path).await {
            Ok(text) => text,
            Err(e) => {
                error!(?path, error = ?e, "failed to read snippet");
                tx.send(BuildMessage::BuildError(
                    "Server failed to read snippet.".to_string(),
                ))
                .ok();
                return;
            }
        };

        // Replace the in-code templates with correct values.
        let text = text
            .replace(USER_CODE_ID, &code)
            .replace(BUILD_ID_ID, id.to_string().as_str());

        if let Err(e) = fs::write(&path, &text).await {
            error!(?path, ?text, error = ?e, "failed to write snippet to new destination");
            tx.send(BuildMessage::BuildError(
                "Server failed to write snippet to new destination.".to_string(),
            ))
            .ok();
            return;
        }
    }

    // Run `dx build` in template
    let mut child = match Command::new("dx")
        .arg("build")
        .arg("--platform")
        .arg("web")
        .arg("--raw-out")
        .current_dir(&template)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!(?template, error = ?e, "dx build failed to execute");
            tx.send(BuildMessage::BuildError(
                "Server failed to build project.".to_string(),
            ))
            .ok();
            return;
        }
    };

    // Get stdout and stderr
    let stdout = match child.stdout.take() {
        Some(out) => out,
        None => {
            error!("failed to take stdout from command");
            tx.send(BuildMessage::BuildError(
                "Server failed to build project.".to_string(),
            ))
            .ok();
            return;
        }
    };

    let stderr = match child.stderr.take() {
        Some(out) => out,
        None => {
            error!("failed to take stderr from command");
            tx.send(BuildMessage::BuildError(
                "Server failed to build project.".to_string(),
            ))
            .ok();
            return;
        }
    };

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    // Get output and wait for process to finish.
    loop {
        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        tx.send(BuildMessage::Message(line)).ok();
                    },
                    Err(e) => {
                        error!(error = ?e, "error reading stdout");
                        tx.send(BuildMessage::BuildError(
                            "Server failed to build project.".to_string(),
                        ))
                        .ok();
                        return;
                    }
                    _ => {},
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        tx.send(BuildMessage::Message(line)).ok();
                    },
                    Err(e) => {
                        error!(error = ?e, "error reading stderr");
                        tx.send(BuildMessage::BuildError(
                            "Server failed to build project.".to_string(),
                        ))
                        .ok();
                        return;
                    },
                    _ => {},
                }
            }
            _ = child.wait() => {
                break;
            }
        }
    }

    if !dist.exists() {
        tx.send(BuildMessage::FinishedWithError).ok();
        return;
    }

    // Copy `dist` contents to `temp/my-uuid`
    let temp_path = PathBuf::from(crate::TEMP_PATH);
    let new_dist = template.join(id.to_string());
    if let Err(e) = fs::rename(&dist, &new_dist).await {
        error!(from = ?dist, to = ?new_dist, error = ?e, "failed to rename built project");
        tx.send(BuildMessage::BuildError(
            "Server failed to finalize built project.".to_string(),
        ))
        .ok();
        return;
    };

    let options = dir::CopyOptions::new().overwrite(true);

    // Copy build and named project to temp directory to serve.
    if let Err(e) = fs_extra::dir::move_dir(&new_dist, &temp_path, &options) {
        error!(from = ?new_dist, to = ?temp_path, error = ?e, "failed to copy built project to new location");
        tx.send(BuildMessage::BuildError(
            "Server failed to finalize built project.".to_string(),
        ))
        .ok();
        return;
    };

    let build_id = id.to_string();
    let final_path = temp_path.join(&build_id);

    // Remove built project after delay.
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(REMOVAL_DELAY)).await;
        if tokio::fs::remove_dir_all(&final_path).await.is_err() {
            error!(path = ?final_path, build_id = ?build_id, "failed to delete built project");
        }
    });

    tx.send(BuildMessage::Finished(id)).ok();
}
