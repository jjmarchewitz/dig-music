use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Song {
    pub song_name: String,
    pub album_name: String,
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Song {
    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn key_string(&self) -> String {
        Song::generate_key_string_from_values(&self.song_name, &self.album_name, &self.artist_name)
    }
}

impl Song {
    pub fn generate_key_string_from_values(
        song_name: &str,
        album_name: &str,
        artist_name: &str,
    ) -> String {
        format!("{}//{}//{}", song_name, album_name, artist_name)
    }
}
