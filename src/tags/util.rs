use std::path::PathBuf;

use crate::redacted::models::Media;
use crate::redacted::models::Media::Vinyl;
use async_recursion::async_recursion;
use audiotags::{Tag, TagType};
use tokio::fs;

pub async fn copy_tags_to_mp3(from: &PathBuf, to: &PathBuf) -> anyhow::Result<()> {
    let from_tag = Tag::default().read_from_path(from)?;

    let mut mp3_tags = from_tag.to_dyn_tag(TagType::Id3v2);
    mp3_tags.write_to_path(to.to_str().unwrap())?;

    return Ok(());
}

#[async_recursion]
pub async fn valid_tags(flac_dir_path: &PathBuf, media: &Media) -> anyhow::Result<(bool, bool)> {
    let mut dir = fs::read_dir(flac_dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            let (valid, is_vinyl) = valid_tags(&path, media).await?;

            if !valid {
                return Ok((true, is_vinyl));
            }
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".flac") {
                let (valid, is_vinyl) = validate_tags_of_file(path, media)?;

                if !valid {
                    return Ok((false, is_vinyl));
                }
            }
        }
    }

    return Ok((true, false));
}

pub fn validate_tags_of_file(path: PathBuf, media: &Media) -> anyhow::Result<(bool, bool)> {
    let tag = Tag::new().read_from_path(&path).unwrap();

    if tag.artist().is_none() {
        return Ok((false, false));
    }

    if tag.album().is_none() {
        return Ok((false, false));
    }

    if tag.title().is_none() {
        return Ok((false, false));
    }

    let (track_number, total_tracks) = tag.track();

    if track_number.is_none() {
        return Ok((false, media == &Vinyl));
    }

    return Ok((true, false));
}
