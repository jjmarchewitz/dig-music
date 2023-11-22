use chrono::{DateTime, Utc};
use core::fmt::Debug;
// use clap::value_parser;

#[derive(Debug, Clone)]
pub enum FilterOn {
    Date(FilterSelection<DateTime<Utc>>),
    PlayCount(FilterSelection<u64>),
}

#[derive(Debug, Clone, Copy)]
pub enum FilterSelection<T: PartialOrd + Debug> {
    Above { min: T },
    Between { min: T, max: T },
    Below { max: T },
}

// impl Parser for FilterOption {
//     fn parse() -> Self {

//     }
// }
