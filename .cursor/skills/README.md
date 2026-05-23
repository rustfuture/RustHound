# RustHound Cursor Skills

Skills teach the Cursor agent *how* to work on this repo. They load from `.cursor/skills/` automatically.

## Project-specific (maintained here)

| Skill | Use when |
|-------|----------|
| [rusthound-architecture](rusthound-architecture/SKILL.md) | Changing modules, follow mode, `ScanState`, data flow |
| [rusthound-rules-toml](rusthound-rules-toml/SKILL.md) | Editing `rules.toml` or correlation config |
| [rusthound-dev-workflow](rusthound-dev-workflow/SKILL.md) | Before commit: build, test, clippy, smoke tests |
| [rusthound-git-attribution](rusthound-git-attribution/SKILL.md) | Commit/PR: Cursor yazar olarak görünmesin, geçmiş düzeltme |

| [rusthound-cargo-tail](rusthound-cargo-tail/SKILL.md) | Watching `cargo build` / `cargo test` / `rusthound --follow` output |

## Rust language (from [ZhangHanDong/rust-skills](https://github.com/ZhangHanDong/rust-skills))

| Skill | Use when |
|-------|----------|
| rust-m01-ownership | Borrowing, lifetimes, ownership errors |
| rust-m06-error-handling | `anyhow`, `Result`, error design |
| rust-m07-concurrency | Tokio, async, `Send`/`Sync` |
| rust-domain-cli | Clap, exit codes, CLI UX |
| rust-router | Routing to the right Rust sub-skill |

## Rust rules bundle

| Skill | Source |
|-------|--------|
| rust-skills-leonardomso | [leonardomso/rust-skills](https://github.com/leonardomso/rust-skills) |

## Agent workflow (from [bluriesophos/cursorskills](https://github.com/bluriesophos/cursorskills))

| Skill | Use when |
|-------|----------|
| prove-it | Task complete — run tests and verify output |
| debug-to-fix | Structured debugging |
| trace-it | Before changing shared modules |
| loose-ends | Before PR — TODOs, missing tests |
| safe-refactor | Refactors without behavior change |

## DevOps / monitoring (from [awesome-cursor-skills](https://github.com/spencerpauly/awesome-cursor-skills))

| Skill | Use when |
|-------|----------|
| monitoring-terminal-errors | Process crashes, stack traces in terminals |
| tailing-build-output | Stream build output (see also rusthound-cargo-tail) |
| setting-up-ci | GitHub Actions for Rust |
| parallel-ci-triage | Multiple failing CI jobs |
| incident-response | Production incident workflow |

## Refresh third-party skills

```bash
./scripts/install-cursor-skills.sh
```

Each vendored skill includes `UPSTREAM.md` with the exact commit. Cache clones live in `.cursor/skills-cache/` (gitignored).
