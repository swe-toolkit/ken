---
scope: fleet
audience: (see scope README)
source: private memory
  `landing-a-config-change-on-main-does-not-apply-it-the-consumer-reads-a-working-tree`
  (R4 triage, 2026-09-26)
---

# Landing on main does not apply it — find the path the consumer reads

"It landed" and "it took effect" are two different claims. For any file a
running process reads by path rather than by git ref, the first does not
imply the second — and a config change that is routed, reviewed, and merged
onto `main` can still leave the fleet unchanged if the launcher opens a
working-tree file that never moved.

## What happened

Directed to reseat two roles, the Steward edited the tracked `moot.toml`,
routed it, and verified the landing by comparing blobs against
`origin/main`. The check was correct and the fleet did not move. `moot exec
<role>` reads `/workspaces/ken/moot.toml` — the **main checkout's
working-tree file** — and that checkout was on `main`, hundreds of commits
behind `origin/main`, with the live seating held as an uncommitted edit to
that same file. The landed candidate had changed a different file from the
one the launcher opens.

The mismatch surfaced only by restarting a seat and reading its own footer
after the routing was already reported complete — the blob check answered
"is this content on `origin/main`?", a question nobody had asked; "what does
the launcher open?" was the operative one, and no git ref answers it.

**A second instance, one day later, was worse: the consumer tree was not
merely stale, it was on a divergent branch** (`operator/adjustments`, past a
merge-base, not an ancestor of `origin/main`). `git log --oneline -1` cannot
tell "behind" apart from "divergent"; `git merge-base --is-ancestor
<consumer-HEAD> origin/main` can, and is the check to run.

## Two mechanism facts worth carrying forward

- A launcher that derives its target from the working directory (for
  example `project = Path.cwd().name`) must be invoked from the directory it
  expects — running it from a worktree can make it resolve a different
  project name and fail, sometimes after it has already torn down the seat
  it was meant to restart.
- A config-inspection command and the launcher can resolve the same
  logical config from different files when invoked from different working
  directories, with no error either way. Two tools, two silently different
  answers.

## How to apply

- Before routing a change to a file something reads at runtime, find the
  exact path the consumer opens — read the launcher's own error text or its
  source, not its documentation.
- Verify the effect at the consumer, not at the commit. For a seating
  change, that is the seat's own footer after a restart; a landed blob is
  evidence about a repository, not about the fleet.
- A dirty working-tree file is the live control surface when a tool reads it
  by path. Do not "tidy" it by syncing or resetting it — that discards state
  nobody recorded. Edit it surgically, back it up first, and touch only the
  keys you were directed to change. A working-tree fix made this way is not
  durable: any later checkout or reset on that tree silently undoes it, and
  the effect reverts at the next relaunch with no error anywhere — say so
  when handing the fix back.
- Measure whether the tracked and live copies actually disagree before
  reporting alarm; a long divergence in comments or prose is not evidence
  that every past change through that mechanism was lost.
- Distinguish "behind" from "divergent" with `git merge-base --is-ancestor`,
  never with a log listing.

Related: [[a-fleet-outage-can-be-platform-partitioned-and-only-a-pane-census-sees-it]]
(the verification that actually closes a seating question is a per-seat
census, not a repository check).
