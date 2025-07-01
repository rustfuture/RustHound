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
    #[clap(long, value_parser)]
    file: Option<PathBuf>,

    /// Path to a directory containing .log files
    #[clap(long, value_parser)]
    dir: Option<PathBuf>,

    /// Path to the rules file (default: rules.toml)
    #[clap(long, value_parser)]
    rules: Option<PathBuf>,

    /// Output format: console, json, both
    #[clap(long, value_parser, default_value = "console")]
    output: String,

    /// Enable tail -f mode
    #[clap(long)]
    follow: bool,

    /// Enable debug output
    #[clap(long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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
            let (offset, line_number, mut detections) = watcher::log_reader::read_file_from_offset(
                &file_path,
                &pattern_matcher,
                &args.output,
                &rules.frequency_rules,
                current_offset,
                current_line_number,
            )
            .await?;
            current_offset = offset;
            current_line_number = line_number;

            output::console::display_detections(&mut detections);

            while (rx.recv().await).is_some() {
                // File modified, read new content
                let (offset, line_number, mut new_detections) = watcher::log_reader::read_file_from_offset(
                    &file_path,
                    &pattern_matcher,
                    &args.output,
                    &rules.frequency_rules,
                    current_offset,
                    current_line_number,
                )
                .await?;
                current_offset = offset;
                current_line_number = line_number;
                output::console::display_detections(&mut new_detections);
            }
        } else {
            let mut detections = watcher::log_reader::read_file_line_by_line(
                &file_path,
                &pattern_matcher,
                &args.output,
                &rules.frequency_rules,
            )
            .await?;
            output::console::display_detections(&mut detections);
        }
    } else if let Some(dir_path) = args.dir {
        // TODO: Implement directory reading
        println!("Directory reading not yet implemented for: {dir_path:?}");
    } else if args.file.is_none() && args.dir.is_none() {
        let default_file_path = PathBuf::from("sample.log");
        let mut detections = watcher::log_reader::read_file_line_by_line(
            &default_file_path,
            &pattern_matcher,
            &args.output,
            &rules.frequency_rules,
        )
        .await?;
        output::console::display_detections(&mut detections);
    }

    Ok(())
}
