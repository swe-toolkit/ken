---
id: RT-CHECKED-SUCCESSOR-EMIT-REACHABILITY
title: "RT-ITREE D1 follow-up — the D1 gate widening (core.rs:12546 dropped the answer_route==CheckedSelectedRecursor conjunct) makes every strict two-case Ret/Vis topology lower a checked successor body UNCONDITIONALLY, even for a Direct/None-frame eliminator where that successor is runtime-dead at control word 0. That production lowering can turn a pre-D1 Ok into Err before the dead branch is selected (Architect-proven: Unsupported(Var, 'no runtime binding for index 2') on a Direct/None recursive-Ret shape that compiled pre-D1). A latent compile-acceptance regression in the widened RuntimeExpr lowering domain; not a confirmed checked-source product regression. Guard the checked-successor emission on proven Checked-control emit-reachability, OR single-lower the Ret body behind a join fed by both payloads. Fold adversary Findings 2 (missing Initial!=ActiveSelfResumption witness) and 3 (tautological status assert_ne)."
status: ready
owner: runtime
size: M
gate: none
depends_on: []
blocks: []
github: null
origin: "Architect disposition evt_kgcdt4837jt4 (thr_6fe6wp65996a8), 2026-08-26, on the adversary M8 post-land hunt (evt_wxfvb470pxh1) of the RT-ITREE D1 squash 21d621303. The landed D1 merge STANDS (CI green, both gates bound byte-identical to reviewed candidate 7b1820194) and D2 is NOT interrupted; this is a separately-framed non-blocking follow-up. Steward-filed per COORDINATION section 2, sequenced after the in-flight D2 on the single runtime ring."
---

> # NON-BLOCKING FOLLOW-UP, SEQUENCED AFTER D2 (Architect evt_kgcdt4837jt4)
>
> The landed D1 route slice ([[RT-ITREE-DEFAULT-SELECTION-PROVENANCE]],
> merged 21d62130) STANDS — this does NOT reopen it. This node closes a
> latent compile-acceptance regression the Architect proved on top of the
> landed D1, arising from the SAME gate widening. It is `ready` but
> SEQUENCED AFTER the in-flight D2 [[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]]
> on the single runtime ring — do NOT dispatch until the D2 object-read cycle
> completes. No `depends_on` D2: this needs no D2 deliverable; the ordering is
> ring contention, not a logical dependency.

## Problem (Architect-proven, exact code argument)

Before D1, `return_case` existed ONLY when the compile-time `answer_route`
was `CheckedSelectedRecursor`. After D1 (core.rs:12546), every strict two-case
Ret/Vis topology lowers the checked successor body, then lets the runtime
control word make that successor DEAD for a Direct edge. Production omits the
`#[cfg(test)]` `checked_frame_id.expect` (adversary Finding 1's confirmed
panic is test-build only), but production does NOT omit `case_body_occurrence`,
`lower_expr` / `lower_computational_producer_expr`, `carried_join_arm`, or
their `?` propagation. So runtime deadness does NOT protect COMPILATION from an
`Err` produced while emitting that block.

The Architect ran the named discriminator against exact pre-D1 `5272a68d4` and
landed `21d621303` (mirroring production under the test build by turning only
the cfg(test) provenance `expect` into a no-op when the frame id is absent;
production already compiles that block out):

- Canonical simple shape (Ret has one nonrecursive binder, body `Var(0)`):
  compiled on landed D1. The widened path is NOT unconditionally broken.
- Discriminating shape (same Direct/None carried header + exact Ret+Vis
  topology, but Ret has one recursive position and its ordinary body uses the
  child at `Var(2)`): compiled pre-D1; landed D1 returned
  `Unsupported(Var, "no runtime binding for index 2")`. The ordinary Ret arm
  has `[IH, child, frame-env…]`, while the newly emitted DEAD checked successor
  supplies only `[whole-scrutinee, frame-env…]`. Exact command:
  `scripts/ken-cargo test -p ken-runtime --lib architect_probe_direct_none_frame_ret_vis_carried_match_compiles -- --nocapture`
  (pre-D1 passed, landed failed exit 101).

