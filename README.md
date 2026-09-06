# RustHound

RustHound is a Rust command-line log analyzer. It reads a log file, applies configured string and regular-expression rules, and can add frequency and correlation detections to console or JSON output.

This is a pre-1.0 portfolio project. The README describes the behavior verified in this repository; it does not claim universal platform support, release artifacts, or benchmark numbers.

## Implemented behavior

- Streaming line-by-line analysis for one file or a directory of `.log` files.
- TOML rules for string patterns, regex patterns, frequency thresholds, and correlated events.
- Console, JSON, and combined output modes.
- Optional follow mode for newly appended log lines.
- Minimum-severity filtering and a default configuration generator.
- Analyzer unit tests for rule precedence, frequency tracking, correlation, and TOML parsing.

## Requirements and build

- Rust 1.85 or newer (edition 2021).
- A log file and a TOML rules file for a meaningful run.

~~~bash
git clone https://github.com/rustfuture/RustHound.git
cd RustHound
cargo build --locked --release
~~~

## Quick start

The repository includes a small sample log and rules file:

~~~bash
cargo run --locked -- \
  --file sample.log \
  --rules rules.toml \
  --output console
~~~

The verified sample run emits eight detections with severity and source-line context. Generate a starter rules file with:

~~~bash
cargo run --locked -- --init-config
~~~

For JSON output, use `--output json`; for both console and JSON, use `--output both`. `--follow` monitors appended content. `--dir PATH` scans regular `.log` files in a directory.

## Rule configuration

The default `rules.toml` supports these sections:

~~~toml
[rules]
error_patterns = ["ERROR", "FATAL", "Exception"]
warning_patterns = ["WARN", "WARNING"]

[[regex_rules]]
name = "authentication_failure"
pattern = "authentication failure|Failed password for"
severity = "high"

[frequency_rules]
max_same_errors_per_minute = 10
time_window_seconds = 60
~~~

Correlated rules can model a sequence such as repeated authentication failures followed by a successful login. The checked-in examples are the source of truth for the accepted TOML schema.

## Verification

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo +1.85.0 check --locked --all-targets
~~~

The current local run passes 12 library tests, no duplicate binary test suite, and the doctest target. The sample CLI invocation above is a real file-processing smoke test, not a benchmark.

## Scope and limitations

- The normal path processes files in a streaming manner; no throughput or memory number is published without a controlled benchmark environment.
- Follow mode is a local file watcher, not a distributed ingestion service.
- Cross-platform behavior beyond the tested macOS environment and CI’s Linux environment requires separate validation.
- The repository has no release artifacts or `cargo install` package published by this portfolio milestone; the source build is the supported installation path.
- `scripts/install-cursor-skills.sh` is development tooling and is not part of the analyzer runtime.

## Architecture

- `src/analyzer/` — pattern matching, frequency tracking, and correlation state.
- `src/config/` — TOML schema and rule loading.
- `src/watcher/` — file reading, offsets, and follow-mode notifications.
- `src/output/` — detection types, console rendering, and JSON writing.

## License and attribution

Apache-2.0. See [LICENSE](LICENSE). Third-party development helpers are not presented as product functionality.
