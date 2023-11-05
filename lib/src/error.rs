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
