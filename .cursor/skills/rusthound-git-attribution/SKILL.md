---
name: rusthound-git-attribution
description: Git commit and PR attribution for RustHound. Use before git commit, git push, opening or updating PRs, or fixing history where cursoragent appears as author.
---

# RustHound — Git attribution (Cursor görünmesin)

## Amaç

PR ve commit geçmişinde **yalnızca repo sahibi** (`rustfuture`) görünsün.  
**Yasak:** `Cursor Agent`, `cursoragent@cursor.com`, `Co-authored-by:` satırları, PR gövdesinde Cloud Agent linkleri.

---

## 1) Cursor ayarları (kullanıcı — bir kez)

Cursor Desktop / Cloud Agent:

1. **Settings** → **Cloud Agents** (veya **Agents**)
2. **Commit attribution** / **Add co-author** / **Sign commits as agent** benzeri seçenekleri **kapatın**
3. Varsa **Git author** alanını boş bırakın veya `rustfuture` yapın

VM’de `~/.cursor/agent-hooks/.../commit-msg.cursor.co-author` hook’u `Co-authored-by` ekleyebilir; ayar kapalı değilse agent `--no-verify` kullanmalı (aşağıda).

---

## 2) Agent kuralları (her commit öncesi)

### Author / committer

```bash
export GIT_AUTHOR_NAME="rustfuture"
export GIT_AUTHOR_EMAIL="rustfuture@users.noreply.github.com"
export GIT_COMMITTER_NAME="rustfuture"
export GIT_COMMITTER_EMAIL="rustfuture@users.noreply.github.com"
```

Veya repo script:

```bash
./scripts/git-commit-as-owner.sh -m "feat: your message"
```

### Commit

- Mesajda **asla** `Co-authored-by:` ekleme
- Hook co-author ekliyorsa: `git commit --no-verify` (yalnızca bu durumda)
- Push öncesi kontrol:

```bash
git log -1 --format='Author: %an <%ae>%nCommitter: %cn <%ce>%n%n%B'
```

Beklenen: `rustfuture` / `rustfuture@users.noreply.github.com` — **değil** `Cursor Agent` veya `cursoragent`.

### PR

- PR başlığı ve gövdesinde **Cursor / Cloud Agent** tanıtım linki ekleme
- Gövde: yalnızca Summary, Changes, Verification (Markdown)
- `<!-- CURSOR_AGENT_PR_BODY_* -->` blokları sistem ekliyorsa, mümkünse PR’ı kullanıcı GitHub UI’dan düzenlesin

---

## 3) Zaten yanlış yazılmış geçmişi düzeltme

`main` veya feature branch’te `cursoragent` commit’leri varsa:

```bash
BASE=<merge-öncesi-son-commit-sha>   # örn. 059f7da

git checkout -B fix-attribution "$BASE"

# Her commit için (sırayla SHA’ları yazın):
for sha in <sha1> <sha2> <sha3>; do
  git cherry-pick "$sha"
  msg=$(git log -1 --format=%B | grep -v '^Co-authored-by:' | sed -e :a -e '/^\n*$/{$d;N;ba}')
  GIT_AUTHOR_NAME=rustfuture GIT_AUTHOR_EMAIL=rustfuture@users.noreply.github.com \
  GIT_COMMITTER_NAME=rustfuture GIT_COMMITTER_EMAIL=rustfuture@users.noreply.github.com \
  git commit --amend --author="rustfuture <rustfuture@users.noreply.github.com>" -m "$msg" --no-verify
done

# Doğrula
git log -3 --format='%h %an <%ae> | %s'

# Onay sonrası (dikkat: force push)
git push origin fix-attribution:main --force-with-lease
```

Alternatif (tüm branch tek seferde):

```bash
FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch -f \
  --env-filter '
export GIT_AUTHOR_NAME="rustfuture"
export GIT_AUTHOR_EMAIL="rustfuture@users.noreply.github.com"
export GIT_COMMITTER_NAME="rustfuture"
export GIT_COMMITTER_EMAIL="rustfuture@users.noreply.github.com"
' "$BASE"..HEAD

FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch -f \
  --msg-filter 'grep -v "^Co-authored-by:"' "$BASE"..HEAD
```

---

## 4) Checklist (push öncesi)

- [ ] `git log -1` → author `rustfuture`
- [ ] Commit mesajında `Co-authored-by` yok
- [ ] `cargo test` geçti ([rusthound-dev-workflow](../rusthound-dev-workflow/SKILL.md))
- [ ] PR gövdesinde agent reklamı / cursor linki yok

---

## İlgili dosyalar

- `scripts/git-commit-as-owner.sh` — güvenli commit wrapper
- `.cursor/AGENT_GIT.md` — kısa özet
