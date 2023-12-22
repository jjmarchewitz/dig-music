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
        // Some of these are missing aggregations on columns that represent whatever the "group"
        // is. For example, Album has no `col("album_name")`. This is because that column is already
        // created as part of the `df.group_by()` call. The GroupTypes that need to have these extra
        // columns (i.e. Episode and Song) need them because they group on the entry's URI, not on
        // the name.
        match self {
            Self::Album => vec![
                col("ms_played").sum(),
                all().exclude(vec!["ms_played", "album_name"]),
            ],
            Self::Artist => vec![
                col("ms_played").sum(),
                col("album_name").first(),
                all().exclude(vec!["ms_played", "album_name", "artist_name"]),
            ],
            Self::Episode => vec![
                col(col::MS_PLAYED).sum(),
                col(col::PODCAST_NAME).first(),
                col(col::EPISODE_NAME).first(),
                all().exclude(vec![col::MS_PLAYED, col::PODCAST_NAME, col::EPISODE_NAME]),
            ],
            Self::Podcast => vec![
                col("ms_played").sum(),
                all().exclude(vec!["ms_played", "podcast_name"]),
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
    df.lazy()
        .group_by([group_by.get_column_name()])
        .agg(group_by.get_aggs())
        .with_column(
            col(col::TIMESTAMP).list().len().alias(col::PLAY_COUNT), // .cast(u64),
        )
        .collect()
}
