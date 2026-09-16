---
id: CI-MAIN-RUNS-CANCEL-EACH-OTHER
title: "Every push to main shares one concurrency group with cancel-in-progress, so each merge kills the CI still running for the previous merge; post-merge runs on main are a record of a specific tree, not a superseded attempt at the same one, and the fix is to stop cancelling them"
status: ready
owner: verify
size: S
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Steward, 2026-09-16. Two dated instances with a measured cost. (1) ff9d0f0ca4562ac38a553fadf325fbe7aab992b2, 2026-09-14 22:28, run 34904280362, cancelled about five seconds in. (2) 10321a158bc69cf50e0cb753e1096fe10fa43ed1, 2026-09-16 04:22, run 35055338045, cancelled about thirteen minutes in by the doc-only merge c4a21bb0fa658477f04e670764485634485dd98f. Combined cost: there is no completed ignored-row sweep on main's lineage after 2026-09-13 (run 34753101365 at 7663ad9b924e7b1cb2c0111215b997b085465aa1), which is the measurement RT-IGNORED-PASSING-ROWS-DISPOSITION D0 exists to consume. Already with the operator as a general item; this node is the structural fix and carries the measurement."
---

> ## RELEASED to Team Verify 2026-09-16 — `ready`, size S, tier T2
>
> **One expression changes. Behaviour on pull-request refs is untouched**, which
> is where the cancellation is wanted and works correctly.

## The defect

`.github/workflows/ci.yml:17-19`:

    concurrency:
      group: ci-${{ github.workflow }}-${{ github.event.pull_request.number
                                          || github.ref }}
      cancel-in-progress: true

For a `push` to `main` the group key resolves to `ci-CI-refs/heads/main`.
**Every post-merge run on `main` shares one group, and each new merge cancels
whatever is still running.**

The rationale in the file (`:15-16`) is *"Cancel superseded runs on the same
PR/ref so rapid pushes don't pile up CI."* **That reasoning is correct for a PR
ref and false for `main`:**

| | a PR ref | `refs/heads/main` |
|---|---|---|
| what a new push means | the same change, revised | a **different** change |
| the older run was validating | a tree nobody will merge | a tree that **is now in `main`'s history** |
| is the older run redundant? | yes | **no — it is the only record of that SHA** |

**A post-merge run on `main` is not an attempt at something the next merge
supersedes. It is the record of what `main` was at that commit**, and for the
jobs gated `mode == 'full'` it is the only place several measurements are ever
produced.

## The measured cost

    2026-09-13 10:55   7663ad9b924e7b1cb2c0111215b997b085465aa1
                       run 34753101365   sweep COMPLETED: "27 selected; 11 passed."

    2026-09-14 22:28   ff9d0f0ca4562ac38a553fadf325fbe7aab992b2
                       run 34904280362   CANCELLED ~5s in
                       sweep cancelled, all 8 shards cancelled

    2026-09-16 04:22   10321a158bc69cf50e0cb753e1096fe10fa43ed1
                       run 35055338045   CANCELLED ~13min in
                       by c4a21bb0fa658477f04e670764485634485dd98f, a DOC-ONLY merge
                       sweep cancelled, all 8 shards cancelled, all native-slow cancelled

⇒ **No completed `ignored-row sweep` exists on `main`'s lineage after
2026-09-13.** `RT-IGNORED-PASSING-ROWS-DISPOSITION`'s `D0` consumes exactly that
job, so the node has been unable to take its own measurement for three days
without anyone noticing the mechanism.

**The second instance is the instructive one.** The cancelling merge was
*legitimately* doc-only and correctly flagged. **A doc-only merge collides in
this group exactly as hard as a `crates/` merge would**, because the group key
does not depend on what either change contains. No publisher-side guard
prevents this, and `PUB-DOC-ONLY-UNVALIDATED-AGAINST-ITS-DIFF` would not have
caught it — that node is about a flag disagreeing with its diff, and here the
flag was right.

## The closure

**Stop cancelling `push` runs. Keep cancelling `pull_request` runs.**

    cancel-in-progress: ${{ github.event_name == 'pull_request' }}

`cancel-in-progress` accepts an expression, so the group key does not need to
change. Queueing on `main` is bounded by the merge rate of a single serialised
publisher, which is the property the `no merge_group` comment at `:7` already
relies on.

**An equivalent fix is to make the push-side key per-SHA** (`github.sha` instead
of `github.ref`). Either is acceptable; **the expression is preferred** because
it states the intent — cancellation is a PR-iteration optimisation — rather than
encoding it in a key.

## Acceptance

**AC-1. Two `push` runs on `main` coexist.** Control: show that a run started at
one commit reaches completion after a later commit has been pushed to `main` and
started its own run. A replay of the 04:22/04:35 sequence is the exact case.

**AC-2. PR-ref cancellation is UNCHANGED.** Control: push twice in quick
succession to a PR branch and show the first run is still cancelled. **This AC
is what keeps the change a scoping fix rather than a removal** — the cancelling
behaviour is correct and wanted there.

**AC-3. The stale rationale comment is corrected.** `:15-16` currently justifies
the behaviour for "the same PR/ref" without distinguishing them. Say which ref
kind cancels and why the other does not.

**AC-4. No change to any job, matrix, or gate condition.** Control: the diff
touches the `concurrency` block and its comment only.

**AC-5. No new decorative glyphs.** The file predates the 2026-08-01 rule.

## Note for whoever picks this up

**Do not also "fix" the sweep's non-blocking posture or its exit-status
handling** while in this file. `ignored-row sweep (findings non-blocking)` is
deliberately non-blocking and that is not in scope here.

**`#3676`'s standing prohibition is unaffected and still binds:** cause 2 is the
gate working, and nobody repairs it toward green. It survives that PR's closure.
