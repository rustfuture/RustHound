use serde::Serialize;
use std::fs::File;
use std::io::{self, Write};

#[derive(Serialize, Debug)]
pub struct AnomalyDetection {
    pub timestamp: String,
    pub severity: String,
    pub rule_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub matched_line: String,
    pub pattern: String,
}

pub fn write_json_output(detection: &AnomalyDetection, output_file: &mut File) -> io::Result<()> {
    let json_string = serde_json::to_string_pretty(detection)?;
    writeln!(output_file, "{}", json_string)?;
    Ok(())
}
