use std::collections::VecDeque;

use chrono::prelude::*;
use polars::prelude::*;
use thiserror::Error;

use crate::error::FilterParsingError;
use crate::filter::{Filter, FilterBy, FilterOperand};

pub fn parse_filters(
    str_filters: Vec<String>,
) -> Result<Vec<Filter<dyn FilterOperand>>, FilterParsingError> {
    str_filters
        .into_iter()
        .map(|s| parse_one_filter(s))
        .collect::<Result<Vec<Filter<dyn FilterOperand>>, FilterParsingError>>()
}

fn parse_one_filter(str_filter: String) -> Result<Filter<dyn FilterOperand>, FilterParsingError> {
    if str_filter.len() <= 0 {
        return Err(FilterParsingError::NoArgumentFound);
    }

    let mut components: VecDeque<&str> = str_filter.split(" ").collect();

    let filter_by = match components.pop_front() {
        Some(filter_by) => parse_filter_by(&str_filter, filter_by)?,
        None => return Err(FilterParsingError::NotLongEnough(str_filter)),
    };

    // let first_str

    dbg!(components);

    // let surrounded_by_single_quotes = str_filter.starts_with("\'") && str_filter.ends_with("\'");
    // let surrounded_by_double_quotes = str_filter.starts_with("\"") && str_filter.ends_with("\"");

    // if !(surrounded_by_single_quotes || surrounded_by_double_quotes) {
    //     return Err(FilterParsingError::NotSurroundedByQuotes(str_filter));
    // }

    todo!()
}

fn parse_filter_by(str_filter: &str, filter_by: &str) -> Result<FilterBy, FilterParsingError> {
    match filter_by.to_lowercase().as_str() {
        "date" => Ok(FilterBy::Date),
        "listen_time" => Ok(FilterBy::ListeningTime),
        "play_count" => Ok(FilterBy::PlayCount),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                str_filter.to_string(),
                filter_by.to_string(),
            ));
        }
    }
}
