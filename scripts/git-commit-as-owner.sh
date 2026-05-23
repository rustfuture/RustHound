#!/usr/bin/env bash
# Commit with rustfuture as sole author/committer (no Cursor co-author hook).
# Usage: ./scripts/git-commit-as-owner.sh -m "feat: message"
#        ./scripts/git-commit-as-owner.sh -a -m "fix: message"
set -euo pipefail

export GIT_AUTHOR_NAME="rustfuture"
export GIT_AUTHOR_EMAIL="rustfuture@users.noreply.github.com"
export GIT_COMMITTER_NAME="rustfuture"
export GIT_COMMITTER_EMAIL="rustfuture@users.noreply.github.com"

if [[ $# -eq 0 ]]; then
  echo "Usage: $0 [-a] -m \"commit message\"" >&2
  exit 1
fi

# --no-verify skips commit-msg hooks that append Co-authored-by on Cloud Agent VMs
git commit --no-verify "$@"

echo "---"
git log -1 --format='Author: %an <%ae>%nCommitter: %cn <%ce>'
