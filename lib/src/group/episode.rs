use super::{GroupMetaData, GroupType, PlayGroup};
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

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> GroupMetaData {
        GroupMetaData::Episode {
            episode_name: self.episode_name.as_str(),
            podcast_name: self.podcast_name.as_str(),
        }
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

    pub fn get_metadata_from_play(play: &Play) -> Option<(&str, &str)> {
        if let (Some(episode_name), Some(podcast_name)) =
            (&play.episode_name, &play.episode_show_name)
        {
            Some((episode_name, podcast_name))
        } else {
            None
        }
    }

    pub fn try_new_from_play(play: &Play) -> Option<Box<dyn PlayGroup>> {
        if let Some((episode_name, podcast_name)) = Episode::get_metadata_from_play(play) {
            Some(Box::new(Episode::new(episode_name, podcast_name)))
        } else {
            None
        }
    }

    pub fn generate_hash(episode_name: &str, podcast_name: &str) -> String {
        format!("{}//{}", episode_name, podcast_name)
    }
}
