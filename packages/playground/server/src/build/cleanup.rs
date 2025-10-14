use std::io;

use crate::app::EnvVars;

/// Check and cleanup any expired built projects or the target dir
pub async fn check_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    check_project_cleanup(env).await?;
    check_target_cleanup(env).await?;
    Ok(())
}

/// Check and cleanup the target dir if it exceeds the max size. The hot reloading
/// cache is inside the target dir so it will also be cleared when the incremental
/// artifacts are removed.
async fn check_target_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    let target_path = env.target_dir();
    // If we just cleaned or this is the first run, the target dir may not exist.
    if !target_path.exists() {
        return Ok(());
    }

    let target_size = dir_size(&target_path).await?;

    if target_size > env.max_target_dir_size {
        tokio::fs::remove_dir_all(&target_path).await?;
    }

    Ok(())
}

/// Check and cleanup any expired built projects. This tends to be much smaller
/// than the target dir, but it can grow large over time as patches accumulate.
async fn check_project_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    // If we just cleaned or this is the first run, the built dir may not exist.
    if !env.built_path.exists() {
        return Ok(());
    }

    let mut dir = tokio::fs::read_dir(&env.built_path).await?;
    let mut dirs_with_size = Vec::new();

    // Go through the project directory and find the size and time modified for each project dir.
    while let Some(item) = dir.next_entry().await? {
        let path = item.path();
        let Some(filename) = path.file_name() else {
            continue;
        };
        let filename = filename.to_string_lossy().to_string();

        let metadata = item.metadata().await;
        let time_elapsed = metadata
            .and_then(|m| m.modified())
            .and_then(|c| c.elapsed().map_err(io::Error::other));
        let size = dir_size(&path).await;
        if let (Ok(time_elapsed), Ok(size)) = (time_elapsed, size) {
            dirs_with_size.push((filename, item, time_elapsed, size));
        } else {
            tracing::trace!("skipping cleanup of {filename} due to error reading metadata")
        }
    }

    // Find the total size of the built directory.
    let total_size: u64 = dirs_with_size.iter().map(|(_, _, _, size)| *size).sum();
    // If it exceeds the max, sort by oldest and remove until under the limit.
    if total_size > env.max_built_dir_size {
        // Sort by oldest first
        dirs_with_size.sort_by_key(|(_, _, time_elapsed, _)| *time_elapsed);
        let mut size = total_size;

        // Remove oldest dirs until under the max size.
        for (pathname, item, _, dir_size) in dirs_with_size {
            if size <= env.max_built_dir_size {
                break;
            }

            let path = item.path();
            // Always cache the examples - only remove the patches
            if example_projects::get_example_projects()
                .iter()
                .any(|p| p.id().to_string() == pathname)
            {
                // Find all files in the wasm folder that contain patch and remove them
                let wasm_path = path.join("wasm");
                if wasm_path.exists() {
                    let mut wasm_dir = tokio::fs::read_dir(&wasm_path).await?;
                    while let Some(entry) = wasm_dir.next_entry().await? {
                        let entry_path = entry.path();
                        if let Some(name) = entry_path.file_name()
                            && name.to_string_lossy().contains("patch")
                        {
                            let patch_size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
                            _ = tokio::fs::remove_file(&entry_path).await;
                            size -= patch_size;
                        }
                    }
                }
            } else {
                _ = tokio::fs::remove_dir_all(&path).await;
                size -= dir_size;
            }
        }
    }

    Ok(())
}

/// Recursively calculate the size of a directory.
async fn dir_size(path: &std::path::Path) -> Result<u64, io::Error> {
    let mut size = 0;
    let mut dirs = vec![path.to_path_buf()];

    while let Some(dir) = dirs.pop() {
        let mut entries = tokio::fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await.ok().flatten() {
            let metadata = entry.metadata().await?;

            if metadata.is_dir() {
                dirs.push(entry.path());
            } else {
                size += metadata.len();
            }
        }
    }

    Ok(size)
}
