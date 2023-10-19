use serde::{Deserialize, Serialize};

/// A struct that represents one entry of an end_song.json file. This struct represents a single "play" of
/// a single song/podcast.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Play {
    pub conn_country: String,
    pub episode_name: Option<String>,
    pub episode_show_name: Option<String>,
    pub incognito_mode: Option<bool>,
    pub ip_addr_decrypted: String,
    pub master_metadata_album_album_name: Option<String>,
    pub master_metadata_album_artist_name: Option<String>,
    pub master_metadata_track_name: Option<String>,
    pub ms_played: u64,
    pub offline: Option<bool>,
    pub offline_timestamp: Option<u64>,
    pub platform: String,
    pub reason_end: String,
    pub reason_start: String,
    pub shuffle: Option<bool>,
    pub skipped: Option<bool>,
    pub spotify_episode_uri: Option<String>,
    pub spotify_track_uri: Option<String>,
    pub ts: String,
    pub user_agent_decrypted: Option<String>,
    pub username: String,
}
