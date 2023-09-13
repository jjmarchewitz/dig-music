use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct AggregatedData {
    pub total_ms_played: u64,
    pub total_plays: u64,
    pub start_reason: HashMap<String, u64>,
    pub end_reason: HashMap<String, u64>,
    pub num_shuffles: u64,
    pub num_skips: u64,
    pub timestamps: Vec<DateTime<Utc>>,
    pub conn_country: HashMap<String, u64>,
}
