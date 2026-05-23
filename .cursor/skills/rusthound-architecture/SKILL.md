---
name: rusthound-architecture
description: RustHound module layout, data flow, and stateful follow-mode rules. Use when changing src layout, log reading, or analyzer integration.
---

# RustHound Architecture

## Module map

```
src/main.rs          CLI entry, orchestration
src/config/rules.rs  TOML loading (Rules, RegexRule, FrequencyRules, CorrelatedRule)
src/watcher/
  log_reader.rs      Line/offset reading, detection pipeline
  file_watcher.rs    notify-based file watch
src/analyzer/
  pattern_matcher.rs String + regex matching
  frequency_tracker.rs Time-window event counts
  correlation_engine.rs Multi-event correlation rules
src/output/
  mod.rs             Detection, Severity
  console.rs         Colored console output
  json_writer.rs     JSON file output
```

## Data flow

1. Load `rules.toml` → `Rules`
2. Build `PatternMatcher` (compile regex at startup)
3. Create `ScanState` (frequency tracker + correlation engine) once per file/session
4. `read_file_from_offset` reads new lines, pushes detections
5. Filter by `--severity`, then `display_detections`

## Stateful follow mode (critical)

`FrequencyTracker` and `CorrelationEngine` **must persist** across incremental reads in `--follow` mode.

- Create `ScanState` once in `main.rs` before the follow loop
- Pass `&mut ScanState` into every `read_file_from_offset` call
- Multi-file follow: store `ScanState` per file in `file_states` HashMap

Do **not** recreate engines inside `read_file_from_offset` on each call.

## Pattern matching priority

In `pattern_matcher.rs`, first match wins:

1. `rules.error_patterns` (substring)
2. `rules.warning_patterns` (substring)
3. `regex_rules` (compiled regex, in file order)

## Binary naming

- Cargo package: `rust_hound`
- User-facing binary: `rusthound` (`[[bin]]` in Cargo.toml)
- Config dir: `~/.config/rusthound/rules.toml`
