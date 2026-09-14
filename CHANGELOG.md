# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added

- Streaming line-by-line analysis for a single file or a directory of `.log` files.
- TOML rules for string patterns, regular expressions, frequency thresholds, and correlated events.
- Console, JSON, and combined output modes.
- Optional follow mode for newly appended log lines.
- Minimum-severity filtering and a default configuration generator (`--init-config`).

### Changed

- The generated default rules template and the remaining development documentation are now English.

### Removed

- The vendored third-party agent-skill bundles under `docs/agent-skills/` and their installer. They
  were development aids unrelated to the analyzer runtime and accounted for 230 of the repository's
  tracked files.

## Scope and limitations

This is a pre-1.0 prototype. It does not claim universal platform support, published release
artifacts, or benchmark numbers. Passing tests are scenario evidence for the covered rules,
frequency tracking, correlation, and configuration parsing.
