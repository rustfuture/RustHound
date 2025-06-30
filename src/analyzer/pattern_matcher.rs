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
