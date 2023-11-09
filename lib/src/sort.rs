use crate::columns as col;
use clap::ValueEnum;
use polars::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Descending
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => f.write_str("Ascending"),
            SortOrder::Descending => f.write_str("Descending"),
        }
    }
}

impl SortOrder {
    pub fn is_descending(&self) -> bool {
        match self {
            Self::Ascending => false,
            Self::Descending => true,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortBy {
    Plays,
    Time,
}

impl SortBy {
    fn get_column_name(&self) -> &str {
        match self {
            Self::Plays => col::PLAY_COUNT,
            Self::Time => col::MS_PLAYED,
        }
    }
}

pub fn sort_grouped_data(
    df: DataFrame,
    sort_by: SortBy,
    descending: bool,
) -> PolarsResult<DataFrame> {
    df.sort([sort_by.get_column_name()], descending, false)
}
