use crate::columns as col;
use polars::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum GroupType {
    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}

impl GroupType {
    fn get_column_name(&self) -> &str {
        match self {
            Self::Album => col::ALBUM_NAME,
            Self::Artist => col::ARTIST_NAME,
            Self::Episode => col::SPOTIFY_EPISODE_URI,
            Self::Podcast => col::PODCAST_NAME,
            Self::Song => col::SPOTIFY_TRACK_URI,
        }
    }

    fn get_aggs(&self) -> Vec<Expr> {
        match self {
            Self::Album => vec![
                col(col::MS_PLAYED).sum(),
                col(col::ALBUM_NAME).first(),
                all().exclude(vec![col::MS_PLAYED, col::ALBUM_NAME]),
            ],
            Self::Artist => vec![
                col(col::MS_PLAYED).sum(),
                col(col::ALBUM_NAME).first(),
                col(col::ARTIST_NAME).first(),
                all().exclude(vec![col::MS_PLAYED, col::ALBUM_NAME, col::ARTIST_NAME]),
            ],
            Self::Episode => vec![
                col(col::MS_PLAYED).sum(),
                col(col::PODCAST_NAME).first(),
                col(col::EPISODE_NAME).first(),
                all().exclude(vec![col::MS_PLAYED, col::PODCAST_NAME, col::EPISODE_NAME]),
            ],
            Self::Podcast => vec![
                col(col::MS_PLAYED).sum(),
                col(col::PODCAST_NAME).first(),
                all().exclude(vec![col::MS_PLAYED, col::PODCAST_NAME]),
            ],
            Self::Song => vec![
                col(col::MS_PLAYED).sum(),
                col(col::ALBUM_NAME).first(),
                col(col::ARTIST_NAME).first(),
                col(col::TRACK_NAME).first(),
                all().exclude(vec![
                    col::MS_PLAYED,
                    col::ALBUM_NAME,
                    col::ARTIST_NAME,
                    col::TRACK_NAME,
                ]),
            ],
        }
    }
}

pub fn group_plays(df: DataFrame, group_by: GroupType) -> PolarsResult<DataFrame> {
    dbg!(df.get_column_names());

    let aggs = group_by.get_aggs();

    dbg!(aggs);

    df.lazy()
        .group_by([group_by.get_column_name()])
        .agg(group_by.get_aggs())
        .with_column(col(col::TIMESTAMP).list().len().alias(col::PLAY_COUNT))
        .collect()
}
