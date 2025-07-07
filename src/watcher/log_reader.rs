use crate::analyzer::correlation_engine::CorrelationEngine;
use crate::analyzer::frequency_tracker::FrequencyTracker;
use crate::analyzer::pattern_matcher::PatternMatcher;
use crate::config::rules::{CorrelatedRule, FrequencyRules};
use crate::output::console::{create_detection, create_frequency_detection};
use crate::output::Detection;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};

pub async fn read_file_line_by_line(
    file_path: &Path,
    pattern_matcher: &PatternMatcher,
    output_format: &str,
    frequency_rules: &Option<FrequencyRules>,
    correlated_rules: &[CorrelatedRule],
) -> anyhow::Result<(Vec<Detection>, CorrelationEngine)> {
    let (_offset, _line_number, detections, correlation_engine) = read_file_from_offset(
        file_path,
        pattern_matcher,
        output_format,
        frequency_rules,
        correlated_rules,
        0,
        0,
    )
    .await?;
    Ok((detections, correlation_engine))
}

pub async fn read_file_from_offset(
    file_path: &Path,
    pattern_matcher: &PatternMatcher,
    output_format: &str,
    frequency_rules: &Option<FrequencyRules>,
    correlated_rules: &[CorrelatedRule],
    mut offset: u64,
    mut current_line_number: usize,
) -> anyhow::Result<(u64, usize, Vec<Detection>, CorrelationEngine)> {
    let mut file = File::open(file_path).await?;
    file.seek(SeekFrom::Start(offset)).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut json_output_file: Option<std::fs::File> = None;
    if output_format == "json" || output_format == "both" {
        let output_path = file_path.with_extension("json");
        json_output_file = Some(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&output_path)?,
        );
    }

    // Only print this message once when the file is first opened
    if offset == 0 && (output_format == "json" || output_format == "both") {
        println!(
            "Writing JSON output to: {}",
            file_path.with_extension("json").display()
        );
    }

    println!("Reading file: {}", file_path.display());

    let mut frequency_tracker = frequency_rules.as_ref().map(|rules| FrequencyTracker::new(
            rules.max_same_errors_per_minute,
            rules.time_window_seconds,
        ));

    let mut correlation_engine = CorrelationEngine::new(correlated_rules.to_vec());

    let mut detections: Vec<Detection> = Vec::new();

    while let Some(line) = lines.next_line().await? {
        current_line_number += 1;
        offset += (line.len() + 1) as u64; // +1 for newline character
        if let Some((severity, pattern_name)) = pattern_matcher.check_for_patterns(&line) {
            let detection = create_detection(
                severity,
                file_path,
                current_line_number,
                &line,
                pattern_name,
            );

            if output_format == "console" || output_format == "both" {
                detections.push(detection.clone());
            }

            if (output_format == "json" || output_format == "both") && json_output_file.is_some() {
                let json_detection = crate::output::json_writer::AnomalyDetection {
                    timestamp: chrono::Local::now().to_rfc3339(),
                    severity: severity.to_string(),
                    rule_name: pattern_name.to_string(),
                    file_path: file_path.to_string_lossy().to_string(),
                    line_number: current_line_number,
                    matched_line: line.clone(),
                    pattern: pattern_name.to_string(),
                };
                crate::output::json_writer::write_json_output(
                    &json_detection,
                    json_output_file.as_mut().unwrap(),
                )?;
            }

            // Frequency tracking
            if let Some(tracker) = &mut frequency_tracker {
                if let Some(count) = tracker.track_event(pattern_name) {
                    if output_format == "console" || output_format == "both" {
                        detections.push(create_frequency_detection(
                            pattern_name,
                            count,
                            frequency_rules.as_ref().unwrap().max_same_errors_per_minute,
                            frequency_rules.as_ref().unwrap().time_window_seconds,
                            file_path,
                            current_line_number,
                            &line,
                        ));
                    }
                    if (output_format == "json" || output_format == "both")
                        && json_output_file.is_some()
                    {
                        let json_detection = crate::output::json_writer::AnomalyDetection {
                            timestamp: chrono::Local::now().to_rfc3339(),
                            severity: "frequency".to_string(),
                            rule_name: format!("Too many {pattern_name} errors"),
                            file_path: file_path.to_string_lossy().to_string(),
                            line_number: current_line_number,
                            matched_line: line.clone(),
                            pattern: pattern_name.to_string(),
                        };
                        crate::output::json_writer::write_json_output(
                            &json_detection,
                            json_output_file.as_mut().unwrap(),
                        )?;
                    }
                }
            }

            // Correlation tracking
            if let Some(correlated_detection) = correlation_engine.add_detection(detection) {
                if output_format == "console" || output_format == "both" {
                    detections.push(correlated_detection);
                }
            }
        }
    }

    Ok((offset, current_line_number, detections, correlation_engine))
}

/// Find all .log files in a directory
pub fn find_log_files(dir_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut log_files = Vec::new();
    
    if !dir_path.exists() {
        return Err(anyhow::anyhow!("Directory does not exist: {}", dir_path.display()));
    }
    
    if !dir_path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", dir_path.display()));
    }
    
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "log" {
                    log_files.push(path);
                }
            }
        }
    }
    
    // Sort files for consistent ordering
    log_files.sort();
    Ok(log_files)
}
