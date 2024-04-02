use super::{GroupMetaData, GroupType, PlayGroup};
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

    fn get_agg_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_agg_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> GroupMetaData {
        GroupMetaData::Podcast {
            podcast_name: self.podcast_name.as_str(),
        }
    }
}

impl Podcast {
    pub fn new(podcast_name: &str) -> Self {
        Self {
            podcast_name: podcast_name.to_owned(),
            ..Default::default()
        }
    }

    pub fn get_metadata_from_play(play: &Play) -> Option<&str> {
        if let Some(podcast_name) = &play.episode_show_name {
            Some(podcast_name)
        } else {
            None
        }
    }

    pub fn try_new_from_play(play: &Play) -> Option<Box<dyn PlayGroup>> {
        if let Some(podcast_name) = Podcast::get_metadata_from_play(play) {
            Some(Box::new(Podcast::new(podcast_name)))
        } else {
            None
        }
    }

    pub fn generate_hash(podcast_name: &str) -> String {
        format!("{}", podcast_name)
    }
}
