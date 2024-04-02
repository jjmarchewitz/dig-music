use chrono::{DateTime, Duration, Utc};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{AddAssign, Deref, DerefMut},
};

use crate::Play;

#[derive(Debug, Clone)]
pub struct Accumulator<T: Default + Debug + Display> {
    pub total: T,
    num_times_play_added: u64,
}

impl<T: Debug + Default + Display> Accumulator<T> {
    fn increment_play_count(&mut self) {
        self.num_times_play_added += 1
    }
}

impl<T: Debug + Default + Display> Default for Accumulator<T> {
    fn default() -> Self {
        Self {
            total: T::default(),
            num_times_play_added: 0,
        }
    }
}

impl<T: Debug + Default + Display> Deref for Accumulator<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.total
    }
}

impl<T: Debug + Default + Display> DerefMut for Accumulator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.total
    }
}

impl<T: Debug + Default + Display> Display for Accumulator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.total, f)
    }
}

impl<T: Debug + Default + Display + AddAssign> AddAssign for Accumulator<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.total += rhs.total;
        self.num_times_play_added += rhs.num_times_play_added;
    }
}

#[derive(Debug, Default, Clone)]
pub struct AggregatedData {
    pub ms_played: Accumulator<u64>,
    pub play_count: u64,
    pub start_reason: HashMap<String, u64>,
    pub end_reason: HashMap<String, u64>,
    pub num_shuffles: Accumulator<u64>,
    pub num_skips: Accumulator<u64>,
    pub timestamps: Vec<DateTime<Utc>>,
    pub conn_country: HashMap<String, u64>,
    pub track_uri: Option<String>,
    pub episode_uri: Option<String>,
}

impl AggregatedData {
    pub fn add_play(&mut self, play: Play) {
        if let Some(ms_played) = play.ms_played {
            *self.ms_played += ms_played;
        }

        self.ms_played.increment_play_count();

        self.play_count += 1;

        // If this Play has `start_reason` data, add it
        if let Some(reason_start) = play.reason_start {
            if let Some(c) = self.start_reason.get_mut(reason_start.as_str()) {
                *c += 1;
            } else {
                self.start_reason.insert(reason_start, 1);
            }
        }

        if let Some(reason_end) = play.reason_end {
            // If this Play has `end_reason` data, add it
            if let Some(c) = self.end_reason.get_mut(reason_end.as_str()) {
                *c += 1;
            } else {
                self.end_reason.insert(reason_end, 1);
            }
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

        if let Some(ts) = play.ts {
            self.timestamps.push(ts);
        }

        if let Some(conn_country) = play.conn_country {
            if let Some(c) = self.conn_country.get_mut(conn_country.as_str()) {
                *c += 1;
            } else {
                self.conn_country.insert(conn_country, 1);
            }
        }

        if let Some(track_uri) = play.spotify_track_uri {
            self.track_uri = Some(track_uri)
        }

        if let Some(episode_uri) = play.spotify_episode_uri {
            self.episode_uri = Some(episode_uri)
        }
    }

    pub fn display_ms_played(&self) -> String {
        let d = Duration::milliseconds(self.ms_played.total.try_into().unwrap());

        let mut output: String = "".to_owned();

        // Probably a bad idea in terms of readability but it makes the impl simpler
        let mut duration_consumed: i64 = 0;

        // This ensures that in a case where a non-zero value is followed by a zero value
        // (i.e. 5h 0m 20s), the zero-value still gets concatenated to the string
        let mut started_concatenating: bool = false;

        if d.num_weeks() > 0 {
            let weeks = d.num_weeks();
            duration_consumed = weeks;

            output.push_str(format!("{}w ", weeks).as_str());
            started_concatenating = true;
        }

        if d.num_days() > 0 || started_concatenating {
            duration_consumed *= 7;
            let days = d.num_days() - duration_consumed;
            duration_consumed += days;

            output.push_str(format!("{}d ", days).as_str());
            started_concatenating = true;
        }

        if d.num_hours() > 0 || started_concatenating {
            duration_consumed *= 24;
            let hours = d.num_hours() - duration_consumed;
            duration_consumed += hours;

            output.push_str(format!("{}h ", hours).as_str());
            started_concatenating = true;
        }

        if d.num_minutes() > 0 || started_concatenating {
            duration_consumed *= 60;
            let minutes = d.num_minutes() - duration_consumed;
            duration_consumed += minutes;

            output.push_str(format!("{}m ", minutes).as_str());
            started_concatenating = true;
        }

        if d.num_seconds() > 0 || started_concatenating {
            duration_consumed *= 60;
            let seconds = d.num_seconds() - duration_consumed;
            duration_consumed += seconds;

            output.push_str(format!("{}s ", seconds).as_str());
        }

        duration_consumed *= 1000;
        let ms = d.num_milliseconds() - duration_consumed;
        output.push_str(format!("{} ms", ms).as_str());

        output
    }
}

impl AddAssign for AggregatedData {
    fn add_assign(&mut self, rhs: Self) {
        self.ms_played += rhs.ms_played;
        self.play_count += rhs.play_count;

        for (new_reason, new_count) in rhs.start_reason.into_iter() {
            if let Some(old_count) = self.start_reason.get(&new_reason) {
                self.start_reason.insert(new_reason, old_count + new_count);
            } else {
                self.start_reason.insert(new_reason, new_count);
            }
        }

        for (new_reason, new_count) in rhs.end_reason.into_iter() {
            if let Some(old_count) = self.end_reason.get(&new_reason) {
                self.end_reason.insert(new_reason, old_count + new_count);
            } else {
                self.end_reason.insert(new_reason, new_count);
            }
        }

        self.num_shuffles += rhs.num_shuffles;
        self.num_skips += rhs.num_skips;
        self.timestamps.extend(rhs.timestamps);

        for (new_country, new_count) in rhs.conn_country.into_iter() {
            if let Some(old_count) = self.conn_country.get(&new_country) {
                self.conn_country.insert(new_country, old_count + new_count);
            } else {
                self.conn_country.insert(new_country, new_count);
            }
        }
    }
}
