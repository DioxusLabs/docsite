use std::path::PathBuf;

use fs_extra::dir::CopyOptions;
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
    let temp_path = PathBuf::from(crate::TEMP_PATH);

    let options = CopyOptions::new().overwrite(true);

    // TODO: Error handling
    fs_extra::dir::copy(template.clone(), temp_path.clone(), &options).unwrap();

    let template = temp_path.join("template");

    let main_rs = template.join("src/main.rs");
    let cargo_toml = template.join("Cargo.toml");
    let dioxus_toml = template.join("Dioxus.toml");

    // Modify template with new id and code.
    // (Dioxus.toml, Cargo.toml, main.rs)

    // TODO: Error handling
    let main_text = fs::read_to_string(main_rs.clone()).await.unwrap();
    let main_text = main_text.replace(USER_CODE_ID, &code);
    fs::write(main_rs, main_text).await.unwrap();

    // TODO: Error handling
    let cargo_text = fs::read_to_string(cargo_toml.clone()).await.unwrap();
    let cargo_text = cargo_text.replace(BUILD_ID_ID, id.to_string().as_str());
    fs::write(cargo_toml, cargo_text).await.unwrap();

    // TODO: Error handling
    let dioxus_text = fs::read_to_string(dioxus_toml.clone()).await.unwrap();
    let dioxus_text = dioxus_text.replace(BUILD_ID_ID, id.to_string().as_str());
    fs::write(dioxus_toml, dioxus_text).await.unwrap();

    // Run `dx build` in template
    // TODO: Error handling & determining if build failed
    Command::new("dx")
        .arg("serve")
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();

    // Copy `dist` contents to `temp/my-uuid`
    let new_path = temp_path.join(id.to_string());
    let dist = template.join("dist");
    let options = CopyOptions::new().overwrite(true);

    // TODO: Error handling
    fs_extra::dir::copy(dist, new_path, &options).unwrap();

    // Delete `temp/template`
    fs::remove_dir_all(template).await.ok();

    id
}
