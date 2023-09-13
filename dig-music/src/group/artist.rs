use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Artist {
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Artist {}
