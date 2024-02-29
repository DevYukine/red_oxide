use crate::ext_deps::util::get_sox_executable;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn make_spectrogram_zoom(
    folder_path: &PathBuf,
    file_path: &PathBuf,
    output_dir: &PathBuf,
) -> anyhow::Result<()> {
    let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
    let filename = file_path.file_name().unwrap().to_str().unwrap();

    let filename_new = filename.replace(".flac", ".spectrogram-zoom.png");

    let mut cmd = Command::new(get_sox_executable());
    cmd.arg(file_path.to_str().unwrap());
    cmd.arg("-n");
    cmd.arg("remix");
    cmd.arg("1");
    cmd.arg("spectrogram");
    cmd.arg("-x");
    cmd.arg("500");
    cmd.arg("-y");
    cmd.arg("1025");
    cmd.arg("-z");
    cmd.arg("120");
    cmd.arg("-w");
    cmd.arg("Kaiser");
    cmd.arg("-S");
    cmd.arg("1:00");
    cmd.arg("-d");
    cmd.arg("0:02");
    cmd.arg("-t");
    cmd.arg(&filename);
    cmd.arg("-c");
    cmd.arg("red_oxide");
    cmd.arg("-o");
    cmd.arg(
        output_dir
            .join(folder_name)
            .join(filename_new)
            .to_str()
            .unwrap(),
    );

    let output = cmd.output().await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to create spectrogram"))
    }
}

pub async fn make_spectrogram_full(
    folder_path: &PathBuf,
    file_path: &PathBuf,
    output_dir: &PathBuf,
) -> anyhow::Result<()> {
    let folder_name = folder_path.file_name().unwrap().to_str().unwrap();
    let filename = file_path.file_name().unwrap().to_str().unwrap();

    let filename_new = filename.replace(".flac", ".spectrogram-full.png");

    let mut cmd = Command::new(get_sox_executable());
    cmd.arg(file_path.to_str().unwrap());
    cmd.arg("-n");
    cmd.arg("remix");
    cmd.arg("1");
    cmd.arg("spectrogram");
    cmd.arg("-x");
    cmd.arg("3000");
    cmd.arg("-y");
    cmd.arg("513");
    cmd.arg("-z");
    cmd.arg("120");
    cmd.arg("-w");
    cmd.arg("Kaiser");
    cmd.arg("-t");
    cmd.arg(&filename);
    cmd.arg("-c");
    cmd.arg("red_oxide");
    cmd.arg("-o");
    cmd.arg(
        output_dir
            .join(folder_name)
            .join(filename_new)
            .to_str()
            .unwrap(),
    );

    let output = cmd.output().await?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to create spectrogram"))
    }
}
