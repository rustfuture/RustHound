# RustHound

RustHound is a command-line log analyzer for developers and system administrators to inspect log streams and detect operational anomalies using configurable pattern, frequency, and correlation rules.

[![CI](https://github.com/rustfuture/RustHound/actions/workflows/ci.yml/badge.svg)](https://github.com/rustfuture/RustHound/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**Status:** Maintained CLI (experimental, pre-1.0 portfolio project).

- Streaming line-by-line log analysis for single files or directories of `.log` files.
- TOML-based rule matching for exact substrings and regular expressions.
- Time-windowed frequency tracking to detect recurring errors exceeding defined thresholds.
- Multi-event correlation rules to alert on sequence patterns (such as repeated authentication failures followed by a login).
- Console, JSON array, and combined output modes, with minimum-severity filtering.
- State-preserving follow mode (`--follow`) monitoring newly appended log lines.

## Requirements and build

- Rust 1.85 or newer (edition 2021).
- A log file and a TOML rules file for analysis.

~~~bash
cargo build --locked --release
~~~

## Quick start

Inspect the CLI options:

~~~bash
cargo run --locked -- --help
~~~

Analyze the bundled [sample.log](sample.log) using [rules.toml](rules.toml):

~~~bash
cargo run --locked -- \
  --file sample.log \
  --rules rules.toml \
  --output console
~~~

The sample run emits eight detections with severity and source-line context.

Run with JSON output or combined mode:

~~~bash
cargo run --locked -- --file sample.log --rules rules.toml --output json
cargo run --locked -- --file sample.log --rules rules.toml --output both
~~~

Filter detections by minimum severity:

~~~bash
cargo run --locked -- --file sample.log --severity high
~~~

Generate a default starter rules file:

~~~bash
cargo run --locked -- --init-config
~~~

## Installation

`setup.sh` builds from the committed lockfile, installs the binary to `~/.local/bin`, and installs the
bundled `rules.toml` as the default:

~~~bash
./setup.sh
~~~

When `--rules` is not given, the binary looks for `rules.toml` in the platform configuration
directory and falls back to the working directory. `setup.sh` writes to that same directory:

| Platform | Configuration path |
| --- | --- |
| macOS | `~/Library/Application Support/rusthound/rules.toml` |
| Linux | `$XDG_CONFIG_HOME/rusthound/rules.toml` (default `~/.config/rusthound/rules.toml`) |

Re-running the installer never overwrites rules you have edited: the bundled default is written
beside them as `rules.toml.new`. Uninstalling keeps your configuration unless you choose the purge
option.

## Rule configuration

The default [rules.toml](rules.toml) supports these sections:

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

Correlated rules can model a sequence such as repeated authentication failures followed by a successful login. [docs/rules-schema.md](docs/rules-schema.md) is the authoritative schema, including the severity values and common configuration errors.

## Verification

Run the test suite and quality checks:

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
~~~

Unit tests sit next to the code they cover: pattern matching, frequency tracking and correlation in [`src/analyzer/`](src/analyzer/), rule deserialization in [`src/config/rules.rs`](src/config/rules.rs), and JSON output in [`src/output/`](src/output/).

## Scope and limitations

- Streaming processing: Processes log lines sequentially; no throughput or memory benchmarks are claimed without a dedicated benchmark environment.
- Single-host monitoring: Follow mode is a local file watcher using the `notify` crate, not a distributed log aggregation service.
- Platform support: Verified on macOS (local development) and Linux (CI matrix). Windows is currently unverified.
- Packaging: No package is published to crates.io; source builds from the repository are the supported installation path.

## Architecture

- `src/analyzer/` — pattern matching, frequency tracking, and correlation state.
- `src/config/` — TOML schema and rule loading.
- `src/watcher/` — file reading, offsets, and follow-mode notifications.
- `src/output/` — detection types, console rendering, and JSON writing.

[docs/architecture.md](docs/architecture.md) covers the module map, data flow, pattern-matching priority, and the requirement that follow mode reuse a single `ScanState` instance across reads.

## Contributing

Build requirements, CI check commands, manual smoke tests, and commit conventions are in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Versioning and support

RustHound follows `0.x` semantics: the version number represents project scope rather than API stability. Breaking changes bump the minor version, and backward-compatible fixes bump the patch version. Detailed release history is documented in [CHANGELOG.md](CHANGELOG.md).

| Platform | Status |
| --- | --- |
| Linux | Verified by CI on Rust 1.85 (MSRV) and stable. |
| macOS | Verified locally against committed source. |
| Windows | Unverified. |

The minimum supported Rust version is 1.85; raising it is a minor-version change.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
