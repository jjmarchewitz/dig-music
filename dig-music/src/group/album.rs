use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Album {
    pub album_name: String,
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Album {}
