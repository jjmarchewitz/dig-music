use polars::error::PolarsError;
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
}

#[derive(Debug, Error)]
pub enum FilterParsingError {
    #[error("no argument found after --filter.")]
    NoArgumentFound,

    #[error("unable to parse filter. You passed in `{0}` and parsing stopped at `{1}`.")]
    GenericParsingError(String, String),

    #[error("not enough arguments were passed into the filter. You passed in `{0}`.")]
    NotLongEnough(String),
}
