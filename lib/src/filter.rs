#![allow(unused)]

mod parse;
pub use parse::parse_filters;

use std::collections::VecDeque;

use chrono::prelude::*;
use polars::prelude::*;
use thiserror::Error;

use crate::error::FilterParsingError;

pub trait FilterOperand {}

impl<T: FilterOperand + ?Sized> FilterOperand for Box<T> {}

pub struct Filter<T: FilterOperand + ?Sized> {
    filter_by: FilterBy,
    filter_type: FilterType<T>,
}

impl<T: FilterOperand + ?Sized> Filter<T> {
    fn new(filter_by: FilterBy, filter_type: FilterType<T>) -> Option<Self> {
        // check if FilterBy and FilterType are
        todo!()
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

pub enum FilterBy {
    Date,
    PlayCount,
    ListeningTime,
}

impl FilterBy {
    fn filter_when(&self) -> FilterWhen {
        match self {
            FilterBy::Date => FilterWhen::Plays,
            FilterBy::PlayCount => FilterWhen::GroupedData,
            FilterBy::ListeningTime => FilterWhen::GroupedData,
        }
    }
}

pub enum FilterWhen {
    Plays,
    GroupedData,
}

pub enum FilterType<T: FilterOperand + ?Sized> {
    Above(Box<T>),
    Between { lower: Box<T>, upper: Box<T> },
    Below(Box<T>),
    Equals(Box<T>),
    Not(Box<T>),
}

pub fn filter<T: FilterOperand>(df: DataFrame, filter: Filter<T>) -> PolarsResult<DataFrame> {
    todo!()
}
