---
name: rusthound-dev-workflow
description: Build, test, and validation workflow for RustHound. Use before committing changes.
---

# RustHound Development Workflow

## Requirements

- Rust **1.83+** stable
- Edition **2021** in Cargo.toml

## Validation commands

Run from repository root:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## Smoke tests

```bash
./target/release/rusthound -f sample.log
./target/release/rusthound -f sample_freq.log -r correlated_rules.toml
./target/release/rusthound --init-config
```

## Severity filter example

```bash
./target/release/rusthound -f sample.log -s high
```

Only detections at or above the given severity are printed.

## Follow mode

```bash
./target/release/rusthound -f sample.log --follow
```

Verify correlation/frequency state persists across file appends (use `ScanState` in main).

## Install script

```bash
./setup.sh
```

Copies `target/release/rusthound` to `~/.local/bin/` and `rules.toml` to `~/.config/rusthound/`.
