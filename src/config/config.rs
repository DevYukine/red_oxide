use crate::config::models::RedOxideConfig;
use crate::redacted::models::ReleaseType::{Flac, Mp3320, Mp3V0};
use crate::{TranscodeCommand, ERROR};
use console::Term;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn apply_config(cmd: &mut TranscodeCommand, term: &Term) -> anyhow::Result<()> {
    if let Some(config_path) = &cmd.config_file {
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
