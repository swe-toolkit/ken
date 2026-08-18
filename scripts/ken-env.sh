# ken-env.sh — source this in every agent's shell to enable "multi-agent mode":
# a shared compiler cache + shared registry so N agents on different branches
# don't each recompile the same dependencies or re-download the registry.
#
#   source scripts/ken-env.sh
#
# Requires `sccache` on PATH (cargo install sccache, or your package manager).
# The shared dirs must be on a path ALL agents can see — if agents run in
# separate containers/worktrees, mount these as shared volumes in the harness.
# Rationale: docs/ops/compute-budget.md.

# Shared compiler cache: a dependency compiled by one agent is reused by all.
# This is the biggest win for many parallel agents building the same workspace.
export RUSTC_WRAPPER="${RUSTC_WRAPPER:-sccache}"
# The cache lives on the /workspaces/ken volume (229G), NOT on $HOME, which is
# the container's 77G overlay. A 20G cache on the overlay competes with every
# image layer and build artifact for the smaller filesystem; the repo volume has
# room for it. Operator, 2026-08-18. If you repoint this at $HOME again, expect
# the overlay to fill and the whole box to wedge.
export SCCACHE_DIR="${SCCACHE_DIR:-${KEN_SHARED_CACHE_ROOT:-/workspaces/ken/.cache}/sccache}"
export SCCACHE_CACHE_SIZE="${SCCACHE_CACHE_SIZE:-20G}"

# sccache and cargo incremental conflict; incremental units aren't cached. With
# many agents the cross-agent dependency cache beats per-agent incremental.
export CARGO_INCREMENTAL=0

# Shared registry + downloaded crate sources (read-mostly; cargo handles
# concurrent access). Avoids N copies of the crate index/sources on disk.
export CARGO_HOME="${KEN_SHARED_CARGO_HOME:-$HOME/.cache/ken-cargo-home}"

# Keep each worktree's target/ local (isolation); sccache provides the sharing.
# Do NOT point CARGO_TARGET_DIR at a shared path — concurrent cargo would contend
# on the target lock and branch differences would thrash it.

# Sanity print (quiet if sccache missing — wrapper just becomes a no-op cost).
command -v sccache >/dev/null 2>&1 || \
  echo "ken-env: sccache not found on PATH; install it for cross-agent cache reuse" >&2
