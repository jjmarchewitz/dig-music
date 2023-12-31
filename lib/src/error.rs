use chrono;
use polars::error::PolarsError;
use regex;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("cannot group plays together when the \"meta\" option is selected.")]
    CannotGroupByMeta,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("unable to parse your data.")]
    ParseError,

    #[error("unable to open the file at the given path.")]
    UnableToOpenFile,

    #[error("unable to load the data from the .zip file at the given path.")]
    UnableToLoadZipData,

    #[error("unable to construct a DataFrame from the given data.")]
    CannotConstructDataframe(#[from] PolarsError),

    #[error("collecting data from the .zip into a DataFrame failed.")]
    FailedToCollectData,

    #[error("this error should never happen. Error building regular expression that matches files in the .zip")]
    RegexError(#[from] regex::Error),
}

#[derive(Debug, Error)]
pub enum FilterParsingError {
    #[error("no argument found after --filter.")]
    NoArgumentFound,

    #[error("unable to parse filter. You passed in `{0}` and parsing stopped at `{1}`.")]
    GenericParsingError(String, String),

    #[error("not enough arguments were passed into the filter. You passed in `{0}`.")]
    NotLongEnough(String),

    #[error("unable to parse string into int.")]
    IntParsingError(#[from] ParseIntError),

    #[error("unable to parse string into date.")]
    DateParsingError(#[from] chrono::ParseError),

    #[error("unable to parse \"{0}\" into FilterBy.")]
    FilterByParsingError(String),

    #[error("unable to parse \"{0}\" into FilterType.")]
    FilterTypeParsingError(String),

    #[error("Unable to construct the filter: `{0}`. You may have provided a combination of arguments that is invalid.")]
    UnknownError(String),
}
