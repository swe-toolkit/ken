---
id: LANG-ELAB-RECURSIVE-FRAME-BUDGET
title: "Logic inlined into the elaborator's recursive cycle (check, infer, check_dependent_branch_body) regressed the stack three times on 2026-10-04, and each regression surfaced only as overflowing CI shards. Reduce the prelude's stack floor and guard the cycle's frame budget with a test that fails at review"
status: ready
owner: verify
size: M
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
blocks: []
github: null
origin: "Operator 2026-10-04 (approve): the prelude stack-floor reduction with a recursive-frame guard as its durable control. Architect carry evt_4pkgt0g9sw4x4 on the REFINEMENT stack stop: RELOCATION, the prelude stack floor and REFINEMENT each regressed the stack the same way. Placed on the verify ring after VERIFY-REUSED-ENV-TRUST-RESIDUE, and after REFINEMENT lands because REFINEMENT re-measures the same frames. Steward-filed per COORDINATION section 2."
---

# The elaborator's recursive cycle has a frame budget

## Objective

The prelude loads and the catalog checks with less stack than today. A test
that fails at review, not in a CI shard, holds the recursive cycle's frame
sizes at or below a recorded ceiling.

## Settled inputs (Architect `evt_4pkgt0g9sw4x4`)

- **The cycle.** One nested-match level recurses through `check`, `infer`,
  `check_dependent_branch_body` and its closure in `elab.rs`. On REFINEMENT's
  base the debug frames were 9,672 B, 28,600 B, 808 B and 8,344 B, so one
  level cost 18,824 B. REFINEMENT's ruled outlining brings the level to
  18,600 B.
- **The mechanism.** New logic lands in a frame on the cycle with no budget
  to stop it. The repair that worked three times is to outline the new block
  with `#[inline(never)]`, which moves it off the cycle.
- **Today's control.** The default 2 MiB test worker overflows, and the
  shard goes red. That fails closed but late, and it reads like a semantic
  failure.
- **Unmeasured:** the prelude's stack floor (the smallest worker stack that
  loads the prelude and checks every catalog package), which frames inside
  it are the largest, and how many tests carry a stack override only
  because of it.

Treat anchors as perishable. Re-measure every frame on the base you build
on.

## Deliverable

1. **AC-0, at the start of the repair.** On main after REFINEMENT lands:
   - measure the prelude stack floor;
   - measure the frame of each function on the cycle, plus the largest
     frames the prelude load passes through;
   - list the tests that set a stack override and whether each still needs
     it.

   The Architect then rules which frames to outline and the guard's form.
   One option is a behavioural guard: elaborating a fixed nested-match depth
   fits in a fixed stack. The other is a recorded per-function ceiling.
2. **The ruled reduction and guard.** The guard is a test in the workspace
   that CI already runs. No workflow file changes.

## Acceptance

- **AC-1.** The measured prelude floor drops by the amount the ruling sets.
  Every test whose override existed only for that floor drops its override
  and passes at the default worker.
- **AC-2 (guard falsifier).** Inlining one outlined block back into the
  cycle reddens the guard locally, before any shard overflows.
- **AC-3.** The elaborator suites and the default native targets stay green.

## Stop conditions

- The guard needs a workflow file change. Stop to the Steward: the operator
  designates the author of any CI workflow edit.
- A reduction needs a kernel or spec change.
