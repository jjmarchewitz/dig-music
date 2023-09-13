use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct AggregatedData {
    total_ms_played: u64,
    total_plays: u64,
    start_reason: HashMap<String, u64>,
    end_reason: HashMap<String, u64>,
    num_shuffles: u64,
    num_skips: u64,
    timestamps: Vec<DateTime<Utc>>,
    conn_country: HashMap<String, u64>,
}
