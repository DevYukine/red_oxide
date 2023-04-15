pub fn create_description(original_torrent_perma_url: String, transcode_command: String) -> String {
    return format!(
        "Transcode of [url={}]{}[/url]\n\nTranscode process:\n[code]{}[/code]\nCreated using [url=https://github.com/DevYukine/red_oxide]red_oxide by DevYukine[/url]",
        original_torrent_perma_url, original_torrent_perma_url, transcode_command
    );
}

pub fn perma_link(group_id: i64, torrent_id: i64) -> String {
    return format!("https://redacted.ch/torrents.php?id={}&torrentid={}#torrent{}", group_id, torrent_id, torrent_id);
}
