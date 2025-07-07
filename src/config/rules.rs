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
