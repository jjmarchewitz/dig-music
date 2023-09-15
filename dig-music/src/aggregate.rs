use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::Play;

#[derive(Debug)]
pub struct Counter<T: Default + Debug> {
    accumulator: T,
    num_times_play_added: u64,
}

impl<T: Debug + Default> Counter<T> {
    fn increment_play_count(&mut self) {
        self.num_times_play_added += 1
    }
}

impl<T: Debug + Default> Default for Counter<T> {
    fn default() -> Self {
        Self {
            accumulator: T::default(),
            num_times_play_added: 0,
        }
    }
}

impl<T: Debug + Default> Deref for Counter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.accumulator
    }
}

impl<T: Debug + Default> DerefMut for Counter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.accumulator
    }
}

#[derive(Debug, Default)]
pub struct AggregatedData {
    pub total_ms_played: Counter<u64>,
    pub total_plays: u64,
    pub start_reason: HashMap<String, u64>,
    pub end_reason: HashMap<String, u64>,
    pub num_shuffles: Counter<u64>,
    pub num_skips: Counter<u64>,
    pub timestamps: Vec<DateTime<Utc>>,
    pub conn_country: HashMap<String, u64>,
}

impl AggregatedData {
    pub fn add_play(&mut self, play: Play) {
        *self.total_ms_played += play.ms_played;
        self.total_ms_played.increment_play_count();

        self.total_plays += 1;

        // If this Play has `start_reason` data, add it
        if let Some(c) = self.start_reason.get_mut(play.reason_start.as_str()) {
            *c += 1;
        } else {
            self.start_reason.insert(play.reason_start, 1);
        }

        // If this Play has `end_reason` data, add it
        if let Some(c) = self.end_reason.get_mut(play.reason_end.as_str()) {
            *c += 1;
        } else {
            self.end_reason.insert(play.reason_end, 1);
        }

        // If this Play has `shuffle` data, add it
        if let Some(shuffled) = play.shuffle {
            if shuffled {
                *self.num_shuffles += 1;
            }

            self.num_shuffles.increment_play_count()
        }

        // If this Play has `skip` data, add it
        if let Some(skipped) = play.skipped {
            if skipped {
                *self.num_skips += 1;
            }

            self.num_skips.increment_play_count()
        }

        self.timestamps.push(play.ts);

        if let Some(c) = self.conn_country.get_mut(play.conn_country.as_str()) {
            *c += 1;
        } else {
            self.end_reason.insert(play.conn_country, 1);
        }
    }
}
