use crate::group::{Album, Artist, Episode, GroupType, Podcast, Song};
use crate::{KeyGenerationError, PlayGroup};
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
    pub fn get_key(&self, group_type: &GroupType) -> Result<String, KeyGenerationError> {
        match group_type {
            GroupType::Album => self.generate_key_album(),
            GroupType::Artist => self.generate_key_artist(),
            GroupType::Episode => self.generate_key_episode(),
            GroupType::Podcast => self.generate_key_podcast(),
            GroupType::Song => self.generate_key_song(),
        }
    }

    pub fn new_play_group(&self, group_type: &GroupType) -> Option<Box<dyn PlayGroup>> {
        match group_type {
            GroupType::Album => Album::try_new_from_play(self),
            GroupType::Artist => Artist::try_new_from_play(self),
            GroupType::Episode => Episode::try_new_from_play(self),
            GroupType::Podcast => Podcast::try_new_from_play(self),
            GroupType::Song => Song::try_new_from_play(self),
        }
    }

    fn generate_key_album(&self) -> Result<String, KeyGenerationError> {
        let Some(album_name) = &self.master_metadata_album_album_name else {
            return Err(KeyGenerationError::MissingAlbumName);
        };

        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerationError::MissingArtistName);
        };

        let key = Album::generate_hash(album_name, artist_name);

        Ok(key)
    }

    fn generate_key_artist(&self) -> Result<String, KeyGenerationError> {
        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerationError::MissingArtistName);
        };

        let key = Artist::generate_hash(artist_name);

        Ok(key)
    }

    fn generate_key_episode(&self) -> Result<String, KeyGenerationError> {
        let Some(episode_name) = &self.episode_name else {
            return Err(KeyGenerationError::MissingEpisodeName);
        };

        let Some(podcast_name) = &self.episode_show_name else {
            return Err(KeyGenerationError::MissingPodcastName);
        };

        let key = Episode::generate_hash(episode_name, podcast_name);

        Ok(key)
    }

    fn generate_key_podcast(&self) -> Result<String, KeyGenerationError> {
        let Some(podcast_name) = &self.episode_show_name else {
            return Err(KeyGenerationError::MissingPodcastName);
        };

        let key = Podcast::generate_hash(podcast_name);

        Ok(key)
    }

    fn generate_key_song(&self) -> Result<String, KeyGenerationError> {
        let Some(song_name) = &self.master_metadata_track_name else {
            return Err(KeyGenerationError::MissingSongName);
        };

        let Some(album_name) = &self.master_metadata_album_album_name else {
            return Err(KeyGenerationError::MissingAlbumName);
        };

        let Some(artist_name) = &self.master_metadata_album_artist_name else {
            return Err(KeyGenerationError::MissingArtistName);
        };

        let key = Song::generate_hash(song_name, album_name, artist_name);

        Ok(key)
    }
}
