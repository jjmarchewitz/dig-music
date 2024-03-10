use std::collections::VecDeque;

use chrono::prelude::*;
// use polars::prelude::*;
use thiserror::Error;

use super::*;
use crate::error::FilterParsingError;

impl FromStr for FilterBy {
    type Err = FilterParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "listen_on_date" => FilterBy::Date,
            "listen_at_datetime" => FilterBy::DateTime,
            "listen_at_time" => FilterBy::Time,

            "total_listen_time" => FilterBy::ListenTime,
            "play_count" => FilterBy::PlayCount,
            "play_duration" => FilterBy::PlayDuration,

            "album" => FilterBy::Album,
            "artist" => FilterBy::Artist,
            "episode" => FilterBy::Episode,
            "podcast" => FilterBy::Podcast,
            "song" => FilterBy::Song,

            _ => {
                return Err(FilterParsingError::FilterByParsingError(s.to_string()));
            }
        };

        Ok(res)
    }
}

pub fn parse_filters(filter_strs: Vec<String>) -> Result<Vec<Filter>, FilterParsingError> {
    filter_strs
        .into_iter()
        .map(|s| parse_one_filter(s))
        .collect::<Result<Vec<Filter>, FilterParsingError>>()
}

fn parse_one_filter(filter_str: String) -> Result<Filter, FilterParsingError> {
    if filter_str.len() <= 0 {
        return Err(FilterParsingError::NoArgumentFound);
    }

    let mut filter_str_components: VecDeque<&str> = filter_str.split(" ").collect();

    let filter_by = match filter_str_components.pop_front() {
        Some(filter_by) => FilterBy::from_str(filter_by)?,
        None => return Err(FilterParsingError::NotLongEnough(filter_str)),
    };

    let filter_type: FilterType =
        parse_filter_type_and_bounds(&filter_str, &mut filter_str_components, &filter_by)?;

    if let Some(filter) = Filter::new(filter_by, filter_type) {
        Ok(filter)
    } else {
        Err(FilterParsingError::UnknownError(filter_str))
    }
}

fn parse_filter_type_and_bounds(
    filter_str: &str,
    filter_str_components: &mut VecDeque<&str>,
    filter_by: &FilterBy,
) -> Result<FilterType, FilterParsingError> {
    let filter_str = filter_str.to_string();

    let Some(filter_type_str) = filter_str_components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(filter_str));
    };

    let Some(first_arg) = filter_str_components.pop_front() else {
        return Err(FilterParsingError::NotLongEnough(filter_str));
    };

    let Ok(filter_type_discriminant) = FilterTypeDiscriminants::from_str(filter_type_str) else {
        return Err(FilterParsingError::FilterTypeParsingError(
            filter_type_str.to_string(),
        ));
    };

    let filter_type = match filter_type_discriminant {
        FilterTypeDiscriminants::Above => {
            FilterType::Above(parse_filter_bound(&filter_by, first_arg)?)
        }
        FilterTypeDiscriminants::Below => {
            FilterType::Below(parse_filter_bound(&filter_by, first_arg)?)
        }
        FilterTypeDiscriminants::Between => {
            let Some(second_arg) = filter_str_components.pop_front() else {
                return Err(FilterParsingError::NotLongEnough(filter_str));
            };

            FilterType::Between {
                lower: parse_filter_bound(&filter_by, first_arg)?,
                upper: parse_filter_bound(&filter_by, second_arg)?,
            }
        }
        FilterTypeDiscriminants::Contains => {
            FilterType::Contains(parse_filter_bound(&filter_by, first_arg)?)
        }
        FilterTypeDiscriminants::Equals => {
            FilterType::Equals(parse_filter_bound(&filter_by, first_arg)?)
        }
        FilterTypeDiscriminants::Not => FilterType::Not(parse_filter_bound(&filter_by, first_arg)?),
        _ => {
            return Err(FilterParsingError::GenericParsingError(
                filter_str.to_string(),
                filter_type_str.to_string(),
            ));
        }
    };

    Ok(filter_type)
}

fn parse_filter_bound(filter_by: &FilterBy, s: &str) -> Result<FilterOperand, FilterParsingError> {
    let operand = match filter_by {
        FilterBy::Date => FilterOperand::NaiveDate(s.parse::<NaiveDate>()?),
        FilterBy::DateTime => FilterOperand::NaiveDateTime(s.parse::<NaiveDateTime>()?),
        FilterBy::Time => FilterOperand::NaiveTime(s.parse::<NaiveTime>()?),

        FilterBy::ListenTime => FilterOperand::U64(s.parse::<u64>()?),
        FilterBy::PlayCount => FilterOperand::U32(s.parse::<u32>()?),
        FilterBy::PlayDuration => FilterOperand::U64(s.parse::<u64>()?),

        FilterBy::Album => FilterOperand::String(s.to_string()),
        FilterBy::Artist => FilterOperand::String(s.to_string()),
        FilterBy::Episode => FilterOperand::String(s.to_string()),
        FilterBy::Podcast => FilterOperand::String(s.to_string()),
        FilterBy::Song => FilterOperand::String(s.to_string()),
    };

    Ok(operand)
}
