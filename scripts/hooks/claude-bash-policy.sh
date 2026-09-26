#!/usr/bin/env bash
# Claude Code PreToolUse hook for Bash: run the shared fleet command policy.
# The hook input is JSON on stdin; exit 2 blocks the call and shows stderr.
set -uo pipefail
dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cmd="$(python3 -c 'import json,sys; print(json.load(sys.stdin).get("tool_input",{}).get("command",""))' 2>/dev/null)" || exit 0
reason="$(printf '%s' "$cmd" | "$dir/bash-policy")"
status=$?
if [[ $status -eq 2 ]]; then
  printf '%s\n' "$reason" >&2
  exit 2
fi
exit 0
