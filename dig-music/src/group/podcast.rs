use super::{GroupType, PlayGroup};
use crate::aggregate::AggregatedData;
use crate::Play;

#[derive(Debug, Default)]
pub struct Podcast {
    podcast_name: String,
    aggregated_data: AggregatedData,
}

impl PlayGroup for Podcast {
    fn group_type(&self) -> GroupType {
        GroupType::Podcast
    }

    fn get_hash(&self) -> String {
        Podcast::generate_hash(&self.podcast_name)
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

impl Podcast {
    pub fn new(podcast_name: &str) -> Self {
        Self {
            podcast_name: podcast_name.to_owned(),
            ..Default::default()
        }
    }
    pub fn generate_hash(podcast_name: &str) -> String {
        format!("{}", podcast_name)
    }
}
