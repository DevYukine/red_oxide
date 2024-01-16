use std::path::PathBuf;

use async_recursion::async_recursion;
use claxon::FlacReader;
use tokio::fs;

#[async_recursion]
pub async fn copy_other_allowed_files(
    dir_path: &PathBuf,
    original_dir_path: &PathBuf,
    transcode_dir_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut dir = fs::read_dir(dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            copy_other_allowed_files(&path, original_dir_path, transcode_dir_path).await?;
        } else {
            let extension = match path.extension() {
                None => "", // fallback if no extension exists
                Some(extension_os) => extension_os.to_str().unwrap().to_lowercase().as_str(),
            };

            let allowed_extension = vec!["gif", "jpeg", "jpg", "pdf", "png", "txt"];

            if allowed_extension.contains(&extension) {
                let relative_path = path.strip_prefix(original_dir_path)?;

                let transcode_dir_path = transcode_dir_path.join(relative_path);

                fs::create_dir_all(transcode_dir_path.parent().unwrap()).await?;

                fs::copy(path, transcode_dir_path).await?;
            }
        }
    }

    Ok(())
}

#[async_recursion]
pub async fn is_24_bit_flac(flac_dir_path: &PathBuf) -> anyhow::Result<bool> {
    let mut dir = fs::read_dir(flac_dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            if is_24_bit_flac(&path).await? {
                return Ok(true);
            }
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".flac") {
                let reader = tokio::task::spawn_blocking(move || FlacReader::open(&path)).await??;

                if reader.streaminfo().bits_per_sample > 16 {
                    return Ok(true);
                }
            }
        }
    }

    return Ok(false);
}

#[async_recursion]
pub async fn is_multichannel(flac_dir_path: &PathBuf) -> anyhow::Result<bool> {
    let mut dir = fs::read_dir(flac_dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            if is_multichannel(&path).await? {
                return Ok(true);
            }
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".flac") {
                let reader = tokio::task::spawn_blocking(move || FlacReader::open(&path)).await??;

                if reader.streaminfo().channels > 2 {
                    return Ok(true);
                }
            }
        }
    }

    return Ok(false);
}
