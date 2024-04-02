use super::{GroupMetaData, GroupType, PlayGroup};
use crate::aggregate::AggregatedData;
use crate::Play;

#[derive(Debug, Default)]
pub struct Song {
    pub song_name: String,
    pub album_name: String,
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Song {
    fn group_type(&self) -> GroupType {
        GroupType::Song
    }

    fn get_hash(&self) -> String {
        Song::generate_hash(&self.song_name, &self.album_name, &self.artist_name)
    }

    fn add_play(&mut self, play: Play) {
        self.aggregated_data.add_play(play);
    }

    fn get_agg_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_agg_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> GroupMetaData {
        GroupMetaData::Song {
            song_name: self.song_name.as_str(),
            album_name: self.album_name.as_str(),
            artist_name: self.artist_name.as_str(),
        }
    }
}

impl Song {
    pub fn new(song_name: &str, album_name: &str, artist_name: &str) -> Self {
        Self {
            song_name: song_name.to_owned(),
            album_name: album_name.to_owned(),
            artist_name: artist_name.to_owned(),
            ..Default::default()
        }
    }

    pub fn get_metadata_from_play(play: &Play) -> Option<(&str, &str, &str)> {
        if let (Some(song_name), Some(album_name), Some(artist_name)) = (
            &play.master_metadata_track_name,
            &play.master_metadata_album_album_name,
            &play.master_metadata_album_artist_name,
        ) {
            Some((song_name, album_name, artist_name))
        } else {
            None
        }
    }

    pub fn try_new_from_play(play: &Play) -> Option<Box<dyn PlayGroup>> {
        if let Some((song_name, album_name, artist_name)) = Song::get_metadata_from_play(play) {
            Some(Box::new(Song::new(song_name, album_name, artist_name)))
        } else {
            None
        }
    }

    pub fn generate_hash(song_name: &str, album_name: &str, artist_name: &str) -> String {
        format!("{}//{}//{}", song_name, album_name, artist_name)
    }
}
