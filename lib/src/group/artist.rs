use super::{GroupMetaData, GroupType, PlayGroup};
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
        Artist::generate_hash(&self.artist_name)
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
        GroupMetaData::Artist {
            artist_name: self.artist_name.as_str(),
        }
    }
}

impl Artist {
    pub fn new(artist_name: &str) -> Self {
        Self {
            artist_name: artist_name.to_owned(),
            ..Default::default()
        }
    }

    pub fn get_metadata_from_play(play: &Play) -> Option<&str> {
        if let Some(artist_name) = &play.master_metadata_album_artist_name {
            Some(artist_name)
        } else {
            None
        }
    }

    pub fn try_new_from_play(play: &Play) -> Option<Box<dyn PlayGroup>> {
        if let Some(artist_name) = Artist::get_metadata_from_play(play) {
            Some(Box::new(Artist::new(artist_name)))
        } else {
            None
        }
    }

    pub fn generate_hash(artist_name: &str) -> String {
        format!("{}", artist_name)
    }
}
