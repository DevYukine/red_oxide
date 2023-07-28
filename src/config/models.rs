use crate::redacted::models::ReleaseType;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedOxideConfig {
    pub api_key: Option<String>,
    pub torrent_directory: Option<PathBuf>,
    pub content_directory: Option<PathBuf>,
    pub transcode_directory: Option<PathBuf>,
    pub spectrogram_directory: Option<PathBuf>,
    pub move_transcode_to_content: Option<bool>,
    pub automatic_upload: Option<bool>,
    pub skip_hash_check: Option<bool>,
    pub skip_spectrogram: Option<bool>,
    pub allowed_transcode_formats: Option<Vec<ReleaseType>>,
    pub concurrency: Option<usize>,
}
