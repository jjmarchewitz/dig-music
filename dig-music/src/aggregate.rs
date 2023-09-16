use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::Play;

#[derive(Debug)]
pub struct Accumulator<T: Default + Debug> {
    pub total: T,
    num_times_play_added: u64,
}

impl<T: Debug + Default> Accumulator<T> {
    fn increment_play_count(&mut self) {
        self.num_times_play_added += 1
    }
}

impl<T: Debug + Default> Default for Accumulator<T> {
    fn default() -> Self {
        Self {
            total: T::default(),
            num_times_play_added: 0,
        }
    }
}

impl<T: Debug + Default> Deref for Accumulator<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.total
    }
}

impl<T: Debug + Default> DerefMut for Accumulator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.total
    }
}

#[derive(Debug, Default)]
pub struct AggregatedData {
    pub ms_played: Accumulator<u64>,
    pub play_count: u64,
    pub start_reason: HashMap<String, u64>,
    pub end_reason: HashMap<String, u64>,
    pub num_shuffles: Accumulator<u64>,
    pub num_skips: Accumulator<u64>,
    pub timestamps: Vec<DateTime<Utc>>,
    pub conn_country: HashMap<String, u64>,
}

impl AggregatedData {
    pub fn add_play(&mut self, play: Play) {
        *self.ms_played += play.ms_played;
        self.ms_played.increment_play_count();

        self.play_count += 1;

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
            self.conn_country.insert(play.conn_country, 1);
        }
    }
}
