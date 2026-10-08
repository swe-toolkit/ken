---
id: RT-NESTED-RESPONSE-OWNER-CALLER
title: "The ignored rt_span_prov_native:355 row refuses at ObjectEmission and, once planned, traps -1 natively, because response owners whose K returns a Vis outside the pending-Vis record protocol are dropped from it silently and emitted Ret-only. Classify every response owner in one stored, validated settlement plane with no silent drops, behaviour-preserving; the planning refusal and the relay settlement that un-ignores the row are RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT"
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
validated plane with no silent drops, and the emitter selects its arm from
that classification alone. Every native row keeps its `c7c05d4e6` result.
The span row
`sp_a_foreign_span_freeze_rejects_own_span_succeeds_on_both_engines`
(`crates/ken-cli/tests/rt_span_prov_native.rs:355`) stays ignored with its
baseline reason. The planning refusal and the un-ignore belong to
`RT-NESTED-RELAY-VIS-NATIVE-SETTLEMENT`, whose D0 reads this plane.

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
   `-1`. The trap is the **mixed** shape: relay Vis 361 beside installed
   Vis 528. A pure-relay owner settles natively through the handler-owned
   local continuation drive (`core.rs` ~6918-6950, gated on
   `deferred_response_at_vis` and `bounded_deferred_response_handler_owner`;
   pinned by `abi_s6_mapping_surface_native.rs:723`), Architect
   `evt_cv76gfj95f8a`.
3. **D1h: done, H2** (`evt_nzs10vn3w8t0`, `evt_6vzphntgv506e`). The
   interpreter reaches relay Vis 361 through owner 4: Context(1)'s K
   returns it forwarding the pattern-bound operation `Var(1)` (FsReadAt),
   the driver dispatches it and resumes the IH, and the K later reaches
   installed Vis 528 by row 5 / Context(6). Steward scope decision
   `evt_2kmfzsvqj53zg`, ruled `evt_14w6eh1d2hwk4`: split. Relay
   settlement is the successor.
4. **Refusal withdrawn** (Architect `evt_ak61svpj64m6`, `evt_3h4b9cm28nfks`;
   §1a at 5). Member shape does not separate the trapping owners from the
   settled ones: ABI-S6 owner 0 is mixed and runs green at baseline, while
   span owner 4 and r2 owners 2 and 3 are mixed and trap. A sound refusal
   needs a discriminant read from the settling plane (the owner loop at
   `units.rs:3256` or the handler-owned drive at `core.rs` ~6918), which is
   the successor's D0. This WP lands (a) only.
5. **R-CHAIN-1 (production, behaviour-preserving).** The exact code is in
   Architect `evt_14w6eh1d2hwk4`, with the emitter arms of
   `evt_ak61svpj64m6`.
   - **(a) Stored classification.** `ResponseOwnerSettlement` (`Protocol`,
     `RetOnly`, `Excluded { reason: Relay | Underived(text) }`) for every
     installed response owner, in `plan.pending_vis_settlements`
     (`PendingVisSettlements { protocol, owners }`).
     `pending_vis_record_protocol()` becomes
     `build_pending_vis_settlements()`, and its two silent drops
     (`returned_vis.rs:125`, `:129`) become classifications: Err is
     `Underived`, all contexts empty is `RetOnly`, `excluded_by_relay` is
     `Relay`, and otherwise `Protocol`. It is built once, immediately after
     phase B's owner check and before the `:1520` rebuild. It is validated
     by rebuild-equality in closure, beside `:2070`, with "pending-Vis
     settlement plane is not the exact closed protocol derivation".
   - **Consumers.** Source (e) (`static_transition.rs:876`) and the
     lowering seed (`core.rs:2879`) read the field. Lowering gains
     `response_owner_settlements`. The emitter (`units.rs` ~4553) selects
     its arm from the classification alone, every arm explicit: `Protocol`
     with successors takes the protocol arm; `RetOnly` and `Excluded` with
     no successors take the baseline Ret-only arm, which is what these
     owners took when they were dropped. Whether an `Excluded` owner's Vis
     is settled natively or traps is the successor's to decide. Any other
     combination is a backend error.
   - **No planning refusal.** No `admit_response_owner_settlements`. The r2
     rows (`rt_escape_second_resource_native.rs` :861, :913, :970) and their
     doc comments keep their `c7c05d4e6` text. The span row and :1015 keep
     their baseline ignore reasons.

## Acceptance

- **AC-2 (plan pins).** 8 Specialized rows plus Vis 1079 Deferred with
  `ContinuationBodyTail`, and owner 6 still Specialized, through the
  feasibility diagnostics.
- **AC-3 (mutation, QA).** Removing the D1 predicate reddens the row.
- **AC-5b (mutation, QA).** Moving the seat install back reddens the row.
- For both: the first refusal may have moved because the settlement
  closure now runs before the old gates. Re-measure on this build and pin
  each mutation's exact first refusal text in the handoff. A result
  identical to the unmutated row's is a stop.
- **AC-6 and AC-6b.** The seat-162 record set gains exactly the 48 S2
  tuples, and the seat-plane bytes are unchanged against `c7c05d4e6`,
  observed through a planner hook.
- **AC-R1 (mutation).** Perturbing one stored member's `base_owner` reddens
  the closure validator.
- **AC-R2.** `build_pending_vis_settlements` is called only at the builder
  site and by the validator. `returned_vis_protocol` keeps its diagnostics
  caller (`:1354`).
- **AC-R6 (behaviour preservation and stop).** Across all 39 native
  suites, every row's result equals `c7c05d4e6`: pass, fail, ignored, or
  refusal text. **Stop** on any difference. SEQUENTIAL and
  `one_bracket_retains_native_parity` are among them.
- **Census (input to the successor's D0, not a gate).** Per owner: its
  class. Per `Excluded` owner, its member count. Per member: `relay`,
  `installed`, successor id, whether `deferred_response_at_vis(origin)` is
  present, and whether `bounded_deferred_response_handler_owner` is
  present and equal to the K's emission owner. Reported in the handoff.
- **AC-4 (controls).** The five sibling rows stay green, the px8ta rows keep
  their labelled failures, and the `units.rs:7831` coverage gate is not
  relaxed.

AC-1 (span-row parity), AC-5, AC-R3 and the planning refusal move to the
successor. AC-R4, AC-R5, AC-R7 and AC-R8 are dropped.

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
   and the emitter reads the gap as Ret-only. Closed by (a) as a
   stored classification; refusal and parity are the successor's.
4. The planner refusal was keyed on returned-member shape (the relay flag)
   without reading the plane that settles relays natively (the
   handler-owned deferred-response drive). Narrowed to mixed owners
   (`evt_cv76gfj95f8a`; §1a at 4).
5. The narrowed refusal was keyed on relay/non-relay member shape, while
   the native settling of non-relay members is decided by the handler-drive
   plane, which was unread. Refusal moved to the successor
   (`evt_ak61svpj64m6`; §1a at 5).

Shared predicate (entries 1-3, Architect `evt_7yy6d7gkm48xy`; entries 3-5
share it, `evt_ak61svpj64m6`): the chained
settlement has no single plan object, so each consumer derives it from its
own plane, and a settlement one consumer discards silently reads to the
next as a different answer.

## Stop conditions

- The repair touches the bracket tree, moves a held ref
  (`wp/RT-BRACKET-PRODUCER-AUTHENTICITY`, `wp/RT-BRACKET-SETTLEMENT-PLANE`,
  `4b4c8565c`, `21c039918`, `7f1a04a40`), or widens the lookup at
  `core.rs:11695`.
- Any kernel, trust or spec change.
