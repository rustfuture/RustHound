---
name: rusthound-rules-toml
description: Valid rules.toml schema for RustHound. Use when editing or documenting configuration files.
---

# RustHound rules.toml Schema

## Structure (authoritative)

```toml
[rules]
error_patterns = ["ERROR", "FATAL"]
warning_patterns = ["WARN", "WARNING"]

[[regex_rules]]
name = "rule_name"          # Used in correlation trigger_on_rule.name
pattern = "regex here"
severity = "critical"       # critical | high | warning | error | info

[frequency_rules]
max_same_errors_per_minute = 10
time_window_seconds = 60

[[correlated_rules]]
name = "Alert Title"
severity = "critical"
description = "Human-readable description"
time_window_seconds = 60
followed_by = "Successful Login"   # Must match a regex_rules.name

[correlated_rules.trigger_on_rule]
name = "authentication_failure"    # Must match a regex_rules.name
count = 10
```

## Reference files

- Default rules: `rules.toml`
- Correlation example: `correlated_rules.toml`

## Common mistakes

- Do **not** use README-style `[patterns]` or `[[regex_rules.rule]]` — they are invalid
- `followed_by` and `trigger_on_rule.name` must match `[[regex_rules]].name` exactly
- `[rules]` table holds `error_patterns` / `warning_patterns` (not `[patterns]`)

## Severity values

Maps to `Severity` enum in `src/output/mod.rs`:

| TOML value | Enum |
|------------|------|
| critical | Critical |
| high | High |
| warn, warning | Warning |
| error, fatal, exception | Error |
| (other) | Info |
