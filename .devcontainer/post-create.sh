#!/bin/bash
set -euo pipefail

# System packages
sudo apt-get update && sudo apt-get install -y tmux curl

# Claude Code CLI — install from npm first (puts `claude` on PATH
# via /usr/local/share/npm-global/bin), use it to register MCP servers,
# then call `claude install` to migrate to the native build at
# ~/.local/bin. The native build is the officially-supported path
# going forward; the TUI nags on first run when it's still npm-based.
npm install -g @anthropic-ai/claude-code

# Codex CLI — standalone installer in non-interactive mode. The installer
# places `codex` in ~/.local/bin by default, which bash -lc picks up through
# the devcontainer user's standard profile.
curl -fsSL https://chatgpt.com/codex/install.sh | CODEX_NON_INTERACTIVE=1 sh

# Python tooling
pip install uv

# Install moot package. --upgrade is load-bearing: a bare `pip install mootup`
# is a no-op against an already-satisfied requirement (how the env drifted stale
# at 0.5.10), whereas --upgrade pulls the latest each build — so future moot
# fixes (e.g. the 0.5.12 codex stranded-paste fix) land on rebuild with no
# manual version bump.
pip install --upgrade mootup

# `--upgrade` above resolves `mcp` fresh, which now yields 2.0.0 — and 2.0.0
# removed `mcp.server.fastmcp`, the API moot's adapters import. A container
# built after that release comes up with BOTH convo MCP servers dead and no
# error until a seat tries to use a convo tool. Pin until moot's adapters move.
pip install 'mcp<2'

# Register MCP servers for Claude Code at user scope so claude finds
# them regardless of cwd (agents launch in worktrees under .worktrees/,
# not the project root). Use absolute paths to the wrapper scripts so
# they resolve from any cwd. The wrappers read CONVO_ROLE at runtime
# to look up the per-role API key from .moot/actors.json.
DEVCONTAINER_DIR="$(realpath .devcontainer)"
PROJECT_ROOT="$(dirname "$DEVCONTAINER_DIR")"
claude mcp add convo "$DEVCONTAINER_DIR/run-moot-mcp.sh" -s user
claude mcp add convo-channel "$DEVCONTAINER_DIR/run-moot-channel.sh" -s user

# Register MCP servers for Codex at user scope so Codex finds them regardless
# of cwd (agents launch in worktrees under .worktrees/, not the project root).
# The wrappers read CONVO_ROLE at runtime to look up the per-role API key from
# .moot/actors.json.
mkdir -p /home/node/.codex
chmod 700 /home/node/.codex
cat > /home/node/.codex/config.toml <<CODEX_CONFIG
approval_policy = "never"
sandbox_mode = "danger-full-access"

# Codex memory feature (off by default upstream). Enabled fleet-wide so Codex
# build seats generate + reuse their own per-seat memories. NOTE: this is
# SUPPLEMENTAL to the curated agent/memory/ corpus, which stays canonical
# (CLAUDE.md). Read at codex startup — a restart is required to take effect.
[features]
memories = true

# Optional: uncomment when using the Ken LLM proxy as Codex model provider.
# The proxy expects LLM_PROXY_SHARED_SECRET in the Codex process environment.
# model_provider = "moot-llm-proxy"
#
# [model_providers.moot-llm-proxy]
# name = "Moot LLM proxy"
# base_url = "http://127.0.0.1:8090/v1"
# env_key = "LLM_PROXY_SHARED_SECRET"

[mcp_servers.convo]
command = "$DEVCONTAINER_DIR/run-moot-mcp.sh"
cwd = "$PROJECT_ROOT"
env_vars = ["CONVO_ROLE", "CONVO_API_URL", "CONVO_WORKTREE"]
startup_timeout_sec = 30

[mcp_servers.convo-channel]
command = "$DEVCONTAINER_DIR/run-moot-channel.sh"
cwd = "$PROJECT_ROOT"
env_vars = ["CONVO_ROLE", "CONVO_API_URL", "CONVO_WORKTREE"]
startup_timeout_sec = 30
CODEX_CONFIG
chmod 600 /home/node/.codex/config.toml

