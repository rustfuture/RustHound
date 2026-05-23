use crate::config::rules::{RegexRule, Rules};
use regex::Regex;

pub struct PatternMatcher {
    error_patterns: Vec<String>,
    warning_patterns: Vec<String>,
    regex_rules: Vec<(RegexRule, Regex)>,
}

impl PatternMatcher {
    pub fn new(rules: &Rules) -> anyhow::Result<Self> {
        let mut compiled_regex_rules = Vec::new();
        for rule in &rules.regex_rules {
            let regex = Regex::new(&rule.pattern)?;
            compiled_regex_rules.push((rule.clone(), regex));
        }

        Ok(PatternMatcher {
            error_patterns: rules.patterns.error_patterns.clone(),
            warning_patterns: rules.patterns.warning_patterns.clone(),
            regex_rules: compiled_regex_rules,
        })
    }

    pub fn check_for_patterns(&self, line: &str) -> Option<(&str, &str)> {
        for pattern in &self.error_patterns {
            if line.contains(pattern) {
                return Some(("ERROR", pattern));
            }
        }

        for pattern in &self.warning_patterns {
            if line.contains(pattern) {
                return Some(("WARNING", pattern));
            }
        }

        for (rule, regex) in &self.regex_rules {
            if regex.is_match(line) {
                return Some((&rule.severity, &rule.name));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::rules::{PatternConfig, RegexRule, Rules};

    fn test_rules() -> Rules {
        Rules {
            patterns: PatternConfig {
                error_patterns: vec!["ERROR".to_string()],
                warning_patterns: vec!["WARN".to_string()],
            },
            regex_rules: vec![RegexRule {
                name: "custom_rule".to_string(),
                pattern: r"disk.*error".to_string(),
                severity: "critical".to_string(),
            }],
            frequency_rules: None,
            correlated_rules: vec![],
        }
    }

    #[test]
    fn matches_error_pattern_first() {
        let matcher = PatternMatcher::new(&test_rules()).unwrap();
        let result = matcher.check_for_patterns("something ERROR happened");
        assert_eq!(result, Some(("ERROR", "ERROR")));
    }

    #[test]
    fn matches_warning_when_no_error() {
        let matcher = PatternMatcher::new(&test_rules()).unwrap();
        let result = matcher.check_for_patterns("WARN: low memory");
        assert_eq!(result, Some(("WARNING", "WARN")));
    }

    #[test]
    fn matches_regex_when_no_string_patterns() {
        let matcher = PatternMatcher::new(&test_rules()).unwrap();
        let result = matcher.check_for_patterns("disk io error on /dev/sda");
        assert_eq!(result, Some(("critical", "custom_rule")));
    }

    #[test]
    fn error_takes_priority_over_regex() {
        let matcher = PatternMatcher::new(&test_rules()).unwrap();
        let result = matcher.check_for_patterns("ERROR disk error");
        assert_eq!(result, Some(("ERROR", "ERROR")));
    }
}
