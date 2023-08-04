use std::collections::HashSet;
use std::env::temp_dir;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;

use clap::{arg, Parser, Subcommand};
use console::Term;
use dialoguer::{Confirm, Input};
use html_escape::decode_html_entities;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use lazy_static::lazy_static;
use redacted::util::create_description;
use regex::Regex;
use strum::IntoEnumIterator;
use tags::util::valid_tags;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use ReleaseType::{Flac24, Mp3320, Mp3V0};

use crate::config::config::apply_config;
use transcode::transcode::transcode_release;

use crate::fs::util::get_all_files_with_extension;
use crate::redacted::api::client::RedactedApi;
use crate::redacted::api::constants::TRACKER_URL;
use crate::redacted::api::constants::FORBIDDEN_CHARACTERS;
use crate::redacted::api::path::is_path_exceeding_redacted_path_limit;
use crate::redacted::models::ReleaseType::Flac;
use crate::redacted::models::{Category, Media, ReleaseType};
use crate::redacted::upload::TorrentUploadData;
use crate::redacted::util::perma_link;
use crate::transcode::util::copy_other_allowed_files;

mod config;
mod ext_deps;
mod fs;
mod imdl;
mod redacted;
mod spectrogram;
mod tags;
mod transcode;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Transcode FLACs to other co-existing formats
    Transcode(TranscodeCommand),
}

#[derive(Parser, Debug, Clone)]
pub struct TranscodeCommand {
    /// If debug logs should be shown
    #[arg(long, default_value = "false")]
    pub debug: bool,

    /// If the upload should be done automatically
    #[arg(long, short, default_value = "false")]
    pub automatic_upload: bool,

    /// How many tasks (for transcoding as example) should be run in parallel, defaults to your CPU count
    #[arg(long)]
    pub concurrency: Option<usize>,

    /// The Api key from Redacted to use there API with
    #[arg(long)]
    pub api_key: Option<String>,

    /// The path to the directory where the downloaded torrents are stored
    #[arg(long)]
    pub content_directory: Option<PathBuf>,

    /// The path to the directory where the transcoded torrents should be stored
    #[arg(long)]
    pub transcode_directory: Option<PathBuf>,

    /// The path to the directory where the torrents should be stored
    #[arg(long)]
    pub torrent_directory: Option<PathBuf>,

    /// The path to the directory where the spectrograms should be stored
    #[arg(long)]
    pub spectrogram_directory: Option<PathBuf>,

    /// The path to the config file
    #[arg(long, short)]
    pub config_file: Option<PathBuf>,

    /// List of allowed formats to transcode to, defaults to all formats if omitted
    #[arg(long, short = 'f')]
    pub allowed_transcode_formats: Vec<ReleaseType>,

    /// If the existing formats check should be bypassed, useful when you want to transcode a torrent again or trump an already existing one, be aware that this will still take allowed_transcode_formats into account
    #[arg(long, default_value = "false")]
    pub skip_existing_formats_check: bool,

    /// If the transcode should be moved to the content directory, useful when you want to start seeding right after you upload
    #[arg(long, short, default_value = "false")]
    pub move_transcode_to_content: bool,

    /// If the hash check of the original torrent should be skipped, defaults to false, not recommended and if enabled done at own risk!
    #[arg(long, default_value = "false")]
    pub skip_hash_check: bool,

    /// If the spectrogram check of the original torrent should be skipped, defaults to false, not recommended and if enabled done at own risk!
    #[arg(long, default_value = "false")]
    pub skip_spectrogram: bool,

    /// If this is a dry run, no files will be uploaded to Redacted
    #[arg(long, short, default_value = "false")]
    pub dry_run: bool,

    /// The Perma URLs (PL's) of torrents to transcode
    pub urls: Vec<String>,
}

const SUCCESS: &str = "[✅]";
const WARNING: &str = "[⚠️]";
const ERROR: &str = "[❌]";
const PAUSE: &str = "[⏸️]";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Transcode(cmd) => transcode(cmd).await?,
    }

    Ok(())
}

async fn transcode(mut cmd: TranscodeCommand) -> anyhow::Result<()> {
    let term = Term::stdout();

    apply_config(&mut cmd, &term).await?;

    let mut api = RedactedApi::new(cmd.api_key.clone().unwrap());
    let index_response = api.index().await?.response;

    term.write_line(&format!(
        "{} Logged in as {} on the Redacted API",
        SUCCESS, index_response.username
    ))?;

    for url in cmd.urls.clone() {
        let result = handle_url(
            url.as_str(),
            &term,
            &mut api,
            cmd.clone(),
            index_response.passkey.clone(),
        )
        .await;

        if let Err(e) = result {
            term.write_line(&format!(
                "{} Skipping due to encountered error: {}",
                ERROR, e
            ))?;
        }
    }

    Ok(())
}

