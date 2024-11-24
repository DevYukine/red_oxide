use crate::redacted::models::Category;
use reqwest::multipart::{Form, Part};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentUploadData {
    pub torrent: Vec<u8>,
    pub torrent_name: String,
    pub r#type: Category,
    pub remaster_year: i64,
    pub remaster_title: String,
    pub remaster_record_label: String,
    pub remaster_catalogue_number: String,
    pub format: String,
    pub bitrate: String,
    pub media: String,
    pub release_desc: String,
    pub group_id: u64,
}

impl Into<Form> for TorrentUploadData {
    fn into(self) -> Form {
        let torrent_part = Part::bytes(self.torrent).file_name(self.torrent_name);

        Form::new()
            .part("file_input", torrent_part)
            .text("type", self.r#type.as_int().to_string())
            .text("remaster_title", self.remaster_title)
            .text("remaster_record_label", self.remaster_record_label)
            .text("remaster_catalogue_number", self.remaster_catalogue_number)
            .text("remaster_year", self.remaster_year.to_string())
            .text("format", self.format)
            .text("bitrate", self.bitrate)
            .text("media", self.media)
            .text("release_desc", self.release_desc)
            .text("groupid", self.group_id.to_string())
    }
}
