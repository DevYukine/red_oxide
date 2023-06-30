use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;

use claxon::FlacReader;
use console::Term;
use indicatif::ProgressBar;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::Semaphore;

use crate::fs::util::get_all_files_with_extension;
use crate::redacted::models::ReleaseType;
use crate::redacted::models::ReleaseType::{Flac, Mp3320};
use crate::transcode::error::TranscodeError;
use crate::transcode::error::TranscodeError::{Invalid24BitFlac, OutputDirectoryExist};
use ReleaseType::{Flac24, Mp3V0};

use crate::ext_deps::util::{get_flac_executable, get_lame_executable, get_sox_executable};
use crate::transcode::util;
use crate::ERROR;

pub async fn transcode_release(
    flac_dir: &PathBuf,
    output_dir: &mut PathBuf,
    folder_name: String,
    format: ReleaseType,
    term: Arc<Term>,
    torrent_id: i64,
    pb_format: ProgressBar,
    pb_main: ProgressBar,
    semaphore_clone: Arc<Semaphore>,
) -> anyhow::Result<(PathBuf, String)> {
    let needs_resample = util::is_24_bit_flac(flac_dir).await?;

    if format == Flac && !needs_resample {
        term.write_line(&format!(
            "{} some file(s) of torrent {} were incorrectly marked as 24bit.",
            ERROR, torrent_id
        ))?;
        return Err(Invalid24BitFlac.into());
    }

    output_dir.push(folder_name);

    let output_dir_metadata = fs::metadata(&output_dir).await;

    if let Some(output_dir_metadata) = output_dir_metadata.as_ref().ok() {
        if output_dir_metadata.is_dir() {
            return Err(OutputDirectoryExist(output_dir.to_owned()).into());
        }
    }

    fs::create_dir(&output_dir).await?;

    let paths = get_all_files_with_extension(&flac_dir, ".flac").await?;

    pb_format.set_message(format!("{} transcoding", format));

    let mut command = "".to_string();

    let mut handles = vec![];
    for path in paths {
        let pb = pb_format.clone();
        let output_dir = output_dir.clone();
        let pb_main = pb_main.clone();
        let semaphore_clone = semaphore_clone.clone();
        handles.push(tokio::spawn(async move {
            let _permit = semaphore_clone.acquire().await?;
            let (output_path, command) = transcode(&path, &output_dir, format).await?;

            if format != Flac {
                crate::tags::util::copy_tags_to_mp3(&path, &output_path).await?;
            }

            pb.inc(1);
            pb_main.inc(1);

            Ok::<String, anyhow::Error>(command)
        }));
    }

    for handle in handles {
        command = handle.await??;
    }

    pb_format.finish_with_message(format!("{} transcoding done", format));

    Ok((output_dir.to_owned(), command))
}

pub async fn transcode(
    flac_file_path: &PathBuf,
    output_dir: &PathBuf,
    format: ReleaseType,
) -> anyhow::Result<(PathBuf, String)> {
    let flac_file_cloned = flac_file_path.clone();
    let reader = tokio::task::spawn_blocking(move || FlacReader::open(flac_file_cloned)).await??;

    let info = reader.streaminfo();

    let sample_rate = info.sample_rate;
    let bits_per_sample = info.bits_per_sample;
    let resample = sample_rate > 48000 || bits_per_sample > 16;

    let needed_sample_rate = if resample {
        if sample_rate % 44100 == 0 {
            Some(44100)
        } else if sample_rate % 48000 == 0 {
            Some(48000)
        } else {
            return Err(TranscodeError::UnknownSampleRateError(
                flac_file_path.clone(),
                sample_rate,
            )
            .into());
        }
    } else {
        None
    };

    if info.channels > 2 {
        return Err(TranscodeError::TranscodeDownmixError(flac_file_path.clone()).into());
    }

    let file_extension_to_use = match format {
        Mp3V0 => ".mp3",
        Mp3320 => ".mp3",
        Flac => ".flac",
        Flac24 => ".flac",
    };

    let output_file_path = output_dir.join(
        flac_file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".flac", file_extension_to_use),
    );

    let flac_file_path_str = flac_file_path.to_str().unwrap();
    let output_file_path_str = output_file_path.to_str().unwrap();

    let mut flac_decoder_command;
    let mut transcoding_commands_str = Vec::new();

    if resample {
        flac_decoder_command = Command::new(get_sox_executable());
        flac_decoder_command.args(&[
            flac_file_path_str,
            "-G",
            "-b",
            "16",
            "-t",
            "wav",
            "-",
            "rate",
            "-v",
            "-L",
            needed_sample_rate.unwrap().to_string().as_str(),
            "dither",
        ]);
        transcoding_commands_str.push(format!(
            "sox input.flac -G -b 16 -t wav - rate -v -L {} dither",
            needed_sample_rate.unwrap().to_string().as_str()
        ));
    } else {
        flac_decoder_command = Command::new(get_flac_executable());
        flac_decoder_command.args(&["-dcs", "--", flac_file_path_str]);
        transcoding_commands_str.push("flac -dcs -- input.flac".to_string());
    }

    let mut transcoding_steps = Vec::new();
    transcoding_steps.push(flac_decoder_command);

    if format == Mp3V0 {
        let mut cmd = Command::new(get_lame_executable());
        cmd.args(&[
            "-S",
            "-V",
            "0",
            "--vbr-new",
            "--ignore-tag-errors",
            "-",
            output_file_path_str,
        ]);
        transcoding_steps.push(cmd);
        transcoding_commands_str
            .push("lame -S -V 0 --vbr-new --ignore-tag-errors - output.mp3".to_string());
    } else if format == Mp3320 {
        let mut cmd = Command::new(get_lame_executable());
        cmd.args(&[
            "-S",
            "-h",
            "-b",
            "320",
            "--ignore-tag-errors",
            "-",
            output_file_path_str,
        ]);
        transcoding_steps.push(cmd);
        transcoding_commands_str
            .push("lame -S -h -b 320 --ignore-tag-errors - output.mp3".to_string());
    } else if format == Flac {
        let mut cmd = Command::new(get_flac_executable());
        cmd.args(&["--best", "-o", output_file_path_str, "-"]);
        transcoding_steps.push(cmd);
        transcoding_commands_str.push("flac --best -o output.flac -".to_string());
    }

    let mut commands = Vec::new();

    if format == Flac && resample {
        let mut cmd = Command::new(get_sox_executable());
        cmd.args(&[
            flac_file_path_str,
            "-G",
            "-b",
            "16",
            output_file_path_str,
            "rate",
            "-v",
            "-L",
            needed_sample_rate.unwrap().to_string().as_str(),
            "dither",
        ]);
        commands.push(cmd);
        transcoding_commands_str.clear();
        transcoding_commands_str.push(format!(
            "sox input.flac -G -b 16 output.flac rate -v -L {} dither",
            needed_sample_rate.unwrap().to_string().as_str()
        ));
    } else {
        for step in transcoding_steps {
            commands.push(step);
        }
    }

    run_commands(commands).await?;

    Ok((output_file_path, transcoding_commands_str.join(" | ")))
}

async fn run_commands(commands: Vec<Command>) -> anyhow::Result<()> {
    let mut content = vec![];

    for mut command in commands {
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let mut child = command.spawn()?;
        if content.len() > 0 {
            let mut stdin = child.stdin.take().unwrap();
            tokio::spawn(async move {
                stdin.write_all(&content).await.unwrap();
            });
        }
        let output = child.wait_with_output().await?;
        content = output.stdout;
    }

    Ok(())
}
