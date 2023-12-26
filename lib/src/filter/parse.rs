use std::collections::VecDeque;

use chrono::prelude::*;
// use polars::prelude::*;
use thiserror::Error;

use super::*;
use crate::error::FilterParsingError;

pub fn parse_filters(
    filter_strs: Vec<String>,
) -> Result<Vec<Filter<dyn FilterOperand>>, FilterParsingError> {
    filter_strs
        .into_iter()
        .map(|s| parse_one_filter(s))
        .collect::<Result<Vec<Filter<dyn FilterOperand>>, FilterParsingError>>()
}

fn parse_one_filter(filter_str: String) -> Result<Filter<dyn FilterOperand>, FilterParsingError> {
    if filter_str.len() <= 0 {
        return Err(FilterParsingError::NoArgumentFound);
    }

    let mut filter_str_components: VecDeque<&str> = filter_str.split(" ").collect();

    let filter_by = match filter_str_components.pop_front() {
        Some(filter_by) => parse_filter_by(&filter_str, filter_by)?,
        None => return Err(FilterParsingError::NotLongEnough(filter_str)),
    };

    let filter_type: FilterType<dyn FilterOperand> =
        parse_filter_type_and_bounds(&filter_str, &mut filter_str_components, &filter_by)?;

    if let Some(filter) = Filter::new(filter_by, filter_type) {
        Ok(filter)
    } else {
        Err(FilterParsingError::UnknownError(filter_str))
    }
}

fn parse_filter_by(filter_str: &str, filter_by: &str) -> Result<FilterBy, FilterParsingError> {
    match filter_by.to_lowercase().as_str() {
        FILTER_BY_DATE_STR => Ok(FilterBy::Date),
        FILTER_BY_DATETIME_STR => Ok(FilterBy::DateTime),
        FILTER_BY_LISTEN_TIME_STR => Ok(FilterBy::ListenTime),
        FILTER_BY_PLAY_COUNT_STR => Ok(FilterBy::PlayCount),
        FILTER_BY_TIME_STR => Ok(FilterBy::Time),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                filter_str.to_string(),
                filter_by.to_string(),
            ));
        }
    }
}

fn parse_filter_type_and_bounds(
    filter_str: &str,
    filter_str_components: &mut VecDeque<&str>,
    filter_by: &FilterBy,
) -> Result<FilterType<dyn FilterOperand>, FilterParsingError> {
    let filter_str = filter_str.to_string();

    let Some(filter_type_str) = filter_str_components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(filter_str));
    };

    let Some(first_arg) = filter_str_components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(filter_str));
    };

    let filter_type = match filter_type_str {
        FILTER_TYPE_ABOVE_STR => FilterType::Above(parse_filter_bound(&filter_by, first_arg)?),
        FILTER_TYPE_BELOW_STR => FilterType::Below(parse_filter_bound(&filter_by, first_arg)?),
        FILTER_TYPE_BETWEEN_STR => {
            let Some(second_arg) = filter_str_components.pop_front() else {
                return Err(FilterParsingError::NotLongEnough(filter_str));
            };
            FilterType::Between {
                lower: parse_filter_bound(&filter_by, first_arg)?,
                upper: parse_filter_bound(&filter_by, second_arg)?,
            }
        }
        FILTER_TYPE_EQUALS_STR => FilterType::Equals(parse_filter_bound(&filter_by, first_arg)?),
        FILTER_TYPE_NOT_STR => FilterType::Not(parse_filter_bound(&filter_by, first_arg)?),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                filter_str.to_string(),
                filter_type_str.to_string(),
            ));
        }
    };

    Ok(filter_type)
}

fn parse_filter_bound(
    filter_by: &FilterBy,
    s: &str,
) -> Result<Box<dyn FilterOperand>, FilterParsingError> {
    match filter_by {
        FilterBy::Date => Ok(Box::new(s.parse::<NaiveDate>()?)),
        FilterBy::DateTime => Ok(Box::new(s.parse::<NaiveDateTime>()?)),
        FilterBy::ListenTime => Ok(Box::new(s.parse::<u64>()?)),
        FilterBy::PlayCount => Ok(Box::new(s.parse::<u32>()?)),
        FilterBy::Time => Ok(Box::new(s.parse::<NaiveTime>()?)),
    }
}
