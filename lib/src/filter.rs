#![allow(unused)]

mod operand_trait;
mod parse;
pub use operand_trait::FilterOperand;
pub use parse::parse_filters;

use chrono::prelude::*;
use polars::prelude::*;

use crate::error::FilterParsingError;

#[derive(Debug)]
pub struct Filter<T: FilterOperand + ?Sized> {
    filter_by: FilterBy,
    filter_type: FilterType<T>,
}

impl<T: FilterOperand + ?Sized> Filter<T> {
    fn new(filter_by: FilterBy, filter_type: FilterType<T>) -> Option<Self> {
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

// impl<T: FilterOperand> TryFrom<&str> for Filter<T> {
//     type Error = FilterParsingError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

#[derive(Debug)]
pub enum FilterBy {
    Date,
    PlayCount,
    ListenTime,
}

const FILTER_BY_DATE_STR: &'static str = "date";
const FILTER_BY_PLAY_COUNT_STR: &'static str = "play_count";
const FILTER_BY_LISTEN_TIME_STR: &'static str = "listen_time";

impl FilterBy {
    fn filter_when(&self) -> FilterWhen {
        match self {
            FilterBy::Date => FilterWhen::Plays,
            FilterBy::PlayCount => FilterWhen::GroupedData,
            FilterBy::ListenTime => FilterWhen::GroupedData,
        }
    }
}

#[derive(Debug)]
pub enum FilterWhen {
    Plays,
    GroupedData,
}

#[derive(Debug)]
pub enum FilterType<T: FilterOperand + ?Sized> {
    Above(Box<T>),
    Between { lower: Box<T>, upper: Box<T> },
    Below(Box<T>),
    Equals(Box<T>),
    Not(Box<T>),
}

const FILTER_TYPE_ABOVE_STR: &'static str = "above";
const FILTER_TYPE_BETWEEN_STR: &'static str = "between";
const FILTER_TYPE_BELOW_STR: &'static str = "below";
const FILTER_TYPE_EQUALS_STR: &'static str = "equals";
const FILTER_TYPE_NOT_STR: &'static str = "not";

pub fn filter<T: FilterOperand>(df: DataFrame, filter: Filter<T>) -> PolarsResult<DataFrame> {
    todo!()
}
