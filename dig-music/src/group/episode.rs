use super::{GroupType, PlayGroup};
use crate::aggregate::AggregatedData;
use crate::Play;

#[derive(Debug, Default)]
pub struct Episode {
    episode_name: String,
    podcast_name: String,
    aggregated_data: AggregatedData,
}

impl PlayGroup for Episode {
    fn group_type(&self) -> GroupType {
        GroupType::Episode
    }

    fn get_hash(&self) -> String {
        Episode::generate_hash(&self.episode_name, &self.podcast_name)
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

impl Episode {
    pub fn new(episode_name: &str, podcast_name: &str) -> Self {
        Self {
            episode_name: episode_name.to_owned(),
            podcast_name: podcast_name.to_owned(),
            ..Default::default()
        }
    }
    pub fn generate_hash(episode_name: &str, podcast_name: &str) -> String {
        format!("{}//{}", episode_name, podcast_name)
    }
}