async fn handle_url(
    url: &str,
    term: &Term,
    api: &mut RedactedApi,
    mut cmd: TranscodeCommand,
    passkey: String,
) -> anyhow::Result<()> {
    lazy_static! {
        static ref RE: Regex = regex::Regex::new(
            r"(https://|http://)?redacted\.ch/torrents\.php\?id=(\d+)&torrentid=(\d+)"
        )
        .unwrap();
    }

    let captures = match RE.captures(url) {
        None => {
            term.write_line(&format!(
                "{} Could not parse permalink {}, please make sure you are using a valid permalink including group id and torrent id",
                ERROR, url
            ))?;
            return Ok(());
        }
        Some(c) => c,
    };

    let group_id = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let torrent_id = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();

    term.write_line(&format!(
        "{} Got torrent {} from group {}",
        SUCCESS, torrent_id, group_id
    ))?;

    let group_info = api.get_torrent_group(group_id).await?;

    let group_torrents = group_info.response.torrents;
    let group = group_info.response.group;

    let torrent_opt = group_torrents
        .iter()
        .find(|torrent| torrent.id == torrent_id);

    let torrent = match torrent_opt {
        None => {
            term.write_line(&format!(
                "{} Could not find torrent {} in group {}, this shouldn't happen...",
                ERROR, torrent_id, group_id
            ))?;
            return Ok(());
        }
        Some(t) => t,
    };

    let mut existing_formats = HashSet::new();

    group_torrents
        .iter()
        .filter(|t| t.remaster_title == torrent.remaster_title
            && t.remaster_record_label == torrent.remaster_record_label
            && t.media == torrent.media
            && t.remaster_catalogue_number == torrent.remaster_catalogue_number)
        .for_each(|t| {
            match t.format.as_str() {
                "FLAC" => match t.encoding.as_str() {
                    "Lossless" => {
                        existing_formats.insert(Flac);
                    }
                    "24bit Lossless" => {
                        existing_formats.insert(Flac24);
                    },
                    _ => {
                        term.write_line(&format!(
                            "{} Unknown encoding {} for torrent {} in group {}, this shouldn't happen...",
                            ERROR, t.encoding, t.id, group_id
                        )).unwrap();
                    }
                },
                "MP3" => {
                    match t.encoding.as_str() {
                        "320" => {
                            existing_formats.insert(Mp3320);
                        }
                        "V0 (VBR)" => {
                            existing_formats.insert(Mp3V0);
                        }
                        _ => {
                            term.write_line(&format!(
                                "{} Unknown encoding {} for torrent {} in group {}, this shouldn't happen...",
                                ERROR, t.encoding, t.id, group_id
                            )).unwrap();
                        }
                    }
                }
                _ => {
                    term.write_line(&format!(
                        "{} Unknown format {} for torrent {} in group {}, this shouldn't happen...",
                        ERROR, t.format, t.id, group_id
                    )).unwrap();
                }
            }
        });

    if !existing_formats.contains(&Flac) && !existing_formats.contains(&Flac24) {
        term.write_line(&format!(
            "{} Torrent {} in group {} has no FLAC base to transcode from... skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    let mut transcode_formats = Vec::new();

    ReleaseType::iter().for_each(|release_type| {
        let format_already_exist = existing_formats.contains(&release_type);
        let release_is_not_flac_24 = release_type != Flac24;
        let release_is_allowed_to_transcode = cmd.allowed_transcode_formats.contains(&release_type);

        let release_is_not_flac_24_and_allowed_to_transcode =
            release_is_not_flac_24 && release_is_allowed_to_transcode;

        if cmd.skip_existing_formats_check {
            if release_is_not_flac_24_and_allowed_to_transcode
                && (release_type != Flac || torrent.format != "FLAC")
            {
                transcode_formats.push(release_type);
            }
        } else {
            if !format_already_exist && release_is_not_flac_24_and_allowed_to_transcode {
                transcode_formats.push(release_type);
            }
        }
    });

    if transcode_formats.is_empty() {
        term.write_line(&format!(
            "{} Torrent {} in group {} has all possible/wanted formats already... skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    term.write_line(&format!(
        "{} Found missing format(s) {} for torrent {} in group {}",
        SUCCESS,
        transcode_formats
            .iter()
            .map(|f| f.to_string())
            .fold(String::new(), |acc, s| acc + &s + ","),
        torrent_id,
        group_id
    ))?;

    let artist = if group.music_info.artists.len() > 1 {
        "Various Artists".to_string()
    } else {
        group.music_info.artists[0].name.clone()
    };

    let mut year = torrent.remaster_year;

    // Fixes edge case where remaster year is 0 (likely unintentional)
    if year == 0 {
        year = group.year;
    }

    let group_name = group.name.replace(":", "");

    let raw_base_name = if torrent.remaster_title.len() > 1 {
        format!(
            "{} - {} ({}) [{}]",
            artist, group_name, torrent.remaster_title, year
        )
    } else {
        format!("{} - {} [{}]", artist, group_name, year)
    };
    let base_name = raw_base_name.replace(&FORBIDDEN_CHARACTERS[..], "_");

    let content_directory = cmd.content_directory.unwrap();

    let flac_path = content_directory.join(decode_html_entities(&torrent.file_path).to_string());

    let media = Media::from(&*torrent.media);

    let (valid, invalid_track_number_vinyl) = valid_tags(&flac_path, &media).await?;

    if !valid && invalid_track_number_vinyl {
        term.write_line(&format!(
            "{} Release is Vinyl and has either no set track number or in a non standard format (e.g. A1, A2 etc), you will be prompted once transcode is done to manually check & adjust the transcode tags as needed!", WARNING
        ))?;

        cmd.automatic_upload = false;
    } else if !valid {
        term.write_line(&format!(
            "{} Torrent {} in group {} has FLAC files with invalid tags, skipping...\n You might be able to trump it.",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    if !cmd.skip_hash_check {
        let downloaded_torrent = api.download_torrent(torrent.id).await?;

        let mut tmp = temp_dir();
        tmp.push(format!("red_oxide-torrent-{}", torrent_id));

        tokio::fs::write(&tmp, downloaded_torrent).await?;

        let result = imdl::hash::verify_torrent_hash(
            flac_path.as_path().to_str().unwrap(),
            tmp.to_str().unwrap(),
        )
        .await?;

        if result {
            term.write_line(&format!(
                "{} Local file torrent hash check succeeded for torrent {} in group {}",
                SUCCESS, torrent_id, group_id
            ))?;
        } else {
            term.write_line(&format!(
                "{} Local file torrent hash check failed for torrent {} in group {}",
                ERROR, torrent_id, group_id
            ))?;
            return Ok(());
        }

        tokio::fs::remove_file(&tmp).await?;
    }

    let spectrogram_directory = cmd.spectrogram_directory.unwrap();
    let flacs = get_all_files_with_extension(&flac_path, ".flac").await?;
    let flacs_count = flacs.len();

    if !cmd.skip_spectrogram {
        let pb = ProgressBar::new(flacs_count as u64);

        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] {msg} {pos:>7}/{len:7} File(s)",
            )?
            .progress_chars("#>-"),
        );

        pb.set_message("Creating Spectrograms... (This may take a while)");

        let parent = flac_path.file_name().unwrap().to_str().unwrap();
        let to_create = spectrogram_directory.join(parent);

        tokio::fs::create_dir_all(&to_create).await?;

        let semaphore = Arc::new(Semaphore::new(cmd.concurrency.unwrap()));
        let mut tasks = vec![];

        for flac in flacs {
            let semaphore = Arc::clone(&semaphore);
            let spectrogram_directory = spectrogram_directory.clone();
            let flac_path = flac_path.clone();
            let flac = flac.clone();
            let pb = pb.clone();
            tasks.push(tokio::spawn(async move {
                let mut join_set = JoinSet::new();

                let semaphore_clone = Arc::clone(&semaphore);
                let spectrogram_directory_clone = spectrogram_directory.clone();
                let flac_path_clone = flac_path.clone();
                let flac_clone = flac.clone();

                join_set.spawn(async move {
                    let _permit = semaphore_clone.acquire().await.unwrap();

                    spectrogram::spectrogram::make_spectrogram_zoom(
                        &flac_path_clone,
                        &flac_clone,
                        &spectrogram_directory_clone,
                    )
                    .await?;

                    Ok::<(), anyhow::Error>(())
                });

                let semaphore_clone = Arc::clone(&semaphore);
                let spectrogram_directory_clone = spectrogram_directory.clone();
                let flac_path_clone = flac_path.clone();
                let flac_clone = flac.clone();
                join_set.spawn(async move {
                    let _permit = semaphore_clone.acquire().await.unwrap();

                    spectrogram::spectrogram::make_spectrogram_full(
                        &flac_path_clone,
                        &flac_clone,
                        &spectrogram_directory_clone,
                    )
                    .await?;

                    Ok::<(), anyhow::Error>(())
                });

                while let Some(result) = join_set.join_next().await {
                    result??;
                }

                pb.inc(1);

                Ok::<(), anyhow::Error>(())
            }));
        }

        for task in tasks {
            task.await??;
        }

        let mut prompt = Confirm::new();

        pb.finish_and_clear();

        term.write_line(&*format!("{} Created Spectrograms at {}, please manual check if FLAC is lossless before continuing!", PAUSE, to_create.to_str().unwrap()))?;

        prompt
            .with_prompt("Do those spectrograms look good?")
            .default(true);

        let response = prompt.interact()?;

        if !response {
            term.write_line(&format!(
                "{} Spectrogram check failed for torrent {} in group {}, skipping",
                ERROR, torrent_id, group_id
            ))?;
            return Ok(());
        }
    }

    if transcode::util::is_multichannel(&flac_path).await? {
        term.write_line(&format!(
            "{} Torrent {} in group {} is a multichannel release which is unsupported, skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::stdout());
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let pb_main = multi_progress.add(ProgressBar::new(
        (flacs_count * transcode_formats.len()) as u64,
    ));
    pb_main.set_style(sty.clone());
    pb_main.set_message("Total");

    pb_main.tick();

    let semaphore = Arc::new(Semaphore::new(cmd.concurrency.unwrap()));
    let mut join_set = JoinSet::new();

    multi_progress.println("[➡️] Transcoding...").unwrap();

    let transcode_directory = cmd.transcode_directory.unwrap();

    for format in &transcode_formats {
        let pb_format =
            multi_progress.insert_before(&pb_main, ProgressBar::new(flacs_count as u64));
        pb_format.set_style(sty.clone());

        let transcode_format_str = match format {
            Flac24 => "FLAC 24bit",
            Flac => "FLAC",
            Mp3320 => "MP3 - 320",
            Mp3V0 => "MP3 - V0",
        };

        let transcode_release_name = format!(
            "{} ({} - {})",
            base_name,
            torrent.media.to_uppercase(),
            transcode_format_str
        );

        let flac_path_clone = flac_path.clone();
        let torrent_id_clone = torrent_id.clone();
        let term = Arc::new(term.clone());
        let mut output_dir = transcode_directory.clone();
        let format = format.clone();
        let pb_main_clone = pb_main.clone();
        let semaphore_clone = semaphore.clone();
        join_set.spawn(tokio::spawn(async move {
            let (folder_path, command) = transcode_release(
                &flac_path_clone,
                &mut output_dir,
                transcode_release_name.clone(),
                format,
                term,
                torrent_id_clone,
                pb_format,
                pb_main_clone,
                semaphore_clone,
            )
            .await?;

            let transcode_folder_path = output_dir.join(&folder_path);

            copy_other_allowed_files(&flac_path_clone, &flac_path_clone, &transcode_folder_path)
                .await?;

            return Ok::<(PathBuf, ReleaseType, String), anyhow::Error>((
                folder_path,
                format,
                command,
            ));
        }));
    }

    let mut path_format_command_triple = Vec::new();

    while let Some(res) = join_set.join_next().await {
        let transcode_folder = res???;

        path_format_command_triple.push(transcode_folder);
    }

    multi_progress.println(format!("{} Transcoding Done!", SUCCESS))?;
    multi_progress.clear()?;

    if invalid_track_number_vinyl {
        let mut prompt = Confirm::new();

        prompt
            .with_prompt(format!("{} Please check tags of trancoded media and adjust as needed (release is vinyl and has either no track number or in an non standard format e.g. A1, A2 etc which the audiotags library used can't parse), continue?", WARNING))
            .default(true);

        prompt.interact()?;
    }

    let torrent_directory = cmd.torrent_directory.unwrap();

    for (path, format, command) in &path_format_command_triple {
        let release_name = path.file_name().unwrap().to_str().unwrap();
        let mut exceeds_red_path_length = is_path_exceeding_redacted_path_limit(&path).await?;

        while exceeds_red_path_length {
            let mut editor = Input::new();

            let edited_text = editor
                .with_prompt(format!(
                    "{} Folder Name {} is too long for RED, please shorten the folder name\n",
                    ERROR, release_name
                ))
                .default(release_name.to_string())
                .interact_text()?;

            let new_path = path.parent().unwrap().join(edited_text);
            exceeds_red_path_length = is_path_exceeding_redacted_path_limit(&new_path).await?;
        }

        let torrent_path = torrent_directory.join(release_name.to_owned() + ".torrent");

        imdl::torrent::create_torrent(
            path,
            &torrent_path,
            format!("{}/{}/announce", TRACKER_URL, passkey),
        )
        .await?;

        term.write_line(&format!(
            "{} Created .torrent files for format {}",
            SUCCESS, format
        ))?;

        let torrent_file_data = tokio::fs::read(&torrent_path).await?;

        let perma_link = perma_link(group_id, torrent_id);
        let description = create_description(perma_link.clone(), command.clone());

        let format_red = match format {
            Flac24 => "FLAC",
            Flac => "FLAC",
            Mp3320 => "MP3",
            Mp3V0 => "MP3",
        };

        let bitrate = match format {
            Flac24 => "24bit Lossless".to_string(),
            Flac => "Lossless".to_string(),
            Mp3320 => "320".to_string(),
            Mp3V0 => "V0 (VBR)".to_string(),
        };

        if cmd.move_transcode_to_content {
            tokio::fs::rename(&path, &content_directory.join(path.file_name().unwrap())).await?;

            term.write_line(&format!(
                "{} Moved transcode release to content directory",
                SUCCESS,
            ))?;
        }

        if !cmd.automatic_upload {
            term.write_line(&*format!(
                "{} Manual mode enabled, skipping automatic upload",
                PAUSE
            ))?;

            let scene = if torrent.scene { "Yes" } else { "No" };
            let format = match format {
                Flac24 => "FLAC",
                Flac => "FLAC",
                Mp3320 => "MP3",
                Mp3V0 => "MP3",
            };

            term.write_line(&*("Link: ".to_owned() + &*perma_link))?;
            term.write_line(&*("Name: ".to_owned() + &*group.name.clone()))?;
            term.write_line(
                &*("Artist(s): ".to_owned()
                    + &group
                        .music_info
                        .artists
                        .iter()
                        .map(|a| a.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")),
            )?;
            term.write_line(&*("Edition Year: ".to_owned() + &*torrent.remaster_year.to_string()))?;
            term.write_line(&*("Edition Title: ".to_owned() + &torrent.remaster_title))?;
            term.write_line(&*("Record Label: ".to_owned() + &torrent.remaster_record_label))?;
            term.write_line(
                &*("Catalogue Number: ".to_owned() + &torrent.remaster_catalogue_number),
            )?;
            term.write_line(&*("Scene: ".to_owned() + scene))?;
            term.write_line(&*("Format: ".to_owned() + format))?;
            term.write_line(&*("Bitrate: ".to_owned() + &bitrate))?;
            term.write_line(&*("Media: ".to_owned() + &torrent.media))?;
            term.write_line("Release Description:")?;
            term.write_line(&description)?;

            let mut prompt = Confirm::new();

            prompt
                .with_prompt("Confirm once you are done uploading...")
                .default(true);

            prompt.interact()?;
        } else if !cmd.dry_run {
            let year = if torrent.remaster_year == 0 {
                group.year
            } else {
                torrent.remaster_year
            };

            let upload_data = TorrentUploadData {
                torrent: torrent_file_data,
                torrent_name: torrent_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                r#type: Category::from(&*group.category_name),
                remaster_year: year,
                remaster_title: torrent.remaster_title.clone(),
                remaster_record_label: torrent.remaster_record_label.clone(),
                remaster_catalogue_number: torrent.remaster_catalogue_number.clone(),
                format: format_red.to_string(),
                bitrate: bitrate.clone(),
                media: torrent.media.clone(),
                release_desc: description.clone(),
                group_id: group.id as u64,
            };

            let res = api.upload_torrent(upload_data).await?;

            term.write_line(&format!("[🔼] Uploaded {} release to REDacted https://redacted.ch/torrents.php?id={}&torrentid={}", format, group_id, res.response.torrent_id))?;
        }
    }

    Ok(())
}
