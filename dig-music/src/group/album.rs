use std::hash;

use super::{AddPlayError, GroupType, PlayGroup};
use crate::aggregate::AggregatedData;
use crate::Play;

#[derive(Debug, Default)]
pub struct Album {
    pub album_name: String,
    pub artist_name: String,
    aggregated_data: AggregatedData,
}

impl PlayGroup for Album {
    fn group_type(&self) -> GroupType {
        GroupType::Album
    }

    fn get_hash(&self) -> String {
        Album::generate(&self.album_name, &self.artist_name)
    }

    fn add_play(&mut self, play: Play) {
        self.aggregated_data.add_play(play)
    }

    fn get_aggregated_data(&mut self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }
}

impl Album {
    pub fn new(album_name: &str, artist_name: &str) -> Self {
        Self {
            album_name: album_name.to_owned(),
            artist_name: artist_name.to_owned(),
            ..Default::default()
        }
    }

    pub fn try_new_from_options(
        album_name: Option<&str>,
        artist_name: Option<&str>,
    ) -> Option<Box<dyn PlayGroup>> {
        if let (Some(album), Some(artist)) = (album_name, artist_name) {
            Some(Box::new(Album::new(&album, &artist)))
        } else {
            None
        }
    }

    pub fn generate(album_name: &str, artist_name: &str) -> String {
        format!("{}//{}", album_name, artist_name)
    }
}
