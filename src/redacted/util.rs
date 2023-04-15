fn create_description(original_torrent_perma_url: String, transcode_command: String) -> String {
    return format!(
        "Transcode of [url={}]{}[/url]\n\nTranscode process:\n[code]{}[/code]",
        original_torrent_perma_url, original_torrent_perma_url, transcode_command
    );
}
