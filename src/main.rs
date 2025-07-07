use clap::Parser;
use std::path::PathBuf;
use anyhow::Context;

mod analyzer;
mod config;
mod output;
mod watcher;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a single log file (default: sample.log)
    #[clap(long, short = 'f', value_parser)]
    file: Option<PathBuf>,

    /// Path to a directory containing .log files
    #[clap(long, short = 'd', value_parser)]
    dir: Option<PathBuf>,

    /// Path to the rules file (default: rules.toml)
    #[clap(long, short = 'r', value_parser)]
    rules: Option<PathBuf>,

    /// Output format: console, json, both
    #[clap(long, short = 'o', value_parser, default_value = "console")]
    output: String,

    /// Enable tail -f mode (real-time monitoring)
    #[clap(long, short = 'F')]
    follow: bool,

    /// Enable debug output
    #[clap(long, short = 'v')]
    verbose: bool,

    /// Generate default configuration file
    #[clap(long)]
    init_config: bool,

    /// Show only specific severity levels (critical,high,warning,error,info)
    #[clap(long, short = 's', value_parser)]
    severity: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Handle init-config command
    if args.init_config {
        return create_default_config();
    }

    // Enable verbose logging if requested
    if args.verbose {
        println!("Debug mode enabled");
        println!("Arguments: {:#?}", args);
    }

    let rules_path = if let Some(path) = args.rules {
        path
    } else if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("rusthound").join("rules.toml");
        if config_path.exists() {
            config_path
        } else {
            PathBuf::from("rules.toml")
        }
    } else {
        PathBuf::from("rules.toml")
    };

    let rules = config::rules::load_rules_from_file(&rules_path)
        .with_context(|| format!("Failed to load rules from {:?}", rules_path))?;

    let pattern_matcher = analyzer::pattern_matcher::PatternMatcher::new(&rules)?;

    if let Some(file_path) = args.file {
        if args.follow {
            let (tx, mut rx) = tokio::sync::mpsc::channel(100);
            watcher::file_watcher::watch_file(file_path.clone(), tx).await?;

            let mut current_offset = 0;
            let mut current_line_number = 0;
            // Initial read of the file
            let (offset, line_number, mut detections, _correlation_engine) = watcher::log_reader::read_file_from_offset(
                &file_path,
                &pattern_matcher,
                &args.output,
                &rules.frequency_rules,
                &rules.correlated_rules,
                current_offset,
                current_line_number,
            )
            .await?;
            current_offset = offset;
            current_line_number = line_number;

            output::console::display_detections(&mut detections);

            while (rx.recv().await).is_some() {
                // File modified, read new content
                let (offset, line_number, mut new_detections, _updated_correlation_engine) = watcher::log_reader::read_file_from_offset(
                    &file_path,
                    &pattern_matcher,
                    &args.output,
                    &rules.frequency_rules,
                    &rules.correlated_rules,
                    current_offset,
                    current_line_number,
                )
                .await?;
                current_offset = offset;
                current_line_number = line_number;
                output::console::display_detections(&mut new_detections);
            }
        } else {
            let (mut detections, _correlation_engine) = watcher::log_reader::read_file_line_by_line(
                &file_path,
                &pattern_matcher,
                &args.output,
                &rules.frequency_rules,
                &rules.correlated_rules,
            )
            .await?;
            output::console::display_detections(&mut detections);
        }
    } else if let Some(dir_path) = args.dir {
        let log_files = watcher::log_reader::find_log_files(&dir_path)?;
        if log_files.is_empty() {
            println!("No .log files found in directory: {dir_path:?}");
            return Ok(());
        }
        
        if args.verbose {
            println!("Found {} log files in directory: {dir_path:?}", log_files.len());
            for file_path in &log_files {
                println!("  - {}", file_path.display());
            }
        }
        
        if args.follow {
            println!("Monitoring {} log files in real-time...", log_files.len());
            // Watch multiple files
            let (tx, mut rx) = tokio::sync::mpsc::channel(100);
            
            // Start watching all files
            for file_path in &log_files {
                let tx_clone = tx.clone();
                let file_path_clone = file_path.clone();
                tokio::spawn(async move {
                    if let Err(e) = watcher::file_watcher::watch_file(file_path_clone, tx_clone).await {
                        eprintln!("Error watching file: {e}");
                    }
                });
            }
            
            // Track offsets for each file and correlation engine state
            let mut file_states: std::collections::HashMap<std::path::PathBuf, (u64, usize, analyzer::correlation_engine::CorrelationEngine)> = 
                std::collections::HashMap::new();

            // Initial read of all files
            for file_path in &log_files {
                let (offset, line_number, mut detections, correlation_engine) = watcher::log_reader::read_file_from_offset(
                    file_path,
                    &pattern_matcher,
                    &args.output,
                    &rules.frequency_rules,
                    &rules.correlated_rules,
                    0,
                    0,
                ).await?;
                file_states.insert(file_path.clone(), (offset, line_number, correlation_engine));
                output::console::display_detections(&mut detections);
            }
            
            // Listen for file changes
            while let Some(changed_file) = rx.recv().await {
                if let Some((current_offset, current_line_number, _correlation_engine)) = file_states.remove(&changed_file) {
                    let (new_offset, new_line_number, mut new_detections, updated_correlation_engine) = watcher::log_reader::read_file_from_offset(
                        &changed_file,
                        &pattern_matcher,
                        &args.output,
                        &rules.frequency_rules,
                        &rules.correlated_rules,
                        current_offset,
                        current_line_number,
                    ).await?;
                    file_states.insert(changed_file, (new_offset, new_line_number, updated_correlation_engine));
                    output::console::display_detections(&mut new_detections);
                }
            }
        } else {
            // Read all files once
            let mut all_detections = Vec::new();
            let _correlation_engine = analyzer::correlation_engine::CorrelationEngine::new(rules.correlated_rules.clone());

            for file_path in &log_files {
                if args.verbose {
                    println!("Processing file: {}", file_path.display());
                }
                let (mut detections, _updated_correlation_engine) = watcher::log_reader::read_file_line_by_line(
                    file_path,
                    &pattern_matcher,
                    &args.output,
                    &rules.frequency_rules,
                    &rules.correlated_rules,
                ).await?;
                all_detections.append(&mut detections);
            }
            output::console::display_detections(&mut all_detections);
        }
    } else if args.file.is_none() && args.dir.is_none() {
        let default_file_path = PathBuf::from("sample.log");
        let (mut detections, _) = watcher::log_reader::read_file_line_by_line(
            &default_file_path,
            &pattern_matcher,
            &args.output,
            &rules.frequency_rules,
            &rules.correlated_rules,
        )
        .await?;
        output::console::display_detections(&mut detections);
    }

    Ok(())
}

