---
id: CI-DOC-ONLY-SHORT-CIRCUIT
title: "Detect doc-only PRs by changed-path classification and short-circuit the expensive CI matrix to a required-green pass, saving GitHub Actions compute given the doc-to-code PR ratio; any PR touching crates/catalog/spec/conformance or a CI-control file forces the full pipeline. The classifier must REPORT every required status-check context as success on the doc-only path (skipped != green under branch protection), must be PATH-based (any crates/ change, including a comment-only .rs edit, is full CI so a /// doctest is never mis-skipped), must be a SELF-CONTAINED path taxonomy whose short-circuit set is a subset of non-compiled/inert paths (decoupled from the publisher's caller-asserted --doc-only; amended 2026-08-30), and must FAIL CLOSED (any error/ambiguity runs full CI)."
status: ready
owner: verify
size: M
gate: none
tier: T2
depends_on: []
blocks: []
github: null
origin: "Operator request 2026-08-30: 'can doc-only PRs be detected in CI and short-circuit to a pass? can we implement a changed-path classifier to do so? this would save significant resources for GH, given the proportion of doc-only PRs to crate/catalog PRs. note: the classifier should also include files which control CI.' Serves the operator's standing priority-1 CI-cost directive (the CI-NATIVE-PARITY-DURATION family). Sibling to CI-SHARD-DURATION-BALANCE on a distinct axis: that node reduces the DURATION of the pipeline; this node avoids RUNNING the pipeline at all on doc-only PRs. Steward-filed per COORDINATION section 2."
---

> # READY — RELEASED to the verify ring (lane 2). The queue condition is met:
> # CI-SHARD-DURATION-BALANCE landed and closed at M7 `8b887de17` (2026-08-30).
> #
> # Operator, 2026-08-30: "frame it and queue it behind shard-balance." That node
> # has closed, so this is now the verify ring's active node.
> #
> # RE-MEASURE against CURRENT main, which now INCLUDES the CI-SHARD-DURATION-
> # BALANCE landing — the required-check topology and the CI-control file set moved.
> # The shard fan-in the sibling node added lives under the denylist (CI-control
> # files force full CI); confirm the exact required-context list against the
> # current workflow before designing the reporting shape (D1). This node's own
> # candidate touches CI-control files, so it is itself in the denylist and MUST
> # run full CI — a doc-only short-circuit of THIS PR would be a bug.
> #
> # AMENDED 2026-08-30 (hard stop `evt_30y7y1y4sdd84`): the classifier is
> # SELF-CONTAINED and DECOUPLED from the publisher `--doc-only` flag (which has no
> # classifier — it is caller-asserted). AC-CLASSIFIER-SINGLE-SOURCE is replaced by
> # AC-SKIP-SET-SAFE-SUBSET; D2 (shared taxonomy) is dropped; D1 is the whole node.
> # The publisher and the classifier are orthogonal mechanisms — see design point 3.

## The waste this closes

Every doc-only PR (frames, tracker, `agent/`, `library/`) currently triggers the
full GitHub Actions matrix — build, test, conformance, and the workspace shards —
even though the publisher already merges it with `--doc-only` and ignores the
result. The CI compute is spent and thrown away. Given the doc-to-code PR ratio,
short-circuiting doc-only PRs to a required-green pass is a large, recurring GH
compute saving.

## Design judgment front-loaded (read before cutting)

**1. The required-check gotcha — skipped is NOT green.** Under branch protection a
required status-check context that is *skipped* stays "expected/pending" and
blocks the merge forever; you cannot simply `paths-ignore` the workflow away on
doc-only. The classifier MUST cause every required context to REPORT `success` on
the doc-only path. Two shapes, pick per the measured topology (D1):
- a single required umbrella check that always runs cheaply and internally either
  fans out to the expensive jobs (code) or reports itself green (doc-only); or
- keep the current required contexts and add a lightweight job that emits each
  required context name as `success` on the doc-only branch.
**Measure the current required-check topology first** (one aggregate context vs
many named contexts, and exactly which contexts branch protection requires) — the
reporting shape depends on it, and getting it wrong wedges every doc-only PR.

**2. PATH-based, not content-based.** Treat ANY `crates/` change as full CI,
including a comment-only `.rs` diff — a `///` comment can carry a compiled,
executed doctest, so "comment-only in a code file" is not safe to skip. A
path-based classifier sidesteps that trap entirely: the doc-only set is a
whitelist of non-compiled paths, and anything else is full CI.

**3. The classifier is a SELF-CONTAINED authority, decoupled from the publisher
`--doc-only` flag (Steward amendment 2026-08-30, hard stop `evt_30y7y1y4sdd84`).**
The original framing was factually wrong: the publisher
(`scripts/scripted-pr-automerge.sh`) has NO changed-path classifier — it accepts a
caller-supplied `--doc-only` and trusts it, and `merge-procedure.md:143-149` is
human guidance that even treats a comment-only `.rs` as `--doc-only`-eligible. That
is a DIFFERENT mechanism: the CI classifier decides whether the expensive matrix
RUNS; the publisher `--doc-only` is a caller judgment on whether the merge WAITS
for CI. They are orthogonal and must not be forced to share a taxonomy. The only
safety coupling is ONE-DIRECTIONAL: the classifier's short-circuit (skip) set must
be a subset of genuinely non-compiled/inert paths, so CI never reports
green-without-running for anything a normal (non-`--doc-only`) merge would need
validated. Being STRICTER than the publisher's liberal prose is safe; the
classifier owns its own path taxonomy and does not reuse the publisher's.

