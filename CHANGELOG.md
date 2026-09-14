# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## [0.1.2] - 2026-09-14

### Fixed

- `--output json` now writes one valid JSON array. A new scan replaces stale output, while follow
  mode appends new detections without producing concatenated top-level JSON objects.
- The crate and binary version now match the published release tag.

### Added

- `CONTRIBUTING.md`, `docs/architecture.md`, and `docs/rules-schema.md`. The module map, the
  follow-mode state rule, the pattern-matching priority, the rules schema, and the manual smoke
  commands previously lived in an agent-skill directory; they are now normal project documentation.

### Removed

- The `.cursor/` directory and `scripts/git-commit-as-owner.sh`. The skills were tooling
  configuration rather than project content, and the commit wrapper only existed to force an author
  identity that is now set in the global Git configuration. The rules schema and architecture notes
  were preserved in the documents listed above.

## [0.1.1] - 2026-09-14

### Fixed

- `setup.sh` was committed without the executable bit, so the documented `./setup.sh` failed with
  "Permission denied". The file mode is now 100755.
- `setup.sh` installed the default rules to `~/.config/rusthound`, but the application resolves the
  configuration directory with `dirs::config_dir()`, which is `~/Library/Application Support` on
  macOS. An installed rules file was therefore never found on macOS. The installer now uses the same
  platform-specific path the application does, and the README documents both.
- `setup.sh` overwrote an existing `rules.toml` on reinstall and `rm -rf`'d the whole configuration
  directory on uninstall, which could destroy rules the user had edited. Reinstalling now keeps the
  existing file and writes the bundled default beside it as `rules.toml.new`; uninstalling keeps the
  configuration unless the purge option is chosen.
- A failed rules-file copy printed "Installation complete" and exited 0. It now fails.
- The development-workflow skill required Rust 1.83 while the crate, README, and CI require 1.85.
- `.cursor/skills/README.md` claimed every skill in the directory was project-authored. One is
  adapted from community skills; that provenance is now recorded in the skill and in the index.

### Added

- An Installation section in the README covering `setup.sh` and the platform configuration paths.

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
