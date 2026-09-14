---
name: rusthound-git-attribution
description: Git commit and PR attribution for RustHound. Use before git commit, git push, opening or updating PRs, or fixing history where cursoragent appears as author.
---

# RustHound — Git attribution

## Goal

Only the repository owner (`rustfuture`) should appear as author or committer in commits and PRs.
**Not allowed:** `Cursor Agent`, `cursoragent@cursor.com`, `Co-authored-by:` trailers, or Cloud Agent
promotional links in a PR body.

The canonical identity for this account is:

```text
rustfuture <121799572+rustfuture@users.noreply.github.com>
```

An empty `user.email` (or `rustfuture <>`) produces commits GitHub cannot link to the account. Set it
explicitly before committing:

```bash
git config user.email "121799572+rustfuture@users.noreply.github.com"
```

---

## 1) Cursor settings (one-time, per user)

In Cursor Desktop / Cloud Agent:

1. Open **Settings** → **Cloud Agents** (or **Agents**).
2. Turn **off** any option such as **Commit attribution**, **Add co-author**, or **Sign commits as agent**.
3. Leave the **Git author** field empty, or set it to `rustfuture`.

A VM-side hook (`~/.cursor/agent-hooks/.../commit-msg.cursor.co-author`) can append a
`Co-authored-by` trailer. If that setting is not disabled, commit with `--no-verify` (see below).

---

## 2) Agent rules (before every commit)

### Author / committer

```bash
export GIT_AUTHOR_NAME="rustfuture"
export GIT_AUTHOR_EMAIL="121799572+rustfuture@users.noreply.github.com"
export GIT_COMMITTER_NAME="rustfuture"
export GIT_COMMITTER_EMAIL="121799572+rustfuture@users.noreply.github.com"
```

Or use the repository wrapper:

```bash
./scripts/git-commit-as-owner.sh -m "feat: your message"
```

### Commit

- **Never** add a `Co-authored-by:` trailer.
- If a hook appends a co-author, use `git commit --no-verify` (only in that case).
- Verify before pushing:

```bash
git log -1 --format='Author: %an <%ae>%nCommitter: %cn <%ce>%n%n%B'
```

Expected: `rustfuture` / `121799572+rustfuture@users.noreply.github.com` — **not** `Cursor Agent`.

### Pull request

- Do not add Cursor / Cloud Agent promotional links to the title or body.
- Keep the body to Summary, Changes, and Verification.
- If `<!-- CURSOR_AGENT_PR_BODY_* -->` blocks are injected, edit the PR from the GitHub UI when possible.

---

## 3) Repairing history that already has wrong authorship

If `main` or a feature branch contains `cursoragent` commits, prefer `git filter-repo` over
`filter-branch`:

```bash
# Preview which commits are affected.
git log --format='%h %an <%ae> | %s' | grep -i cursoragent

# Rewrite author, committer, and strip co-author trailers on a scratch clone.
git filter-repo --force \
  --email-callback '
    return email.replace(b"cursoragent@cursor.com", b"121799572+rustfuture@users.noreply.github.com")
  ' \
  --name-callback '
    return name.replace(b"Cursor Agent", b"rustfuture")
  ' \
  --message-callback '
    return b"\n".join(l for l in message.split(b"\n") if not l.lower().startswith(b"co-authored-by:"))
  '

# Verify, then push with an explicit lease.
git log -3 --format='%h %an <%ae> | %s'
git push --force-with-lease origin main
```

A rewrite changes every commit hash. Coordinate before force-pushing a shared branch.

---

## 4) Pre-push checklist

- [ ] `git log -1` shows author `rustfuture` with a non-empty email.
- [ ] No `Co-authored-by` trailer in the commit message.
- [ ] `cargo test` passes ([rusthound-dev-workflow](../rusthound-dev-workflow/SKILL.md)).
- [ ] The PR body contains no agent advertising or Cursor links.

---

## Related files

- `scripts/git-commit-as-owner.sh` — safe commit wrapper.
- `.cursor/AGENT_GIT.md` — short summary.
