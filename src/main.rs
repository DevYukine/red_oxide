use clap::{arg, Parser, Subcommand};
use command::self_update;
use console::Term;
use log::debug;
use std::mem::discriminant;
use std::path::PathBuf;
use updater::release;

use crate::github::api::GithubApi;
use crate::redacted::models::ReleaseType;
use crate::updater::constants::{GH_REPO, GH_USER};
use crate::updater::release::ReleaseVersionCompareResult;

mod command;
mod config;
mod ext_deps;
mod fs;
mod github;
mod imdl;
mod redacted;
mod spectrogram;
mod tags;
mod transcode;
mod updater;
mod util;

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

    /// Update red_oxide to the latest version
    SelfUpdate(SelfUpdateCommand),
}

#[derive(Parser, Debug, Clone)]
pub struct SelfUpdateCommand {
    /// If debug logs should be shown
    #[arg(long, default_value = "false")]
    pub debug: bool,
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
const INFO: &str = "[ℹ️]";
const PAUSE: &str = "[⏸️]";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .parse_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    cleanup_old_executable().await?;

    let cli = Cli::parse();

    let term = Term::stdout();
    let mut github = GithubApi::new()?;

    if discriminant(&cli.command)
        != discriminant(&Commands::SelfUpdate(SelfUpdateCommand { debug: false }))
    {
        check_for_new_version_and_notify(&mut github, &term).await?;
    }

    match cli.command {
        Commands::Transcode(cmd) => command::transcode::transcode(cmd, &term).await?,
        Commands::SelfUpdate(_cmd) => self_update::self_update(&term, &mut github).await?,
    }

    Ok(())
}

async fn cleanup_old_executable() -> anyhow::Result<()> {
    let current_exe = std::env::current_exe()?;
    let current_exe_parent = current_exe.parent().unwrap();

    debug!("Checking if outdated executable exists");

    let outdated_exe = current_exe_parent.join("red_oxide_old");

    let exists = tokio::fs::try_exists(&outdated_exe).await?;

    debug!("Outdated executable exists: {}", exists);

    if exists {
        tokio::fs::remove_file(&outdated_exe).await?;

        debug!("Removed outdated executable: {:?}", outdated_exe);
    }

    Ok(())
}

async fn check_for_new_version_and_notify(
    github_api: &mut GithubApi,
    term: &Term,
) -> anyhow::Result<()> {
    let latest_release = github_api
        .get_latest_release_version(GH_USER, GH_REPO)
        .await?;

    let current_version = release::get_current_release_version();

    let compared_version_result =
        release::compare_latest_release_to_current_version(&latest_release, &current_version);

    match compared_version_result {
        ReleaseVersionCompareResult::OutdatedMajor => {
            term.write_line(&format!(
                "{} Update available: New major version. Use the self-update command. Major updates may change things significantly. See the Github page for details.",
                WARNING
            ))?;
        }
        ReleaseVersionCompareResult::OutdatedMinor => {
            term.write_line(&format!(
				"{} Update available: New minor version. Use the self-update command. Minor updates add new features and improvements.",
				WARNING
			))?;
        }
        ReleaseVersionCompareResult::OutdatedPatch => {
            term.write_line(&format!( "{} Update available: New patch version. Use the self-update command. Patch updates fix bugs and make small improvements.", WARNING))?;
        }
        ReleaseVersionCompareResult::EqualOrNewer => {}
    }

    Ok(())
}