# ── pi harness (OpenRouter/Fireworks) ─────────────────────────────────────
# Per-role harness selection, for whichever roles moot.toml assigns
# harness = "pi". Runs BEFORE `claude install`, which deletes the npm `claude`
# symlink — nothing that shells out to an npm-installed binary may follow it.
# Fail-open: a pi hiccup must not abort post-create and strand claude/codex
# seats.
{
  command -v pi >/dev/null 2>&1 || \
    npm install -g --ignore-scripts @earendil-works/pi-coding-agent

  # The convo tool surface for pi seats. `pi install npm:<pkg>` both records
  # the package in ~/.pi/agent/settings.json AND installs its runtime deps
  # (ws, undici, typebox) under ~/.pi. A hand-written settings entry does
  # neither — pi does not vendor deps for packages it did not install itself.
  pi install npm:@mootup/pi-convo

  mkdir -p /home/node/.pi/agent
  chmod 700 /home/node/.pi /home/node/.pi/agent 2>/dev/null || true

  # Seed the three settings a DETACHED seat needs, without touching
  # `packages` — `pi install` above owns that key.
  #
  #   defaultProjectTrust  a detached seat has nobody to answer a trust prompt
  #   doubleEscapeAction   "none"; double-Esc otherwise wedges the tree selector
  #   quietStartup         keeps the banner out of captured pane output
  #
  # Absent-only per key, so an operator edit survives a rebuild — the same
  # policy as mootup's _seed_pi_trust. This is a merge rather than convo's
  # heredoc regeneration + `.moot-generated` marker: ken installs the
  # extension from npm, so there is no absolute path to keep in sync and
  # nothing to regenerate.
  python3 - <<'PI_SETTINGS'
import json, os
p = "/home/node/.pi/agent/settings.json"
try:
    with open(p) as f:
        d = json.load(f)
    if not isinstance(d, dict):
        d = {}
except (OSError, json.JSONDecodeError):
    d = {}
for k, v in (("defaultProjectTrust", "always"),
             ("doubleEscapeAction", "none"),
             ("quietStartup", True)):
    d.setdefault(k, v)
with open(p, "w") as f:
    json.dump(d, f, indent=2)
os.chmod(p, 0o600)
PI_SETTINGS

  # trust.json — flat map, path -> bool; lookup walks up parents, so the repo
  # root entry covers every worktree under .worktrees/. Absent-only: a stored
  # `false` is a real operator decision.
  #
  # NEVER touch auth.json — pi owns its credential store, and ken's pi seats
  # authenticate from OPENROUTER_API_KEY injected per-role by moot.toml.
  if [ ! -e /home/node/.pi/agent/trust.json ]; then
    printf '{\n  "/workspaces/ken": true\n}\n' > /home/node/.pi/agent/trust.json
    chmod 600 /home/node/.pi/agent/trust.json
  fi
} || true

# Migrate from the npm-installed claude to the native build. This runs
# LAST (after `claude mcp add`) because `claude install` deletes the
# npm symlink — anything calling `claude` after this point must rely on
# ~/.local/bin/claude, which `bash -lc` picks up via ~/.profile's
# standard "$HOME/.local/bin" snippet. Agent tmux sessions launch with
# `bash -lc`, so they find the native binary automatically.
claude install

# Rebind tmux prefix to Ctrl-Space. Claude Code intercepts Ctrl-B (the
# default prefix), so the usual `Ctrl-B d` detach never reaches tmux.
# Ctrl-Space is rarely claimed by TUIs and leaves readline-style editing
# bindings (Ctrl-A/E/etc.) untouched inside claude's input line.
cat > /home/node/.tmux.conf <<'TMUX_CONF'
unbind C-b
set -g prefix C-Space
bind C-Space send-prefix

# Mouse on: scroll-wheel scrolls the pane, click selects a pane/window,
# drag copies. Without this, scrollback is only reachable via the
# copy-mode keybind (<prefix> [) which is a tmux-literacy tax users
# shouldn't have to pay just to read recent output.
set -g mouse on
TMUX_CONF

# Register a /detach slash command for claude so the user can leave a
# tmux session without having to fight for the prefix key. The command
# calls `tmux detach-client`, which disconnects the terminal but leaves
# claude running in the session so `moot attach` picks up where it left
# off. User-scope so every worktree sees it.
mkdir -p /home/node/.claude/commands
cat > /home/node/.claude/commands/detach.md <<'DETACH_MD'
---
description: Detach from the tmux session (leaves claude running in the background)
allowed-tools: Bash(bash:*)
---

!bash -c 'SOCK=$(find /tmp /run -maxdepth 3 -name default -type s 2>/dev/null | head -1); if [ -n "$SOCK" ]; then tmux -S "$SOCK" detach-client; else echo "tmux socket not found"; fi'
DETACH_MD

# Context-awareness hooks (self-compact signal). Deploy the in-house scripts and
# register them in the global Claude Code settings: the statusline extracts the
# context-window %, and a PreToolUse hook nudges the self-compacting singletons
# (steward/architect/librarian) to checkpoint + compact at a clean
# seam. Role-scoped (teams get only the statusline) and fail-safe. Source of
# truth is .devcontainer/hooks/; see its README.md.
HOOKS_SRC="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/hooks"
mkdir -p /home/node/.claude/hooks
cp "$HOOKS_SRC"/ctx-*.sh /home/node/.claude/hooks/
chmod +x /home/node/.claude/hooks/ctx-*.sh
if [ -f /home/node/.claude/settings.json ]; then
  jq -s '.[0] * .[1]' /home/node/.claude/settings.json "$HOOKS_SRC/settings-fragment.json" \
    > /home/node/.claude/settings.json.tmp \
    && mv /home/node/.claude/settings.json.tmp /home/node/.claude/settings.json
else
  cp "$HOOKS_SRC/settings-fragment.json" /home/node/.claude/settings.json
fi
echo "[post-create] context-awareness hooks installed."

# Git hooks (work-item tracker gates). Versioned in .githooks/ rather than
# .git/hooks so they are reviewable and travel with the repo. Worktrees share
# the common .git dir, so this one setting covers every agent worktree.
# NOTE: a hook is fast feedback, NOT enforcement — it is bypassable with
# --no-verify and absent wherever core.hooksPath is unset. The enforcing
# check is the CI job tracked as CI-TRACKER-GATE.
if [ -d /workspaces/ken/.githooks ]; then
  git -C /workspaces/ken config core.hooksPath .githooks
  chmod +x /workspaces/ken/.githooks/* 2>/dev/null || true
  echo "[post-create] git hooks installed (core.hooksPath=.githooks)."
fi

echo "Container ready."
