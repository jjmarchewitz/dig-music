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
            Self::Album => "album_name",
            Self::Artist => "artist_name",
            Self::Episode => "spotify_episode_uri",
            Self::Podcast => "podcast_name",
            Self::Song => "spotify_track_uri",
        }
    }

    fn get_aggs(&self) -> Vec<Expr> {
        // Some of these are missing aggregations on columns that represent whatever the "group"
        // is. For example, Album has no `col("album_name")`. This is because that column is already
        // created as part of the `df.group_by()` call. The GroupTypes that need to have these extra
        // columns (i.e. Episode and Song) need them because they group on the entry's URI, not on
        // the name. We need extra logic to capture the rest of the columns we care about (album name,
        // artist name, track name, etc.)
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
                col("ms_played").sum(),
                col("podcast_name").first(),
                col("episode_name").first(),
                all().exclude(vec!["ms_played", "podcast_name", "episode_name"]),
            ],
            Self::Podcast => vec![
                col("ms_played").sum(),
                all().exclude(vec!["ms_played", "podcast_name"]),
            ],
            Self::Song => vec![
                col("ms_played").sum(),
                col("album_name").first(),
                col("artist_name").first(),
                col("track_name").first(),
                all().exclude(vec!["ms_played", "album_name", "artist_name", "track_name"]),
            ],
        }
    }
}

pub fn group_plays(df: DataFrame, group_by: GroupType) -> PolarsResult<DataFrame> {
    df.lazy()
        .group_by([group_by.get_column_name()])
        .agg(group_by.get_aggs())
        .with_columns(vec![col("timestamp").list().len().alias("play_count")])
        .collect()
}
