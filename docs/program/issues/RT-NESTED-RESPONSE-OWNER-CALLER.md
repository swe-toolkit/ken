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

**Recut (Architect `evt_2091hd1wtkhs5`, on Research `evt_1834zja7bes7c`;
§1a at 3).** The WP id is kept. One thing is replaced: each consumer's own
derivation of the chained settlement. Everything already proved is
retained.
- **Retained:** the D1 `ContinuationBodyTail` predicate
  (`evt_1qzcc8q84xk0w`); source (e) (`evt_38ayfhrhqq8q3`), which changes
  only what it reads; the seat-install move from `b135b25b5`.

1. **R-CHAIN-0 (D0, measurement only, on `b135b25b5`).** This folds in
   D1f (`evt_6d7m1z6sy31h4`):
   - the keyed census of S2/seat-162 records at `:1520` and at closure,
     against the D1e TSV;
   - where the `-1` comes from: the function, owner and block, the source
     of the dispatch token, the last effect, and whether funcid67 and the
     owner-0 pending loop are entered;
   - the SEQUENTIAL and one_bracket controls;
   - the single writer (file:line) of each protocol input:
     `static_response_continuations`, the return-protocol candidate rows,
     the continuation contexts, the emittable units and call edges, and
     `host_effect_seats`.

   **Stop** if any of those inputs is written after phase B's owner check
   (`construction.rs` around 1497-1515).
2. **R-CHAIN-1 (production).**
   - A stored, validated plane, `plan.pending_vis_settlements`. It is
     built once, after phase B's owner check and before the `:1520`
     rebuild.
   - It is validated by rebuild-equality at install and in the whole-plan
     closure. The error reads "pending-Vis settlement plane is not the
     exact closed protocol derivation".
   - Ownership source (e) and the lowering seed (`core.rs:2879`) read that
     field. `pending_vis_record_protocol()` keeps only three callers: the
     builder, the validator and the diagnostics.
   - The frame discriminant that `emit_pending_vis_owner_loop` writes, and
     the arm that reads it, both come from the stored row id. A discriminant
     that matches no arm becomes a catalogued, fail-closed trap, never
     `UnclassifiedRuntimeTrap { -1 }`. The Architect rules the site on
     R-CHAIN-0.

## Acceptance

- **AC-1.** `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
  is un-ignored and green: both engines exit 0 with the exact freeze
  sequence it asserts.
- **AC-2 (plan pins).** 8 Specialized rows plus Vis 1079 Deferred with
  `ContinuationBodyTail`, and owner 6 still Specialized.
- **AC-3 (mutation, QA).** Removing the D1 predicate brings back the exact D0
  refusal (owner 0, context 2, `disposition=None`).
- **AC-5, AC-5b, AC-6 and AC-6b.** As ruled in `evt_38ayfhrhqq8q3` and
  later, retained.
- **AC-R1 (mutation).** Perturbing one stored member's `base_owner` reddens
  the closure validator.
- **AC-R2.** `pending_vis_record_protocol()` has no caller outside the
  builder, the validator and the diagnostics.
- **AC-R4 (mutation).** A discriminant mutation yields the catalogued trap,
  not `-1`.
- **AC-4 (controls).** The five sibling rows, SEQUENTIAL with its
  plan-row pins, and `one_bracket_retains_native_parity` stay green. The
  px8ta rows keep their labelled failures. The `units.rs:7831` coverage gate
  is not relaxed.

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
