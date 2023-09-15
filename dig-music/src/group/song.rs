use super::{GroupType, PlayGroup};
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

    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
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
    pub fn generate_hash(song_name: &str, album_name: &str, artist_name: &str) -> String {
        format!("{}//{}//{}", song_name, album_name, artist_name)
    }
}
