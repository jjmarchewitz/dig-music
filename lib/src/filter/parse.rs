use std::collections::VecDeque;

use chrono::prelude::*;
// use polars::prelude::*;
use thiserror::Error;

use super::*;
use crate::error::FilterParsingError;

impl FilterBy {
    fn parse_into_boxed_data(&self, s: &str) -> Result<Box<dyn FilterOperand>, FilterParsingError> {
        match self {
            FilterBy::PlayCount => Ok(Box::new(s.parse::<u32>()?)),
            _ => todo!(),
        }
    }
}

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

    let Some(filter_type_str) = components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(str_filter));
    };

    let Some(first_arg) = components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(str_filter));
    };

    let filter_type: FilterType<dyn FilterOperand> = match filter_type_str {
        FILTER_TYPE_ABOVE_STR => FilterType::Above(filter_by.parse_into_boxed_data(first_arg)?),
        FILTER_TYPE_BELOW_STR => FilterType::Below(filter_by.parse_into_boxed_data(first_arg)?),
        FILTER_TYPE_BETWEEN_STR => {
            let Some(second_arg) = components.pop_front() else {
                return Err(FilterParsingError::NotLongEnough(str_filter));
            };
            FilterType::Between {
                lower: filter_by.parse_into_boxed_data(first_arg)?,
                upper: filter_by.parse_into_boxed_data(second_arg)?,
            }
        }
        FILTER_TYPE_EQUALS_STR => FilterType::Equals(filter_by.parse_into_boxed_data(first_arg)?),
        FILTER_TYPE_NOT_STR => FilterType::Not(filter_by.parse_into_boxed_data(first_arg)?),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                str_filter.to_string(),
                filter_type_str.to_string(),
            ));
        }
    };

    if let Some(filter) = Filter::new(filter_by, filter_type) {
        Ok(filter)
    } else {
        Err(FilterParsingError::UnknownError(str_filter))
    }
}

fn parse_filter_by(str_filter: &str, filter_by: &str) -> Result<FilterBy, FilterParsingError> {
    match filter_by.to_lowercase().as_str() {
        FILTER_BY_DATE_STR => Ok(FilterBy::Date),
        FILTER_BY_LISTEN_TIME_STR => Ok(FilterBy::ListenTime),
        FILTER_BY_PLAY_COUNT_STR => Ok(FilterBy::PlayCount),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                str_filter.to_string(),
                filter_by.to_string(),
            ));
        }
    }
}
