---
name: rusthound-cargo-tail
description: Monitor RustHound and Cargo command output in real time (build, test, clippy, rusthound --follow). Use when debugging compile errors, test failures, or live log analysis sessions.
---

# RustHound Cargo / Log Tail Workflow

Adapted for this repo from community tailing/monitoring skills. Prefer this over generic Node/webpack tail skills when working in RustHound.

## Commands to watch

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo run --bin rusthound -- -f sample.log
cargo run --bin rusthound -- -f sample.log --follow
cargo run --bin rusthound -- -f sample_freq.log -r correlated_rules.toml
```

Release smoke:

```bash
cargo build --release
./target/release/rusthound -f sample.log -s high
```

## Workflow

1. Run the command in a terminal (or tmux session).
2. Read terminal output as it streams; do not assume success without seeing `Finished` or test `ok`.
3. On compile error: fix the first error, rebuild — rustc errors are often cascading.
4. On test failure: run `cargo test <module> -- --nocapture` for the failing test only.
5. On `rusthound --follow`: append lines to the log file to verify `ScanState` (frequency/correlation) persists.

## RustHound-specific checks

- Follow mode must reuse `ScanState` in `main.rs`, not recreate engines per read.
- `--severity` filters in `display_detections`, not at match time.
- Rules must match [rusthound-rules-toml](../rusthound-rules-toml/SKILL.md) schema.

## When done

Invoke **prove-it**: all of `cargo test`, `cargo clippy -- -D warnings`, and a smoke `rusthound` run must pass.
