use crate::{aggregate::AggregatedData, Play};
use std::{collections::HashMap, fmt::Display};
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

    fn get_aggregated_data(&self) -> &AggregatedData;
    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData;

    fn get_metadata(&self) -> GroupMetaData;
}

#[derive(Debug, Clone, Copy)]
pub enum GroupType {
    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}

pub enum GroupMetaData<'a> {
    Album {
        album_name: &'a str,
        artist_name: &'a str,
    },
    Artist {
        artist_name: &'a str,
    },
    Episode {
        episode_name: &'a str,
        podcast_name: &'a str,
    },
    Podcast {
        podcast_name: &'a str,
    },
    Song {
        song_name: &'a str,
        album_name: &'a str,
        artist_name: &'a str,
    },
}

impl<'a> Display for GroupMetaData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupMetaData::Album {
                album_name,
                artist_name,
            } => write!(f, "\"{}\" by {}", album_name, artist_name),
            GroupMetaData::Artist { artist_name } => write!(f, "{}", artist_name),
            GroupMetaData::Episode {
                episode_name,
                podcast_name,
            } => write!(f, "\"{}\" by {}", episode_name, podcast_name),
            GroupMetaData::Podcast { podcast_name } => write!(f, "{}", podcast_name),
            GroupMetaData::Song {
                song_name,
                album_name,
                artist_name,
            } => write!(
                f,
                "\"{}\" on the album \"{}\" by {}",
                song_name, album_name, artist_name
            ),
        }
    }
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
            let Some(mut pg) = play.new_play_group(&group_type) else {
                continue;
            };

            pg.add_play(play);

            grouped_data.insert(key, pg);
        }
    }

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
