use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Album {
    pub album_name: String,
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Album {
    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn key_string(&self) -> String {
        Album::generate_key_string_from_values(&self.album_name, &self.artist_name)
    }
}

impl Album {
    pub fn generate_key_string_from_values(album_name: &str, artist_name: &str) -> String {
        format!("{}//{}", album_name, artist_name)
    }
}
