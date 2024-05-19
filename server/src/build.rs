use fs_extra::{dir, file};
use std::path::PathBuf;
use tokio::{
    fs,
    process::Command,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

const USER_CODE_ID: &str = "{USER_CODE}";
const BUILD_ID_ID: &str = "{BUILD_ID}";

pub type QueueType = (oneshot::Sender<Uuid>, String);

pub async fn start_build_watcher() -> mpsc::UnboundedSender<QueueType> {
    let (tx, mut rx) = mpsc::unbounded_channel::<QueueType>();

    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            let id = build(item.1).await;
            item.0.send(id).ok();
        }
    });

    tx
}

async fn build(code: String) -> Uuid {
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
    // TODO: Error handling & determining if build failed
    Command::new("dx")
        .arg("build")
        .arg("--platform")
        .arg("web")
        .arg("--release")
        .current_dir(template.clone())
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();

    // Copy `dist` contents to `temp/my-uuid`
    let temp_path = PathBuf::from(crate::TEMP_PATH);
    let new_dist = template.join(id.to_string());
    fs::rename(dist, new_dist.clone()).await.unwrap();

    let options = dir::CopyOptions::new().overwrite(true);

    // TODO: Error handling
    fs_extra::dir::move_dir(new_dist, temp_path, &options).unwrap();

    id
}
