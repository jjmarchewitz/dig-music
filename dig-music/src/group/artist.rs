use super::{GroupType, PlayGroup};
use crate::aggregate::AggregatedData;
use crate::Play;

#[derive(Debug, Default)]
pub struct Artist {
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Artist {
    fn group_type(&self) -> GroupType {
        GroupType::Artist
    }

    fn get_hash(&self) -> String {
        Artist::generate(&self.artist_name)
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

impl Artist {
    pub fn new(artist_name: &str) -> Self {
        Self {
            artist_name: artist_name.to_owned(),
            ..Default::default()
        }
    }
    pub fn generate(artist_name: &str) -> String {
        format!("{}", artist_name)
    }
}
