use std::collections::HashSet;
use std::env::temp_dir;
use std::path::PathBuf;
use std::string::ToString;
use std::sync::Arc;

use clap::{arg, Parser, Subcommand};
use console::Term;
use dialoguer::Confirm;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use lazy_static::lazy_static;
use redacted::util::create_description;
use regex::Regex;
use strum::IntoEnumIterator;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task::JoinSet;

use transcode::transcode::transcode_release;

use crate::config::models::RedOxideConfig;
use crate::fs::util::count_files_with_extension;
use crate::redacted::api::client::RedactedApi;
use crate::redacted::api::constants::TRACKER_URL;
use crate::redacted::models::{Category, ReleaseType};
use crate::redacted::upload::TorrentUploadData;
use crate::redacted::util::perma_link;
use crate::transcode::util::copy_other_allowed_files;

mod config;
mod fs;
mod imdl;
mod redacted;
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

#[derive(Parser, Debug)]
struct TranscodeCommand {
    /// If debug logs should be shown
    #[arg(long, default_value = "false")]
    debug: bool,

    /// If the upload should be done automatically
    #[arg(long, short, default_value = "false")]
    automatic_upload: bool,

    /// The number of THREADS to use, defaults to the amount of cores available
    #[arg(long, short)]
    threads: Option<u32>,

    /// If multiple formats should be transcoded in parallel
    #[arg(long, default_value = "false")]
    transcode_in_parallel: bool,

    /// The Api key from Redacted to use there API with
    #[arg(long)]
    api_key: Option<String>,

    /// The path to the directory where the downloaded torrents are stored
    #[arg(long)]
    content_directory: PathBuf,

    /// The path to the directory where the transcoded torrents should be stored
    #[arg(long)]
    transcode_directory: PathBuf,

    /// The path to the directory where the torrents should be stored
    #[arg(long)]
    torrent_directory: PathBuf,

    /// The path to the config file
    #[arg(long, short)]
    config_file: Option<PathBuf>,

    /// If the transcode should be moved to the content directory, useful when you want to start seeding right after you upload
    #[arg(long, default_value = "false")]
    move_transcode_to_content: bool,

    /// If the hash check of the original torrent should be skipped, defaults to false, not recommended and if enabled done at own risk!
    #[arg(long, default_value = "false")]
    skip_hash_check: bool,

    /// If this is a dry run, no files will be uploaded to Redacted
    #[arg(long, short, default_value = "false")]
    dry_run: bool,

    /// The url of torrents to transcode
    urls: Vec<String>,
}

const SUCCESS: &str = "[✅]";
const WARNING: &str = "[⚠️]";
const ERROR: &str = "[❌]";

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

    if let Some(config_path) = &cmd.config_file {
        let mut file = File::open(config_path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        let config: RedOxideConfig = serde_json::from_slice(&*contents)?;
        if cmd.api_key.is_none() {
            cmd.api_key = Some(config.api_key);
        }
    }

    if cmd.api_key.is_none() {
        term.write_line(&format!(
            "{} You have to specify API key either as argument or in the config file",
            ERROR
        ))?;

        return Ok(());
    }

    if cmd.threads.is_none() {
        cmd.threads = Some(num_cpus::get() as u32);
    }

    let mut api = RedactedApi::new(cmd.api_key.clone().unwrap());
    let index_response = api.index().await?.response;

    term.write_line(&format!(
        "{} Logged in as {} on the Redacted API",
        SUCCESS, index_response.username
    ))?;

    for url in &cmd.urls {
        handle_url(url, &term, &mut api, &cmd, index_response.passkey.clone()).await?;
    }

    Ok(())
}

