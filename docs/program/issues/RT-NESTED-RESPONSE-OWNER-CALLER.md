---
id: RT-NESTED-RESPONSE-OWNER-CALLER
title: "The ignored rt_span_prov_native:355 row refuses at ObjectEmission: a forward-declared response owner (the second BufferFreeze) has no verified selected incoming call, because its selected caller is planned inside the first BufferFreeze's specialization and lowering records no disposition for it. Measure where that caller site is lowered, then repair so the caller carries a recorded disposition and the row passes on both engines"
status: active
owner: runtime
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Architect fresh-D0 boundary ruling evt_v7p1c6t7kmw6 on main ff0a35f24 (Steward request evt_396hg4b5bbpc0): the span row is not bracket-tree work (9 Specialized rows, 0 Deferred). Re-homed from RT-COMPMATCH-TREE-SCRUTINEE inventory line 1; RT-IGNORED-ROWS-NEXT-GROUP left it with no successor. Serves the L1 objective (operator 2026-09-17). Steward-filed per COORDINATION section 2."
---

# A nested response owner's caller has a disposition

## Objective

`sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
(`crates/ken-cli/tests/rt_span_prov_native.rs:355`) is un-ignored and passes
on both engines.

## Settled inputs (Architect `evt_v7p1c6t7kmw6`, at `ff0a35f24`)

- **The refusal.** ObjectEmission: "a forward-declared response owner has no
  verified selected incoming call: owner=StaticResponseOwnerId(0),
  context=ContinuationContextId(2), preexisting=false". The caller's token
  has emission_owner Specialization(1), producer construct origin 1079,
  alternative 1, target Specialization(2), and `disposition=None`.
- **The population.** 9 Specialized rows, 0 Deferred. Owner 0 is the
  BufferFreeze response at Vis 1079 (k-spec 2, new context). Its selected
  caller is planned inside Specialization(1), the k-spec of the other
  BufferFreeze response (Vis 1147, preexisting context 0).
- **Not the bracket tree.** The bracket rows (px8ta) ride the Deferred
  forward-Ret route. This row has no Deferred rows, and its failing owner is
  a Specialized BufferFreeze, not a settlement.
- **The instrument.** The coverage gate at `units.rs:7831` reports the
  miss. A forward declaration never satisfies it.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

**D1 ruling (Architect `evt_1qzcc8q84xk0w`, on D0c `evt_49wn8cq76k0f3`):
a planning selection defect.**
- **What D0c showed.** Eight of nine owners settle at a consumer seat in
  their emission Function: by direct claim (owner 5) or at a producer
  match's bypassed-candidate bridge (owners 1-4 and 6-8). Owner 0's
  producer, Vis 1079, is the body root of the continuation worker
  (funcid66), so it has no seat.
- **The predicate.** In `planning/static_transition/responses.rs` (phase
  B), a row whose selected caller has
  `producer_construct_origin == worker_body_origin(emission_owner)` is not
  Specialized.
- **The classification.** It goes to the Deferred residual under a new
  `DeferredResponseSubCase::ContinuationBodyTail`. Every total match over
  that enum is updated explicitly, with no `_` arm.
- **The fences.** No lowering change. The coverage gate, the license, the
  effect guard and the escaped-K guard are unchanged.
- **Measure in order.**
  - D1a: the plan has 8 Specialized rows plus Vis 1079 Deferred, and
    owners 1-8 are unchanged.
  - D1b: the row builds and runs at parity.
  - If D1b refuses elsewhere or diverges, stop and report it verbatim. The
    alternative, owner chaining, is ruled only on that evidence.

## Acceptance

- **AC-1.** `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
  is un-ignored and green: both engines exit 0 with the exact freeze
  sequence it asserts.
- **AC-2 (plan pins).** The test asserts 8 Specialized rows plus Vis 1079
  Deferred with `ContinuationBodyTail`, and owner 6 (a Specialization
  emission owner) still Specialized.
- **AC-3 (mutation, QA).** Removing the predicate brings back owner 0 and
  the exact D0 refusal (`owner=StaticResponseOwnerId(0)`,
  `context=ContinuationContextId(2)`, `disposition=None`).
- **AC-4 (controls).** The five sibling rows in `rt_span_prov_native.rs`,
  the SEQUENTIAL distinguishable witness and its plan-row pins, and
  `one_bracket_retains_native_parity` stay green. The px8ta rows keep
  their labelled failures. Any other row whose Specialized or Deferred
  classification moves is listed with its cause. The `units.rs:7831`
  coverage gate is not relaxed.

## SYMPTOM INVENTORY

From Architect `evt_6d7m1z6sy31h4`: three hard stops in the owner-0
settlement chain (`evt_1fvvjj40kk6s9`, `evt_1dynjvvkh7mnf`,
`evt_5v223fdev9eq7`). Clean checkpoint `b135b25b5`. A research advisory
and D1f are pending, and the recut scope follows them.

1. The ownership authority (`inline_synthesized_seat_emission_owners`)
   was keyed on response disposition, while emission was keyed on unit
   and target selection. Fixed by source (e) from the pending-Vis
   protocol.
2. Source (e)'s protocol gate was keyed on the host-effect seat plane,
   which was installed after ownership. Fixed by reordering the seat
   install.
3. After the planner closes, the linked native run traps `-1`, a
   malformed token. The cause is not yet localized.

Shared predicate (1 and 2; 3 provisional): owner 0's settlement is a
cross-owner chained settlement that no plan object names, so each
consumer re-derives the owner of seat 162 from its own plane.

## Stop conditions

- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or widens the lookup at
  `core.rs:11695`.
- Any kernel, trust or spec change.