**4. FAIL CLOSED.** A classifier that errors, cannot determine the changed paths,
or is ambiguous must run FULL CI — never short-circuit on uncertainty. A
classifier bug that let a code change skip CI would merge unvalidated code; that
failure mode is unacceptable and the safety property is that the short-circuit is
only ever taken on a positively-proven doc-only diff.

## The classifier taxonomy

- **Forces full CI (denylist — any hit runs the real pipeline):** `crates/`,
  `catalog/`, `spec/`, `conformance/` fixtures, AND CI-control files —
  `.github/workflows/*`, the CI/publisher scripts under `scripts/`, `nextest` /
  shard configuration, `Dockerfile`s and base-image pins. A CI-control change is
  exactly what you cannot validate by short-circuiting it, so it must run.
- **Doc-only (short-circuit — all changed paths must fall here and nowhere in the
  denylist):** `docs/`, `*.md` anywhere outside the denylist, `agent/`,
  `library/`, and other non-compiled documentation paths.
- The rule is set-disjointness: doc-only iff (changed-paths subset of allowlist)
  AND (changed-paths intersect denylist == empty). The allowlist is this node's
  OWN definition (not the publisher's); confirm its exact members at D1 against the
  live repo layout rather than hard-coding a guess here.

## Deliverables

**D1 — classifier + short-circuit + required-context reporting.** Measure the
current required-check topology, then add the changed-path classifier to CI: on
a positively-proven doc-only diff, skip the expensive jobs and report every
required context `success`; otherwise run the full pipeline. Fail closed. D1 is
the core and is independently mergeable.

**D2 — none (DROPPED by the 2026-08-30 amendment).** The original D2 factored a
shared taxonomy between the CI classifier and the publisher `--doc-only`. That
coupling is withdrawn (design point 3): the classifier is self-contained, so D1 is
the whole node. If, while building D1, a concrete case shows the publisher's
caller-asserted `--doc-only` can actually bypass a validation the classifier proves
is needed, that is a hard stop to the Steward (a separate publisher-policy
question), NOT a deliverable here.

## Acceptance criteria (each carries its own control)

- **AC-DOC-ONLY-GREEN.** A PR whose changed paths are all in the allowlist and
  disjoint from the denylist short-circuits: the required check(s) report green
  and the expensive build/test/conformance/shard jobs do NOT execute. Control: on
  such a PR, those jobs are measurably skipped/absent (not merely fast), yet every
  required context is green and the PR is mergeable.
- **AC-CODE-FORCES-FULL.** A PR touching any denylist path runs the full pipeline.
  Control (two arms): a PR touching a CI-control file (e.g. `.github/workflows/*`)
  runs full CI, NOT short-circuited; a comment-only `.rs` change under `crates/`
  runs full CI (the doctest-safety case). Both must be exhibited.
- **AC-REQUIRED-CONTEXTS-REPORTED.** The short-circuit reports EVERY required
  status-check context branch protection expects, as success — none left pending.
  Control: enumerate the required contexts; a doc-only PR shows all green;
  removing the reporting for one context leaves it pending and blocks the merge
  (proving the reporting is load-bearing, not incidental).
- **AC-SKIP-SET-SAFE-SUBSET.** The classifier's short-circuit (skip) set is a
  subset of genuinely non-compiled/inert paths: every path that can carry compiled
  or executed code (anything under `crates/`, INCLUDING a comment-only `.rs`, and
  every denylist path) forces full CI. Control: for each denylist category, a PR
  touching only it does NOT short-circuit; and exhibit the boundary showing no
  allowlist member reaches a compiled/executed artifact (e.g. a `///` doctest lives
  under `crates/`, which is denylist, so it can never enter the skip set).
- **AC-FAIL-CLOSED.** The classifier runs full CI on any error, ambiguity, or
  inability to determine the changed paths. Control: a malformed/empty diff, an
  unresolvable base, or an injected classifier error forces full CI, never a green
  short-circuit.

## Returns a hard stop (to the Steward/Architect) if

- branch protection's required contexts cannot be made to report green on a
  short-circuit without disabling a protection the fleet relies on (a
  policy/topology wall, not a coding task);
- a concrete case shows the publisher's caller-asserted `--doc-only` can bypass a
  validation this classifier proves is needed (e.g. a comment-only `.rs` doctest
  merged without CI) — that is a SEPARATE publisher-policy question; route it to the
  Steward, do not fold a publisher-semantics change into this node.

## Reviewers, sequencing, contention

- **Reviewer:** independent Verify QA (all five AC controls, both arms of
  AC-CODE-FORCES-FULL and AC-FAIL-CLOSED proven) and the Architect (the
  required-check reporting and fail-closed correctness surface is real — this is
  where a defect merges unvalidated code). Publisher CI is the gate; a resolved
  merge Decision is required. No Conformance Validator (CI infrastructure, not
  kernel/conformance).
- **Sequencing:** QUEUED behind CI-SHARD-DURATION-BALANCE; released to the verify
  ring when that node closes. D1 is the whole node (D2 dropped by the amendment). Note the
  self-consistency check: because this node edits CI-control files, its own
  candidate is in the denylist and MUST run full CI — a doc-only short-circuit of
  this PR would itself be a bug.
- **Contention:** touches `.github/workflows/*` and CI/publisher scripts under
  `scripts/` — coordinate any publisher-path edit with the Steward/lieutenant. No
  crate/catalog contention with the concurrent lanes. Targeted local checks only;
  the real behavior (required-context reporting, job skipping) is provable ONLY in
  CI on GitHub.
