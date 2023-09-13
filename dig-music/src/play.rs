use crate::error::KeyGenerateError;
use crate::group::{Album, Artist, Episode, Podcast, Song};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// A struct that represents one entry of an end_song.json file. This struct represents a single "play" of
/// a single song/podcast.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Play {
    pub conn_country: String,
    pub episode_name: Option<String>,
    pub episode_show_name: Option<String>,
    pub incognito_mode: Option<bool>,
    pub ip_addr_decrypted: Ipv4Addr,
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
    pub ts: DateTime<Utc>,
    pub user_agent_decrypted: Option<String>,
    pub username: String,
}

impl Play {
    fn generate_key_album(&self) -> Result<String, KeyGenerateError> {
        let Some(album_name) = &self.master_metadata_album_album_name else {
            return Err(KeyGenerateError::MissingAlbumName);
        };

        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerateError::MissingArtistName);
        };

        let key = Album::generate_key_string_from_values(album_name, artist_name);

        Ok(key)
    }

    fn generate_key_artist(&self) -> Result<String, KeyGenerateError> {
        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerateError::MissingArtistName);
        };

        let key = Artist::generate_key_string_from_values(artist_name);

        Ok(key)
    }

    fn generate_key_episode(&self) -> Result<String, KeyGenerateError> {
        let Some(episode_name) = &self.episode_name else {
            return Err(KeyGenerateError::MissingEpisodeName);
        };

        let Some(podcast_name) = &self.episode_show_name else {
            return Err(KeyGenerateError::MissingPodcastName);
        };

        let key = Episode::generate_key_string_from_values(episode_name, podcast_name);

        Ok(key)
    }

    fn generate_key_podcast(&self) -> Result<String, KeyGenerateError> {
        let Some(podcast_name) = &self.episode_show_name else {
            return Err(KeyGenerateError::MissingPodcastName);
        };

        let key = Podcast::generate_key_string_from_values(podcast_name);

        Ok(key)
    }

    fn generate_key_song(&self) -> Result<String, KeyGenerateError> {
        let Some(song_name) = &self.master_metadata_track_name else {
            return Err(KeyGenerateError::MissingSongName);
        };

        let Some(album_name) = &self.master_metadata_album_album_name else {
            return Err(KeyGenerateError::MissingAlbumName);
        };

        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerateError::MissingArtistName);
        };

        let key = Song::generate_key_string_from_values(song_name, album_name, artist_name);

        Ok(key)
    }
}
