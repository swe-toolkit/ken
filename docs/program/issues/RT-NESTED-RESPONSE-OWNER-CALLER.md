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

   **Passed** (Architect `evt_46qkytaf6tx33`, on `evt_4rp1njqq3fvs5`): no
   input is written after the check, and the keyed census matched 48/48.
   The `-1` is owner 4's guard: Response(4) for Vis 1496 in
   funcid61/block673. Its K, Context(2), which is also owner 0's K, returns
   a tag that is not the planned Ret word `0x101d_0000_002a`. Owner 0's
   successor is entered and its unknown-member exit is not taken.
2. **D1g: done, G3** (`evt_37n0bngpgej4s`, ruled `evt_7yy6d7gkm48xy`).
   Owner 4's K is Context(1)/funcid66, not Context(2), so G1 and G2 are
   void. Owner 4's Ret-only body receives the installed Vis 528 (row 5)
   from its K. One relay member, Vis 361, excluded the whole owner:
   `returned_vis_protocol(Vis1496)` is Ok with members, but
   `pending_vis_record_protocol` skips owner 4 at `returned_vis.rs:129`
   (`excluded_by_relay ... continue`). The emitter (`units.rs:4572-4625`)
   reads the absence as Ret-only and emits the `require_i64` that returns
   `-1`. Lowering has no relay arm, so the planner fails open.
3. **D1h (measurement only, on `b135b25b5`, probes restored).** Report:
   - **the reference semantics:** the interpreter alone on the span row,
     through a probe that skips the native-first ordering. For each
     invocation of owner 4's K, which returned Vis occurs (361, 528 or
     both), and how the interpreter settles each one: the handler or row,
     and the owner the result flows to;
   - **Context(1)'s census:** each member with its relay flag, successor
     row and k_context; the context-to-funcid map for Contexts 1 and 6; and
     which binder relay Vis 361's operation `Var` forwards;
   - **the whole-owner rule's provenance:** the commit and ruling event
     that introduced "A relay makes the WHOLE owner ineligible"
     (`returned_vis.rs:326`), from `git log -S`.

   Outcomes, ruled in advance:
   - **H1.** The interpreter reaches only 528 for owner 4; 361 is present
     but not reached. R-CHAIN-1 gains clause (c) below, and the Architect
     rules its exact code once the provenance item shows the whole-owner
     rule's rationale does not forbid it.
   - **H2.** The interpreter reaches relay 361 through owner 4. Relay
     settlement is a new mechanism. **Stop**; the Steward decides the
     scope.
   - **H3.** Anything else. **Stop.**

   No guard is relaxed, and the `0x101d_0000_002a` word is not edited.
4. **R-CHAIN-1 (production; its edits wait for D1h and the Architect's
   ruling on it).** It closes all three inventory entries.
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
     `UnclassifiedRuntimeTrap { -1 }`.
   - **(a) No silent drop.** The stored plane holds a classification for
     every response owner, as an enum with no `_` arm: `Protocol(..)`,
     `RetOnly` (the K census has zero returned-Vis members) and
     `Excluded { reason }` (a relay, or a swallowed Err with its text).
     The emitter selects its arm from this classification only.
   - **(b) Fail closed at planning.** An `Excluded` owner whose census has
     any non-relay installed member is refused at planning with "response
     owner K returns an installed Vis outside a pending-Vis protocol". It
     is never emitted Ret-only.
   - **(c) Mixed owners, on H1 only.** Non-relay members enter the
     pending-Vis record path as owner 0's do. Each relay member gets an
     explicit planned arm in the owner loop that traps through a
     planner-catalogued trap ("relay Vis returned to a pending-Vis
     owner"), interned like `malformed_dynamic_constructor_trap`.

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
- **AC-R5.** On `b135b25b5`'s plan with clause (c) disabled, the span row
  gets the exact (b) planner refusal, not `-1`.
- **AC-4 (controls).** The five sibling rows, SEQUENTIAL with its
  plan-row pins, and `one_bracket_retains_native_parity` stay green. The
  px8ta rows keep their labelled failures. The `units.rs:7831` coverage gate
  is not relaxed.

## SYMPTOM INVENTORY

From Architect `evt_6d7m1z6sy31h4`: three hard stops in the owner-0
settlement chain (`evt_1fvvjj40kk6s9`, `evt_1dynjvvkh7mnf`,
`evt_5v223fdev9eq7`). Clean checkpoint `b135b25b5`. Research advisory
`evt_1834zja7bes7c`; D1f ruled `evt_46qkytaf6tx33`.

1. The ownership authority (`inline_synthesized_seat_emission_owners`)
   was keyed on response disposition, while emission was keyed on unit
   and target selection. Fixed by source (e) from the pending-Vis
   protocol.
2. Source (e)'s protocol gate was keyed on the host-effect seat plane,
   which was installed after ownership. Fixed by reordering the seat
   install.
3. After the planner closes, the linked native run traps `-1`. Measured
   (D1g): one relay member excludes owner 4 from the protocol silently,
   and the emitter reads the gap as Ret-only.

Shared predicate (all three, Architect `evt_7yy6d7gkm48xy`): the chained
settlement has no single plan object, so each consumer derives it from its
own plane, and a settlement one consumer discards silently reads to the
next as a different answer.

## Stop conditions

- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or widens the lookup at
  `core.rs:11695`.
- Any kernel, trust or spec change.
