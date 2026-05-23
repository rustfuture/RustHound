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
        let entry = self.trackers.entry(pattern_name.to_string()).or_default();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_alert_below_threshold() {
        let mut tracker = FrequencyTracker::new(3, 60);
        assert!(tracker.track_event("error").is_none());
        assert!(tracker.track_event("error").is_none());
        assert!(tracker.track_event("error").is_none());
    }

    #[test]
    fn alerts_when_threshold_exceeded() {
        let mut tracker = FrequencyTracker::new(2, 60);
        assert!(tracker.track_event("error").is_none());
        assert!(tracker.track_event("error").is_none());
        assert_eq!(tracker.track_event("error"), Some(3));
    }

    #[test]
    fn tracks_patterns_independently() {
        let mut tracker = FrequencyTracker::new(1, 60);
        assert!(tracker.track_event("a").is_none());
        assert_eq!(tracker.track_event("a"), Some(2));
        assert!(tracker.track_event("b").is_none());
    }
}
