use crate::imdl::error::ImdlError;
use crate::imdl::util;
use tokio::process::Command;

pub async fn verify_torrent_hash(
    content_path: &str,
    torrent_path: &str,
) -> Result<bool, ImdlError> {
    let mut cmd = Command::new(util::get_executable_name());
    cmd.arg("torrent");
    cmd.arg("verify");
    cmd.arg(torrent_path);
    cmd.arg("--content");
    cmd.arg(content_path);

    let output = match cmd.output().await {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to run imdl: {}", e);
            return Err(ImdlError::HashError);
        }
    };

    Ok(output.status.success())
}
