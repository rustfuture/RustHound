use clap::Parser;
use std::path::PathBuf;

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
    #[clap(long, value_parser, default_value = "rules.toml")]
    rules: PathBuf,

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

    println!("Parsed arguments: {args:?}");

    let rules = config::rules::load_rules_from_file(&args.rules)?;
    println!("Loaded rules: {rules:?}");

    let pattern_matcher = analyzer::pattern_matcher::PatternMatcher::new(&rules)?;

    if let Some(file_path) = args.file {
        if args.follow {
            let (tx, mut rx) = tokio::sync::mpsc::channel(100);
            watcher::file_watcher::watch_file(file_path.clone(), tx).await?;

            let mut current_offset = 0;
            let mut current_line_number = 0;
            // Initial read of the file
            let (offset, line_number) = watcher::log_reader::read_file_from_offset(
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

            while (rx.recv().await).is_some() {
                // File modified, read new content
                let (offset, line_number) = watcher::log_reader::read_file_from_offset(
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
            }
        } else {
            watcher::log_reader::read_file_line_by_line(
                &file_path,
                &pattern_matcher,
                &args.output,
                &rules.frequency_rules,
            )
            .await?;
        }
    } else if let Some(dir_path) = args.dir {
        // TODO: Implement directory reading
        println!("Directory reading not yet implemented for: {dir_path:?}");
    } else if args.file.is_none() && args.dir.is_none() {
        let default_file_path = PathBuf::from("sample.log");
        println!("No --file or --dir argument provided. Defaulting to: {default_file_path:?}");
        watcher::log_reader::read_file_line_by_line(
            &default_file_path,
            &pattern_matcher,
            &args.output,
            &rules.frequency_rules,
        )
        .await?;
    }

    Ok(())
}
