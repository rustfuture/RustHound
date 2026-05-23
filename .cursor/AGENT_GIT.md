# Git attribution for Cloud Agents

When using Cursor Cloud Agents on this repository:

- Commits should be authored **only** by the repository owner (`rustfuture`), not `cursoragent`.
- Do **not** append `Co-authored-by:` trailers to commit messages.
- In Cursor: disable automatic co-author / attribution for agent commits if your plan exposes that setting.

Cloud Agent VMs may run a `commit-msg` hook that adds co-authors; turn that off in Cursor Settings before pushing from the agent.
