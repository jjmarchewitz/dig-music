use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Artist {
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Artist {
    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn key_string(&self) -> String {
        Artist::generate_key_string_from_values(&self.artist_name)
    }
}

impl Artist {
    pub fn generate_key_string_from_values(artist_name: &str) -> String {
        format!("{}", artist_name)
    }
}
