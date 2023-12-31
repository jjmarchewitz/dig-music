#![allow(unused)]

mod operand_trait;
mod parse;
pub use operand_trait::FilterOperand;
pub use parse::parse_filters;

use chrono::prelude::*;
use polars::prelude::*;
use std::str::FromStr;
use strum_macros::{EnumDiscriminants, EnumString};

use crate::error::FilterParsingError;

#[derive(Debug)]
pub struct Filter {
    filter_by: FilterBy,
    filter_type: FilterType,
}

impl Filter {
    fn new(filter_by: FilterBy, filter_type: FilterType) -> Option<Self> {
        let mut is_valid = false;

        // TODO: check if FilterBy and FilterType are compatible
        is_valid = true;

        if is_valid {
            Some(Self {
                filter_by,
                filter_type,
            })
        } else {
            None
        }
    }

    fn filter_when(&self) -> FilterWhen {
        self.filter_by.filter_when()
    }
}

// TODO: Add Song, Artist, Album, Podcast, Podcast Episode, Duration
#[derive(Debug)]
pub enum FilterBy {
    Date,
    DateTime,
    ListenTime,
    PlayCount,
    Time,
}

impl FromStr for FilterBy {
    type Err = FilterParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "date" => Ok(FilterBy::Date),
            "datetime" => Ok(FilterBy::DateTime),
            "listen_time" => Ok(FilterBy::ListenTime),
            "play_count" => Ok(FilterBy::PlayCount),
            "time" => Ok(FilterBy::Time),
            _ => {
                return Err(FilterParsingError::FilterByParsingError(s.to_string()));
            }
        }
    }
}

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

#[derive(Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString), strum(serialize_all = "snake_case"))]
pub enum FilterType {
    Above(FilterOperand),
    Between {
        lower: FilterOperand,
        upper: FilterOperand,
    },
    Below(FilterOperand),
    Contains(FilterOperand),
    Equals(FilterOperand),
    Not(FilterOperand),
}

impl FilterType {
    fn is_valid() -> bool {
        todo!()
    }
}

pub fn filter(df: DataFrame, filter: Filter) -> PolarsResult<DataFrame> {
    todo!()
}
