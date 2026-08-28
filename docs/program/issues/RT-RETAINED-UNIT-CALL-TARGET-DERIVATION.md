---
id: RT-RETAINED-UNIT-CALL-TARGET-DERIVATION
title: "Call-target-resolution successor — a retained body (StaticOriginId) has no graph-derived call target in its unit, so object emission refuses at calls.rs:1638 (call_declared_unit / unit_calls map) after M3's effect-seat crossing succeeds"
status: ready
owner: runtime
size: M
gate: none
tier: T1
depends_on: [RT-CARRIED-IH-DISPATCH-SITEOP]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect object-distinctness ruling (evt_317adj9ebfw86) on M3 WIP 3e9821c5d. M3's effect-seat crossing succeeds; px8f then stops at object emission because a retained body's call target is not derived in the unit call graph. DISTINCT call-graph-derivation object — NOT M3's effect-seat object, NOT the worker_calls finite-static-apply facet (calls.rs:1605), NOT reject_carried_residual_arguments (core.rs:2935). New node, Steward framing call per COORDINATION section 2."
---

> # Call-target-resolution successor — SECOND of M3's two successors (DRAFT stub)
>
> Node minted on the M3 accept-COMPLETE-for-object disposition
> (Steward evt_3v7t4qcp9m8gt). Sequenced SECOND and distinct; this cut is larger —
> it crosses planner ownership and function-local call-target derivation. The
> first sibling was recut 2026-08-25: [[RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT]] is
> `closed`/falsified (Architect hard-stop #3, evt_1vhmndq7fscd1) and replaced by
> [[RT-DYNAMIC-CONSTRUCTOR-DISPATCH-PROVENANCE]]. That ruling creates NO
> dependency or sequencing change for this node (Architect: it remains distinct);
> Steward owns lane order.
>
> **FRAMED AND `ready` (Steward, 2026-08-28), re-anchored to origin/main
> `f4045946d`.** The dependency [[RT-CARRIED-IH-DISPATCH-SITEOP]] is `merged`, so
> nothing gates this node. It is NOT gated on
> [[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]]: that chain is a different
> boundary (continuation binding), and this one is unit-call-graph derivation.
> The single ring is the only reason it waits — release it when the runtime seat
> frees.
>
> **The stub's "clean the stale 'Ignored pending M3' narrative comment"
> instruction is DISCHARGED** — that comment is gone from the tree; the row's
> `#[ignore]` now names this node directly. Do not go looking for it.

## Objective

After M3's effect-seat crossing succeeds, px8f object emission stops at `retained
body StaticOriginId(1236) has no graph-derived call target in this unit`. The unit
call graph does not derive a call target for a retained body. **Derive that target
from existing graph/claim authority before emission**, so object emission
completes and the row runs end-to-end.

## Fixed inputs

Re-measured by the Steward at origin/main `f4045946d`, with provenance separated
from inheritance. **Do not treat an inherited number as a current measurement.**

CURRENT-TREE, verified at `f4045946d`:

- The refusal is at
  `crates/ken-runtime/src/cranelift_backend/lowering/calls.rs:1638`, inside
  `call_declared_unit` (declared at `calls.rs:1624`), where
  `self.function_local.unit_calls.get(&body_origin)` returns `None`.
- A DISTINCT sibling lives in the same file: `call_declared_unit_target` at
  `calls.rs:1828`. Establish which of the two owns the derivation before editing
  either.
- Trigger row: `crates/ken-cli/tests/px8f_buffer_native.rs:201`,
  `linked_checked_write_all_observes_short_progress_and_matches_interpreter`. It
  is the ONLY `#[ignore]` in that suite, and its ignore reason names this node
  and this refusal.

INHERITED from Architect `evt_317adj9ebfw86` measured at `5fff430db`, NOT
re-verified by the Steward: the active owner at the lookup is
`Specialization(ContinuationSpecializationId(2))`, whose `unit_calls` keys are
`800, 1008, 1321, 1374`, missing the retained `StaticOriginId(1236)`. **These are
the numbers D0 must reproduce or correct.** The Steward did not re-run the native
witness: it is a native-codegen build competing for this box with the runtime
seat's active T1 work, and it is D0's job, not framing's.

DISTINCTNESS, still binding. This object is NOT: (a) M3's effect-seat
marshalling object; (b) the foldable `worker_calls[body]` finite-static-apply
facet on a different path (`calls.rs:1605`, `call_boundary_closure_environment`,
whose distinct message is "the statically selected boundary closure body has no
declared target in this function"); (c) `reject_carried_residual_arguments`
(`core.rs:2935`, [[RT-SITEOP-CARRIED-WITNESS]] D2).

## Design judgment, front-loaded (Architect ruling, binding)

**The target must be DERIVED from existing graph/claim authority, never
synthesized.** Specifically forbidden as a source of the target: the lookup miss
itself, numeric-origin coincidence, and retained-body shape. A derivation that
consults the miss to decide what to return is the defect this node exists to
avoid, not a repair of it.

**The fail-closed lookup stays as the backstop.** Widening or deleting the
refusal so the miss stops occurring is not a repair — it removes the instrument.
Wrong-target and ambiguous-target outcomes must be REJECTED, not resolved by
preference.

## Deliverables

- **D0 — measure before changing anything.** At the release SHA, reproduce the
  refusal on the trigger row and report the ACTUAL active owner, the ACTUAL
  `unit_calls` key set, and the ACTUAL retained `StaticOriginId`. If they differ
  from the inherited `Specialization(ContinuationSpecializationId(2))` /
  `{800, 1008, 1321, 1374}` / `1236`, the frame's inherited numbers are wrong and
  D0 corrects them. **A D0 that finds the refusal no longer reproduces is a
  SUCCESS that stops and returns coordinates** — it does not authorize inventing
  a derivation for a refusal that is not there.
- **D1 — derive the specialization-owned call target** from existing graph/claim
  authority, before emission reaches the lookup. Name the authority the
  derivation reads and show it was already present.
- **D2 — controls**, below. Ambiguity and wrong-target rejection are part of the
  deliverable, not follow-up work.

## Acceptance criteria, each with its control

- **AC-DERIVE.** The trigger row runs end-to-end and its `#[ignore]` is removed.
  Control: the row must FAIL before D1 and pass after, on the same base.
- **AC-NO-SYNTHESIS.** A compile-preserving mutation that derives the target from
  the lookup miss, from numeric-origin coincidence, or from retained-body shape
  must REDDEN a control. **Two-sided, as the corpus requires: apply the evasion,
  show the build still succeeds, and show the corrected control reddens.** A
  control that cannot fail under this mutation has not been demonstrated to
  discriminate and does not satisfy this AC.
- **AC-FAIL-CLOSED.** The `calls.rs:1638` refusal still fires, unwidened, for a
  body whose target genuinely is not derivable. Control: a witness that still
  reaches the refusal after D1.
- **AC-AMBIGUOUS.** Two legal candidate targets for one retained body are
  REFUSED, not disambiguated by preference or by first-match. Control: a witness
  driving the ambiguous case to its named refusal.

## Contention check

Touches `crates/ken-runtime/src/cranelift_backend/lowering/calls.rs` and
`crates/ken-cli/tests/px8f_buffer_native.rs`. The in-flight
[[RT-RESULT-CONTINUATION-BINDING-PROVENANCE]] consumer works
`lowering/source.rs` and the planner's static-transition aggregates, so the file
sets are disjoint on current evidence. **Re-check at release** — that consumer
has recut eight times and its footprint has moved before.

## Sequencing

`ready`. Not blocked by any node. Release when the runtime seat frees; the
Architect reviews the WP at release. Estimated tier T1 — the work turns on the
derive-versus-synthesize argument, not on a differential diff.
