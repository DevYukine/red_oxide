use std::path::PathBuf;

use tokio::process::Command;

use crate::imdl::util;

pub async fn create_torrent(
    content_path: &PathBuf,
    torrent_path: &PathBuf,
    announce_url: String,
) -> anyhow::Result<()> {
    let mut cmd = Command::new(util::get_executable_name());
    cmd.arg("torrent");
    cmd.arg("create");
    cmd.arg(content_path.to_str().unwrap());
    cmd.arg("-P");
    cmd.arg("-a");
    cmd.arg(announce_url);
    cmd.arg("-s");
    cmd.arg("RED");
    cmd.arg("-o");
    cmd.arg(torrent_path.to_str().unwrap());

    let output = cmd.output().await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to create torrent"))
    }
}
