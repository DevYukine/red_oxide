use reqwest::multipart::{Form, Part};
use crate::redacted::models::{Category, Format, Release};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentUploadData {
	pub torrent: Vec<u8>,
	pub torrent_name: String,
	pub title: String,
	pub year: i64,
	pub r#type: i64,
	pub releasetype: i64,
	pub artists: Vec<String>,
	pub remaster_year: Option<i64>,
	pub remaster_title: String,
	pub remaster_record_label: String,
	pub remaster_catalogue_number: String,
	pub scene: bool,
	pub format: Format,
	pub bitrate: String,
	pub media: String,
	pub release_desc: String,
	pub groupid: u64,
	pub tags: Vec<String>
}

impl Into<Form> for TorrentUploadData {
	fn into(self) -> Form {
		let torrent_part = Part::bytes(self.torrent).file_name(self.torrent_name);

		let mut form = Form::new()
			.part("file_input", torrent_part)
			.text("type", self.r#type.to_string())
			.text("releasetype", self.releasetype.to_string())
			.text("title", self.title)
			.text("year", self.year.to_string())
			.text("remaster_title", self.remaster_title)
			.text("remaster_record_label", self.remaster_record_label)
			.text("remaster_catalogue_number", self.remaster_catalogue_number)
			.text("scene", self.scene.to_string())
			.text("format", self.format.as_int().to_string())
			.text("bitrate", self.bitrate)
			.text("media", self.media)
			.text("release_desc", self.release_desc)
			.text("groupid", self.groupid.to_string())
			.text("tags", self.tags.join(","));

		if let Some(remaster_year) = self.remaster_year {
			form = form.text("remaster_year", remaster_year.to_string());
		}

		let index = 0;
		for artist in self.artists {
			form = form.text(format!("artists[{}]", index), artist);
			form = form.text(format!("importance[{}]", index), "1");
		}

		return form;
	}
}
