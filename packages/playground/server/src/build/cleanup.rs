use std::io;

use crate::app::EnvVars;

/// Check and cleanup any expired built projects or the target dir
pub async fn check_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    check_project_cleanup(env).await?;
    check_target_cleanup(env).await?;
    Ok(())
}

/// Check and cleanup the target dir if it exceeds the max size.
async fn check_target_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    let target_path = env.target_dir();
    if !target_path.exists() {
        return Ok(());
    }

    let target_size = dir_size(&target_path).await?;

    if target_size > env.max_target_dir_size {
        tokio::fs::remove_dir_all(&target_path).await?;
    }

    Ok(())
}

/// Check and cleanup any expired built projects
async fn check_project_cleanup(env: &EnvVars) -> Result<(), io::Error> {
    if !env.built_path.exists() {
        return Ok(());
    }

    let mut dir = tokio::fs::read_dir(&env.built_path).await?;
    let mut dirs_with_size = Vec::new();

    while let Some(item) = dir.next_entry().await? {
        let path = item.path();
        let pathname = path.file_name().unwrap().to_string_lossy();

        // Always cache the examples - don't remove those.
        if example_projects::get_example_projects()
            .iter()
            .any(|p| p.id().to_string() == pathname)
        {
            continue;
        }

        let metadata = item.metadata().await;
        let time_elapsed = metadata
            .and_then(|m| m.created().or_else(|_| m.modified()))
            .and_then(|c| c.elapsed().map_err(io::Error::other));
        let size = dir_size(&path).await;
        if let (Ok(time_elapsed), Ok(size)) = (time_elapsed, size) {
            dirs_with_size.push((item, time_elapsed, size));
        } else {
            tracing::trace!("skipping cleanup of {pathname} due to error reading metadata")
        }
    }

    // Find the total size of the built directory.
    let total_size: u64 = dirs_with_size.iter().map(|(_, _, size)| *size).sum();
    // If it exceeds the max, sort by oldest and remove until under the limit.
    if total_size > env.max_built_dir_size {
        dirs_with_size.sort_by_key(|(_, time_elapsed, _)| *time_elapsed);
        let mut size = total_size;

        for (item, _, dir_size) in dirs_with_size {
            if size <= env.max_built_dir_size {
                break;
            }

            let path = item.path();
            _ = tokio::fs::remove_dir_all(&path).await;
            size -= dir_size;
        }
    }

    Ok(())
}

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
