#![allow(unused)]

mod operand_trait;
mod parse;
pub use operand_trait::FilterOperand;
pub use parse::parse_filters;

use chrono::prelude::*;
use polars::prelude::*;

use crate::error::FilterParsingError;

#[derive(Debug)]
pub struct Filter {
    filter_by: FilterBy,
    filter_type: FilterType,
}

impl Filter {
    fn new(filter_by: FilterBy, filter_type: FilterType) -> Option<Self> {
        // TODO: check if FilterBy and FilterType are compatible
        Some(Self {
            filter_by,
            filter_type,
        })
    }

    fn filter_when(&self) -> FilterWhen {
        self.filter_by.filter_when()
    }
}

// TODO: Add Song, Artist, Album, Podcast, Podcast Episode
#[derive(Debug)]
pub enum FilterBy {
    Date,
    DateTime,
    ListenTime,
    PlayCount,
    Time,
}

const FILTER_BY_DATE_STR: &'static str = "date";
const FILTER_BY_DATETIME_STR: &'static str = "datetime";
const FILTER_BY_LISTEN_TIME_STR: &'static str = "listen_time";
const FILTER_BY_PLAY_COUNT_STR: &'static str = "play_count";
const FILTER_BY_TIME_STR: &'static str = "time";

impl FilterBy {
    fn filter_when(&self) -> FilterWhen {
        match self {
            FilterBy::Date => FilterWhen::Plays,
            FilterBy::DateTime => FilterWhen::Plays,
            FilterBy::ListenTime => FilterWhen::GroupedData,
            FilterBy::PlayCount => FilterWhen::GroupedData,
            FilterBy::Time => FilterWhen::Plays,
        }
    }
}

#[derive(Debug)]
pub enum FilterWhen {
    Plays,
    GroupedData,
}

// TODO: Add "Contains"
#[derive(Debug)]
pub enum FilterType {
    Above(FilterOperand),
    Between {
        lower: FilterOperand,
        upper: FilterOperand,
    },
    Below(FilterOperand),
    Equals(FilterOperand),
    Not(FilterOperand),
}

impl FilterType {
    fn is_valid() -> bool {
        todo!()
    }
}

const FILTER_TYPE_ABOVE_STR: &'static str = "above";
const FILTER_TYPE_BETWEEN_STR: &'static str = "between";
const FILTER_TYPE_BELOW_STR: &'static str = "below";
const FILTER_TYPE_EQUALS_STR: &'static str = "equals";
const FILTER_TYPE_NOT_STR: &'static str = "not";

pub fn filter(df: DataFrame, filter: Filter) -> PolarsResult<DataFrame> {
    todo!()
}
