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

#[derive(Debug)]
pub enum FilterBy {
    Date,
    DateTime,
    Time,

    ListenTime,
    PlayCount,
    PlayDuration,

    Album,
    Artist,
    Episode,
    Podcast,
    Song,
}

impl FilterBy {
    fn filter_when(&self) -> FilterWhen {
        match self {
            FilterBy::Date => FilterWhen::Plays,
            FilterBy::DateTime => FilterWhen::Plays,
            FilterBy::Time => FilterWhen::Plays,

            FilterBy::ListenTime => FilterWhen::GroupedData,
            FilterBy::PlayCount => FilterWhen::GroupedData,
            FilterBy::PlayDuration => FilterWhen::GroupedData,

            FilterBy::Album => FilterWhen::Plays,
            FilterBy::Artist => FilterWhen::Plays,
            FilterBy::Episode => FilterWhen::Plays,
            FilterBy::Podcast => FilterWhen::Plays,
            FilterBy::Song => FilterWhen::Plays,
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
    fn is_valid(&self) -> bool {
        todo!()
    }
}

pub fn filter(df: DataFrame, filter: Filter) -> PolarsResult<DataFrame> {
    todo!()
}
