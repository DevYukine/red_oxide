use std::path::PathBuf;

use async_recursion::async_recursion;
use claxon::FlacReader;
use tokio::fs;

#[async_recursion]
pub async fn copy_other_allowed_files(
    original_dir_path: &PathBuf,
    transcode_dir_path: &PathBuf,
) -> anyhow::Result<()> {
    let mut dir = fs::read_dir(original_dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            copy_other_allowed_files(&path, transcode_dir_path).await?;
        } else {
            let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();

            let allowed_extension = vec!["gif", "jpeg", "jpg", "nfo", "pdf", "png", "sfv", "txt"];

            if allowed_extension.contains(&extension.as_str()) {
                let relativ_path = path.strip_prefix(original_dir_path)?;

                let transcode_dir_path = transcode_dir_path.join(relativ_path);

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

pub(crate) fn get_flac_executable() -> String {
    if cfg!(target_os = "windows") {
        "flac.exe".to_string()
    } else {
        "flac".to_string()
    }
}

pub(crate) fn get_sox_executable() -> String {
    if cfg!(target_os = "windows") {
        "sox.exe".to_string()
    } else {
        "sox".to_string()
    }
}

pub(crate) fn get_lame_executable() -> String {
    if cfg!(target_os = "windows") {
        "lame.exe".to_string()
    } else {
        "lame".to_string()
    }
}
