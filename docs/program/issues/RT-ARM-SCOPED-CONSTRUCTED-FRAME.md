---
id: RT-ARM-SCOPED-CONSTRUCTED-FRAME
title: "give each runtime-selected branch arm its own creation-site constructed-context frame and exact callable-body identity, so a recursive position inside the selected arm can supply its ProducerLocal captures without admitting the nonselected sibling or inventing entry-ABI membership -- the representation the four px7l/px7m rows lack (RT-CONTEXT-CAPTURE-CLAIM-ABSENCE outcome B). Design/feasibility first: D0 inventory and a component sketch to the Architect before any code; not a promise that all four rows clear"
status: ready
owner: runtime
size: L
gate: none
tier: T1
depends_on: [RT-CONTEXT-CAPTURE-CLAIM-ABSENCE]
blocks: []
github: null
origin: "Architect sizing ruling evt_5mfgjmrhyrwcp (answering Steward evt_37qy3rx513f7s): independent of the held bracket control-region tree and of parked px8ta; mechanism class is a creation-site, arm-scoped constructed frame. Restructuring admitted by operator ruling 2026-09-21 and sized by the Steward. Steward-filed per COORDINATION section 2."
---

# Each selected arm needs its own constructed frame

## Settled inputs -- Architect `evt_5mfgjmrhyrwcp`

- Rows: both in `crates/ken-cli/tests/px7l_checked_host_recursive_bind.rs`
  (selected-body, selected-result) and both in
  `crates/ken-cli/tests/px7m_hostresult_computational_match.rs` (`Ok`/`Err`
  matches). Each stops at `L1`
  `recursive_position_captures_all_planner_recoverable` with a correct
  `BoundaryCarrier` arity refusal. Behind it sit `L2`
  `agreeing_recursive_body_unit` and `L3` `resolve_context_capture_claim`.
  `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE` showed that the missing `L3` claim is
  correct under the current ABI.
- The mismatch is **branch-selected creation-site constructed-context frame
  and callable-body identity**. Today's one frame is keyed to a sibling
  `worker_body_origin`, `L1` queries another, and `L2` refuses distinct
  declared units. The dynamic-err row's two bodies differ in content, so
  equating them or picking one would be unsound.
- **Mechanism class: a creation-site, arm-scoped constructed frame.** Not a
  fabricated `ProducerLocal` predeclared `EntryFrame` claim, and not a bare
  generated-context `Some`.
- Starting interfaces, NOT evidence that selected-arm transport works:
  - the `core.rs` construction (`~10800-10912`);
  - matching admission (`~13613`);
  - the `calls.rs` validated constructed route (`~988-1050`).
- Neither the held bracket control-region IR (acquisition to settlement order)
  nor parked px8ta (response-owner liveness) supplies the selected arm's
  activation-local captured operands or exact callable target.

- **This node also owns the `L2` relation question.**
  `agreeing_recursive_body_unit` (`core.rs:1230`) compares declared-unit
  identity. `RT-CONTEXT-FRAME-LABEL-CORRECTION` `§D3` called the gap between
  node identity and body equality "real and should be repaired", and filed no
  owner. `RT-CONTEXT-CAPTURE-CLAIM-ABSENCE` calls `L2` correct, but its
  evidence (the unit test near `:1271`) compares ids only, so it passes under
  both readings. Adversary `evt_132c307qm7jeg`.

## Deliverables

- **D0, before any code.** On current `main`, for each of the four fixtures,
  inventory **both** branch alternatives: the exact
  `(continuation origin, recursive position, worker body origin,
  caller/activation)`, and the creation-site constructed frame's separate
  worker-capture and context-capture runs. Join each alternative to the
  **actual** `L1` queried body and its eventual retarget key. Record
  `frame present / matching / absent` per alternative. Distinguish the three
  bodies that render equal but are distinct from dynamic-err's genuinely
  differing ones. A single global frame or debug-byte equality is a negative,
  not a route.
- **D1, sketch to the Architect before code.** Using D0, show whether the
  selected activation can supply its own complete frame to its exact call:
  - without admitting the nonselected sibling;
  - without inventing entry-ABI membership;
  - without changing `agreeing_recursive_body_unit` for paths that still rely
    on one unit.
  D1 also answers the `L2` question: either the arm-scoped frame makes the
  identity comparison right, because each arm gets its own unit, or the
  Architect rules `:1230` correct. The candidate then corrects whichever
  record is wrong (`§D3` there, or the exemption here).
- **D2, only after the Architect approves D1.** Build it. Un-ignore each row it
  clears. Rewrite the label of each row it does not clear to the measured
  stop. In the same candidate, correct the stale owner comment on
  `predeclared_entry_frame_slot`
  (`planning/static_transition/continuations.rs:~4334`, "`D4b` owns making
  such a value capturable"): `D4b` was an admission closeout, and this node
  is the owner.

## Acceptance criteria

- **AC-1.** The `L1`, `L2` and `L3` refusals and `calls.rs::gather_cannot_serve`
  are unchanged. A control shows that a nonselected sibling's frame is still
  refused at the selected call.
- **AC-2.** Every un-ignored row passes differentially
  (`assert_native_matches_interpreter` plus the row's own assertions).
  Native-only green does not count.
- **AC-3.** The handback reports D0 per row and per alternative, not as a sum.
- **AC-4.** Targeted builds only, through `scripts/ken-cargo`. No-regression
  means green in CI.

## Stop conditions

- If D1 cannot be proved without a new cross-boundary callable carrier, STOP
  and return the measured dependency to the Steward. It is an operator-facing
  residual.
- **Not authorized:** repurposing the bracket control-region IR, changing
  `gather_cannot_serve`, bypassing `L1`/`L2`/`L3`, a fabricated entry-frame
  claim, or picking one of two differing bodies.
- **Held work:** never move `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the
  child-2 checkpoint, and do not edit `planning/static_transition/responses.rs`.
