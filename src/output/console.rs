use colored::Colorize;
use std::path::Path;
use super::{Detection, Severity};

pub fn display_detections(detections: &mut Vec<Detection>) {
    detections.sort_by(|a, b| a.line_number.cmp(&b.line_number));

    for detection in detections {
        let colored_severity = match detection.severity {
            Severity::Critical => "CRITICAL".red().bold(),
            Severity::High => "HIGH".bright_red().bold(),
            Severity::Warning => "WARNING".yellow().bold(),
            Severity::Error => "ERROR".red().bold(),
            Severity::Info => "INFO".blue().bold(),
        };

        println!(
            "[{}] {} (Line: {}, File: {}): {}",
            colored_severity,
            detection.pattern_name.cyan(),
            detection.line_number.to_string().yellow(),
            detection.file_path.bright_magenta(),
            detection.matched_line
        );
    }
}

pub fn create_detection(
    severity: &str,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
    rule_name: &str,
) -> Detection {
    Detection {
        severity: Severity::from(severity),
        file_path: file_path.to_string_lossy().to_string(),
        line_number,
        matched_line: matched_line.to_string(),
        pattern_name: rule_name.to_string(),
    }
}

pub fn create_frequency_detection(
    pattern_name: &str,
    count: u32,
    max_same_errors_per_minute: u32,
    time_window_seconds: u32,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
) -> Detection {
    Detection {
        severity: Severity::Warning, // Frequency detections can be warnings or higher depending on threshold
        file_path: file_path.to_string_lossy().to_string(),
        line_number,
        matched_line: matched_line.to_string(),
        pattern_name: format!(
            "Too many \"{}\" errors ({} in {}s, threshold: {})",
            pattern_name, count, time_window_seconds, max_same_errors_per_minute
        ),
    }
}