async fn handle_url(
    url: &str,
    term: &Term,
    api: &mut RedactedApi,
    cmd: &TranscodeCommand,
    passkey: String,
) -> anyhow::Result<()> {
    lazy_static! {
        static ref RE: Regex = regex::Regex::new(
            r"(https://|http://)?redacted\.ch/torrents\.php\?id=(\d+)&torrentid=(\d+)"
        )
        .unwrap();
    }

    let captures = RE.captures(url).unwrap();

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
                        existing_formats.insert(ReleaseType::Flac);
                    }
                    "24bit Lossless" => {
                        existing_formats.insert(ReleaseType::Flac24);
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
                            existing_formats.insert(ReleaseType::Mp3320);
                        }
                        "V0 (VBR)" => {
                            existing_formats.insert(ReleaseType::Mp3V0);
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

    if !existing_formats.contains(&ReleaseType::Flac)
        && !existing_formats.contains(&ReleaseType::Flac24)
    {
        term.write_line(&format!(
            "{} Torrent {} in group {} has no FLAC base to transcode from... skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    let mut transcode_formats = Vec::new();

    ReleaseType::iter().for_each(|release_type| {
        if !existing_formats.contains(&release_type) && release_type != ReleaseType::Flac24 {
            transcode_formats.push(release_type);
        }
    });

    if transcode_formats.is_empty() {
        term.write_line(&format!(
            "{} Torrent {} in group {} has all formats already... skipping",
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

    let base_name = if torrent.remaster_title.len() > 1 {
        format!(
            "{} - {} ({}) [{}]",
            artist, group_name, torrent.remaster_title, year
        )
    } else {
        format!("{} - {} [{}]", artist, group_name, year)
    };

    let flac_path = cmd.content_directory.join(torrent.file_path.clone());

    if transcode::util::is_multichannel(&flac_path).await? {
        term.write_line(&format!(
            "{} Torrent {} in group {} is a multichannel release which is unsupported, skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    if !tags::util::valid_tags(&flac_path).await? {
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

    if transcode::util::is_multichannel(&flac_path).await? {
        term.write_line(&format!(
            "{} Torrent {} in group {} is a multichannel release which is unsupported, skipping",
            WARNING, torrent_id, group_id
        ))?;
        return Ok(());
    }

    let m = MultiProgress::with_draw_target(ProgressDrawTarget::stdout());
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let mut join_set = JoinSet::new();

    let files = count_files_with_extension(&flac_path, ".flac").await?;

    m.println("[➡️] Transcoding...").unwrap();

    for format in &transcode_formats {
        let pb = m.add(ProgressBar::new(files));
        pb.set_style(sty.clone());

        let transcode_format_str = match format {
            ReleaseType::Flac24 => "FLAC 24bit",
            ReleaseType::Flac => "FLAC",
            ReleaseType::Mp3320 => "MP3 - 320",
            ReleaseType::Mp3V0 => "MP3 - V0",
        };

        let transcode_release_name = format!(
            "{} ({} - {})",
            base_name,
            torrent.media.to_uppercase(),
            transcode_format_str
        );

        let flac_path_clone = flac_path.clone();
        let threads = cmd.threads.clone().unwrap();
        let torrent_id_clone = torrent_id.clone();
        let term = Arc::new(term.clone());
        let mut output_dir = cmd.transcode_directory.clone();
        let format = format.clone();
        join_set.spawn(tokio::spawn(async move {
            let (folder_path, command) = transcode_release(
                &flac_path_clone,
                &mut output_dir,
                transcode_release_name.clone(),
                format,
                threads,
                term,
                torrent_id_clone,
                pb,
            )
            .await?;

            let transcode_folder_path = output_dir.join(&folder_path);

            copy_other_allowed_files(&flac_path_clone, &transcode_folder_path).await?;

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

    m.println(format!("{} Transcoding Done!", SUCCESS))?;
    m.clear()?;

    for (path, format, command) in &path_format_command_triple {
        let release_name = path.file_name().unwrap().to_str().unwrap();

        let torrent_path = cmd
            .torrent_directory
            .join(release_name.to_owned() + ".torrent");

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
            ReleaseType::Flac24 => "FLAC",
            ReleaseType::Flac => "FLAC",
            ReleaseType::Mp3320 => "MP3",
            ReleaseType::Mp3V0 => "MP3",
        };

        let bitrate = match format {
            ReleaseType::Flac24 => "24bit Lossless".to_string(),
            ReleaseType::Flac => "Lossless".to_string(),
            ReleaseType::Mp3320 => "320".to_string(),
            ReleaseType::Mp3V0 => "V0 (VBR)".to_string(),
        };

        if !cmd.dry_run && cmd.automatic_upload {
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

            api.upload_torrent(upload_data).await?;
        }

        if cmd.move_transcode_to_content {
            tokio::fs::rename(
                &path,
                &cmd.content_directory.join(path.file_name().unwrap()),
            )
            .await?;

            term.write_line(&format!(
                "{} Moved transcoded release to content directory",
                SUCCESS,
            ))?;
        }

        if !cmd.automatic_upload {
            term.write_line("[⏸️] Manual mode enabled, skipping automatic upload")?;

            let scene = if torrent.scene { "Yes" } else { "No" };
            let format = match format {
                ReleaseType::Flac24 => "FLAC",
                ReleaseType::Flac => "FLAC",
                ReleaseType::Mp3320 => "MP3",
                ReleaseType::Mp3V0 => "MP3",
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
        }
    }

    Ok(())
}
