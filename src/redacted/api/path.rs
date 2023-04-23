use crate::fs::util::get_all_files_with_extension;
use std::path::PathBuf;

const REDACTED_MAX_LENGTH: usize = 180;

pub async fn is_path_exceeding_redacted_path_limit(folder_path: &PathBuf) -> anyhow::Result<bool> {
    let folder_name = folder_path.file_name().unwrap().to_str().unwrap();

    let dir = get_all_files_with_extension(folder_path, ".flac").await?;

    for flac_path in dir {
        let flac_name = flac_path.file_name().unwrap().to_str().unwrap();

        let full_path = format!("{}/{}", folder_name, flac_name);

        if full_path.len() > REDACTED_MAX_LENGTH {
            return Ok(true);
        }
    }

    Ok(false)
}
