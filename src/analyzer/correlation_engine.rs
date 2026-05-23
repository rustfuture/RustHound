use crate::config::rules::CorrelatedRule;
use crate::output::{Detection, Severity};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

struct Event {
    timestamp: Instant,
    detection: Detection,
}

pub struct CorrelationEngine {
    rules: Vec<CorrelatedRule>,
    recent_events: VecDeque<Event>,
}

impl CorrelationEngine {
    pub fn new(rules: Vec<CorrelatedRule>) -> Self {
        Self {
            rules,
            recent_events: VecDeque::new(),
        }
    }

    pub fn add_detection(&mut self, detection: Detection) -> Option<Detection> {
        let now = Instant::now();
        self.recent_events.push_back(Event {
            timestamp: now,
            detection: detection.clone(),
        });

        // Remove old events
        self.cleanup_old_events();

        // Check rules
        self.check_rules()
    }

    fn cleanup_old_events(&mut self) {
        let oldest_rule_window = self
            .rules
            .iter()
            .map(|r| r.time_window_seconds)
            .max()
            .unwrap_or(60);
        let cutoff = Instant::now() - Duration::from_secs(oldest_rule_window);

        while let Some(event) = self.recent_events.front() {
            if event.timestamp < cutoff {
                self.recent_events.pop_front();
            } else {
                break;
            }
        }
    }

    fn check_rules(&self) -> Option<Detection> {
        for rule in &self.rules {
            let window_start = Instant::now() - Duration::from_secs(rule.time_window_seconds);

            // Find the 'followed_by' event first
            if let Some(followed_by_event_pos) = self
                .recent_events
                .iter()
                .rposition(|e| e.detection.pattern_name == rule.followed_by)
            {
                let followed_by_event = &self.recent_events[followed_by_event_pos];

                // Count trigger events that occurred *before* the 'followed_by' event
                let trigger_count = self
                    .recent_events
                    .iter()
                    .take(followed_by_event_pos)
                    .filter(|e| {
                        e.timestamp >= window_start
                            && e.detection.pattern_name == rule.trigger_on_rule.name
                    })
                    .count();

                if trigger_count >= rule.trigger_on_rule.count {
                    return Some(Detection {
                        severity: Severity::from(rule.severity.as_str()),
                        file_path: followed_by_event.detection.file_path.clone(),
                        line_number: followed_by_event.detection.line_number,
                        matched_line: followed_by_event.detection.matched_line.clone(),
                        pattern_name: rule.name.clone(),
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::rules::{CorrelatedRule, TriggerRule};
    use crate::output::{Detection, Severity};

    fn sample_detection(pattern_name: &str, line: usize) -> Detection {
        Detection {
            severity: Severity::High,
            file_path: "/var/log/auth.log".to_string(),
            line_number: line,
            pattern_name: pattern_name.to_string(),
            matched_line: "sample line".to_string(),
        }
    }

    fn brute_force_rules() -> Vec<CorrelatedRule> {
        vec![CorrelatedRule {
            name: "Potential Brute-Force Attack".to_string(),
            severity: "critical".to_string(),
            description: "test".to_string(),
            time_window_seconds: 60,
            trigger_on_rule: TriggerRule {
                name: "authentication_failure".to_string(),
                count: 3,
            },
            followed_by: "Successful Login".to_string(),
        }]
    }

    #[test]
    fn triggers_after_enough_failures_then_success() {
        let mut engine = CorrelationEngine::new(brute_force_rules());
        assert!(engine
            .add_detection(sample_detection("authentication_failure", 1))
            .is_none());
        assert!(engine
            .add_detection(sample_detection("authentication_failure", 2))
            .is_none());
        assert!(engine
            .add_detection(sample_detection("authentication_failure", 3))
            .is_none());
        let correlated = engine
            .add_detection(sample_detection("Successful Login", 4))
            .expect("correlation should fire");
        assert_eq!(correlated.pattern_name, "Potential Brute-Force Attack");
        assert_eq!(correlated.severity, Severity::Critical);
    }

    #[test]
    fn no_trigger_without_followed_by_event() {
        let mut engine = CorrelationEngine::new(brute_force_rules());
        for i in 1..=5 {
            assert!(engine
                .add_detection(sample_detection("authentication_failure", i))
                .is_none());
        }
    }
}
