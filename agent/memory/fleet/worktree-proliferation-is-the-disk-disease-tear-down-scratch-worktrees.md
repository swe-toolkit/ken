---
scope: fleet
audience: (see scope README)
source: 2026-09-07, Steward — devcontainer disk at 99% (2.7G free); root cause
  measured as 100 git worktrees (193G of cargo target/, ~161G in 7 live build
  seats, the rest in ~60 abandoned scratch/review worktrees). A safe prune to 40
  worktrees recovered ~12G disk + ~1.7G RAM.
---

# Worktree proliferation is the disk disease — tear down scratch worktrees

Every git worktree carries its own cargo `target/` (17–31G for an active build
seat). The devcontainer disk fills not because any one target is wrong but because
**scratch/review/reconcile worktrees are created for a task and never removed** —
they accumulate to ~100, and their targets are the tail while their COUNT is the
disease. `df` at 99% is the symptom; `git worktree list | wc -l` at 100 is the
cause.

**Tear down what you spin up.** Whoever creates a scratch worktree (an Architect
review checkout, a lieutenant reconcile worktree, a `wp/*` build worktree) removes
it with `git worktree remove` when the task lands or is abandoned. This is fleet
law (`COORDINATION §12b`). A clean worktree holds no un-captured work — its branch
and commits live in the shared `.git` — so removing it loses nothing and it can be
re-added on demand.

**The Steward runs the periodic safe prune** (`steward/worktree-hygiene.md`):
protect `main` + every `.worktrees/<role>` primary; remove only worktrees that are
clean AND (merged-to-`origin/main` OR admin-`HEAD` older than ~48h). `git worktree
remove` without `--force` refuses a dirty worktree on its own — never `--force` it;
a dirty scratch worktree may hold real work, so surface it to its owner instead.
`/tmp/*` worktrees are tmpfs = RAM, not the disk volume — pruning those frees RAM.

**Do NOT "fix" this with a shared `CARGO_TARGET_DIR`.** One target dir across ~15
seats on ~15 branches makes cargo (which keys artifacts by crate+source+flags)
constantly invalidate each seat's build — cross-branch cache thrash turns a disk
win into a fleet-wide rebuild-time loss, plus lock contention on concurrent builds.
Per-worktree targets isolate that on purpose. Build-time sharing, if ever wanted,
is `sccache` (content-keyed, no shared target dir) — a different concern from disk.
Debuginfo is already trimmed in the root `Cargo.toml`; `incremental/` is only ~20%
of a hot target. Prune + idle-target sweep are the durable levers, not a shared
target. See also
[a-full-disk-presents-as-a-test-regression-and-df-slash-cannot-see-it](a-full-disk-presents-as-a-test-regression-and-df-slash-cannot-see-it.md).
