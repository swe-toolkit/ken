---
id: LANG-DEPENDENT-BRANCH-RETRY-NESTED-COST
title: "A failing nested indexed peel costs about 4x per nesting level since 76064f39c (2.3x before): the retained-then-eliminated retry in check_dependent_branch_regimes reruns the whole arm, including each nested method's own retry. Bound the retry so refusal cost stays linear in depth with an unchanged diagnostic"
status: draft
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
  in `check_dependent_branch_regimes` (elab.rs ~:9405) rolls back (~:9458)
  and reruns the arm eliminated. That reruns every nested method's own
  retry.

Treat anchors as perishable. If a fixed input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

D0 first: instrument the retry count per depth to confirm the
attribution. Then bound the retry, for example by not retrying a nested
method whose retained attempt already failed under the same outer
regime. Accepted verdicts and diagnostics do not change.

## Acceptance

- **AC-1.** Refusal time for `deepD` at depth 7 is within 2x of the
  pre-merge 255 ms, on the same stack. The diagnostic is byte-identical.
- **AC-2 (differential).** The retry count grows linearly in depth,
  measured at depths 3, 5 and 7, so a constant-factor speedup fails.
- **AC-3.** `lang_solved_index_substitution.rs` is unchanged and green.

## Stop conditions

- Bounding the retry changes any accept or refuse verdict.
- The instrumented count does not support the attribution.
