use crate::REMOVAL_DELAY;
use dioxus_logger::tracing::warn;
use fs_extra::{dir, file};
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc::{self, UnboundedSender},
};
use uuid::Uuid;

const USER_CODE_ID: &str = "{USER_CODE}";
const BUILD_ID_ID: &str = "{BUILD_ID}";

pub type QueueType = (UnboundedSender<BuildMessage>, String);

pub enum BuildMessage {
    Message(String),
    Finished(Uuid),
    FinishedWithError,
}

pub async fn start_build_watcher() -> UnboundedSender<QueueType> {
    let (tx, mut rx) = mpsc::unbounded_channel::<QueueType>();

    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            build(item.0, item.1).await;
        }
    });

    tx
}

async fn build(tx: UnboundedSender<BuildMessage>, code: String) {
    let id = Uuid::new_v4();
    let template = PathBuf::from(crate::BUILD_TEMPLATE_PATH);
    let dist = template.join("dist");

    // Delete template/dist if it exists (clean slate)
    fs::remove_dir_all(dist.clone()).await.ok();

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

    // TODO: Error handling
    // Enumerates over a list of paths to copy and copies them, replacing the existing ones.
    // Allows us to reset the template for the new build.
    for (i, path) in snippets_from_copy.iter().enumerate() {
        let new_path = snippets_to_copy[i].clone();
        fs_extra::file::copy(path, new_path, &options).unwrap();
    }

    // TODO: Error handling
    // Modify template with new id and code.
    // (Dioxus.toml, Cargo.toml, main.rs)
    for path in snippets_to_copy {
        let text = fs::read_to_string(path.clone()).await.unwrap();
        let text = text
            .replace(USER_CODE_ID, &code)
            .replace(BUILD_ID_ID, id.to_string().as_str());
        fs::write(path, text).await.unwrap();
    }

    // Run `dx build` in template
    let mut child = Command::new(
        "C:\\Users\\Darka\\Documents\\Projects\\DioxusLabs\\dioxus\\target\\release\\dx",
    )
    .arg("build")
    .arg("--platform")
    .arg("web")
    .arg("--release")
    .arg("--raw-out")
    .current_dir(template.clone())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();

    let stdout = child.stdout.take().unwrap();
    let mut stdout_reader = BufReader::new(stdout).lines();

    let stderr = child.stderr.take().unwrap();
    let mut stderr_reader = BufReader::new(stderr).lines();

    // Get output and wait for process to finish.
    loop {
        tokio::select! {
            result = stdout_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        tx.send(BuildMessage::Message(line)).ok();
                    },
                    Err(_) => return,
                    _ => {},
                }
            }
            result = stderr_reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        tx.send(BuildMessage::Message(line)).ok();
                    },
                    Err(_) => return,
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
    fs::rename(dist, new_dist.clone()).await.unwrap();

    let options = dir::CopyOptions::new().overwrite(true);

    // TODO: Error handling
    fs_extra::dir::move_dir(new_dist, temp_path.clone(), &options).unwrap();

    let build_id = id.to_string();
    let final_path = temp_path.join(build_id.clone());

    // Remove built project after delay.
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(REMOVAL_DELAY)).await;
        if tokio::fs::remove_dir_all(final_path.clone()).await.is_err() {
            warn!(path = ?final_path, build_id = ?build_id, "failed to delete built project");
        }
    });

    tx.send(BuildMessage::Finished(id)).ok();
}