The synthetic recursive-Ret shape is NOT evidence that checked Ken source can
naturally produce the defect (canonical ITree `Ret` should be nonrecursive), so
this is NOT a live product regression and D1 is NOT reopened. But it
mechanically refutes the universal safety claim: the widened production
lowering CAN turn a pre-D1 `Ok` into `Err` before the runtime-dead branch is
selected. A simple canonical positive does not close the population of return
bodies or affine/planner failures.

## Repair authority (Architect-bound)

PROHIBITED (both reintroduce or mis-key the D1 defect):

- Do NOT restore the old `eliminator.answer_route == CheckedSelectedRecursor`
  conjunct — the shared header is compiled under an initial Direct edge while
  `ActiveSelfResumption` may lawfully arrive Checked; the conjunct reintroduces
  the exact D1 bug.
- Do NOT use `checked_frame_id.is_some()` as route selection — frame presence
  does not distinguish the two predecessor edges.

REQUIRED — one of:

- (a) Prevent lowering a checked successor UNLESS the closed
  header-predecessor population proves that exact Checked control is
  emit-reachable; or
- (b) Lower the Ret continuation body ONCE behind a join fed by the ordinary
  projected payload AND the checked whole-answer payload.

Form (b) is PREFERABLE if buildable — it removes the duplicate lowering and its
affine/planner risk rather than merely proving one dead branch absent. If (b)
is not buildable, hard-stop with the object.

## Deliverables

- The checked-successor emission repaired per form (a) or (b) above, so a
  Direct/None-frame strict Ret/Vis carried match no longer lowers a
  compile-failing dead checked successor, while the genuine mixed
  Initial-Direct / Active-Checked program still emits and selects the checked
  successor from exact edge authority.
- The five controls below.

## Acceptance criteria

- AC-1 (Direct/None default, no dead lowering) — a Direct + None-frame +
  strict Ret/Vis carried fixture proves default behavior and NO
  checked-successor body lowering; its test instrumentation does not panic.
- AC-2 (mixed program preserved) — the existing mixed Initial-Direct /
  Active-Checked full program remains green and proves the checked successor is
  still emitted and selected from exact edge authority.
- AC-3 (regression discriminator) — a mutation that unconditionally lowers the
  checked successor recreates the pre-D1-`Ok` / post-D1-`Err` discriminator
  (the `Unsupported(Var, …)` shape); byte-restore afterward.
- AC-4 (closed predecessor census) — the predecessor population stays exactly
  `Initial` + `ActiveSelfResumption`; no frame / origin / family / body-shape
  inference becomes route authority.
- AC-5 (fold adversary Findings 2 and 3) — add a genuinely differing
  Initial-vs-Active edge witness (one fixture where the two edges carry
  DIFFERENT control words, so a swap between the two
  `carried_computational_loop_control_word` call sites, core.rs:12216 Active vs
  12232 Initial, reddens); and replace the tautological
  `assert_ne!(success.status.code(), trapped.status.code())`
  (object_linker_packaging.rs:3397) with a non-vacuous expected-outcome
  relation.
- AC-NO-REGRESSION — whole-suite green in CI; local targeted `-p ken-runtime` /
  `-p ken-cli` only, never `--workspace`.

## Reviewers

Architect (the repair is form (a) emit-reachability or form (b) single-lower
behind a join — NOT the prohibited conjunct restoration or
`checked_frame_id.is_some()` route selection; the closed predecessor census is
preserved; no body-shape/frame/origin inference becomes route authority) +
runtime-qa (AC-1..AC-5 controls red/green as specified; AC-3 recreates the
exact `Unsupported(Var,…)` discriminator; the mixed program stays green). A
design gap in choosing form (a) vs (b) HARD-STOPS to the Architect.

## Capability tier

T1 — a soundness-bearing lowering repair with a design fork between two
Architect-named forms (emit-reachability guard vs single-lower-behind-a-join,
the latter carrying affine/planner-identity risk), reviewed on the provenance
argument, not a differential diff. Size M.

## Sequencing

Lane-1 (runtime, priority), NON-BLOCKING, SEQUENCED AFTER the in-flight D2
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] on the single runtime ring. Do
NOT interrupt the D2 continuous turn to start this; the Steward kicks it once
the D2 object-read cycle completes. Adversary Findings 2 and 3 are folded here
(AC-5), not cut as separate nodes. No relation to PX8, which stays blocked
until the whole native carried-value program lands.
