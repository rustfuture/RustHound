# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## [0.1.0] - 2026-09-14

### Added

- Streaming line-by-line analysis for a single file or a directory of `.log` files.
- TOML rules for string patterns, regular expressions, frequency thresholds, and correlated events.
- Console, JSON, and combined output modes.
- Optional follow mode for newly appended log lines.
- Minimum-severity filtering and a default configuration generator (`--init-config`).

### Changed

- The generated default rules template and the remaining development documentation are now English.
- The README now carries a CI badge and a support matrix that states which platforms are verified.
- CHANGELOG.md was added and is linked from the README.

### Removed

- The vendored third-party agent-skill bundles under `docs/agent-skills/` and their installer. They
  were development aids unrelated to the analyzer runtime and accounted for 230 of the repository's
  tracked files.

### Fixed

- `.gitignore` had lost a newline, merging `makefile.*` with `.cursor/skills-cache/` into a single
  pattern, so `makefile.*` was not ignored. The stray analyzer log that had been committed under
  `logs/` was untracked.

## Scope and limitations

This is a pre-1.0 prototype. It does not claim universal platform support or benchmark numbers.
Passing tests are scenario evidence for the covered rules, frequency tracking, correlation, and
configuration parsing.
