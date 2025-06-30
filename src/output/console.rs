use colored::*;
use std::path::Path;

pub fn print_frequency_detection(
    pattern_name: &str,
    count: u32,
    limit: u32,
    time_window: u32,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
) {
    println!(
        "[{}] {} FREQ: Too many same errors\n  └─ Pattern: \"{}\"\n  └─ Count: {} in last {}s (limit: {})",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "⚠️".yellow().bold(),
        pattern_name,
        count,
        time_window,
        limit
    );
    println!(
        "  └─ File: {}:{} \n  └─ Line: \"{}\"",
        file_path.display(),
        line_number,
        matched_line
    );
}

pub fn print_detection(
    severity: &str,
    file_path: &Path,
    line_number: usize,
    matched_line: &str,
    pattern_name: &str,
) {
    let severity_colored = match severity {
        "ERROR" => "ERROR".red().bold(),
        "FATAL" => "FATAL".red().bold(),
        "Exception" => "Exception".red().bold(),
        "WARN" => "WARN".yellow().bold(),
        "WARNING" => "WARNING".yellow().bold(),
        "high" => "HIGH".red().bold(),
        "critical" => "CRITICAL".red().bold(),
        _ => severity.normal(),
    };

    println!(
        "[{}] {} detected\n  └─ File: {}:{} \n  └─ Pattern: {}\n  └─ Line: \"{}\"",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        severity_colored,
        file_path.display(),
        line_number,
        pattern_name,
        matched_line
    );
}
