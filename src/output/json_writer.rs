use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

use super::Detection;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnomalyDetection {
    pub timestamp: String,
    pub severity: String,
    pub rule_name: String,
    pub file_path: String,
    pub line_number: usize,
    pub matched_line: String,
    pub pattern: String,
}

impl AnomalyDetection {
    pub fn from_detection(detection: &Detection) -> Self {
        Self {
            timestamp: chrono::Local::now().to_rfc3339(),
            severity: detection.severity.as_str().to_owned(),
            rule_name: detection.pattern_name.clone(),
            file_path: detection.file_path.clone(),
            line_number: detection.line_number,
            matched_line: detection.matched_line.clone(),
            pattern: detection.pattern_name.clone(),
        }
    }
}

pub fn write_json_output(
    output_path: &Path,
    detections: Vec<AnomalyDetection>,
    append: bool,
) -> io::Result<()> {
    let mut output = if append && output_path.exists() {
        let bytes = std::fs::read(output_path)?;
        serde_json::from_slice::<Vec<AnomalyDetection>>(&bytes)?
    } else {
        Vec::new()
    };
    output.extend(detections);

    let mut bytes = serde_json::to_vec_pretty(&output)?;
    bytes.push(b'\n');
    std::fs::write(output_path, bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn detection(rule_name: &str) -> AnomalyDetection {
        AnomalyDetection {
            timestamp: "2026-09-14T00:00:00Z".to_owned(),
            severity: "warning".to_owned(),
            rule_name: rule_name.to_owned(),
            file_path: "sample.log".to_owned(),
            line_number: 1,
            matched_line: "WARN sample".to_owned(),
            pattern: rule_name.to_owned(),
        }
    }

    fn output_path() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rusthound-json-writer-{}-{nonce}.json",
            std::process::id()
        ))
    }

    #[test]
    fn writes_one_valid_json_array() {
        let path = output_path();
        write_json_output(&path, vec![detection("first"), detection("second")], false)
            .expect("JSON output should be written");

        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).expect("JSON output should be readable"))
                .expect("output should be one valid JSON document");
        assert_eq!(value.as_array().map(Vec::len), Some(2));

        std::fs::remove_file(path).expect("temporary output should be removable");
    }

    #[test]
    fn converts_console_detections_without_losing_severity() {
        let source = Detection {
            severity: super::super::Severity::Critical,
            file_path: "auth.log".to_owned(),
            line_number: 42,
            pattern_name: "Potential Brute-Force Attack".to_owned(),
            matched_line: "login accepted".to_owned(),
        };

        let output = AnomalyDetection::from_detection(&source);
        assert_eq!(output.severity, "critical");
        assert_eq!(output.rule_name, source.pattern_name);
        assert_eq!(output.line_number, 42);
    }

    #[test]
    fn replaces_on_new_scan_and_appends_in_follow_mode() {
        let path = output_path();
        write_json_output(&path, vec![detection("old")], false)
            .expect("initial JSON output should be written");
        write_json_output(&path, vec![detection("replacement")], false)
            .expect("a new scan should replace stale output");
        write_json_output(&path, vec![detection("follow")], true)
            .expect("follow mode should append to the existing array");

        let output: Vec<AnomalyDetection> =
            serde_json::from_slice(&std::fs::read(&path).expect("JSON output should be readable"))
                .expect("output should remain a valid JSON array");
        assert_eq!(output.len(), 2);
        assert_eq!(output[0].rule_name, "replacement");
        assert_eq!(output[1].rule_name, "follow");

        std::fs::remove_file(path).expect("temporary output should be removable");
    }
}
