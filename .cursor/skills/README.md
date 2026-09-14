# RustHound Cursor Skills

Skills teach the Cursor agent *how* to work on this repo. They load from `.cursor/skills/` automatically.

The skills in this directory are project-authored and covered by the repository's Apache-2.0 license,
with one exception: `rusthound-cargo-tail` is adapted from the community tailing/monitoring skills in
[spencerpauly/awesome-cursor-skills](https://github.com/spencerpauly/awesome-cursor-skills)
(CC0-1.0), and its provenance is recorded in that skill. No vendored third-party bundles are included
in this repository.

| Skill | Use when |
|-------|----------|
| [rusthound-architecture](rusthound-architecture/SKILL.md) | Changing modules, follow mode, `ScanState`, data flow |
| [rusthound-rules-toml](rusthound-rules-toml/SKILL.md) | Editing `rules.toml` or correlation config |
| [rusthound-dev-workflow](rusthound-dev-workflow/SKILL.md) | Before commit: build, test, clippy, smoke tests |
| [rusthound-git-attribution](rusthound-git-attribution/SKILL.md) | Commits and PRs: keep the author attribution correct |
| [rusthound-cargo-tail](rusthound-cargo-tail/SKILL.md) | Watching `cargo build` / `cargo test` / `rusthound --follow` output |

These are development aids for this repository. They are not part of the analyzer runtime and are not
required to build, test, or use RustHound.
