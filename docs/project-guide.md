# Project guide

## What RustHound does

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

## Quick start options

Inspect the CLI options:

~~~bash
cargo run --locked -- --help
~~~

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

The default [rules.toml](../rules.toml) supports these sections:

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

Correlated rules can model a sequence such as repeated authentication failures followed by a successful login. [docs/rules-schema.md](rules-schema.md) is the authoritative schema, including the severity values and common configuration errors.

## Verification and contribution

Unit tests sit next to the code they cover: pattern matching, frequency tracking and correlation in [`src/analyzer/`](../src/analyzer/), rule deserialization in [`src/config/rules.rs`](../src/config/rules.rs), and JSON output in [`src/output/`](../src/output/).

Build requirements, CI check commands, manual smoke tests, and commit conventions are in
[CONTRIBUTING.md](../CONTRIBUTING.md).

## Versioning and support

RustHound follows `0.x` semantics: the version number represents project scope rather than API stability. Breaking changes bump the minor version, and backward-compatible fixes bump the patch version. Detailed release history is documented in [CHANGELOG.md](../CHANGELOG.md).

| Platform | Status |
| --- | --- |
| Linux | Verified by CI on Rust 1.85 (MSRV) and stable. |
| macOS | Verified locally against committed source. |
| Windows | Unverified. |

The minimum supported Rust version is 1.85; raising it is a minor-version change.
