use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Podcast {
    podcast_name: String,
    aggregated_data: AggregatedData,
}

impl PlayGroup for Podcast {
    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn key_string(&self) -> String {
        Podcast::generate_key_string_from_values(&self.podcast_name)
    }
}

impl Podcast {
    pub fn generate_key_string_from_values(podcast_name: &str) -> String {
        format!("{}", podcast_name)
    }
}
