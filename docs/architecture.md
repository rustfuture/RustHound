# Architecture

## Module map

~~~text
src/main.rs             CLI entry, orchestration
src/config/rules.rs     TOML loading (Rules, RegexRule, FrequencyRules, CorrelatedRule)
src/watcher/
  log_reader.rs         Line/offset reading, detection pipeline
  file_watcher.rs       notify-based file watch
src/analyzer/
  pattern_matcher.rs    String + regex matching
  frequency_tracker.rs  Time-window event counts
  correlation_engine.rs Multi-event correlation rules
src/output/
  mod.rs                Detection, Severity
  console.rs            Colored console output
  json_writer.rs        JSON file output
~~~

## Data flow

1. Load `rules.toml` into `Rules`.
2. Build a `PatternMatcher`, compiling every regex once at startup.
3. Create one `ScanState` (frequency tracker plus correlation engine) per file or session.
4. `read_file_from_offset` reads new lines and pushes detections into that state.
5. Filter by `--severity`, then render through `display_detections`.

## Stateful follow mode

`FrequencyTracker` and `CorrelationEngine` must persist across incremental reads in `--follow` mode.
Recreating them per read silently breaks window counts and correlation, and the symptom is a rule that
fires on a full scan but never in follow mode.

- Create `ScanState` once in `main.rs` before the follow loop.
- Pass `&mut ScanState` into every `read_file_from_offset` call.
- For multi-file follow, keep one `ScanState` per file in a `file_states` map.

Do not construct engines inside `read_file_from_offset`.

## Pattern matching priority

In `pattern_matcher.rs` the first match wins:

1. `rules.error_patterns` (substring)
2. `rules.warning_patterns` (substring)
3. `regex_rules` (compiled regex, in file order)

## Binary and configuration naming

- Cargo package: `rust_hound`
- User-facing binary: `rusthound` (the `[[bin]]` entry in `Cargo.toml`)
- Configuration directory: resolved with `dirs::config_dir()` —
  `~/Library/Application Support/rusthound/rules.toml` on macOS and
  `$XDG_CONFIG_HOME/rusthound/rules.toml` (default `~/.config/rusthound/rules.toml`) on Linux. When
  no `--rules` is given the binary tries that path and falls back to `rules.toml` in the working
  directory.
