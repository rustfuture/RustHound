# RustHound

[![CI](https://github.com/rustfuture/RustHound/actions/workflows/ci.yml/badge.svg)](https://github.com/rustfuture/RustHound/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![License: Apache--2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

RustHound is a Rust command-line log analyzer. It reads a log file, applies configured string and regular-expression rules, and can add frequency and correlation detections to console or JSON output.

This is a pre-1.0 portfolio project. The README describes the behavior verified in this repository; it does not claim universal platform support or benchmark numbers. See [CHANGELOG.md](CHANGELOG.md) for the change history.

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
- The repository publishes no `cargo install` package; the source build, or the tagged release's source archive, is the supported installation path.
- `.cursor/skills/` and `scripts/git-commit-as-owner.sh` are repository development tooling and are not part of the analyzer runtime.

## Architecture

- `src/analyzer/` — pattern matching, frequency tracking, and correlation state.
- `src/config/` — TOML schema and rule loading.
- `src/watcher/` — file reading, offsets, and follow-mode notifications.
- `src/output/` — detection types, console rendering, and JSON writing.

## Versioning and support

RustHound follows `0.x` semantics: the version number is a statement about scope, not a compatibility
promise. While the major version is 0, a breaking change to the CLI, the rules-file schema, or the
JSON output shape bumps the minor version, and a compatible fix bumps the patch version. Every change
is recorded in [CHANGELOG.md](CHANGELOG.md).

| Platform | Status |
| --- | --- |
| Linux | Verified by CI on Rust 1.85 (the minimum supported version) and stable. |
| macOS | Verified locally against the committed source; not part of the CI matrix. |
| Windows | Not verified. |

The minimum supported Rust version is 1.85; raising it is a minor-version change. A `1.0` would mean
the existing command surface, rules schema, and JSON output have stopped moving, not that every idea
in the issue tracker has been implemented.

## License and attribution

Apache-2.0. See [LICENSE](LICENSE).
