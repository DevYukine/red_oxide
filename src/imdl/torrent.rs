use std::path::PathBuf;

use crate::ext_deps::util::get_imdl_executable_name;
use tokio::process::Command;

pub async fn create_torrent(
    content_path: &PathBuf,
    torrent_path: &PathBuf,
    announce_url: String,
) -> anyhow::Result<()> {
    let mut cmd = Command::new(get_imdl_executable_name());
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
        let output_info = match String::from_utf8(output.stderr) {
            Ok(string) => string,
            Err(..) => "<failed to load command output>".to_string()
        };
        Err(anyhow::anyhow!(format!("Failed to create torrent: {}", output_info)))
    }
}
