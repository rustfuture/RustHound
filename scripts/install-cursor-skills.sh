#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="${ROOT}/.cursor/skills"
CACHE_DIR="${ROOT}/.cursor/skills-cache"

mkdir -p "$SKILLS_DIR" "$CACHE_DIR"

clone_or_pull() {
  local url="$1"
  local name="$2"
  local dir="${CACHE_DIR}/${name}"
  if [[ -d "${dir}/.git" ]]; then
    git -C "$dir" pull --ff-only >/dev/null
  else
    git clone --depth 1 "$url" "$dir" >/dev/null
  fi
  printf '%s' "$dir"
}

copy_skill() {
  local src="$1"
  local dest_name="$2"
  local dest="${SKILLS_DIR}/${dest_name}"
  rm -rf "$dest"
  mkdir -p "$dest"
  cp -r "${src}/." "$dest/"
  [[ -f "${dest}/SKILL.md" ]] || { echo "missing SKILL.md: ${dest_name}" >&2; return 1; }
  echo "  + ${dest_name}"
}

write_upstream() {
  local dest_name="$1" cache_key="$2" repo_url="$3" path="$4"
  local commit
  commit="$(git -C "${CACHE_DIR}/${cache_key}" rev-parse HEAD)"
  cat > "${SKILLS_DIR}/${dest_name}/UPSTREAM.md" <<EOF
# Upstream

- Repository: ${repo_url}
- Path: \`${path}\`
- Commit: \`${commit}\`

Re-run \`./scripts/install-cursor-skills.sh\` to refresh.
EOF
}

echo "==> Fetching sources..."
ZH="$(clone_or_pull https://github.com/ZhangHanDong/rust-skills.git zhang-rust-skills)"
LSO="$(clone_or_pull https://github.com/leonardomso/rust-skills.git leonardomso-rust-skills)"
CS="$(clone_or_pull https://github.com/bluriesophos/cursorskills.git bluriesophos-cursorskills)"
ACS="$(clone_or_pull https://github.com/spencerpauly/awesome-cursor-skills.git awesome-cursor-skills)"

echo "==> Rust skills (ZhangHanDong)..."
for skill in m06-error-handling m07-concurrency domain-cli m01-ownership; do
  [[ -d "${ZH}/skills/${skill}" ]] || continue
  copy_skill "${ZH}/skills/${skill}" "rust-${skill}"
  write_upstream "rust-${skill}" "zhang-rust-skills" "https://github.com/ZhangHanDong/rust-skills" "skills/${skill}"
done
[[ -f "${ZH}/skills/rust-router/SKILL.md" ]] && copy_skill "${ZH}/skills/rust-router" "rust-router" && write_upstream "rust-router" "zhang-rust-skills" "https://github.com/ZhangHanDong/rust-skills" "skills/rust-router"

echo "==> leonardomso/rust-skills..."
copy_skill "${LSO}" "rust-skills-leonardomso"
write_upstream "rust-skills-leonardomso" "leonardomso-rust-skills" "https://github.com/leonardomso/rust-skills" "."

echo "==> Workflow skills (bluriesophos)..."
for skill in prove-it debug-to-fix trace-it loose-ends safe-refactor; do
  copy_skill "${CS}/${skill}" "${skill}"
  write_upstream "${skill}" "bluriesophos-cursorskills" "https://github.com/bluriesophos/cursorskills" "${skill}"
done

echo "==> DevOps skills (awesome-cursor-skills)..."
for skill in monitoring-terminal-errors tailing-build-output setting-up-ci parallel-ci-triage incident-response; do
  src="${ACS}/resources/${skill}"
  [[ -d "$src" ]] || continue
  copy_skill "$src" "${skill}"
  write_upstream "${skill}" "awesome-cursor-skills" "https://github.com/spencerpauly/awesome-cursor-skills" "resources/${skill}"
done

echo "==> Done:"
ls -1 "$SKILLS_DIR"
