---
id: RT-NESTED-RESPONSE-OWNER-CALLER
title: "The ignored rt_span_prov_native:355 row refuses at ObjectEmission and, once planned, traps -1 natively, because response owners whose K returns a Vis outside the pending-Vis record protocol are dropped from it silently and emitted Ret-only. Classify every response owner in one stored, validated settlement plane and refuse at planning every owner the protocol cannot settle; the relay settlement that un-ignores the row is RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT"
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

Every response owner's settlement route is classified once, in a stored and
validated plane. An owner the pending-Vis protocol cannot settle is refused
at planning with a typed error instead of trapping `-1` natively. The span
row `sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
(`crates/ken-cli/tests/rt_span_prov_native.rs:355`) stays ignored, with
that refusal as its reason. `RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT`
un-ignores it.

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
3. **D1h: done, H2** (`evt_nzs10vn3w8t0`, `evt_6vzphntgv506e`). The
   interpreter reaches relay Vis 361 through owner 4: Context(1)'s K
   returns it forwarding the pattern-bound operation `Var(1)` (FsReadAt),
   the driver dispatches it and resumes the IH, and the K later reaches
   installed Vis 528 by row 5 / Context(6). Steward scope decision
   `evt_2kmfzsvqj53zg`, ruled `evt_14w6eh1d2hwk4`: split. This WP lands
   (a) and (b); relay settlement is the successor.
4. **R-CHAIN-1 (production).** The exact code is in Architect
   `evt_14w6eh1d2hwk4`.
   - **(a) Stored classification.** `ResponseOwnerSettlement` (`Protocol`,
     `RetOnly`, `Excluded { reason: Relay | Underived(text) }`) for every
     installed response owner, in `plan.pending_vis_settlements`
     (`PendingVisSettlements { protocol, owners }`).
     `pending_vis_record_protocol()` becomes
     `build_pending_vis_settlements()`, and its two silent drops
     (`returned_vis.rs:125`, `:129`) become classifications. It is built
     once, immediately after phase B's owner check and before the `:1520`
     rebuild. It is validated by rebuild-equality in closure, beside
     `:2070`, with "pending-Vis settlement plane is not the exact closed
     protocol derivation".
   - **Consumers.** Source (e) (`static_transition.rs:876`) and the
     lowering seed (`core.rs:2879`) read the field. Lowering gains
     `response_owner_settlements`. The emitter (`units.rs` ~4572) selects
     its arm from the classification alone, and any other combination is a
     backend error, not a Ret-only body.
   - **(b) Refusal at planning.** `admit_response_owner_settlements()`, run
     in `finish` right after `self.plan.validate()?`, refuses every
     `Excluded` owner, mixed or not, with "response owner .. K returns a
     Vis outside the pending-Vis protocol (..)". It runs after closure, so
     every plane is built and validated first.

## Acceptance

The implementer confirms each pin can be observed through the existing
test-support hooks. A pin that cannot be observed before `finish` returns
Err moves to the successor and is reported.

- **AC-2 (plan pins).** 8 Specialized rows plus Vis 1079 Deferred with
  `ContinuationBodyTail`, and owner 6 still Specialized, through the
  feasibility diagnostics.
- **AC-3 (mutation, QA).** Removing the D1 predicate brings back the exact D0
  refusal (owner 0, context 2, `disposition=None`), distinct from (b).
- **AC-5b (mutation, QA).** Moving the seat install back yields the
  `closure.rs:2055` refusal, distinct from (b).
- **AC-6 and AC-6b.** The seat-162 record set gains exactly the 48 S2
  tuples, and the seat-plane bytes are unchanged against `c7c05d4e6`,
  observed through a planner hook.
- **AC-R1 (mutation).** Perturbing one stored member's `base_owner` reddens
  the closure validator.
- **AC-R2.** `build_pending_vis_settlements` is called only at the builder
  site and by the validator. `returned_vis_protocol` keeps its diagnostics
  caller (`:1354`).
- **AC-R5.** The span row gets exactly the (b) refusal, naming owner 4 with
  `Relay`, and its ignore reason is that text.
- **AC-R6 (census and stop).** A classification census (counts of
  `Protocol`, `RetOnly`, `Excluded{Relay}` and `Excluded{Underived}`) over
  the native suites run locally. SEQUENTIAL, `one_bracket_retains_native_parity`
  and every native row green on `c7c05d4e6` stay green. **If a previously
  green row newly refuses under (b), stop** and report the row and its
  classification.
- **AC-4 (controls).** The five sibling rows stay green, the px8ta rows keep
  their labelled failures, and the `units.rs:7831` coverage gate is not
  relaxed.

AC-1 (span-row parity), AC-5 and AC-R3 move to the successor. AC-R4 is
dropped with clause (c).

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
   and the emitter reads the gap as Ret-only. Closed by (a) and (b) as a
   typed refusal; parity is the successor's.

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
