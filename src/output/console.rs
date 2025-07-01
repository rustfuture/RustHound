use colored::*;
use std::collections::HashMap;
use std::path::Path;
use super::{Detection, Severity};

pub fn create_frequency_detection(
    pattern_name: &str,
    count: u32,
    limit: u32,
    time_window: u32,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
) -> Detection {
    let pattern_info = format!(
        "FREQUENCY_ALERT: {} (Count: {}, Limit: {}, Time: {}s)",
        pattern_name, count, limit, time_window
    );
    Detection {
        severity: Severity::Warning,
        file_path: file_path.display().to_string(),
        line_number,
        pattern_name: pattern_info,
        matched_line: matched_line.to_string(),
    }
}

pub fn create_detection(
    severity: &str,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
    pattern_name: &str,
) -> Detection {
    Detection {
        severity: Severity::from(severity),
        file_path: file_path.display().to_string(),
        line_number,
        pattern_name: pattern_name.to_string(),
        matched_line: matched_line.to_string(),
    }
}

pub fn display_detections(detections: &mut Vec<Detection>) {
    if detections.is_empty() {
        println!("No issues detected.");
        return;
    }

    detections.sort_by(|a, b| a.severity.cmp(&b.severity));

    println!("\n{}", "═".repeat(80).blue().bold());
    println!("{:^80}", "RUSTHOUND ANALYSIS REPORT".blue().bold());
    println!("{}", "═".repeat(80).blue().bold());
    println!(); // Add an empty line for spacing

    let mut summary_counts: HashMap<Severity, usize> = HashMap::new();

    for detection in detections {
        *summary_counts.entry(detection.severity.clone()).or_insert(0) += 1;

        let (severity_colored, symbol) = match detection.severity {
            Severity::Critical => ("CRITICAL".red().bold(), "🚨"),
            Severity::High => ("HIGH".red().bold(), "🔥"),
            Severity::Warning => ("WARNING".yellow().bold(), "⚠️"),
            Severity::Error => ("ERROR".red().bold(), "❌"),
            Severity::Info => ("INFO".normal(), "ℹ️"),
        };

        println!(
            "{} {:<8} [{}:{}] {}: {}", // Adjusted spacing
            symbol,
            severity_colored,
            detection.file_path,
            detection.line_number,
            detection.pattern_name,
            detection.matched_line
        );
    }

    println!(); // Add an empty line for spacing
    println!("{}", "═".repeat(80).blue().bold());
    println!("{:^80}", "SUMMARY".blue().bold());
    println!("{}", "═".repeat(80).blue().bold());
    println!(); // Add an empty line for spacing

    let mut sorted_summary: Vec<(&Severity, &usize)> = summary_counts.iter().collect();
    sorted_summary.sort_by(|(a_sev, _), (b_sev, _)| a_sev.cmp(b_sev));

    for (severity, count) in sorted_summary {
        let severity_colored = match severity {
            Severity::Critical => "CRITICAL".red().bold(),
            Severity::High => "HIGH".red().bold(),
            Severity::Warning => "WARNING".yellow().bold(),
            Severity::Error => "ERROR".red().bold(),
            Severity::Info => "INFO".normal(),
        };
        println!("  {:<10}: {}", severity_colored, count); // Adjusted spacing
    }

    println!(); // Add an empty line for spacing
    println!("{}", "═".repeat(80).blue().bold());
    println!("{:^80}", "END OF REPORT".blue().bold());
    println!("{}", "═".repeat(80).blue().bold());
}