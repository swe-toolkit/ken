# Worktree + cargo `target/` hygiene (Steward procedure)

The devcontainer's disk fills with cargo `target/` dirs, one per git worktree.
Measured 2026-09-07: **100 worktrees, 193G of `target/`** — ~161G in the 7 live
build seats, the rest in ~60 abandoned scratch/review worktrees that were created
for a task and never torn down. That reclaim (prune to 40 worktrees) recovered
~12G disk + ~1.7G RAM in one pass. The proliferation is the disease; this
procedure is the cure and the prevention.

## The disk splits into two populations — different fixes

- **Live build seats** (`.worktrees/<role>` primaries) — legitimately large
  (17–31G each). Do NOT `rm` a live seat's `target/` from outside; it may build at
  any moment and you would only force a full rebuild under it. Reclaim these by
  **idle-target sweep** (below) or the seat's own `cargo clean`, never unilaterally.
- **Abandoned scratch/review worktrees** — the tail AND the count problem. These
  are safe to prune, and pruning is the main lever.

## Safe prune — the criteria (removing a CLEAN worktree loses NOTHING)

A worktree is just a checkout; its branch ref and commits live in the shared
`.git`. Removing a **clean** worktree therefore loses no work — the seat can
`git worktree add` again if it needs it. So the safe set is:

1. **NOT a protected primary.** Protect `/workspaces/ken` (main) and every
   `.worktrees/<role>` where `<role>` is a live federation seat (adversary,
   architect, conformance-validator, doc-author, doc-leader, ergo-*, foundation-*,
   kernel-*, language-*, librarian, lieutenant, research, runtime-*, spec-author,
   spec-leader, steward, verify-*). These are live regardless of what branch or
   detached SHA they are checked out to — never remove them.
2. **Clean.** No staged, unstaged, or untracked changes. `git worktree remove`
   (no `--force`) refuses a dirty worktree on its own — that is a feature, not a
   thing to override. A dirty scratch worktree may hold un-captured work: **skip
   it and surface it to its owner**, never `--force` it away.
3. **Done.** Either its HEAD is an ancestor of `origin/main` (work landed) OR its
   admin `HEAD` (`.git/worktrees/<name>/HEAD`) is older than ~48h (abandoned).

The classifier (dry-run first, then drop the guard to execute):

```sh
cd /workspaces/ken
PROTECT="adversary architect conformance-validator doc-author doc-leader \
ergo-implementer ergo-leader ergo-qa foundation-implementer foundation-leader \
foundation-qa kernel-implementer kernel-leader kernel-qa language-implementer \
language-leader language-qa librarian lieutenant research runtime-implementer \
runtime-leader runtime-qa spec-author spec-leader steward verify-implementer \
verify-leader verify-qa"
MAIN=$(git rev-parse origin/main); now=$(date +%s)
git worktree list --porcelain | awk '/^worktree /{print $2}' | while read -r wt; do
  base=$(basename "$wt"); [ "$wt" = /workspaces/ken ] && continue
  case " $PROTECT " in *" $base "*) \
    [ "$(dirname "$wt")" = /workspaces/ken/.worktrees ] && continue ;; esac
  git -C "$wt" diff --quiet 2>/dev/null && git -C "$wt" diff --cached --quiet \
    2>/dev/null && [ -z "$(git -C "$wt" status --porcelain 2>/dev/null)" ] \
    || { echo "SKIP-DIRTY $wt"; continue; }
  head=$(git -C "$wt" rev-parse HEAD 2>/dev/null)
  anc=no; git merge-base --is-ancestor "$head" "$MAIN" 2>/dev/null && anc=MERGED
  adm=".git/worktrees/$base/HEAD"; age=0
  [ -f "$adm" ] && age=$(( (now-$(stat -c %Y "$adm"))/3600 ))
  if [ "$anc" = MERGED ] || [ "$age" -ge 48 ]; then
    git worktree remove "$wt" && echo "REMOVED $wt" || echo "KEEP(refused) $wt"
  else echo "SKIP-RECENT $wt"; fi
done
git worktree prune -v
```

`/tmp/*` worktrees are on **tmpfs = RAM**, not the disk volume — pruning those
frees RAM, not the 246G disk. Count them toward RAM pressure, not disk.

## Idle-target sweep (the live seats' 17–31G)

Reclaiming a live seat's `target/` is the seat's call, not yours to `rm`. The
standing discipline: **a seat that will be idle for a while runs `cargo clean` (or
`cargo sweep`) on its own worktree**; the next build pays the rebuild once. When
the whole fleet is quiet (credit hold, overnight), the Steward may run a
coordinated sweep across idle primaries — but verify a seat is genuinely idle
(pane at `Working`? recent commits?) before touching its `target/`.

## Do NOT set a shared `CARGO_TARGET_DIR` across worktrees

Tempting (one `target/` instead of N) but wrong for this fleet: cargo keys
artifacts by crate+source+flags, so ~15 seats on ~15 different branches sharing one
target dir would **constantly invalidate each other's builds** — cross-branch
cache thrash turns a disk win into a fleet-wide rebuild-time loss, plus lock
contention on concurrent builds. Per-worktree targets exist precisely to isolate
that. If build-time sharing is ever wanted, that is `sccache` (a content-keyed
compile cache that dedupes across seats without a shared target dir), a separate
concern from disk. Debuginfo is already trimmed in the root `Cargo.toml`
(`[profile.dev] debug = "line-tables-only"`, deps `debug = false`) — that lever is
spent; `incremental/` is only ~20% of a hot target, so `incremental = false` buys
little for a real warm-rebuild cost. Prune + idle-sweep are the durable levers.

## Cadence

- **At task close (every role, fleet law — `COORDINATION §12b`):** whoever created
  a scratch/review/reconcile worktree removes it (`git worktree remove`) when the
  task lands or is abandoned. Do not leave a clean scratch worktree behind.
- **Steward watchdog (periodic):** when `git worktree list | wc -l` climbs past
  ~45, or a filesystem drops below the 5G threshold, run the safe prune above.
