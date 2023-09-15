use crate::{aggregate::AggregatedData, group, Play};
use std::{collections::HashMap, hash::Hash};
use thiserror::Error;

mod album;
mod artist;
mod episode;
mod podcast;
mod song;

pub use album::Album;
pub use artist::Artist;
pub use episode::Episode;
pub use podcast::Podcast;
pub use song::Song;

use std::fmt::Debug;

use rayon::prelude::*;

pub trait PlayGroup: Debug + Sync + Send {
    fn group_type(&self) -> GroupType;
    fn get_hash(&self) -> String;

    /// Does nothing on with the optional fields where play has None
    fn add_play(&mut self, play: Play);

    fn get_aggregated_data(&mut self) -> &AggregatedData;
    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData;
}

pub enum GroupType {
    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}

pub fn group_plays_together(plays: Vec<Play>, group_type: GroupType) -> Vec<Box<dyn PlayGroup>> {
    let mut grouped_data: HashMap<String, Box<dyn PlayGroup>> = HashMap::new();

    for play in plays.into_iter() {
        let Ok(key) = play.get_key(&group_type) else {
            continue;
        };

        if let Some(play_group) = grouped_data.get_mut(key.as_str()) {
            play_group.add_play(play);
        } else {
            // let pg: Box<dyn PlayGroup> = match group_type {
            //     GroupType::Album => Album::new(album_name, artist_name)
            // }
        }
    }

    dbg!(&grouped_data);

    let grouped_data: Vec<Box<dyn PlayGroup>> =
        grouped_data.into_par_iter().map(|(_, v)| v).collect();

    grouped_data
}

#[derive(Error, Debug)]
pub enum KeyGenerationError {
    #[error("no album name exists for this item.")]
    MissingAlbumName,
    #[error("no artist name exists for this item.")]
    MissingArtistName,
    #[error("no podcast episode name exists for this item.")]
    MissingEpisodeName,
    #[error("no podcast name exists for this item.")]
    MissingPodcastName,
    #[error("no song name exists for this item.")]
    MissingSongName,
}

#[derive(Error, Debug)]
pub enum AddPlayError {
    #[error("Unable to generate key.")]
    KeyGenerationError(#[from] KeyGenerationError),
}
