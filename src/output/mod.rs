// src/output/mod.rs
pub mod console;
pub mod json_writer;

#[derive(Debug, Clone)]
pub struct Detection {
    pub severity: Severity,
    pub file_path: String,
    pub line_number: usize,
    pub pattern_name: String,
    pub matched_line: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Severity {
    Critical,
    High,
    Warning,
    Error,
    Info,
}

impl From<&str> for Severity {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "warn" | "warning" => Severity::Warning,
            "error" | "fatal" | "exception" => Severity::Error,
            _ => Severity::Info,
        }
    }
}