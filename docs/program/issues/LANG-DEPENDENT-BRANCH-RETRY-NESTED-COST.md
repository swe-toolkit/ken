---
id: LANG-DEPENDENT-BRANCH-RETRY-NESTED-COST
title: "A failing nested indexed peel costs about 4x per nesting level since 76064f39c (2.3x before): the retained-then-eliminated retry in check_dependent_branch_regimes reruns the whole arm, including each nested method's own retry. Bound the retry so refusal cost stays linear in depth with an unchanged diagnostic"
status: closed
owner: language
size: M
tier: T1
gate: architect
depends_on: [LANG-NESTED-ARM-STALE-OUTER-COORDINATE]
blocks: []
github: null
origin: "Adversary M8 finding evt_1beqd0ntawbqc (F2) on LANG-SOLVED-INDEX-SUBSTITUTION (76064f39c): an undesirable-behavior gap, not a correctness defect. Steward-filed per COORDINATION section 2."
---

# Nested retry cost in dependent-branch regimes

## Objective

A refused nested `Vec` peel of depth 7 costs no more than it did on
`60f4527f2` (about 255 ms), with the same diagnostic.

## Fixed inputs (main `d8ec57615`, Adversary `evt_1beqd0ntawbqc`)

- **Measured.** `deepD`, with a type error at the leaf, on a 256 MiB
  stack. Refusal time at depth 3, 5 and 7: 6.4, 46 and 255 ms before the
  merge; 22.7, 336 and 4520 ms after. The accepted twins are unchanged,
  at about 46 ms for depth 7.
- **Attributed by elimination, not instrumented:** the retained attempt
  in `check_dependent_branch_regimes` rolls back and reruns the arm
  eliminated. That reruns every nested method's own retry. On main
  `218387582` the function is at elab.rs ~:9431, the retained attempt at
  ~:9455 and `snapshot.rollback` at ~:9484.
- **Base moved.** `45b7dfeeb` (LANG-NESTED-ARM-STALE-OUTER-COORDINATE)
  and `6a00e978c` changed `elab.rs` after the measurement. The timings
  above are from `d8ec57615`.
- **The fixture.** `deepD` is the Adversary's probe in `evt_1beqd0ntawbqc`,
  not a file in the repo. The D0 rebuilds it from that report.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

D0 first, at `218387582`: rebuild `deepD`, re-measure refusal time at
depths 3, 5 and 7, and instrument the retry count per depth to confirm
the attribution. Then bound the retry, for example by not retrying a nested
method whose retained attempt already failed under the same outer
regime. Accepted verdicts and diagnostics do not change.

## Acceptance

- **AC-1.** Refusal time for `deepD` at depth 7 is within 2x of the
  pre-merge 255 ms, on the same stack. The diagnostic is byte-identical.
- **AC-2 (differential).** The retry count grows linearly in depth,
  measured at depths 3, 5 and 7, so a constant-factor speedup fails. The
  counts are pinned in a test. Time is measured locally and is not
  asserted in CI.
- **AC-3.** `lang_solved_index_substitution.rs` is unchanged and green.

## Stop conditions

- Bounding the retry changes any accept or refuse verdict.
- The instrumented count does not support the attribution.
- The D0 re-measure at `218387582` no longer shows the regression.

Checks: 4 fired: the anchors were re-read on `218387582`, and the base
moved since the measurement, so the D0 re-measures first. 8 fired: a
constant-factor speedup would also pass AC-1, so AC-2 pins the count's
growth.

## Closeout

Closed without merge, superseded by LANG-SOLVED-INDEX-EQUATION-UNIFIER
(Steward scope call `evt_1xjmh1w0g8nxa`, on Architect `evt_730agekm23atv`).

- The D0 at `2cb7c6ec4` reproduced the cost: 13, 121 and 1093 retries at
  d = 3, 5 and 7. Each failing level runs the arm body on three distinct
  states: the retained attempt, its refined-goal fallback and the
  eliminated regime. There is no exact pruning, so AC-2 cannot be met by
  trial selection.
- The proposed skip refuses some programs the full search accepts, and it
  fired 0 times across 48 binaries, so the corpus cannot certify it.
- The cost affects refusals only. Accepted programs are unchanged.
- The structural fix, selecting the regime before the body is elaborated,
  is carried to stage 2 as its AC-4. Evidence:
  `/workspaces/ken/local/architect-shared/retry-cost-d0/`.
