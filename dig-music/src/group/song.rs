use super::PlayGroup;
use crate::aggregate::AggregatedData;

pub struct Song {
    pub song_name: String,
    pub album_name: String,
    pub artist_name: String,
    pub aggregated_data: AggregatedData,
}

impl PlayGroup for Song {}
