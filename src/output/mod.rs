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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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

impl Severity {
    pub fn rank(&self) -> u8 {
        match self {
            Severity::Critical => 5,
            Severity::High => 4,
            Severity::Error => 3,
            Severity::Warning => 2,
            Severity::Info => 1,
        }
    }

    pub fn meets_minimum(&self, minimum: &Severity) -> bool {
        self.rank() >= minimum.rank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_meets_minimum() {
        assert!(Severity::Critical.meets_minimum(&Severity::High));
        assert!(Severity::High.meets_minimum(&Severity::High));
        assert!(!Severity::Warning.meets_minimum(&Severity::High));
        assert!(!Severity::Info.meets_minimum(&Severity::Error));
    }
}
