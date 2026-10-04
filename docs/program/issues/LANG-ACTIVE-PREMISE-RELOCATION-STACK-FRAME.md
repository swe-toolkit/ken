---
id: LANG-ACTIVE-PREMISE-RELOCATION-STACK-FRAME
title: "relocate_active_premise_term spends about 22.5 KB of stack per level of the term it walks, so relocating a deep checked term during a nested dependent match overflows the 2 MiB test worker: the Vector package with the lookup_zip_with proof needs 2.56 MiB against 2 MiB on main. Cut the per-level cost so the walk's stack does not scale with that frame size"
status: ready
owner: verify
size: S
tier: T1
gate: architect
depends_on: [VERIFY-CALL-SITE-PRECONDITION-DISCHARGE]
blocks: [CAT-VECTOR-DEFERRED-LAWS]
github: null
origin: "Architect ruling evt_2xpqwxn09fkah on the CAT-VECTOR-DEFERRED-LAWS item 5 resource stop: the decision fixed in advance. Foundation D0 evt_4kp6w90qgapq0 measured that the repeating cycle is not the nested-match chain, so increment 5 is blocked on an elaborator per-level frame reduction. Placed in the Verify ring after VERIFY-CALL-SITE, because the Language ring holds LANG-REFINEMENT-INTRODUCTION-OBLIGATION. Steward-filed per COORDINATION section 2."
---

# Active-premise relocation fits the default stack

## Objective

Relocating an active premise through a deep checked term does not overflow
the default 2 MiB libtest worker. The Vector package with the
`lookup_zip_with` proof loads within that worker with margin, without a stack
change.

## Settled inputs (Foundation D0 `evt_4kp6w90qgapq0`, at `61fde2caf`)

- **The measurement.** `ken check` of `Vector.ken.md` was run under a stated
  `ulimit -s`, at 256 KiB steps, with the same freshly built debug binary.
  - Base source passes at 2048 KiB and aborts at 1792.
  - WIP source `4e2ab6520` passes at 2560 and aborts at 2304.
- **The cycle.** LLDB at 2048 KiB stops inside
  `relocate_active_premise_term` (`elab.rs:8228`).
  - Frames #0 to #112 are that function, its `go` closure, and the iterator
    collection of an `Elim`'s `params`, `methods` and `indices`. There are 40
    direct frames.
  - The direct-call/closure pair costs 45,152 bytes, from the unwound `rsp`
    deltas. Frames #0 to #115 take 1,804,936 bytes.
  - 55 more frames lie below #112.
- **The callers.** `ActivePremiseEmbedding::translate_from_original`
  (`:8368`) is called from `kernel_check_in_context_current`. That chain
  runs under `check_generalized_branch_goal`,
  `check_match_dependent_refined_fallback`, `check_dependent_branch_body`
  and `check_match_dependent_mode`. The nested matches call the relocation,
  but they are not the repeating cycle, so factoring the proof into private
  helpers does not help (Architect `evt_2xpqwxn09fkah`).
- **Call sites.** There are four: `:7593`, the recursive `go` at `:8237`,
  `:8379` and `:8460`.
- **Stated stacks.** No `RUST_MIN_STACK`, no test-thread stack, and no
  `ulimit` change is a repair.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch; do not build around it.

## Deliverable

`relocate_active_premise_term`'s stack per level of term depth is cut to a
small fraction of today's 22.5 KB. It must not grow with the number of
`Term` variants. The relocation result is unchanged for every input. The
technique belongs to the ring and the Architect. Two candidates are
outlining the arms into `#[inline(never)]` helpers, and an explicit work
stack. Report the per-level bytes before and after, measured from the `rsp`
deltas as D0 did. Then re-measure the frames below #112. If another
recursive walk on the same caller chain now dominates, it is part of this WP
(check 7).

## Acceptance

- **AC-1 (the failing observation).** Use the debug `ken` binary built from
  the candidate. `ken check` of `Vector.ken.md` with the WIP source
  `4e2ab6520` passes at a stated `ulimit -s` of 1536 KiB. The same
  measurement aborts on the `origin/main` binary. Report the bisected
  minimum for base and WIP source.
- **AC-2 (durable control).** A test relocates a synthetic term of stated
  depth on a thread with a stated stack. The fix passes it, and restoring
  the old per-level frame overflows it. Name the depth, the stack and the
  red output in the handoff.
- **AC-3.** Relocation output is identical: the elaborator suites that
  reach `translate_from_original` stay green at the default stack. Those
  include the LANG-SIBLING-GOAL rows, the CALL-SITE rows, and the four call
  sites' existing tests. CI is green.

## Stop conditions

- The cut needs a change to `Term`, to the kernel, or to the relocation's
  semantics.
- After the cut, the Vector WIP still fails 1536 KiB, and the dominant cycle
  is outside this caller chain. Stop to the Architect with its composition.
