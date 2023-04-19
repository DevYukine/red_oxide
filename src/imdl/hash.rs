use crate::ext_deps::util::get_imdl_executable_name;
use crate::imdl::error::ImdlError;
use tokio::process::Command;

pub async fn verify_torrent_hash(
    content_path: &str,
    torrent_path: &str,
) -> Result<bool, ImdlError> {
    let mut cmd = Command::new(get_imdl_executable_name());
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
