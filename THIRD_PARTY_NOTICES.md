# Third-party notices

RustHound itself is licensed under Apache-2.0 (see [`LICENSE`](LICENSE)).

The development helper skills under `docs/agent-skills/` (previously `.cursor/skills/`) are copied from
upstream repositories. They are development aids and are **not** part of the product functionality.
Each bundle records its source in an `UPSTREAM.md` file. Upstream license metadata was checked with
`gh api repos/<repo> --jq .license.spdx_id` on 2026-09-10.

| Bundle (directories) | Upstream | Pinned commit | Stated license | Notes |
|---|---|---|---|---|
| `rust-m01-ownership`, `rust-m06-error-handling`, `rust-m07-concurrency`, `rust-domain-cli`, `rust-router` | `ZhangHanDong/rust-skills` | `37e050f490a731445eb17d339f7e95878be71d70` | MIT declared in the upstream README; **no LICENSE file at the pinned commit** | Ambiguous. This is a documentation gap upstream, not a confirmed violation. |
| `rust-skills-leonardomso` | `leonardomso/rust-skills` | `89910e8585331dabbecd400ae132b4070ecf24af` | MIT | Upstream `LICENSE` is retained in the bundle. |
| `prove-it`, `debug-to-fix`, `trace-it`, `loose-ends`, `safe-refactor` | `bluriesophos/cursorskills` | `0ebd86c11e02f32d7fc90a4b3b0f5609b0262069` | MIT | |
| `monitoring-terminal-errors`, `tailing-build-output`, `setting-up-ci`, `parallel-ci-triage`, `incident-response` | `spencerpauly/awesome-cursor-skills` | `773b571db3693d2edc1cf656903c292d037e25b8` | CC0-1.0 | |
| `rusthound-*` | project-authored | – | Apache-2.0 | |

If a redistributed upstream bundle is found to be licensed incompatibly, the bundle should be removed
rather than silently retained. No such incompatibility is established today.