/// Create default configuration file
fn create_default_config() -> anyhow::Result<()> {
    let config_content = r#"[rules]
error_patterns = ["ERROR", "FATAL", "Exception"]
warning_patterns = ["WARN", "WARNING"]

[[regex_rules]]
name = "oom_kill"
pattern = "Out of memory: Kill process|Memory.*exceeded"
severity = "critical"

[[regex_rules]]
name = "cpu_soft_lockup"
pattern = "soft lockup - CPU.*stuck for"
severity = "critical"

[[regex_rules]]
name = "kernel_panic"
pattern = "kernel: (?:general protection fault|segfault at|Call Trace:|NOHZ: local_softirq_pending)"
severity = "critical"

[[regex_rules]]
name = "service_start_failure"
pattern = "Failed to start|Starting .* failed|Unit .* entered failed state|Dependency failed for"
severity = "high"

[[regex_rules]]
name = "filesystem_mount_failure"
pattern = "Failed to mount|Read-only file system|No space left on device|Inode limit reached"
severity = "high"

[[regex_rules]]
name = "authentication_failure"
pattern = "authentication failure|Failed password for|Invalid user|Too many authentication failures"
severity = "high"

[[regex_rules]]
name = "network_issue"
pattern = "Network is unreachable|Name or service not known|No route to host|Connection refused|Interface .* is down"
severity = "high"

[[regex_rules]]
name = "disk_io_error"
pattern = "Disk I/O error"
severity = "critical"

[[regex_rules]]
name = "sql_error"
pattern = "SQL.*Error|Database.*failed"
severity = "high"

[frequency_rules]
max_same_errors_per_minute = 10
time_window_seconds = 60

[[correlated_rules]]
name = "Potential Brute-Force Attack"
severity = "critical"
description = "Çok sayıda başarısız giriş denemesinin ardından başarılı bir giriş tespit edildi."
time_window_seconds = 60
followed_by = "Successful Login"

[correlated_rules.trigger_on_rule]
name = "authentication_failure"
count = 10
"#;

    let config_path = "rules.toml";
    if std::path::Path::new(config_path).exists() {
        println!("Configuration file already exists: {}", config_path);
        print!("Overwrite? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Configuration file creation cancelled.");
            return Ok(());
        }
    }

    std::fs::write(config_path, config_content)?;
    println!("Default configuration file created: {}", config_path);
    println!("You can now edit this file to customize your log analysis rules.");
    
    Ok(())
}
