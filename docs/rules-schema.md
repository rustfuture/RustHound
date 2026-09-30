# rules.toml schema

This is the authoritative schema for the rules file. `rules.toml` and `correlated_rules.toml` in the
repository root are working examples of it.

## Structure

~~~toml
[rules]
error_patterns = ["ERROR", "FATAL"]
warning_patterns = ["WARN", "WARNING"]

[[regex_rules]]
name = "rule_name"          # referenced by correlation trigger_on_rule.name
pattern = "regex here"
severity = "critical"       # see the severity table below

[frequency_rules]
max_same_errors_per_minute = 10
time_window_seconds = 60

[[correlated_rules]]
name = "Alert Title"
severity = "critical"
description = "Human-readable description"
time_window_seconds = 60
followed_by = "Successful Login"   # must match a regex_rules.name

[correlated_rules.trigger_on_rule]
name = "authentication_failure"    # must match a regex_rules.name
count = 10
~~~

## Window timing

`time_window_seconds` in `[frequency_rules]` and in `[[correlated_rules]]` is measured on processing
time: the moment RustHound reads a line (`Local::now` in `src/analyzer/frequency_tracker.rs`,
`Instant::now` in `src/analyzer/correlation_engine.rs`). It is not derived from timestamps inside the
log. The windows therefore describe bursts in live or follow mode. When scanning an existing file, lines
logged hours apart are read within moments of each other and are counted as inside the same window.

## Common mistakes

- `[patterns]` and `[[regex_rules.rule]]` are not valid. The `[rules]` table holds
  `error_patterns` and `warning_patterns`.
- `followed_by` and `trigger_on_rule.name` must match a `[[regex_rules]]` `name` exactly. A typo
  produces a rule that never fires rather than an error.
- Rule `name` values are the identifiers correlation refers to, so renaming one is a breaking change
  to any rules file that references it.

## Severity values

These map to the `Severity` enum in `src/output/mod.rs`:

| TOML value | Enum |
| --- | --- |
| `critical` | Critical |
| `high` | High |
| `warn`, `warning` | Warning |
| `error`, `fatal`, `exception` | Error |
| anything else | Info |
