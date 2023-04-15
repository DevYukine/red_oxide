use crate::hash::error::HashError;
use tokio::process::Command;

pub async fn run_imdl_verification(
    content_path: &str,
    torrent_path: &str,
) -> Result<bool, HashError> {
    let program_name = if cfg!(target_os = "windows") {
        "imdl.exe"
    } else {
        "imdl"
    };

    let mut cmd = Command::new(program_name);
    cmd.arg("torrent");
    cmd.arg("verify");
    cmd.arg(torrent_path);
    cmd.arg("--content");
    cmd.arg(content_path);

    let output = match cmd.output().await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to run imdl: {}", e);
            return Err(HashError::HashError());
        }
    };

    Ok(output.status.success())
}
