use chrono::{DateTime, Duration, Local};
use std::collections::HashMap;

pub struct FrequencyTracker {
    trackers: HashMap<String, Vec<DateTime<Local>>>,
    max_same_errors_per_minute: u32,
    time_window_seconds: u32,
}

impl FrequencyTracker {
    pub fn new(max_same_errors_per_minute: u32, time_window_seconds: u32) -> Self {
        FrequencyTracker {
            trackers: HashMap::new(),
            max_same_errors_per_minute,
            time_window_seconds,
        }
    }

    pub fn track_event(&mut self, pattern_name: &str) -> Option<u32> {
        let now = Local::now();
        let entry = self
            .trackers
            .entry(pattern_name.to_string())
            .or_insert_with(Vec::new);

        // Remove old timestamps outside the time window
        let time_window = Duration::seconds(self.time_window_seconds as i64);
        entry.retain(|&timestamp| now - timestamp < time_window);

        entry.push(now);

        let count = entry.len() as u32;
        if count > self.max_same_errors_per_minute {
            Some(count)
        } else {
            None
        }
    }
}
