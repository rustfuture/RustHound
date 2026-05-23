use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct PatternConfig {
    #[serde(default)]
    pub error_patterns: Vec<String>,
    #[serde(default)]
    pub warning_patterns: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Rules {
    #[serde(rename = "rules", default)]
    pub patterns: PatternConfig,
    #[serde(default)]
    pub regex_rules: Vec<RegexRule>,
    #[serde(default)]
    pub frequency_rules: Option<FrequencyRules>,
    #[serde(default)]
    pub correlated_rules: Vec<CorrelatedRule>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegexRule {
    pub name: String,
    pub pattern: String,
    pub severity: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FrequencyRules {
    pub max_same_errors_per_minute: u32,
    pub time_window_seconds: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CorrelatedRule {
    pub name: String,
    pub severity: String,
    pub description: String,
    pub time_window_seconds: u64,
    pub trigger_on_rule: TriggerRule,
    pub followed_by: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TriggerRule {
    pub name: String,
    pub count: usize,
}

pub fn load_rules_from_file(path: &std::path::Path) -> anyhow::Result<Rules> {
    let content = std::fs::read_to_string(path)?;
    let rules: Rules = toml::from_str(&content)?;
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_rules_toml() {
        let rules = load_rules_from_file(std::path::Path::new("rules.toml")).unwrap();
        assert!(!rules.patterns.error_patterns.is_empty());
        assert!(!rules.regex_rules.is_empty());
        assert!(rules.frequency_rules.is_some());
    }

    #[test]
    fn deserializes_correlated_rules_toml() {
        let rules = load_rules_from_file(std::path::Path::new("correlated_rules.toml")).unwrap();
        assert_eq!(rules.correlated_rules.len(), 1);
        assert_eq!(
            rules.correlated_rules[0].name,
            "Potential Brute-Force Attack"
        );
    }
}
