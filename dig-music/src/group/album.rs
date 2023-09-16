use super::{GroupMetaData, GroupType, PlayGroup};
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
        Album::generate_hash(&self.album_name, &self.artist_name)
    }

    fn add_play(&mut self, play: Play) {
        self.aggregated_data.add_play(play)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> GroupMetaData {
        GroupMetaData::Album {
            album_name: self.album_name.as_str(),
            artist_name: self.artist_name.as_str(),
        }
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

    pub fn get_metadata_from_play(play: &Play) -> Option<(&str, &str)> {
        if let (Some(album_name), Some(artist_name)) = (
            &play.master_metadata_album_album_name,
            &play.master_metadata_album_artist_name,
        ) {
            Some((album_name, artist_name))
        } else {
            None
        }
    }

    pub fn try_new_from_play(play: &Play) -> Option<Box<dyn PlayGroup>> {
        if let Some((album_name, artist_name)) = Album::get_metadata_from_play(play) {
            Some(Box::new(Album::new(album_name, artist_name)))
        } else {
            None
        }
    }

    pub fn generate_hash(album_name: &str, artist_name: &str) -> String {
        format!("{}//{}", album_name, artist_name)
    }
}
