use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaSearchType {
    #[serde(rename = "CD")]
    CD,
    #[serde(rename = "DVD")]
    DVD,
    #[serde(rename = "Vinyl")]
    Vinyl,
    #[serde(rename = "Soundboard")]
    Soundboard,
    #[serde(rename = "SACD")]
    SACD,
    #[serde(rename = "DAT")]
    DAT,
    #[serde(rename = "WEB")]
    WEB,
    #[serde(rename = "Blu-ray")]
    BLURAY,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseReceived<T> {
    pub status: String,
    pub response: Option<T>,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub status: String,
    pub response: T,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
    pub username: String,
    pub id: i64,
    pub authkey: String,
    pub passkey: String,
    #[serde(rename = "api_version")]
    pub api_version: String,
    pub notifications: Notifications,
    pub userstats: Userstats,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponse {
    pub username: String,
    pub avatar: String,
    pub is_friend: bool,
    pub profile_text: String,
    pub bb_profile_text: String,
    pub profile_album: ProfileAlbum,
    pub stats: Stats,
    pub ranks: Ranks,
    pub personal: Personal,
    pub community: Community,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistResponse {
    pub id: i64,
    pub name: String,
    pub notifications_enabled: bool,
    pub has_bookmarked: bool,
    pub image: String,
    pub body: String,
    pub vanity_house: bool,
    pub tags: Vec<Tag>,
    pub similar_artists: Vec<SimilarArtist>,
    pub statistics: Statistics,
    pub torrentgroup: Vec<Group>,
    pub requests: Vec<Request>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentResponse {
    pub group: Group,
    pub torrent: Torrent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TorrentGroupResponse {
    pub group: Group,
    pub torrents: Vec<Torrent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notifications {
    pub messages: i64,
    pub notifications: i64,
    pub new_announcement: bool,
    pub new_blog: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Userstats {
    pub uploaded: i64,
    pub downloaded: i64,
    pub ratio: f64,
    pub requiredratio: f64,
    pub class: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAlbum {
    pub id: String,
    pub name: String,
    pub review: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub joined_date: String,
    pub last_access: String,
    pub uploaded: i64,
    pub downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranks {
    pub uploaded: i64,
    pub downloaded: i64,
    pub uploads: i64,
    pub requests: i64,
    pub bounty: i64,
    pub posts: i64,
    pub artists: i64,
    pub overall: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personal {
    pub class: String,
    pub paranoia: i64,
    pub paranoia_text: String,
    pub donor: bool,
    pub warned: bool,
    pub enabled: bool,
    pub passkey: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub posts: i64,
    pub torrent_comments: i64,
    pub collages_started: i64,
    pub collages_contrib: i64,
    pub requests_filled: i64,
    pub requests_voted: i64,
    pub perfect_flacs: i64,
    pub uploaded: i64,
    pub groups: i64,
    pub seeding: i64,
    pub leeching: i64,
    pub snatched: i64,
    pub invited: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarArtist {
    pub artist_id: i64,
    pub name: String,
    pub score: i64,
    pub similar_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub num_groups: i64,
    pub num_torrents: i64,
    pub num_seeders: i64,
    pub num_leechers: i64,
    pub num_snatches: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    pub id: i64,
    pub media: String,
    pub format: String,
    pub encoding: String,
    pub remastered: bool,
    pub remaster_year: Option<i64>,
    pub remaster_title: String,
    pub remaster_record_label: String,
    pub remaster_catalogue_number: String,
    pub scene: bool,
    pub has_log: bool,
    pub has_cue: bool,
    pub log_score: i64,
    pub file_count: i64,
    pub size: i64,
    pub seeders: i64,
    pub leechers: i64,
    pub snatched: i64,
    #[serde(rename = "has_snatched")]
    pub has_snatched: bool,
    pub trumpable: bool,
    pub lossy_web_approved: bool,
    pub lossy_master_approved: bool,
    pub free_torrent: bool,
    pub is_neutralleech: bool,
    pub is_freeload: bool,
    pub reported: bool,
    pub time: String,
    pub description: String,
    pub file_list: String,
    pub file_path: String,
    pub user_id: i64,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub request_id: i64,
    pub category_id: i64,
    pub title: String,
    pub year: i64,
    pub time_added: String,
    pub votes: i64,
    pub bounty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub wiki_body: String,
    pub bb_body: String,
    pub wiki_image: String,
    pub id: i64,
    pub name: String,
    pub year: i64,
    pub record_label: String,
    pub catalogue_number: String,
    pub release_type: i64,
    pub category_id: i64,
    pub category_name: String,
    pub time: String,
    pub vanity_house: bool,
    pub is_bookmarked: bool,
    pub tags: Vec<String>,
    pub music_info: MusicInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicInfo {
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
}
