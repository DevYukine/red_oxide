use crate::config::constants::{
    CONFIG_FILE_NAME, CONFIG_PATH, HOME_ENV, PROJECT_NAME, WINDOWS_APPDATA_ENV,
    WINDOWS_HOMEDRIVE_ENV, WINDOWS_HOMEPATH_ENV, WINDOWS_USERPROFILE_ENV, XDG_CONFIG_ENV,
};
use crate::config::models::RedOxideConfig;
use crate::redacted::models::ReleaseType::{Flac, Mp3320, Mp3V0};
use crate::{TranscodeCommand, ERROR};
use console::Term;
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub fn search_config_in_default_locations() -> anyhow::Result<Option<PathBuf>> {
    let mut path;

    let current_dir = env::current_dir()?;
    let current_dir_config = current_dir.join(CONFIG_FILE_NAME);

    if current_dir_config.exists() {
        path = Some(current_dir_config);
        return Ok(path);
    }

    if cfg!(windows) {
        path = get_config_path_from_default_location_by_env(WINDOWS_APPDATA_ENV);

        if let Some(path) = path {
            return Ok(Some(path));
        }
    }

    path = get_config_path_from_default_location_by_env(XDG_CONFIG_ENV);

    if let Some(path) = path {
        return Ok(Some(path));
    }

    let home_env = get_home_env();

    if let Some(home) = home_env {
        let mut home_config = PathBuf::from(home.clone())
            .join(CONFIG_PATH)
            .join(PROJECT_NAME)
            .join(CONFIG_FILE_NAME);

        if home_config.exists() {
            path = Some(home_config);
            return Ok(path);
        }

        home_config = PathBuf::from(home).join(CONFIG_FILE_NAME);

        if home_config.exists() {
            path = Some(home_config);
            return Ok(path);
        }
    }

    Ok(path)
}

fn get_config_path_from_default_location_by_env(env: &str) -> Option<PathBuf> {
    let env_resolved = env::var(env).unwrap_or(String::new());

    if !env_resolved.is_empty() {
        let env_config_home = PathBuf::from(env_resolved)
            .join(PROJECT_NAME)
            .join(CONFIG_FILE_NAME);

        if env_config_home.exists() {
            return Some(env_config_home);
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn get_home_env() -> Option<String> {
    if let Ok(home) = env::var(HOME_ENV) {
        return Some(home);
    }

    if let Ok(user_profile) = env::var(WINDOWS_USERPROFILE_ENV) {
        return Some(user_profile);
    }

    let home_drive = env::var(WINDOWS_HOMEDRIVE_ENV).unwrap_or(String::new());
    let home_path = env::var(WINDOWS_HOMEPATH_ENV).unwrap_or(String::new());

    if !home_drive.is_empty() && !home_path.is_empty() {
        return Some(format!("{}\\{}", home_drive, home_path));
    }

    None
}

#[cfg(not(target_os = "windows"))]
fn get_home_env() -> Option<String> {
    if let Ok(home) = env::var(HOME_ENV) {
        return Some(home);
    }

    None
}

pub async fn apply_config(cmd: &mut TranscodeCommand, term: &Term) -> anyhow::Result<()> {
    let found_config = match &cmd.config_file {
        None => search_config_in_default_locations()?,
        Some(config_file) => Some(config_file.clone()),
    };

    if let Some(config_path) = found_config {
        let mut file = File::open(config_path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        let config: RedOxideConfig = serde_json::from_slice(&*contents)?;

        if cmd.api_key.is_none() {
            cmd.api_key = config.api_key;
        }

        if cmd.torrent_directory.is_none() {
            cmd.torrent_directory = config.torrent_directory;
        }

        if cmd.content_directory.is_none() {
            cmd.content_directory = config.content_directory;
        }

        if cmd.transcode_directory.is_none() {
            cmd.transcode_directory = config.transcode_directory;
        }

        if cmd.spectrogram_directory.is_none() {
            cmd.spectrogram_directory = config.spectrogram_directory;
        }

        if let Some(move_transcode_to_content) = &config.move_transcode_to_content {
            cmd.move_transcode_to_content = *move_transcode_to_content;
        }

        if let Some(automatic_upload) = &config.automatic_upload {
            cmd.automatic_upload = *automatic_upload;
        }

        if let Some(skip_hash_check) = &config.skip_hash_check {
            cmd.skip_hash_check = *skip_hash_check;
        }

        if let Some(skip_spectrogram) = &config.skip_spectrogram {
            cmd.skip_spectrogram = *skip_spectrogram;
        }

        if let Some(allowed_transcode_formats) = &config.allowed_transcode_formats {
            cmd.allowed_transcode_formats = allowed_transcode_formats.clone();
        }

        if let Some(concurrency) = &config.concurrency {
            cmd.concurrency = Some(*concurrency);
        }
    }

    verify_final_config(cmd, term)?;

    Ok(())
}

pub fn verify_final_config(cmd: &mut TranscodeCommand, term: &Term) -> anyhow::Result<()> {
    if cmd.api_key.is_none() {
        term.write_line(&format!(
            "{} You have to specify API key either as argument or in the config file",
            ERROR
        ))?;
        std::process::exit(1);
    }

    if cmd.torrent_directory.is_none() {
        term.write_line(&format!(
            "{} You have to specify torrent directory either as argument or in the config file",
            ERROR
        ))?;
        std::process::exit(1);
    }

    if cmd.content_directory.is_none() {
        term.write_line(&format!(
            "{} You have to specify content directory either as argument or in the config file",
            ERROR
        ))?;
        std::process::exit(1);
    }

    if cmd.transcode_directory.is_none() {
        term.write_line(&format!(
            "{} You have to specify transcode directory either as argument or in the config file",
            ERROR
        ))?;
        std::process::exit(1);
    }

    if cmd.spectrogram_directory.is_none() {
        term.write_line(&format!(
            "{} You have to specify spectrogram directory either as argument or in the config file",
            ERROR
        ))?;
        std::process::exit(1);
    }

    if cmd.allowed_transcode_formats.is_empty() {
        cmd.allowed_transcode_formats = vec![Flac, Mp3320, Mp3V0];
    }

    if cmd.concurrency.is_none() {
        cmd.concurrency = Some(num_cpus::get());
    }

    Ok(())
}
