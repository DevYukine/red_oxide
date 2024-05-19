use crate::built_info;
use crate::redacted::api::model::Torrent;
use crate::redacted::models::ReleaseType;
use crate::redacted::models::ReleaseType::{Flac, Flac24, Mp3320, Mp3V0};
use regex::Regex;
use std::collections::HashSet;

pub fn create_description(original_torrent_perma_url: String, transcode_command: String) -> String {
    return format!(
        "Transcode of [url]{}[/url]\n\nTranscode process:\n[code]{}[/code]\nCreated using [url=https://github.com/DevYukine/red_oxide]red_oxide v{} by DevYukine[/url]",
        original_torrent_perma_url, transcode_command, built_info::PKG_VERSION
    );
}

pub fn perma_link(group_id: i64, torrent_id: i64) -> String {
    return format!(
        "https://redacted.ch/torrents.php?id={}&torrentid={}#torrent{}",
        group_id, torrent_id, torrent_id
    );
}

pub fn get_group_id_from_url(url: &str) -> Option<i64> {
    Regex::new(r"^https?://redacted\.ch/torrents\.php\?id=(\d+)&torrentid=(\d+)$")
        .ok()?
        .captures(url)?
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .ok()
}

pub fn get_torrent_id_from_url(url: &str) -> Option<i64> {
    get_torrent_id_from_group_url(url).or_else(|| get_torrent_id_from_torrent_url(url))
}

fn get_torrent_id_from_group_url(url: &str) -> Option<i64> {
    Regex::new(r"^https?://redacted\.ch/torrents\.php\?id=(\d+)&torrentid=(\d+)$")
        .ok()?
        .captures(url)?
        .get(2)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .ok()
}

fn get_torrent_id_from_torrent_url(url: &str) -> Option<i64> {
    Regex::new(r"^https?://redacted\.ch/torrents\.php\?torrentid=(\d+)$")
        .ok()?
        .captures(url)?
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .ok()
}

pub fn get_existing_release_types(
    torrent: &Torrent,
    group_torrents: &Vec<Torrent>,
) -> HashSet<Option<ReleaseType>> {
    group_torrents
        .iter()
        .filter(|t| {
            t.remaster_title == torrent.remaster_title
                && t.remaster_record_label == torrent.remaster_record_label
                && t.media == torrent.media
                && t.remaster_catalogue_number == torrent.remaster_catalogue_number
        })
        .map(|t| get_release_type(t))
        .collect()
}

fn get_release_type(t: &Torrent) -> Option<ReleaseType> {
    match t.format.as_str() {
        "FLAC" => match t.encoding.as_str() {
            "Lossless" => Some(Flac),
            "24bit Lossless" => Some(Flac24),
            _ => None,
        },
        "MP3" => match t.encoding.as_str() {
            "320" => Some(Mp3320),
            "V0 (VBR)" => Some(Mp3V0),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_group_id_from_url() {
        let url = "https://redacted.ch/torrents.php?id=2259978&torrentid=4871992";
        let result = get_group_id_from_url(url).unwrap();
        assert_eq!(result, 2259978);
    }

    #[test]
    fn can_get_torrent_id_from_group_url() {
        let url = "https://redacted.ch/torrents.php?id=2259978&torrentid=4871992";
        let result = get_torrent_id_from_url(url).unwrap();
        assert_eq!(result, 4871992);
    }

    #[test]
    fn can_get_torrent_id_from_torrent_url() {
        let url = "https://redacted.ch/torrents.php?torrentid=4871992";
        let result = get_torrent_id_from_url(url).unwrap();
        assert_eq!(result, 4871992);
    }
}
