# Contributing

RustHound is a small, single-maintainer project. This file records how to build it, how to check a
change before committing, and where the design notes live.

## Requirements

- Rust 1.85 or newer — the minimum supported version, enforced by CI
- Edition 2021

## Before you commit

Run the same checks CI runs, from the repository root:

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
~~~

## Manual checks

The sample files in the repository root make a real end-to-end run possible:

~~~bash
cargo run --locked -- --file sample.log --rules rules.toml --output console
cargo run --locked -- --file sample_freq.log --rules correlated_rules.toml
cargo run --locked -- --file sample.log --severity high
cargo run --locked -- --init-config
~~~

The first command prints eight detections with severity and source-line context. `--severity` filters
the output, not the matching: detections below the threshold are still counted for frequency and
correlation rules.

`--follow` watches appended content. To confirm that frequency and correlation state survives an
append, run it against a file and then append a matching line:

~~~bash
cargo run --locked -- --file sample.log --follow
~~~

On a compile error, fix the first one and rebuild — rustc errors are usually cascading. On a test
failure, narrow it down before changing anything:

~~~bash
cargo test --locked <module> -- --nocapture
~~~

## Installing locally

~~~bash
./setup.sh
~~~

Installs the release binary to `~/.local/bin` and the bundled `rules.toml` into the platform
configuration directory the binary resolves. Re-running it never overwrites rules you have edited.

## Commit conventions

Commits are authored by the repository owner. Do not add `Co-authored-by:` trailers.

## Scope

The repository ships no `cargo install` package; the source build is the supported installation path.

## Design notes

- [docs/architecture.md](docs/architecture.md) — module map, data flow, and the follow-mode state rule.
- [docs/rules-schema.md](docs/rules-schema.md) — the authoritative `rules.toml` schema.
