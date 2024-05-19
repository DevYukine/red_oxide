use crate::github::api::GithubApi;
use crate::updater::constants::{GH_REPO, GH_USER};
use crate::updater::release;
use crate::updater::release::ReleaseVersionCompareResult;
use crate::{ERROR, INFO, SUCCESS};
use console::Term;
use futures::StreamExt;
use log::debug;
use std::env::temp_dir;
use tokio::fs::{create_dir_all, File};
use tokio::io;
use tokio::io::BufWriter;

pub async fn self_update(term: &Term, github_api: &mut GithubApi) -> anyhow::Result<()> {
    let latest_version = github_api
        .get_latest_release_version(GH_USER, GH_REPO)
        .await?;

    let current_version = release::get_current_release_version();

    let compared_version_result =
        release::compare_latest_release_to_current_version(&latest_version, &current_version);

    if compared_version_result == ReleaseVersionCompareResult::EqualOrNewer {
        term.write_line(&format!(
            "{} You are already on the latest version {}",
            SUCCESS, latest_version
        ))?;
        return Ok(());
    }

    term.write_line(&format!(
        "{} New version {} available, updating...",
        INFO, latest_version
    ))?;

    let temp_folder_name = temp_dir().join("red_oxide-update");

    create_dir_all(&temp_folder_name).await?;

    debug!("Created temp folder: {:?}", temp_folder_name);

    let filename = match release::get_filename_for_current_target_triple() {
        Ok(file_name) => file_name,
        Err(_) => {
            term.write_line(&format!(
                "{} No prebuild found for your platform, you'll have to build it yourself.",
                ERROR
            ))?;
            return Ok(());
        }
    };

    debug!("Got Github release filename: {}", filename);

    let mut file_byte_stream = github_api
        .get_latest_release_file_by_name(GH_USER, GH_REPO, filename.as_str())
        .await?;

    let temp_file_path = temp_folder_name.join("red_oxide");

    let file = File::create(&temp_file_path).await?;

    let mut buffered_file = BufWriter::new(file);

    while let Some(item) = file_byte_stream.next().await {
        io::copy(&mut item?.as_ref(), &mut buffered_file).await?;
    }

    debug!("Downloaded new release to: {:?}", temp_file_path);

    let current_exe = std::env::current_exe()?;

    let current_exe_renamed = current_exe.clone().parent().unwrap().join("red_oxide_old");

    tokio::fs::rename(&current_exe, &current_exe_renamed).await?;

    debug!("Renamed current executable to: {:?}", current_exe_renamed);

    tokio::fs::rename(&temp_file_path, &current_exe).await?;

    debug!("Renamed temporary downloaded file to {:?}", current_exe);

    tokio::fs::remove_dir(&temp_folder_name).await?;

    debug!("Removed temp folder: {:?}", &temp_folder_name);

    term.write_line(&format!(
        "{} Updated to version {} (be aware that the old executable will be deleted on next use)",
        SUCCESS, latest_version
    ))?;

    Ok(())
}
