---
id: RT-EMITTER-CONTROL-JOINS-SPLIT
title: "Move the control and joins emitter family out of the lowering files -- branch, match and join emission, against the join disposition the planner slice already owns"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-EMITTER-CALLS-RETURNS-SPLIT]
blocks: [RT-EMITTER-AGGREGATES-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 14; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.


> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 14 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/core.rs` and `cranelift_backend/lowering/mod.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The control and joins emitter family.** Branch and match emission, join
emission, and block/terminator construction on the emitter side.

**Item 9 owns the planner half of joins and traps.** `D0` must state the
planner/emitter boundary for every joint symbol so neither slice claims the
other's.

> **Modules own semantic lifecycles.** The durable direction of the whole phase
> is *plan construction -> validated read-only views -> lowering state and source
> machine -> concrete backend mutation -> independent evidence -> closure ->
> publication*. **Do not name a permanent module after a temporary campaign
> node**, and do not size modules to be equal.


# `D0` — THE LEDGER. No code moves in this deliverable.

**Produce the exact old/new symbol ledger and test-property ledger for this
owner**, derived from the Stage A inventories and **re-measured at a named SHA**
on the current tree:

| inventory | what it supplies here |
|---|---|
| [type ownership](../backend-split-census-type-ownership.md) | declaring owner, visibility, external mint-shape files and all external reference files, per type |
| [lifecycles](../backend-split-census-lifecycles.md) | authority and ledger mint / transition / close / terminal sites |
| [re-exports](../backend-split-census-reexports.md) | the 57 re-export statements by build profile |
| [tests](../backend-split-census-tests.md) | `#[test]` functions, mutation surfaces and fixtures |
| [co-change](../backend-split-census-cochange.md) | which files historically move together |

**The census is a starting point, not an authority on today's tree.** It was
taken at a pinned SHA and the tree has moved. **Re-measure every count you rely
on and name the SHA you measured at** — a census row is a claim about when it
was written, not about what the tree now contains.

**State the blind spots you inherit.** The type-ownership selector cannot see
private types, macro-generated declarations, declarations whose visibility and
type keyword are split across lines, traits, constants, functions, or fields.
A ledger that does not say what its selector missed is not a ledger.


## `D0` ledger — IN PROGRESS, not yet endorsed or complete

Measured at `367b846d1` (origin/main at pickup, 0 behind). No code moved. This
section records grounded findings as they are traced, per the campaign's own
durability lesson (item 12's/13's D0 ledgers were built the same way, in
multiple addenda, and this file is the durable artifact if a hand-back is
needed before the ledger closes). Bound files at this SHA: `core.rs` 15,568
lines, `mod.rs` 18,067 lines (both re-measured directly, not inherited from
any prior item's frame or census row — both shrank again under item 13).

### Method used

Production-injection-point tracing (items 11/12/13's discipline), reconciled
against the two nearest LANDED ledgers rather than re-deriving their
domains: item 13's own landed D0/D1 ledger (`docs/program/issues/RT-
EMITTER-CALLS-RETURNS-SPLIT.md`, the calls-and-returns emitter, merged
immediately before this item) and item 9's landed D0 ledger
(`docs/program/issues/RT-PLANNER-JOINS-TRAPS-SPLIT.md`, the joins/traps
planner half, merged earlier). Item 9's own ledger explicitly pre-identifies
several pieces of this item's boundary (see "reconciliation against item 9"
below) — a genuine head start, used as a starting hypothesis and
independently re-verified against the current tree, not taken on faith.

**`control.rs` is byte-identical since item 13's D2 landed** (`git diff
b67c805a2 origin/main -- .../control.rs` is empty at this pickup) — so
this item's AC-2 can reconcile against my own exhaustive item-13 AC-2 read
of the exact same 30,222-line file (all 220 `#[test]` functions
individually read in place during that item's own D0), rather than
re-reading it blind. AC-2's own reconciliation section below states this
explicitly and names what still needs a fresh look.

### Reconciliation against item 9's landed ledger (planner/emitter boundary)

Item 9's own D0 (`de402e255`) resolved the frame's own "planned trap seats,
trap provenance events" prose as **not planner-owned at all** — both live in
`lowering/mod.rs` already, zero presence in `static_transition.rs`. Item 9
explicitly named this as "the emitter's half of the pair (item 14's, per the
frozen predicate)" and flagged three specific `control.rs` tests as
"emitter-owned... item 14's, not this slice's domain":
`typed_trap_exit_preserves_the_planner_identity_across_two_unit_calls`,
`typed_trap_exit_rejects_a_deleted_or_root_misclassified_unit_lane`,
`typed_trap_exit_identity_and_caller_protocol_mutations_are_discriminating`
(all three re-verified present at their item-13-era positions, `control.rs`
being byte-identical since).

**Independently re-traced, not taken on item 9's word alone** (see the
test-property section): the first two are genuinely mixed/shared once their
full apparatus is read (already established during item 13's own D2 sanity
re-read, which this item's AC-2 reconciles against rather than repeating
blind) — the third crosses into item 13's own `TrapCallerProtocolMutation`
axis, already moved to `calls.rs`, so it stays a shared/end-to-end control
regardless of which single item's perspective reads it. Item 9's "item 14's,
not [item 9's]" framing is correct in the planner-vs-emitter sense; it is not
itself a single-owner-to-item-14 verdict, and this ledger does not read it as
one.

Item 9's ledger also named the exact emitter-side consumption sites for the
planner-owned `StaticTransitionPlan` join/trap accessors it retained
(`join_plan_token`, `join_plan_token_if_planned`, `required_join_origins`,
`source_join_origins_in_owner_subtree`, `trap_identity`, `trap_catalog`) —
each at a `lowering/mod.rs`/`lowering/core.rs` line number from its own
pickup SHA. Re-derived those call sites fresh in this item's own trace
below (not copied forward) since both files have shrunk since item 9's
`de402e255`.

### AC-1 — MOVE population traced so far

**Join emission and disposition (`mod.rs`, unless noted):**

- `carried_join_arm` (`core.rs`, already `pub(super)`) — emits a join
  predecessor's carrier-word value at a carried match's merge; calls
  `self.transfer_into_carrier`/`self.emit_process_exit_status` (item 13's,
  already `pub(super)`) and `self.emit_carrier_immediate` (RETAIN, item 15's
  aggregate/carrier domain, cross-module call).
- `append_planned_join_params` (`core.rs`, already `pub(super)`) — appends
  CLIF block params to an already-created merge block from a `JoinPlanToken`.
- `jump_planned_join_arm` (`core.rs`, private) — emits `builder.ins().jump`
  into the merge, dispatching on `JoinResultRepresentation`.
- `finish_planned_join` (`core.rs`, already `pub(super)`) — switches to the
  merge block and recovers the typed result after every predecessor has run.
- `consume_join_plan`, `consumed_join_plan_token` (`mod.rs`, private) —
  mint/reborrow a `JoinPlanToken` at emission time, from `self.static_
  transition_plan.join_plan_token(origin)` (item 9's landed planner
  accessor, cross-module call, unchanged).
- `disposition_statically_unselected_source_subtree`,
  `disposition_statically_unselected_match_cases`,
  `close_statically_unselected_match_cases` (`mod.rs`, private) — mark a
  statically-dead branch/case's joins and match-case selections as
  dispositioned rather than emitted, calling `self.static_transition_plan.
  source_join_origins_in_owner_subtree`/`source_match_case_body_origins`
  (item 9's/planner's, cross-module, unchanged).
- `validate_join_plan_consumption`, `finalize_join_disposition`,
  `validate_materialized_dead_join_cfg`, `validate_materialized_dead_join_
  cfg_for` (`mod.rs`, private) — the generated-function-boundary closure
  proving every planned join was either emitted or dispositioned, and that
  every dead-classified merge block is genuinely CFG-unreachable.
- `merge_scalar_branch`, `merge_scalar_operand` (`mod.rs`, private) — the
  native-scalar-pair join-merge consumer (as distinct from the carrier-word
  merge `carried_join_arm` handles); calls `Self::unwrap_terminal_ret`
  (item 13's, already `pub(super)`) and `lowered.specialized_join_arm`
  (below).
- `record_scalar_merge_kind` (`mod.rs`, private, associated fn on
  `Lowering`) — the one-shot "every dynamic arm agrees on result kind"
  check shared by both merge consumers above.
- `LoweringOperand::specialized_join_arm` (`mod.rs`, private method on
  `LoweringOperand`, not `Lowering`) — "every call is an inventory entry
  for the join work" per its own doc comment; **sole caller is `merge_
  scalar_operand`** (grep-confirmed, 1 call site crate-wide), unlike its
  siblings `specialized_at`/`specialized_ref_at`/`effect_seat_phase` (12+
  call sites spanning many domains — those stay hub-stays, see RETAIN
  below).

**Branch and match emission (`core.rs` unless noted):**

- `lower_carried_match`, `lower_nonborrowed_carried_match`,
  `lower_carried_constructor_match`, `lower_borrowed_match`,
  `lower_borrowed_option_match`, `lower_dynamic_host_result_match`,
  `lower_bounded_nat_match`, `lower_dynamic_constructor_match` — the
  ordinary (non-checked-invocation) match-case dispatch family:
  constructor-tag comparison chains (`builder.ins().icmp`/`.brif`), per-case
  block creation, and per-arm join-arm emission via the join-family
  functions above. `lower_borrowed_match`/`lower_borrowed_option_match`/
  `lower_dynamic_host_result_match`/`lower_bounded_nat_match`/`lower_
  dynamic_constructor_match` were already individually traced and confirmed
  NOT item 13's during that item's own D0 ("match-lowering domain..., a
  different, not-yet-split item's territory") — that finding reconciles
  cleanly into this item's own MOVE set.

**Trap-exit machinery — methods MOVE, most of the surrounding vocabulary
RETAINS (see the hub-stays findings below):**

- `emit_current_trap` (`mod.rs`, private) — the sole production site that
  materializes a `RuntimeTrap`'s identity as a CLIF value at a trap exit
  (unit-frame TrapWord store, root process-sentinel, or root trap token).
  Cross-file RETAINED callers exist in `units.rs` (4 sites, opaque method
  calls — same shape as item 13's widely-called movers, not a hub-stays
  case; see the discriminator below).
- `seal_source_trap_branch` (`mod.rs`, private) — "does this branch resolve
  to a trap; if so, emit it and return instead of joining." Called from
  `core.rs` (many of the match functions above, both this item's and item
  12's computational-match family) and from `source.rs` (item 12's landed
  module, 7 sites) — widely shared, opaque method calls only.
- `bind_unit_trap_frame` (`FunctionLocalRefs::`, `mod.rs`, private) — binds
  the one `TrapExitAuthority::UnitFrame` a generated unit gets; refuses a
  double bind. Callers are exclusively in `units.rs` (4 sites) and `core/
  tests/constructors.rs` (2 sites, direct test construction) — zero callers
  within `core.rs`/`mod.rs` itself today.
- `TrapIdentityMutation` (enum), `TRAP_IDENTITY_MUTATION` (`#[cfg(test)]`
  static), `set_trap_identity_mutation` (fn) — **exclusive to `emit_
  current_trap`** (the sole production touch site, grep-confirmed against
  the whole crate); moves with it.

**The discriminator used above, stated once so it is not re-derived per
symbol:** a RETAINED file that calls a mover **opaquely** (`x.method(...)`,
never touching the callee's internals) does not make the callee hub-stays —
that is the ordinary cross-sibling-visibility case item 13 resolved
repeatedly with `pub(super)` widening. A RETAINED file that **constructs
or matches a type's own variants/fields directly** is the hub-stays case
(item 13's `StaticWorkerEmission`/`StaticWorkerCallOutcome` precedent). The
methods above are all the first shape; the types below are the second.

### RETAIN, hub-stays — types genuinely shared across a moving and a staying
### consumer, checked by direct construction/match, not by call count

- **`TrapExitAuthority`** (enum, `mod.rs`) — `units.rs` directly constructs
  `TrapExitAuthority::Root { .. }` (2 sites) and matches
  `TrapFrameBindingMutation`'s variants to decide which `TrapExitAuthority`
  variant to build (`units.rs:5770`). Stays at the hub; `emit_current_trap`/
  `bind_unit_trap_frame` (moving) reference it via `use super::*`.
- **`TrapFrameBindingMutation`** (enum), **`TRAP_FRAME_BINDING_MUTATION`**
  (`#[cfg(test)]` static), **`set_trap_frame_binding_mutation`** (fn) —
  `units.rs` reads the static directly (`units.rs:5921`) and matches the
  enum's variants directly (`units.rs:5770,5774`) to decide whether to call
  `bind_unit_trap_frame` at all. **Zero production consumer inside
  `core.rs`/`mod.rs` itself** — the real "does this unit get a trap frame"
  decision is entirely `units.rs`'s (item 8's/`RT-FNSPLIT-B2F`'s territory, a
  different, already-completed campaign, outside this slice's bound files).
  This is not this item's owner ("branch/match/join/terminator emission on
  the emitter side") so much as it is a unit-emission concern that happens
  to be declared in the shared `mod.rs` hub. Stays put.
- **`Px8trTrapProvenanceEvent`** (enum), **`px8tr_record_trap_provenance`**
  (fn), **`PX8TR_TRAP_PROVENANCE`** (`pub(super)` static) — a generic
  cross-domain trap-provenance observability log. Its variants are
  constructed from FOUR different files/domains: `emit_current_trap`
  (`PlannedTrapEmitted`, this item's own candidate), `lower_carried_
  computational_match_inner` (`CarriedAnswerRouteEmitted`, item 12's
  checked-invocation domain, `core.rs`), `source.rs` twice
  (`CheckedRecursorDefault`/`DeforestedAnswerResumed`, item 12's landed
  module), and `units.rs` once (`FinalProcessObjectTrap`). Unambiguously
  shared observability infrastructure, not owned by any one emitter slice.
  Stays at the hub.
- **`PlannedTrapSeat`** (enum) — a judgment call, flagged for the
  Architect: its own three variants are constructed ONLY inside `emit_
  current_trap` (this item's own mover) today, which would make it a clean
  MOVE candidate on the discriminator above. It is kept RETAIN here instead
  because it is a field type inside `Px8trTrapProvenanceEvent::
  PlannedTrapEmitted` (hub-stays, per the previous bullet) and is declared
  immediately adjacent to it (`mod.rs:664`, right after
  `Px8trTrapProvenanceEvent` at `:574`) as part of the same trap-provenance
  vocabulary cluster — moving it alone would leave the hub-owned enum's own
  variant referencing a child module's type, an unusual parent-references-
  child shape this ledger did not find a precedent for in items 11-13. If
  the Architect judges the exclusive-construction-site discriminator should
  win here regardless, this is a one-line reclassification, not a re-trace.
  **Superseded by Addendum 7** — the exclusive-construction premise above
  is FALSE (`calls.rs`, item 13's landed module, also constructs all three
  variants); RETAIN stands but is no longer a judgment call. Addendum 7's
  Finding 3 is the corrected reasoning, not relied on here.
- **`specialized_at`, `specialized_ref_at`, `effect_seat_phase`**
  (`LoweringOperand`/related methods, `mod.rs`) — general-purpose "read this
  operand's specialized template or fail closed" utilities, 12+ call sites
  each spanning calls/returns (item 13, already-moved), match/join (this
  item), and other not-yet-split domains. RETAIN, hub-stays — confirmed by
  call-site breadth, the same test item 13 applied to its own `specialized_
  at`-adjacent methods.

### RETAIN, other domain (checked by production role, not by name) —
### recorded so the same names are not re-traced

- **The `emit_carrier_*` family** (`mod.rs`, ~20 methods: `emit_carrier_
  transfer`, `_alloc`, `_immediate`, `_spillable_immediate`, `_native_int`,
  `_region_limbed_int`, `_bytes`, `_bytes_runtime_span`, `_store_tag_id`,
  `_store_scalar`, `_dynamic_constructor`, `_store_field`, `_store_name`,
  `_tag`, `_class`, `_host_success`, `_host_payload`, `_scalar`,
  `_field_count`, `_field`, `_record_field`) — boundary-carrier value
  construction/allocation, item 15's (`RT-EMITTER-AGGREGATES-SPLIT`, "the
  emitter half of the aggregate lifecycle", explicitly this item's own
  `blocks:` successor per the frontmatter) territory, not this item's. This
  item's own moving functions call several of these cross-module
  (`carried_join_arm` calls `emit_carrier_immediate`/`emit_carrier_scalar`
  via `transfer_into_carrier`; `lower_carried_constructor_match` calls
  `emit_carrier_tag`/`emit_carrier_field_count`) — ordinary cross-domain
  calls, not a reason to reclassify either side.
- **`lower_computational_match_expr`, `lower_computational_producer_expr`,
  `lower_computational_match_value_composed`, `lower_carried_computational_
  match`, `lower_carried_computational_match_inner`** (`core.rs`) — despite
  "match" in every name, all operate on `EliminatorFrame`/`Computational
  EliminatorFrame`/`checked_computational_frame`/`mint_recursor_frame_
  provenance`/`active_carried_computational_eliminations` — the checked-
  invocation/eliminator-frame descent machinery, item 12's landed domain
  (confirmed: `lower_computational_match_value_composed` cites `RT-LEXICAL-
  R3-FUSION-EMITTER` and `Architect evt_43ng4f578mdvv` directly in its own
  comment; `lower_carried_computational_match_inner` emits `D6aRouteEvent`,
  already classified `RT-CONTSRC-PRODUCER-LOCAL`/item-12-adjacent during
  item 13's own D0). A second confirmed instance of the "carried"/
  "computational" naming trap items 11-13 have each hit at least once —
  the discriminator is the SUBJECT (checked-invocation elimination vs.
  ordinary value-representation match dispatch), not the presence of CLIF
  emission primitives in the body (both domains use `builder.ins()`/
  `builder.create_block()` identically).
- **`planned_join_site_for_frame`, `require_complete_join_plan_
  consumption`** (`mod.rs`) — despite "join" in both names, these operate
  on `native_join_plan`/`NativeJoinPlanV1`/`active_join_site`/`consumed_
  join_sites` — the CHECKED join-plan bookkeeping for oriented-
  subcontinuation validation, a different "join" concept from `JoinPlanToken`/
  `JoinResultRepresentation` (this item's). Confirmed by caller: `planned_
  join_site_for_frame`'s only callers are in `source.rs` (item 12's landed
  module); `require_complete_join_plan_consumption`'s sole caller sits
  beside `require_complete_dynamic_splice_edge_consumption` in `compile_
  expr_into_module`'s own checked-plan closeout sequence (`core.rs:2787`,
  RETAINED top-level orchestrator). RETAIN, item 12's territory.

### Blind spots / NOT YET CLOSED (stated, not closed — do not read as a
### plan to skip them)

- **AC-2 (test-property ledger) has not started its own fresh pass.**
  `control.rs` is byte-identical to the file exhaustively read during item
  13's own D0 AC-2 (all 220 `#[test]` fns individually read in place at
  that time) — this item's AC-2 will reconcile against that read rather
  than re-reading blind, but the reconciliation itself (which of the
  already-classified RETAIN tests are actually THIS item's domain,
  distinct from item 13's) has not been done yet, beyond the three
  `typed_trap_exit_*` tests item 9 flagged (addressed above).
- **The four Architect-required compiler-blind classes** (re-exports,
  cfg/attribute-gated items, macro-produced items, source-text oracles) —
  not yet swept for this item's MOVE set, per the standing bar the
  Architect set at item 13's D0 and the leader's kickoff restated as
  binding for every item in this phase.
- **`BACKEND_PRODUCTION_SOURCES`/the companion census's own row for a new
  child module** — anticipated, not yet needed until `D1`.
- **Anticipated child module name** — not yet decided; the frame's own
  "modules own semantic lifecycles, do not name a permanent module after a
  temporary campaign node" rule applies. A name capturing "branch/match/join/
  terminator emission" is `D1`'s call once the full population is closed,
  matching how `calls.rs`'s name was settled at that item's own D1, not D0.

### Addendum 2 — top-level item census, the collision-risk names individually
### checked

A full top-level census (types/consts/statics/traits, matching item 13's
Addendum 2/14 method — regex over `struct`/`enum`/`const`/`static`/`type`/
`trait`/`thread_local!` declarations, distinct from the method-level census
above) measured **49 items in `core.rs`, 203 in `mod.rs`** (252 total).
Checked every name plausibly join/match/branch/block/terminator/trap/
carrier/control/scalar-adjacent against its actual production role, not its
name — the exact discipline items 12/13 needed repeatedly:

**core.rs (3 checked, all RETAIN, item 12's):**

- `MatchRecursorCensusRow` (+ `MRC_CENSUS`, `with_match_recursor_census`) —
  the `mrc_census_begin`/`RT-MATCH-RECURSOR-CONSUMERS` census family,
  already named RETAIN/item-12's in item 13's own D0 ("match-recursor-
  census, item 12's landed 'carried_match/static-worker/recursor-position-
  unit cluster'"). Confirmed again here despite "Match" in the name.
- `BranchedScrutineeUnitBodyRoute1` (+ its RAII/recorder family) — the
  `branched_scrutinee_unit_body_observer_tests` domain, confirmed RETAIN
  during item 13's own exhaustive AC-2 read ("source-branch/eliminator-
  frame-descent domain, item 12's landed cluster"). "Branched" is not
  this item's "branch".
- `CheckedFrameBranchScope` — sole consumer is `source.rs` (4 direct
  `CheckedFrameBranchScope::capture`/argument sites, item 12's landed
  module), already named in `source.rs`'s own header comment as staying at
  the `core` hub. "Branch" in the name is again a different sense
  (checked-invocation frame scoping, not CFG branching).

**mod.rs (9 checked; 2 new MOVE findings, 7 RETAIN):**

- **`ScalarMergeKind`** (enum) — **MOVE.** Every use site in `core.rs` sits
  inside `carried_join_arm`/`jump_planned_join_arm`/`finish_planned_join`
  (already-confirmed MOVE); every use in `mod.rs` sits inside `merge_
  scalar_branch`/`merge_scalar_operand`/`record_scalar_merge_kind`
  (already-confirmed MOVE). No RETAINED consumer found anywhere.
  **Superseded by Addendum 7** — this census missed the field-embedding
  in the RETAINED `SourceJoinTarget` (`mod.rs:14128`); reclassified
  MOVE -> RETAIN, hub-stays, there. Not relied on here.
- **`DasmC2ScalarMergeObservation`, `DASM_C2_SCALAR_MERGE_OBSERVATIONS`,
  `DASM_C2_SCALAR_MERGE_OBSERVATION_ENABLED`, `dasm_c2_record_scalar_
  merge`, `dasm_c2_take_scalar_merge_observations`, `DasmC2ScalarMerge
  ObservationScope`** — **MOVE.** The "`RT-DYNAMIC-ARM-SCALAR-MERGE`"
  diagnostic-observation cluster for the scalar-merge decision;
  `dasm_c2_record_scalar_merge`'s sole call site is inside `merge_scalar_
  operand` (already-confirmed MOVE), grep-confirmed against the whole
  crate. **Flagged for `D1`'s care, not a `D0` blocker**: `DasmC2ScalarMerge
  Observation`/`DasmC2ScalarMergeObservationScope` are `pub`/`#[doc(hidden)]`
  under `#[cfg(any(test, feature = "dasm-c2-observation"))]`/
  `#[cfg(feature = "dasm-c2-observation")]` respectively — a crate-external
  diagnostic surface (the doc comment names "the real D5 package path"),
  so `D1` must re-derive its actual external consumer(s) before moving it,
  the same care item 13 gave `RECURSIVE_POSITION_UNIT_CALLS`'s cfg-gating.
- `JoinConsumptionMutation` (enum) — **RETAIN, hub-stays.** Matched
  directly both inside this item's own MOVE-set functions (`mod.rs`, the
  `consume_join_plan`/`finalize_join_disposition` family, and `core.rs`'s
  `lower_dynamic_host_result_match`, already MOVE) AND directly inside
  `source.rs` (`source.rs:457`, matching `OmitSourceMachineComputational
  MatchSelection` by name) — the `StaticWorkerCallOutcome` hub-stays shape
  exactly: a moving consumer and a staying consumer both directly match
  the enum's own variants.
- `NativeScalarPairV1` (struct) — **RETAIN, hub-stays.** Directly
  constructed (struct-literal) from FOUR different production sites
  spanning three domains: `finish_planned_join`/`lower_dynamic_host_
  result_match` (this item's, MOVE), `lower_bounded_nat_computational`
  (item 12's checked-invocation/eliminator-frame domain, confirmed by its
  own body's `EliminatorFrame::Computational`/`EliminatorFrame::Active`/
  `resume_active_continuation` machinery — a sibling of `lower_bounded_
  nat_match`, item 14's, but a genuinely different function), and `lower_
  big_int_constant`/`lower_unsigned_u64_int` (primitive-integer lowering,
  a fourth, not-yet-split domain). A shared native-scalar-pair ABI
  encoding used by whichever domain happens to produce a scalar value —
  not owned by any one emitter slice.
- `OpenControlObligation`, `OrientedControlLedgerEntry` — **RETAIN,
  item 12's.** Despite "Control" in both names, both operate on
  `RecursorUnwindStack`/`ComputationalRecursorLayer`/`CheckedRecursive
  InvocationInstance`/`AffineSpliceCapability`/"OrientedSubcontinuation" —
  checked-invocation control-EXTENT bookkeeping (a continuation/effect
  sense of "control"), not CFG control-flow (this item's sense). A fourth
  instance of "control"/"branch"/"join" being heavily overloaded across
  this codebase's two "control" senses, confirmed only by reading the
  body, never by the name.
- `SourceJoinTarget`, `SourceBranchFanout` — **RETAIN, item 12's.** Part of
  the source-machine's own checked-invocation join/branch-fanout
  vocabulary (`SourcePredecessorEdge`, `SourcePrefixTemplate`,
  `ContinuationCursorId`, `EliminatorFrame` all appear directly in their
  own field types) — the source machine's OWN notion of a "join target"/
  "branch fan-out" during checked-continuation evaluation, a different
  concept from `JoinPlanToken`'s ordinary CFG merge (this item's). Matches
  `source.rs`'s own header comment naming `SourceControl` (their sibling,
  already known RETAIN) as staying at the `mod.rs` hub "shared with
  retained checked-invocation/continuation-frame machinery." "Join" and
  "Branch" in both names are the third and fourth instances of this exact
  naming trap found in this single top-level pass.
- `BoundaryCarrierRefs`, `CarrierAllocationRequest` — **RETAIN, item 15's**
  (the boundary-carrier/aggregate-allocation domain, same territory as the
  `emit_carrier_*` family already named RETAIN above).

**Net for this pass:** two new MOVE findings (`ScalarMergeKind`,
the `DasmC2ScalarMergeObservation*` cluster), zero reclassifications of
anything already traced, and four more confirmed instances of the
"control"/"branch"/"join"/"match" naming trap — the discriminating read is
always the body, never the identifier.

**Still not individually re-traced:** the remaining ~240 top-level items in
the two files not flagged by the collision-risk name scan above. Per the
Architect's item-13 ruling (ordinary RETAIN items are compiler-backstopped
by `E0603` plus his own mandatory `D1` per-mover visibility review, and
domain-cluster attribution grounded in this pass's own tracing is a
sufficient prior, not requiring individual re-derivation of every ordinary
RETAIN item) — this ledger states that as its own prior for the residual
~240, the same shape item 13's Addendum 14 used, not a fresh escalation.

### Addendum 3 — AC-2 reconciliation: a marker scan plus a name sweep, both
### zero, plus the honest gap this does NOT close

**Marker scan, string/comment-aware (the same defect class item 13's own
D0 Addendum 1 first draft had, fixed here before trusting it):** scanned
every one of `control.rs`'s 220 `#[test]` function bodies (brace-matched
with a string/char-literal-aware tokenizer, not a naive `{`/`}` count —
the naive version falsely inflated several spans on this exact file because
of a `"...{...}..."` string literal inside an unrelated census test) for
every name in this item's traced MOVE set (the join family, the ordinary
match-dispatch family, `emit_current_trap`/`seal_source_trap_branch`/
`bind_unit_trap_frame`, `TrapIdentityMutation`, the `DasmC2ScalarMerge
Observation` cluster, `JoinConsumptionMutation`).

**Six hits, all already accounted for and RETAIN:**

- `a_trap_arm_and_its_trap_free_twin_both_functionize`,
  `d8_dynamic_host_result_merge_enters_materialized_dead_cfg_population`,
  `d8_every_required_join_plan_is_consumed_exactly_once` — item 9's own
  ledger already named these three of its "6 Class-4 end-to-end controls"
  (the other three don't hit this item's MOVE-set names at all).
- `typed_trap_exit_preserves_the_planner_identity_across_two_unit_calls`,
  `typed_trap_exit_rejects_a_deleted_or_root_misclassified_unit_lane`,
  `typed_trap_exit_identity_and_caller_protocol_mutations_are_discriminating`
  — the shared trap-exit cluster, already addressed in the
  reconciliation section above.

**Zero hits on this item's own function names** (`lower_carried_match`,
`carried_join_arm`, `merge_scalar_operand`, and every other MOVE-set
symbol traced in AC-1) — a genuinely different result from item 13's own
AC-2, which found several direct hits before its one invisible-to-marker-
scan finding. Per the standing discipline (do not certify a residual
population empty from a marker scan alone), this is a lower bound, not a
closure — so a second, independent check follows.

**Second check: every `#[test]` whose own NAME contains "match" or "branch"
regardless of domain tag**, to catch anything the marker scan's symbol
list might have missed by testing the property indirectly (item 13's own
`d6_a_functionized_recursive_declaration_accepts_a_changing_argument_
constructor` shape — reachable through the full pipeline, naming no MOVE-
set symbol). Found 6 real `#[test]` functions (the rest of the grep hits
are fixture-producer helpers, not tests themselves) —
`checked_frame_branch_scope_harness_uses_live_lowering_ledger` (ties to
`CheckedFrameBranchScope`, already RETAIN/item 12's), `contkey_wrong_
inner_match_eliminator_seed_is_rejected` (`contkey`-prefixed planner/
checked-invocation domain), `distinguished_root_cannot_discharge_missing_
match_site_marker` (the CHECKED "match site" — `NativeJoinPlanV1`'s own
concept, already RETAIN/item 12's per `planned_join_site_for_frame`'s own
finding above), `computational_match_declaration_ref_emits_and_runs_the_
declaration_owned_unit` (`lower_computational_match_expr`'s own domain,
already RETAIN/item 12's), `d9b_the_assembled_ordinary_run_matches_the_
planner_role_sequence_by_position` ("matches" is English usage here, not
`RuntimeExpr::Match`; `D9b`/`RT-CONTSRC-PRODUCER-LOCAL`, already known
planner-adjacent), `refusal_pins_rehomed_computational_match_without_
selector_exclusion` (`RT-REFUSAL-PINS-REHOMED`, `computational_match` ties
to item 12's again). **Every one reconciles to an already-known RETAIN
domain; none is a new finding.**

**A fifth confirmed instance of the naming trap surfaced along the way:**
`RT-MATCH-SCRUTINEE-DISPOSITION` (the domain tag on `rt_d2_trace_shows_
the_marker_propagated_and_never_reaching_the_composed_consumer` and
siblings) has "MATCH" in its own name but is about which ROUTE a RECURSOR's
scrutinee takes (`rt_d2_backedge_propagations`, `active-resume` vocabulary)
— item 12's checked-invocation/recursor domain, not this item's ordinary
case-dispatch sense of "match."

**The honest gap, stated rather than papered over:** this reconciliation
is a marker scan (properly tooled) plus a targeted name sweep, cross-
checked against the domain classifications from my own exhaustive,
individual, in-place read of every one of these 220 tests during item 13's
own D0 AC-2 (completed on this exact byte-identical file). It is **not** a
fresh line-by-line re-read performed as its own dedicated pass under this
item's own name. The file has not changed since that read, and every domain
this pass's two independent scans surfaced matches that read's own
classifications exactly — but the campaign's own bar for AC-2 is an
individual read, and this ledger states plainly that the literal re-read
has not been separately re-executed here. Given the population this bar
would be checking is (per both independent scans) empty, and matches item
8's own "mirror image... zero finding" shape (item 9's ledger's own words
for its analogous situation) — this is flagged for the Architect's judgment:
whether the marker scan + name sweep + reconciliation against an already-
exhaustive same-file read discharges AC-2 for a genuinely-empty-by-all-
evidence population, or whether a literal re-read is still required before
the endorsing vote. Not decided unilaterally here.

**If the population is confirmed empty**, `D2` for this item states that
explicitly and moves nothing — the mirror image of item 8's own `D2`,
which the campaign has already landed once.

### Addendum 4 — the four Architect-required item classes, filtered to the
### closed MOVE set (per the standing bar set at item 13's D0)

#### Class 1 — re-exports: ONE real finding, a genuine crate-external API

Live crate-wide grep of every `use`/`pub use` statement for each traced
MOVE-set name: zero hits, **except** the pinned-SHA re-export census
(`docs/program/backend-split-census-reexports.md:245-248`), which names
`cranelift_backend.rs:122-124`:

```rust
pub use lowering::{
    dasm_c2_scalar_merge_observation_scope, DasmC2ScalarMergeObservation,
    DasmC2ScalarMergeObservationScope,
};
```

Cross-checked live against the current tree (the census is a pinned-SHA
snapshot, confirmed still accurate): this is real, at `cranelift_backend.
rs:122`. It also surfaced a MOVE-set member this ledger's earlier passes
had not yet found — `dasm_c2_scalar_merge_observation_scope` (`mod.rs:
14412`, `#[cfg(feature = "dasm-c2-observation")] pub fn`, the entry-point
constructor for the whole `DasmC2ScalarMergeObservation` cluster's scope)
and `impl Drop for DasmC2ScalarMergeObservationScope` (`mod.rs:14401-14406`)
— both added to the MOVE set now. **Disposition: `D1` must update this
facade re-export's path** from `lowering::{..}` to `lowering::<new-
module>::{..}` once the module is named — the one real re-export
disposition question this item's MOVE set raises, and it is a path
update, not a widening (the names were already `pub`/crate-facade-visible
before the move).

#### Class 2 — cfg/attribute-gated items: several real surfaces, all named

Scanned every MOVE-set function's own body (not just its signature) for
internal `cfg(...)` attributes, using the same string-aware brace-matcher
as AC-2's scan:


- `carried_join_arm` — two `#[cfg(test)]` sites (a population-counter
  increment in the `Carried` arm, another in the `Specialized` arm).
- `lower_dynamic_host_result_match` — two `#[cfg(test)]` sites.
- `consume_join_plan` — one `#[cfg(test)] match` block over
  `JoinConsumptionMutation` (the hub-stays enum, Addendum 2) — carries the
  gate, references a type that stays at the hub.
- `disposition_statically_unselected_source_subtree` — one `#[cfg(test)]`
  site (an early-return under `JoinConsumptionMutation::
  IncludeStaticallyUnselected`).
- `disposition_statically_unselected_match_cases` — one `#[cfg(test)]` site.
- `close_statically_unselected_match_cases` — two `#[cfg(test)]` sites.
- `finalize_join_disposition` — two `#[cfg(test)]` sites (the `LRC_D2B_
  JOIN_OBSERVATION` recorder and the `JoinConsumptionMutation` compensating
  branch).
- `validate_materialized_dead_join_cfg_for` — one `#[cfg(test)]` site.
  (`validate_materialized_dead_join_cfg`'s own grep hit was its NAME
  containing the substring "cfg" — a false positive, verified by reading;
  zero real cfg attributes in that function's body.)
- `merge_scalar_operand` — two `#[cfg(any(test, feature = "dasm-c2-
  observation"))]` sites (the `dasm_c2_record_scalar_merge` calls,
  Addendum 2's cluster).
- `emit_current_trap` — five sites: a `#[cfg(test)]`/`#[cfg(not(test))]`
  pair reading `TRAP_IDENTITY_MUTATION` (production writes the exact ABI
  word, test can zero/substitute it) plus three `#[cfg(test)] px8tr_
  record_trap_provenance(...)` calls (into the hub-stays observability
  type, Addendum 1) — every one of these must carry its gate exactly, and
  the `px8tr_record_trap_provenance` calls reference a type that stays at
  the hub, so the cross-module reference needs updating, not the gate.

No `#[repr(...)]` or non-standard `#[derive(...)]` found on any MOVE-set
type (only ordinary `Clone`/`Copy`/`Debug`/`Eq`/`PartialEq`, confirmed by
direct reading of `TrapIdentityMutation`, `ScalarMergeKind`,
`DasmC2ScalarMergeObservation`). No signature-level `#[cfg(...)]` parameter
found on any MOVE-set function (unlike item 13's `launch_ingress` case) —
every cfg site above is inside a function body, not on its signature.

#### Class 3 — macro-produced owned items: CLOSED, zero found

No `macro_rules!` defined in either bound file (`grep -c` confirms 0 in
both). No MOVE-set name appears adjacent to any macro invocation beyond
the already-covered `thread_local!` block (`TRAP_IDENTITY_MUTATION`'s own
declaration, already named in Addendum 1).

#### Class 4 — source-text oracles: CLOSED, zero symbol-name string matches

Direct grep for every MOVE-set symbol name appearing as a quoted string
literal in `core.rs`, `mod.rs`, `core/tests/control.rs`, or `core/tests/
mod.rs`: zero hits. `BACKEND_PRODUCTION_SOURCES` (the same self-defending
roster item 13's `D1` added `calls.rs` to) will need this item's new module
added at `D1`, following the identical precedent — already stated as an
anticipated non-move hunk in the blind-spots list, not a silent hazard (the
roster's own companion assertion catches an omission loudly).

**Summary for the endorsing vote**: Classes 3 and 4 are fully closed with
zero findings. Class 1 has one real re-export (a path-update disposition,
not a widening) that also completed the MOVE set by two more members. Class
2 has ten named cfg-gated sites across eight functions, all "carry this
gate exactly at `D1`," none a new MOVE/RETAIN boundary finding. None of the
four classes revises the boundary already closed in Addenda 1-3.

### Addendum 5 — the closed MOVE set, restated for a single reference point

**`core.rs`:** `lower_carried_match`, `lower_nonborrowed_carried_match`,
`lower_carried_constructor_match`, `lower_borrowed_match`, `lower_
borrowed_option_match`, `lower_dynamic_host_result_match`, `lower_
bounded_nat_match`, `lower_dynamic_constructor_match`, `carried_join_arm`,
`append_planned_join_params`, `jump_planned_join_arm`, `finish_planned_
join`.

**`mod.rs`:** `consume_join_plan`, `consumed_join_plan_token`,
`disposition_statically_unselected_source_subtree`,
`disposition_statically_unselected_match_cases`,
`close_statically_unselected_match_cases`, `validate_join_plan_consumption`,
`finalize_join_disposition`, `validate_materialized_dead_join_cfg`,
`validate_materialized_dead_join_cfg_for`, `merge_scalar_branch`,
`merge_scalar_operand`, `record_scalar_merge_kind`,
`LoweringOperand::specialized_join_arm`, `emit_current_trap`,
`seal_source_trap_branch`, `FunctionLocalRefs::bind_unit_trap_frame`,
`ScalarMergeKind` (enum; **superseded by Addendum 7** — reclassified
RETAIN, hub-stays, not part of the MOVE set; not relied on here),
`TrapIdentityMutation` (enum), `TRAP_IDENTITY_MUTATION` (thread_local),
`set_trap_identity_mutation`, `DasmC2ScalarMergeObservation` (struct),
`DASM_C2_SCALAR_MERGE_OBSERVATIONS`/`DASM_C2_SCALAR_MERGE_OBSERVATION_
ENABLED` (thread_local), `dasm_c2_record_scalar_merge`, `dasm_c2_take_
scalar_merge_observations`, `dasm_c2_scalar_merge_observation_scope`,
`DasmC2ScalarMergeObservationScope` (struct + its `impl` + `impl Drop`).

**RETAIN, hub-stays (do not re-trace):** `TrapExitAuthority`,
`TrapFrameBindingMutation`+`TRAP_FRAME_BINDING_MUTATION`+`set_trap_frame_
binding_mutation`, `Px8trTrapProvenanceEvent`+`px8tr_record_trap_provenance`
+`PX8TR_TRAP_PROVENANCE`, `PlannedTrapSeat` (flagged judgment call),
`specialized_at`/`specialized_ref_at`/`effect_seat_phase`,
`JoinConsumptionMutation`, `NativeScalarPairV1`.

**RETAIN, other domain (do not re-trace):** the `emit_carrier_*` family
(item 15's), `lower_computational_match_expr`/`lower_computational_
producer_expr`/`lower_computational_match_value_composed`/`lower_carried_
computational_match`/`lower_carried_computational_match_inner`/`lower_
bounded_nat_computational` (item 12's), `planned_join_site_for_frame`/
`require_complete_join_plan_consumption` (item 12's checked native-join-
plan), `MatchRecursorCensusRow`/`BranchedScrutineeUnitBodyRoute1`/
`CheckedFrameBranchScope`/`OpenControlObligation`/`OrientedControlLedger
Entry`/`SourceJoinTarget`/`SourceBranchFanout`/`BoundaryCarrierRefs`/
`CarrierAllocationRequest` (item 12's or item 15's, per Addendum 2).

**AC-2 population:** empty. See Addendum 6 for the corrected, evidenced
discharge — Addendum 3's own "byte-identical since item 13's D2 landed"
premise was FALSE as stated (`runtime-leader` caught this at object-store
verify) and is superseded there, not relied on.

### Addendum 6 — CORRECTION to Addendum 3's premise; AC-2's actual discharge

**Addendum 3 asserted `control.rs` was "byte-identical since item 13's D2
landed" (comparing `b67c805a2` to `367b846d1`, which ARE identical) and
used that to justify reconciling against my own item-13 exhaustive read
without a fresh pass. That comparison answered the wrong question.** The
file I actually read exhaustively (during item 13's own D0 AC-2) was at
`edb69247e` — item 13's D0 pickup SHA, well before item 13's own D1 (+25
lines) and D2 (-92 lines) touched `control.rs`. `edb69247e` and
`367b846d1` are NOT identical, and stating that they were is a false
premise, caught by `runtime-leader` at object-store verify (this item's
kickoff thread, `evt_91c5zzng99x4`).

**The correction, and it is evidence rather than a repaired assertion:**
`git diff edb69247e 367b846d1 -- .../control.rs` — the exact delta between
the file actually read and the file at this item's pickup — is precisely
three hunks, independently re-verified by `runtime-leader` reading the same
diff directly (not my summary of it):

1. The `d6_a_functionized_recursive_declaration_accepts_a_changing_
   argument_constructor` test's removal (item 13's own D2 move to
   `calls.rs`) — already independently confirmed moved, not a hidden
   change to this item's population.
2. One `Census` struct-literal row added for `calls.rs` (item 13's D1
   housekeeping — pure data, no test logic, in the always-residual
   cross-cutting census class).
3. One `BACKEND_PRODUCTION_SOURCES` roster entry + one declared-module-
   list entry added for `calls.rs` (same housekeeping, same class).

**Nothing else differs, confirmed by the diff itself rather than
asserted.** Every one of the 220 tests still in `control.rs` at this
item's pickup is therefore byte-identical to the text individually read
during item 13's own AC-2 — not because the two SHAs happen to coincide
(they do not), but because the ONLY delta between them is fully accounted
for and touches no test-content bearing on any item's domain boundary.

**AC-2 for this item is discharged by: (a) my own exhaustive, individual,
in-place read of all 220 (223 including the 3 core.rs-embedded tests) at
item 13's own D0, plus (b) this exact `git diff`, independently re-read by
`runtime-leader`, proving the file at this item's pickup differs from that
read by nothing that could bear on this item's domain.** This is the actual
discharge — not the "reconciliation against an unchanged file" framing
Addendum 3 used, which rested on a false comparison. A partial, properly-
tooled re-read (through line ~6600/30222 of a fresh pass, all
continuation/checked-invocation domain, zero new findings, per `runtime-
leader`'s own instruction to run it before the diff surfaced) independently
corroborates the same conclusion but is not itself the discharge and was
stopped once the diff-based proof was confirmed (`runtime-leader`,
`evt_7q9fwnzrqx4qb`) — completing it further would have been provably
redundant.

### Addendum 7 — Architect CHANGES-REQUESTED re-cut: three findings in the
### scalar-merge/trap-provenance cluster, Addendum 5's MOVE set superseded

**Architect vote `evt_5a1c9rpf2w6ez` on `e7b4f877d`: CHANGES REQUESTED.**
Root cause, stated once: Addenda 1/2's census enumerated direct call-sites
and named-uses of each MOVE-set symbol, but not (a) the symbol embedded as
a struct FIELD of a RETAINED type, or (b) a delegating/wrapper method that
carries the symbol across the domain boundary. Both missed-site classes
produced a false "exclusive"/"no-retained-consumer" claim, localized to the
scalar-merge/trap-provenance cluster — confirmed NOT a wholesale census
failure (the Architect independently re-verified `TrapIdentityMutation`
holds; my own re-check below of the rest of the MOVE set, widened to the
same two missed-site classes, found no further instances). Each finding
independently re-verified against the source before this addendum, not
taken on the Architect's word alone.

**FINDING 1 (blocking) — `ScalarMergeKind` reclassified MOVE -> RETAIN,
hub-stays.** `SourceJoinTarget<'a>` (`mod.rs:14124`, already RETAIN/item
12's per Addendum 2) declares `required_kind: ScalarMergeKind` as a field
(`mod.rs:14128`, verified). `source.rs` (item 12's landed module)
constructs `SourceJoinTarget` at 6 sites. A RETAINED type embedding this
item's candidate enum as a field is the decisive hub-stays signal — the
same `StaticWorkerCallOutcome` shape item 13 established. `ScalarMergeKind`
is module-private today (`mod.rs:14325`, no `pub`); hub-stays needs ZERO
widening (movers reach it via `use super::*`, since a private parent item
is visible to descendant modules); MOVING it would force `>=pub(super)` so
`SourceJoinTarget` (retained) and `source.rs` (retained, cross-module) could
still see it — a widening to make a move compile, which BANNED SCOPE names
as a finding. RETAIN.

**FINDING 2 (blocking, AC-1 partition gap) — two functions absent from
the ledger entirely.** Grep-confirmed against the committed file: neither
`merge_planned_scalar_branch` (`mod.rs:16173`) nor `lowered_from_scalar_
pair` (`mod.rs:16189`) appeared in any MOVE or RETAIN list. AC-1 requires
every declaration classified; "neither is a gap, not a non-event." Both
independently re-verified:

- `merge_planned_scalar_branch` — its own doc comment names it "a **planned**
  join — same phase-bearing role as `Self::merge_scalar_branch`, same
  pending boundary, named the same way" — a genuine doppelganger of this
  item's own `merge_scalar_branch` (Addendum 1), missed by name-collision
  risk scanning because the names differ by one word. **Sole caller:
  `source.rs:1141`** (RETAINED, item 12's, outside this item's bound files
  entirely) — grep-confirmed, no other caller anywhere in the crate.
  Internally it calls `self.merge_scalar_operand` (this item's own MOVE
  candidate) opaquely. **RETAIN** — a RETAINED entry point that reaches
  into a mover, the same shape as `lower_expr` calling `call_static_worker`
  in item 13.
- `lowered_from_scalar_pair` — callers, grep-confirmed exhaustively:
  `lower_bounded_nat_computational` (`core.rs:6077,6121` — RETAINED, item
  12's checked-invocation domain), `finish_planned_join` (`core.rs:10332` —
  this item's own MOVE), `lower_dynamic_host_result_match`
  (`core.rs:15260` — this item's own MOVE), `lower_big_int_constant`/
  `lower_unsigned_u64_int` (`mod.rs:17718,17764` — RETAINED, primitive-
  integer-lowering, a fourth, not-yet-split domain). **RETAIN, hub-stays**
  — directly analogous to `NativeScalarPairV1`'s own finding in Addendum 1:
  shared between two moving and three staying consumers, no single domain
  owns it.

**FINDING 3 (correction; RETAIN classification stands, false premise
struck) — `PlannedTrapSeat`.** Addendum 1 stated its variants "are
constructed ONLY inside `emit_current_trap`," which was false and is
struck. **Independently re-verified:** `calls.rs` (item 13's own LANDED
module) constructs all three `PlannedTrapSeat` variants too, at
`calls.rs:1771,1786,1798` (confirmed by direct read), as the `seat:` field
of `Px8trTrapProvenanceEvent::UnitTrapWordPropagated` — a **different**
variant from `emit_current_trap`'s own `PlannedTrapEmitted`, both under
`#[cfg(test)]`. `PlannedTrapSeat` is therefore straightforwardly part of
the hub-stays `Px8trTrapProvenanceEvent` observability cluster, shared with
an already-landed sibling — not a judgment call weighing an exclusive-
construction site against its parent enum's declaration position. **RETAIN
stands; the reasoning is corrected, not the verdict.**

**Re-verification of the REST of the MOVE set against the same two missed-
site classes (field-embedding, delegating-wrapper), per the Architect's
re-cut instruction:** every remaining MOVE-set method (`carried_join_arm`,
`append_planned_join_params`, `jump_planned_join_arm`, `finish_planned_
join`, `consume_join_plan`, `consumed_join_plan_token`, `lower_carried_
match` and its five case-dispatch siblings, `emit_current_trap`,
`seal_source_trap_branch`, `bind_unit_trap_frame`) re-checked crate-wide
for every call site: all are opaque `self.method(..)` calls from
`source.rs`/`units.rs`/`core.rs`, none construct or destructure a MOVE-set
type's internals, matching the discriminator's first (mover) shape, not its
second (hub-stays) shape — unchanged from Addendum 1. The
`DasmC2ScalarMergeObservation` cluster re-checked crate-wide (not just the
bound files): its only consumers beyond `merge_scalar_operand` itself are
two cross-CRATE integration tests (`ken-elaborator/tests/nc14_data_match_
lowering.rs`, `ken-cli/tests/dasm_c2_observation_artifact_identity.rs`)
calling the public `dasm_c2_scalar_merge_observation_scope()` entry point
via its crate-facade path — the same re-export already named in Addendum 4,
no new field-embedding or wrapper found. `TrapIdentityMutation` re-checked:
no field-embedding anywhere in the crate (grep-confirmed). **No further
reclassifications.**

**The corrected MOVE set, restated (supersedes Addendum 5's list, which is
now wrong on `ScalarMergeKind` and incomplete on the two added RETAIN
functions):**

**`core.rs`:** unchanged from Addendum 5 — `lower_carried_match`, `lower_
nonborrowed_carried_match`, `lower_carried_constructor_match`, `lower_
borrowed_match`, `lower_borrowed_option_match`, `lower_dynamic_host_
result_match`, `lower_bounded_nat_match`, `lower_dynamic_constructor_
match`, `carried_join_arm`, `append_planned_join_params`, `jump_planned_
join_arm`, `finish_planned_join`.

**`mod.rs`:** `consume_join_plan`, `consumed_join_plan_token`,
`disposition_statically_unselected_source_subtree`,
`disposition_statically_unselected_match_cases`,
`close_statically_unselected_match_cases`, `validate_join_plan_consumption`,
`finalize_join_disposition`, `validate_materialized_dead_join_cfg`,
`validate_materialized_dead_join_cfg_for`, `merge_scalar_branch`,
`merge_scalar_operand`, `record_scalar_merge_kind`,
`LoweringOperand::specialized_join_arm`, `emit_current_trap`,
`seal_source_trap_branch`, `FunctionLocalRefs::bind_unit_trap_frame`,
`TrapIdentityMutation` (enum), `TRAP_IDENTITY_MUTATION` (thread_local),
`set_trap_identity_mutation`, `DasmC2ScalarMergeObservation` (struct),
`DASM_C2_SCALAR_MERGE_OBSERVATIONS`/`DASM_C2_SCALAR_MERGE_OBSERVATION_
ENABLED` (thread_local), `dasm_c2_record_scalar_merge`, `dasm_c2_take_
scalar_merge_observations`, `dasm_c2_scalar_merge_observation_scope`,
`DasmC2ScalarMergeObservationScope` (struct + `impl` + `impl Drop`).
**`ScalarMergeKind` REMOVED** (now RETAIN, hub-stays).

**RETAIN, hub-stays — additions to Addendum 1's list:** `ScalarMergeKind`
(enum, `mod.rs:14325` — field of RETAINED `SourceJoinTarget`),
`merge_planned_scalar_branch` (`mod.rs:16173` — sole caller `source.rs`,
RETAINED), `lowered_from_scalar_pair` (`mod.rs:16189` — shared between two
moving and three staying callers).

**AC-2 unaffected**: Finding 1 only shrinks the MOVE set (RETAIN gains a
member, MOVE loses one); Findings 2/3 add RETAIN members and correct a
RETAIN finding's reasoning. No test-property implication — the AC-2
population (empty, Addendum 3/6) does not depend on which side of the
MOVE/RETAIN line these symbols land.

Re-requesting the endorsing vote on the corrected ledger.

Ready for `runtime-leader`'s object-store verify and the Architect's
endorsing vote.

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

### `D1` executed — `lowering/joins.rs`, against the corrected Addendum 7
### MOVE set, base `93f2e2827`

**Module name: `joins.rs`.** Not the campaign node's own words ("control
and joins") — `control` collides with this crate's own heavily-overloaded
sense of the word (checked-invocation control, `control.rs`'s own test-file
name) and this ledger's own naming-trap discipline (five confirmed instances
in `D0` alone) rules it out. `joins` names the single largest, most cohesive
cluster this owner contains (join-plan consumption, disposition, validation,
and scalar-merge completion — 13 of the 30 moved items) and is the noun
every other cluster (branch/match dispatch, trap exits, the `DasmC2`
scalar-merge observability cluster) ultimately serves or completes into.
Extends the `boundary.rs`/`source.rs`/`calls.rs` seam (items 11-13) exactly.

**Transport manifest — old path -> new path, full preservation, every
moved item verbatim (doc comments, attrs, cfg gates, bodies unchanged)
except the named `pub(super)` widenings below:**

`core.rs` -> `joins.rs` (12 items, unchanged from Addendum 7):
`carried_join_arm`, `append_planned_join_params`, `jump_planned_join_arm`,
`finish_planned_join`, `lower_carried_match`, `lower_nonborrowed_carried_
match`, `lower_carried_constructor_match`, `lower_borrowed_match`, `lower_
borrowed_option_match`, `lower_dynamic_host_result_match`, `lower_bounded_
nat_match`, `lower_dynamic_constructor_match`.

`mod.rs` -> `joins.rs` (18 items, unchanged from Addendum 7): `consume_
join_plan`, `consumed_join_plan_token`, `disposition_statically_unselected_
source_subtree`, `disposition_statically_unselected_match_cases`,
`close_statically_unselected_match_cases`, `validate_join_plan_consumption`,
`finalize_join_disposition`, `validate_materialized_dead_join_cfg`,
`validate_materialized_dead_join_cfg_for`, `merge_scalar_branch`, `merge_
scalar_operand`, `record_scalar_merge_kind`,
`LoweringOperand::specialized_join_arm`, `emit_current_trap`,
`seal_source_trap_branch`, `FunctionLocalRefs::bind_unit_trap_frame`,
`TrapIdentityMutation` (enum) + `TRAP_IDENTITY_MUTATION` (thread_local,
split out of a shared block with the RETAINED `STATIC_WORKER_MUTATION`/
`TRAP_FRAME_BINDING_MUTATION`, which stay in `mod.rs`) + `set_trap_
identity_mutation`, the `DasmC2ScalarMergeObservation` cluster (struct +
its own thread_local + `dasm_c2_record_scalar_merge` + `dasm_c2_take_
scalar_merge_observations` + `DasmC2ScalarMergeObservationScope` struct/
`impl`/`impl Drop` + `dasm_c2_scalar_merge_observation_scope`, moved as one
contiguous, already-self-contained block).

**`ScalarMergeKind`, `merge_planned_scalar_branch`, `lowered_from_scalar_
pair` did NOT move** — confirmed still declared in `mod.rs`, RETAIN per
Addendum 7 Findings 1-2. Every moved method that references
`ScalarMergeKind` reaches it via `use super::*` with no visibility change,
exactly as Addendum 7 predicted (module-private, visible to every
`lowering` descendant without widening).

**Widenings — `pub(super)`, each load-bearing, found by compiler-driven
iteration (build, read every `E0624`, widen exactly the flagged item), not
hand-predicted in advance:**
- Every one of the 30 moved items above except the ones already `pub(super)`
  from item 13's own `D1` (`carried_join_arm`, `append_planned_join_params`,
  `finish_planned_join`) needed `pub(super)` — each has a RETAINED caller in
  `core.rs`/`mod.rs`/`source.rs`/`units.rs` that reaches it opaquely
  (`self.method(...)`), the ordinary cross-sibling-visibility case this
  ledger's own hub-stays discriminator distinguishes from a true hub-stays
  finding. `jump_planned_join_arm` in particular has retained callers
  outside the moving match-dispatch cluster (`core.rs` sites at that SHA
  4288/4365/4572/12244/12554), not visible from the D0 census alone.
- `TrapIdentityMutation` and `set_trap_identity_mutation`: constructed/
  called directly by the not-yet-moved tests in `core/tests/control.rs` (a
  descendant of `lowering::core`, a sibling of `lowering::joins`) —
  reachable before the move only because the callee sat in the `lowering`
  module itself (private-to-parent is visible-to-every-descendant); as a
  sibling module the same reachable set needs the visibility spelled out.
  `mod.rs` re-exports both via `#[cfg(test)] use joins::{set_trap_identity_
  mutation, TrapIdentityMutation};`, the same test-glob-chain mechanism
  already established for `source.rs`'s and `calls.rs`'s own `#[cfg(test)]`
  re-export blocks.
- One REVERSE widening: `lower_computational_producer_expr` (`core.rs`,
  item 12's checked-invocation domain, RETAIN) is called opaquely from
  `joins.rs`'s own `lower_borrowed_match`; it was previously reachable only
  because `core.rs` was `joins.rs`'s ancestor before the move (private
  items reach descendants). Widened to `pub(super)` — a RETAINED item
  reached from a new sibling, the reverse case of every widening above but
  the same rule.

**`cranelift_backend.rs:122-124`** — the `dasm-c2-observation`-gated
re-export updated from `pub use lowering::{...}` to `pub use lowering::
joins::{...}`, a path update only (the three names were already `pub`
before the move, per Addendum 4's Class-1 finding).

**AC-4 / AC-4b — scoped build + test, `ken-cargo` only, never
`--workspace`:**
- `scripts/ken-cargo build -p ken-runtime --lib`: clean. 61 warnings, and a
  baseline build at the pre-move SHA (`93f2e2827`, stashed working tree)
  produces the identical 61 warnings at shifted line numbers — this move
  introduces zero new warnings.
- `scripts/ken-cargo build -p ken-runtime --lib --features dasm-c2-
  observation`: clean, confirming the re-export path update compiles under
  the gated feature.
- `scripts/ken-cargo build -p ken-runtime --tests`: clean.
- `scripts/ken-cargo test -p ken-runtime --lib`: **926 passed, 0 failed, 4
  ignored** — identical to the pre-move baseline (item 13's own landed
  count), confirming `D1` moved production code only. Two tests initially
  failed on first run and were fixed as part of this same `D1` changeset
  (not deferred to `D2`, since they pin a still-resident, not-yet-moved
  test's hardcoded inventory, not a test that itself moved):
  `the_backend_production_surface_inventory_is_closed` and
  `correspondence_adds_no_emitted_unit_to_the_production_census`, both in
  `core/tests/control.rs`, both requiring a new `("lowering/mod.rs",
  "joins")` / `("lowering/joins.rs", ...)` row — the exact `calls.rs`
  precedent from item 13's own `D1`.
- **AC-4b line counts:** `joins.rs` created at **2,213 lines** (well under
  10k). `core.rs`: 15,568 -> 14,384 (shrunk). `mod.rs`: 18,067 -> 17,184
  (shrunk). `core/tests/control.rs`: 30,222 -> 30,247 (+25 lines of
  housekeeping — one `BACKEND_PRODUCTION_SOURCES` row, one declared-module-
  list row, one `Census` row, each following the `calls.rs` row's exact
  shape). `control.rs` is already far past 10k and was **before this slice
  touched it** (`D0`'s own text above records it at 33,969 lines as of
  `a1cf83622`, and item 9/12/13's own `D1`s already logged it there) — its
  own decomposition is `D2`'s subject via test moves, not a condition this
  slice's own `AC-4b` claim (which covers only what THIS slice creates or
  enlarges: `joins.rs`, created at 2,213, and `control.rs`, enlarged by 25
  housekeeping lines) needs to resolve.

**AC-5 — adapter/facade debt ledger: empty.** No transitional adapter or
facade was introduced. Every moved item's callers were repointed by
`pub(super)` widening alone (compiler-driven, the same mechanism items
11-13 used), never by a forwarding shim.

**AC-6 — this slice's own claim, not a phase closure.** `D1` transfers
production code only; the campaign's overall closure condition (every
`RT-BACKEND-MODULE-SPLIT` item landed) is untouched by this slice. `D2`
(the companion test move) is a separate, not-yet-started accepted partial.

**Banned scope check:** no visibility widening beyond `pub(super)` was
needed anywhere (no `pub(crate)`/`pub` escalation), no new module was
introduced beyond the one named child, no adapter/facade was added, no
test was moved (that is `D2`'s exclusive scope) — the two `core/tests/
control.rs` edits above are inventory-row housekeeping in a resident test,
not a test relocation.

Branch `wp/RT-EMITTER-CONTROL-JOINS-SPLIT`, built on `93f2e2827` (current
`origin/main` at pickup, 0 behind). Ready for `runtime-leader`'s object-
store verify and the Architect's mandatory per-mover visibility review.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (branch, match, join and terminator emission
controls). Place multi-leaf fixtures **once**,
at their lowest common ancestor — `tests/mod.rs` or a narrowly named
`support.rs` — and never duplicate them. **Leave genuinely lowering-wide
controls in the residual `control.rs`.**

> ### DO NOT DECOMPOSE `control.rs` ON PRODUCTION FILE BOUNDARIES OR BY LINE RANGES
>
> **Architect `evt_6r403ez3m2m69`.** `control.rs` holds several independent
> populations — planner/occurrence, continuation/fusion, function-state and
> source-machine, emitter and join/trap controls, plus cross-cutting census and
> closure tests. **That is not one production owner**, and partitioning it by
> where the code under test happens to live today re-homes tests twice.
>
> **There is no upfront "split all the tests" phase**, deliberately: it would
> choose owners before their production boundaries exist and churn the same
> imports and fixtures a second time.

**`D1` and `D2` are separate accepted partials by default.** Combine them into
one candidate **only** when an exact compile or mutation-restoration dependency
makes the pair semantically atomic — and say which it was.

### `D2` executed — zero tests move, confirmed rather than assumed, base
### `1e1eedda1`

**Re-verified the empty-population conclusion at this pickup SHA, not
inherited on faith.** `git diff 367b846d1 1e1eedda1 --
lowering/core/tests/control.rs` (`367b846d1` is `D0`'s own pickup, the SHA
against which `AC-2`'s emptiness was closed — Addendum 6's diff-proof, the
Architect's ruling in Addendum 7 and again at the `D1` endorsing vote): the
**only** delta is the three hunks `D1` itself authored — the `Census` row,
the `BACKEND_PRODUCTION_SOURCES` row, and the declared-module-list row, all
three already known by content (I wrote them) and already accounted for in
`D1`'s own AC-4/4b report above. No test body, fixture, or import changed;
no line bears on any test's domain classification. The empty-population
conclusion **holds unchanged** at this pickup — this is the same
diff-based discharge method Addendum 6 established and the Architect
ratified (`D0` `AC-2` ruling, carried through both endorsing votes and
restated in the `D1` kickoff), applied fresh at `D2`'s own pickup rather
than assumed to still be true.

**Zero tests moved.** Basis: `D0`'s own `AC-2` closure (Addendum 3's marker
scan + name sweep, corrected by Addendum 6 to the diff-proof discharge
after the false "byte-identical" premise was caught; the underlying
population is the implementer's own exhaustive item-13-era read of every
test in `control.rs`, re-confirmed empty for this item's domain) — not
re-derived from scratch here, per the ruling's own explicit instruction.
The three tests already known to reference this item's moved production
surface (`typed_trap_exit_identity_and_caller_protocol_mutations_are_
discriminating` and its siblings, shared with item 13's `calls.rs`) are
genuinely cross-cutting/Class-4 end-to-end controls, not primarily-
discriminated-by-this-owner tests, and stay in `control.rs` per this
item's own `D0` ruling — unaffected by `D1`'s move since they reach
`joins.rs`'s moved items only through the already-`pub(super)` production
surface, exactly as before the move.

**AC-4b:** no file created or enlarged by this deliverable (it moves
nothing). `joins.rs` unchanged at 2,213 lines; `control.rs` unchanged at
30,247 lines (D1's own housekeeping, already reported); neither newly
approaches the 10k ceiling as a consequence of `D2`.

**AC-5:** inapplicable — no adapter or facade, transitional or otherwise,
is introduced by moving zero tests.

**AC-6:** this slice's own transfer (of nothing) is complete and is
**item 14's own end-to-end closure** — `D0` (ledger) + `D1` (production
move) + `D2` (companion test move, confirmed empty) exhausts this item's
scope. This is not a claim about `RT-BACKEND-MODULE-SPLIT`'s own phase
closure, which depends on the campaign's other items.

No build or test change results from this deliverable (no file touched);
the last-known-green result is `D1`'s own 926 passed / 0 failed / 4
ignored, unaffected by a zero-diff `D2`.

Branch `wp/RT-EMITTER-CONTROL-JOINS-SPLIT`, at `1e1eedda1` (current
`origin/main` at pickup, 0 behind) — this deliverable makes no code or
test changes, only this ledger section. Ready for `runtime-leader`'s
object-store verify.


# ACCEPTANCE

**Amended on the Architect's whole-plan verdict `evt_14x1bqgrj4yze`.** The
first-cut acceptance did not prove the completeness or the preservation it
claimed; what follows is the corrected bar.

- **`AC-1` — an EXACT move ledger, closed over every Rust item class.**
  > **"Record the blind spots" is honest and it does not close them.** Stage
  > A's type selector sees 278 non-private types and does **not** see 694
  > `pub fn`, 25 `pub const`, 7 `pub static`, 5 `pub mod`, private items,
  > traits, impl methods, macros, split-line declarations, or fields. A ledger
  > built on it alone is not exact, whatever it says about its own limits.

  Enumerate **every** moved item class: modules and re-exports; types with
  their fields and variants; traits, impls and methods; functions; consts and
  statics; cfg, attributes, derive, repr and visibility; and macro-produced
  owned items. **Each class needs its own fresh selector or syntax inventory,
  plus an explicit manual closure for what that selector cannot see.**

  **For the cfg / attribute / derive / repr / visibility class specifically:
  this records what the moved population carries TODAY. Preservation across
  the move is `AC-3`'s job at `D1`** — do not fold the two questions
  together.

  **A group label is not a ledger entry.** "ABI preflight helpers" names a
  set without enumerating it and does not discharge "exact".

  **PARTITION every declaration in the bound file(s).** Each one is either
  **moved to exactly one named owner**, or **EXPLICITLY RETAINED with its
  owning domain named**. A declaration that is neither is a **gap, not a
  non-event**.
  > **A moved-set universal is not the property that discharges "exact".** A
  > ledger can name four moved items perfectly and remain silent on the other
  > hundred-odd in the same files. Item 4's first candidate did exactly that
  > — 25 reconciled against 142 the selector returned — and it read as
  > complete.

  Research `evt_1pwq0rssre6d8`: *"A selector count plus a blind-spot
  paragraph cannot discharge a universal."* **Declare the selector
  population for each class AND close its blind classes.** A declared
  population **bounds** the claim; it does not **discharge** it. Do not
  claim the universal on the strength of the count.
  > **The clause above is conjunctive, and the word "either" was the
  > defect** (Architect `evt_1dh3mj0janmfp`, revising its own correction on
  > item 4's evidence). Declaring the population is what makes "exact" a
  > **well-formed** universal rather than an unbounded one — so it is
  > required *as well as* the closure, never *instead of* it.

  **Source-text oracles and `include_str!` paths belong in the ledger** — Stage A
  found **49** such lines, and relocation can change what they mean without
  changing production behaviour.

- **`AC-2` — test identity and DISCOVERY, before the mutation proof.**
  > **Mutation restoration proves the discriminating tests that have
  > mutations. It does not prove that every moved test is still DISCOVERED**
  > under the same cfg and profile. A test that silently stops being
  > collected passes every mutation check that remains.

  Produce a **before/after test identity and discovery ledger for each
  relevant build profile**; execute directly and record a **nonzero selected-
  test count**; **then** the mutation proof — each moved mutation reds the
  **same reached property**, with the same **nonzero** denominator, restored.
  **Enumerate any source-oracle path or text rewrite as a non-move hunk.**

  **Each test-ledger row carries its CLASS and its exact old/new production
  INJECTION POINT.** Research `evt_1pwq0rssre6d8`, from the program report's
  four-way partition: **domain tests, shared fixtures, mutation controls at
  their production injection point, and end-to-end controls crossing planning
  through execution.**
  > **Class 4 legitimately REMAINS in the residual integration module.** A
  > ledger row without a class invites an end-to-end control to be converted
  > into a domain test, or moved by size — which is exactly what the report
  > forbids.

- **`AC-3` — a TRANSPORT MANIFEST, not a line-pairing review aid.**
  > **Pairing removed lines with added lines is not a behaviour-preservation
  > control.** Attributes, cfg, visibility, field and variant order,
  > derives, imports and name resolution, re-export surfaces and diagnostics
  > can all change while every line still pairs.

  For **every** moved item record **old path, new path**, and an item
  comparison preserving **body, attributes, cfg, repr/derive, field and
  variant order, visibility, diagnostic text, hashes and serialization, and
  public/export profile**.

  **Permitted normalization, and nothing else:** module declarations, imports
  and path qualification, and **explicitly ledgered** adapter/re-export
  scaffolding. **Enumerate every other hunk as a non-move. A semantic hunk
  hard-stops the slice.** `git diff --color-moved` may support the review;
  **it cannot be the gate.**

- **`AC-4`** — the affected library configuration **and** the targeted test
  configurations both compile. **Control:** scoped `scripts/ken-cargo` runs only;
  the workspace gate is **CI's**, never a local run.

- **`AC-4b` — the TARGET CHILD's size is constrained, not just the root's.**
  Record the resulting line count of **every file this slice creates or
  enlarges**. **No move may CREATE OR ENLARGE any file past 10k**, and a move
  that would is a finding to route rather than a transfer to complete.
  > **"Create" alone did not match this criterion's own recording
  > obligation**, which already covers every file the slice *creates or
  > enlarges*. The gap sat on the most likely path in the plan:
  > `lowering/core/tests/constructors.rs` is **9,727** lines — 273 under the
  > ceiling, in the very directory the fifteen `D2` companion-test moves
  > deposit into, and already **+436** with no test moved yet.

  **Where a slice moves nothing this criterion is INAPPLICABLE, not
  satisfied** — `RT-PLANNER-ROOT-CLOSURE-SPLIT` under outcome 1, and the
  closure node, which deletes rather than moves. Restate it as inapplicable;
  do not tick it.
  > Research `evt_1pwq0rssre6d8`: none of the fifteen move frames
  > constrained the target child's size, so the phase could shrink every
  > root while producing a fresh violation.
- **`AC-5` — the ADAPTER AND FACADE DEBT LEDGER.** Any `D1` that introduces
  transitional scaffolding **appends an exact ledger** naming the symbol, why
  it is temporarily required, and **the final-closure deletion obligation**.
  > **[[RT-BACKEND-SPLIT-CLOSURE]] cannot prove it deleted "every adapter"
  > if the earlier slices never closed the population.** This criterion is
  > what makes that closure checkable, and it is owed by every slice that
  > leaves scaffolding behind.

- **`AC-6`** — this slice's own transfer is stated as complete, and **phase
  closure is explicitly NOT claimed.** Reporting a bound file's new line
  count as evidence the phase is done fails this criterion.

> ### LABEL THE THREE EVIDENCE SEATS IN THE LEDGER. Guardrail 7.
>
> **Research `evt_1pwq0rssre6d8`.** The common gate already says plans and
> commands never count as emitted evidence. The ledger must additionally
> label, per moved item, the **intention producer**, the **independent
> artifact observer / evidence decoder**, and the **closeout / publication
> seat** — **so a convenient emitter-family move cannot silently collapse
> them into one.**

# THE FROZEN STAGE PREDICATE — so `D0` cannot choose the boundary opportunistically

**Architect `evt_14x1bqgrj4yze`.** The per-domain symbol sets are
deliberately **not** pre-enumerated here — that would duplicate `D0` and go
stale. What is frozen is the total predicate:

- **The planner owns** plan identities, minting, relation and seat
  construction, validation and closure, and read-only projections.
- **The emitter owns** concrete CLIF/backend mutation that consumes a
  validated plan, and **may not mint or reshape planner identity**.
- **Aggregate, effect, and join/trap symbols are assigned EXACTLY ONCE
  across their planner/emitter pair.** The later `D0` **reconciles against
  the earlier LANDED ledger, not against its frame.**

That settles items 7/15, 8/16 and 9/14 as a boundary question. **The exact
names remain `D0`'s job.**

# BANNED SCOPE

- **No semantic change of any kind.** An exposed behavioural dependency
  **stops the move** and returns for a ruling; it is not repaired inside a
  pure move.
- **No grouping with another slice to reduce node count**, and no planner or
  lowering mega-diff. A census merge permits one frame with independently
  reviewable commits — it permits nothing else.
- **No facade that recreates the monolith**, and no widened visibility to
  make a move compile. If a symbol must widen, that is a finding.
- **No renaming for tidiness.** A move that also renames cannot be reviewed
  as a move.
- **No line-count-driven extraction.** The constraint is architectural
  soundness with a 10k ceiling, not equal-sized files.

# CONTENTION

**Bound files: `cranelift_backend/lowering/core.rs` and
`cranelift_backend/lowering/mod.rs`.**

> ### CHECK CONTENTION BY FILE INTERSECTION AT PICKUP, NOT BY THIS NODE LIST
>
> **Architect `evt_14x1bqgrj4yze`.** A frame that names today's live
> semantic nodes is **deliberately perishable** — the claim was true when
> written and decays silently.
>
> **The durable rule:** a **planner** slice checks active semantic candidates
> against `static_transition.rs` and `control.rs`; a **lowering or emitter**
> slice checks `core.rs`, `mod.rs` and `control.rs`. **A non-empty
> intersection holds the slice.**
>
> The sequencing preference stands — planner work first, lowering and
> emitter work only after semantic work has left those files.

> ### THE CHAIN'S WARRANT IS ARTIFACT DEPENDENCY, NOT SEAT COUNT
>
> **Corrected on the Architect's verdict.** This frame first justified the
> strict chain partly by there being one implementer seat. **Seat count is
> scheduling state, not architecture, and it must not be encoded as a
> dependency.**
>
> **The chain is nevertheless honest, for a real reason:** every `D2` reads
> and edits the same `lowering/core/tests/control.rs`, and each later `D0`
> must **remeasure the tree after the preceding production and test
> relocation**. Within the planner and the lowering/emitter groups the
> production roots also collide.
>
> ⇒ **If production and test moves were ever split into independent nodes**,
> the planner-production and lowering-production chains could **fork**, with
> final closure joining them. **With the current frames they cannot.**

**Re-derive every symbol by name at pickup**, never by line offset. `core.rs`
was 20,413 lines and `mod.rs` 21,200 at `7509c77a7`; both are under active
pressure from this phase itself.

# GATES BINDING EVERY STRUCTURAL FRAME IN THIS PHASE

These are not this slice's invention. They bind every child of
[[RT-BACKEND-MODULE-SPLIT]] and are reproduced here so a pickup does not
have to open the phase record to learn them.

- **Exact old/new symbol and test-property ledgers.**
- **No representation, diagnostic, hash, serialization, behaviour or trust
  change.** This phase is behaviour-preserving.
- **No widened production API, and no facade that recreates the monolith.**
- **Affected library and targeted test configurations both compile.**
- **Each moved mutation reds the same reached property**, with the same
  **nonzero** denominator, and is restored.
- **Plans and commands never count as emitted evidence.**
- **Source text is a census aid, not the only semantic oracle.**
- **Scoped local checks plus CI's workspace gate — never a local workspace
  run** (`COORDINATION section 12`).

> ### AN EXPOSED BEHAVIOURAL DEPENDENCY STOPS THE MOVE. It is not repaired here.
>
> If the move reveals that two regions are coupled by behaviour rather than
> by namespace, **return it for a semantic ruling.** Repairing it inside a
> "pure move" is what makes a structural slice unreviewable, because the
> diff then contains both a relocation and a change and neither can be
> checked against the other.

> ### THE THREE STANDING AMENDMENTS
>
> - **The graph foundation is not an `ids.rs` drawer.** `PredeclaredFunctionId`
>   stays unit-owned; `StaticOriginId` and source/child correspondence stay
>   occurrence-owned.
> - **`boundary_value_clif.rs` is not absorbed merely for size.** Its
>   lifecycle and consumers must be proven first.
> - **The source machine is relocation only in this phase**, never a
>   transition IR. Generated traps receive **no fabricated source origin**.

