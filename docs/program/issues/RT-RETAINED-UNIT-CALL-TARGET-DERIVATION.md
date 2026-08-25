---
id: RT-RETAINED-UNIT-CALL-TARGET-DERIVATION
title: "Call-target-resolution successor — a retained body (StaticOriginId) has no graph-derived call target in its unit, so object emission refuses at calls.rs:1638 (call_declared_unit / unit_calls map) after M3's effect-seat crossing succeeds"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-CARRIED-IH-DISPATCH-SITEOP]
blocks: []
github: null
origin: "Steward, 2026-08-25, from the Architect object-distinctness ruling (evt_317adj9ebfw86) on M3 WIP 3e9821c5d. M3's effect-seat crossing succeeds; px8f then stops at object emission because a retained body's call target is not derived in the unit call graph. DISTINCT call-graph-derivation object — NOT M3's effect-seat object, NOT the worker_calls finite-static-apply facet (calls.rs:1605), NOT reject_carried_residual_arguments (core.rs:2935). New node, Steward framing call per COORDINATION section 2."
---

> # Call-target-resolution successor — SECOND of M3's two successors (DRAFT stub)
>
> Node minted on the M3 accept-COMPLETE-for-object disposition
> (Steward evt_3v7t4qcp9m8gt). Sequenced SECOND, after
> [[RT-EXITCODE-FAILURE-PAYLOAD-TRANSPORT]] (Architect evt_4kkspzs62gtn6: no fold,
> no technical dependency; this cut is larger — it crosses planner ownership and
> function-local call-target derivation). Frame it when ExitCode lands.
>
> RE-ANCHORED to landed origin/main 5fff430db (Architect re-ran the witness at
> that exact commit; object unchanged). Updated grounding: the production refusal
> is `calls.rs:1631-1640`; the active owner at the lookup is
> `Specialization(ContinuationSpecializationId(2))`, whose `unit_calls` keys are
> `800, 1008, 1321, 1374` — not the retained `StaticOriginId(1236)`, hence the
> miss. The eventual frame must DERIVE the missing specialization-owned target
> from existing graph/claim authority BEFORE emission — never synthesize a target
> from the lookup miss, numeric-origin coincidence, or retained-body shape;
> preserve the fail-closed lookup as the backstop and pin wrong/ambiguous-target
> rejection (Architect ruling). Also clean this witness's stale "Ignored pending
> M3" narrative comment when framed.

## Objective

After M3's effect-seat crossing succeeds, px8f object emission stops at `retained
body StaticOriginId(1236) has no graph-derived call target in this unit`. The unit
call graph does not derive a call target for a retained body. Derive it so object
emission completes and the row runs end-to-end.

## Fixed inputs (Architect evt_317adj9ebfw86, object DB `f0292222`)

- The refusal is at
  `crates/ken-runtime/src/cranelift_backend/lowering/calls.rs:1638` inside
  `call_declared_unit`, where `self.function_local.unit_calls.get(&body_origin)`
  returns None — the general declared-unit call-graph derivation (`unit_calls`
  map).
- DISTINCT from: (a) M3's effect-seat marshalling object; (b) the foldable
  `worker_calls[body]` finite-static-apply facet on a different path (calls.rs:1605,
  `call_boundary_closure_environment`, distinct message "the statically selected
  boundary closure body has no declared target in this function"); (c)
  `reject_carried_residual_arguments` (core.rs:2935,
  [[RT-SITEOP-CARRIED-WITNESS]] D2). It is unit-call-graph retention/derivation
  work — M3's crossing let emission proceed to a retained body whose call target the
  unit graph did not derive.
- Trigger row: `crates/ken-cli/tests/px8f_buffer_native.rs:200`
  (`linked_checked_write_all_observes_short_progress_and_matches_interpreter`).
  Re-pointed to this node by M3's finalization.

## Sequencing

Draft. Gated behind M3's crossing (depends_on). Full WP frame + release queue
behind M3's landing; the Architect reviews the WP at release.
