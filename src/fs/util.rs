use std::path::PathBuf;

use async_recursion::async_recursion;
use tokio::fs;

#[async_recursion]
pub async fn get_all_files_with_extension(
    dir_path: &PathBuf,
    extension: &str,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut dir = fs::read_dir(dir_path).await?;
    let mut files = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut get_all_files_with_extension(&path, extension).await?);
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(extension) {
                files.push(path);
            }
        }
    }

    return Ok(files);
}

#[async_recursion]
pub async fn count_files_with_extension(
    dir_path: &PathBuf,
    extension: &str,
) -> Result<u64, anyhow::Error> {
    let mut dir = fs::read_dir(dir_path).await?;
    let mut count = 0;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            count += count_files_with_extension(&path, extension).await?;
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(extension) {
                count += 1;
            }
        }
    }

    return Ok(count);
}
