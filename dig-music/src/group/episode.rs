use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Episode {
    episode_name: String,
    podcast_name: String,
    aggregated_data: AggregatedData,
}

impl PlayGroup for Episode {
    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn key_string(&self) -> String {
        Episode::generate_key_string_from_values(&self.episode_name, &self.podcast_name)
    }
}

impl Episode {
    pub fn generate_key_string_from_values(episode_name: &str, podcast_name: &str) -> String {
        format!("{}//{}", episode_name, podcast_name)
    }
}
