---
id: RT-EMITTER-EFFECTS-SPLIT
title: "Move the effects emitter family out of the lowering files -- the emitter half of the effect-seat lifecycle whose planner half item 8 already owns"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-EMITTER-AGGREGATES-SPLIT]
blocks: [RT-EMITTER-TERMINALS-CLEANUP-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 16; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 16 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/core.rs` and `cranelift_backend/lowering/mod.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The effects emitter family.** Effect-seat emission, host-call emission, and
the effect-side operand construction.

**The planner half is item 8**, and the same reconcile-against-the-landed-ledger
rule applies.

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

### Method used

Production-injection-point tracing (items 11-15's discipline), widened
field-embedding + delegating-wrapper census from the start (item 14's
Architect-required correction, now standing bar). Pickup: `ceac66c25`
(current `origin/main`, 0 behind). Bound files at this SHA: `lowering/
core.rs` **14384** lines, `lowering/mod.rs` **13826** lines, `lowering/
core/tests/control.rs` **30292** lines (re-measured directly, not
inherited from any prior item's own count).

### Reconciliation against item 8's landed ledger (planner/emitter boundary)

Item 8 (`RT-PLANNER-EFFECTS-SPLIT`, merged, `docs/program/issues/
RT-PLANNER-EFFECTS-SPLIT.md`) explicitly named the emitter-owned half in
its own "Boundary proposal" section: `EffectSeatGroupId`,
`EffectSeatLedger`, `EffectSeatClosure`, `EffectSeatVisitMutation`,
`EffectSeatDispatchMutation` -- "all live in `lowering/mod.rs` already,
all consume a validated `PlannedEffectSeat` population by claiming/
closing seats, and none of them mint or reshape a seat's identity" --
this item's territory, confirmed independently below. **Items 8/16
settled**, matching the frame's own frozen stage predicate.

### AC-1 -- MOVE population traced so far (THREE separate clusters found
### in `mod.rs`, plus TWO functions in `core.rs` -- the largest single
### item this campaign has moved, `lower_process_host_effect`, is one of
### them)

**A major structural finding not anticipated by item 8's own line-count
estimate (which only covered types/statics/small methods visible from
the PLANNER's own bound file, `static_transition.rs`):**
`lower_process_host_effect` (`core.rs:12806-14145`, **1,340 lines**) is
`core.rs`'s own top-level dispatch entry for `RuntimeExpr::Effect`
occurrences -- structurally the exact same shape as item 14's
`lower_carried_match`/`lower_dynamic_constructor_match` family, which
moved to `joins.rs` as this campaign's established precedent for "a
`core.rs` dispatch entry for one expression kind moves with its own
domain." Confirmed by tracing every mutation-hook read inside it
(`effect_seat_visit_mutation()`, `effect_seat_dispatch_mutation()`,
`effect_seat_next_visit_index()` -- all called only from here) and by
its own sole call site (`core.rs:12761`, inside `core.rs`'s general
expression-dispatch match, the same calling shape `lower_carried_match`
had before item 14's `D1`).

**Cluster 1 -- `mod.rs:404-530`, the two test-mutation families.**
`EffectSeatVisitMutation` (enum, `#[cfg(test)]`, `pub(in crate::
cranelift_backend)`) + its thread_local (`EFFECT_SEAT_VISIT_MUTATION`,
`EFFECT_SEAT_VISIT_INDEX`) + `set_effect_seat_visit_mutation` +
`effect_seat_visit_mutation` + `effect_seat_next_visit_index`;
`EffectSeatDispatchMutation` (enum, `#[cfg(test)]`) + its thread_local
(`EFFECT_SEAT_DISPATCH_MUTATION`, `SITE_OPERAND_SUBSTITUTION_HITS`) +
`set_effect_seat_dispatch_mutation` + `effect_seat_dispatch_mutation` +
`site_operand_substitution_hits`. Bounded before by the unrelated
`scale_b_record_*` family (`ScaleBEmitter` domain) and after by
`BoundedNatLoweringMutation` (bounded-nat domain) -- both confirmed
different domains by reading, not by proximity. The `use crate::
cranelift_backend::planning::CRANELIFT_HOST_EFFECT_CONSUMERS_V1;`
import immediately preceding this cluster is a re-export of the
PLANNER's own (item 8's, already-moved) admitted-operation const --
stays as an import, not a moved item itself, associated with wherever
`host_effect_operation`'s admission check consumes it.

**Cluster 2 -- `mod.rs:7255-8610`, the seat-group claim/close methods
plus the `EffectSeatLedger` type family.** `open_host_effect_seat_group`
(7255-7288), `claim_host_effect_seat` (7290-7372), `close_host_effect_
seat_group` (7374-7391) -- all three confirmed sole-called from
`lower_process_host_effect` (core.rs), zero other callers crate-wide.
Then, after the already-established RETAIN carrier/emit_carrier family
(`carrier_identity_immediate`/`carrier_out_slot`/`carrier_position_
immediate`/`emit_carrier_*`, items 14/15's own hub-stays findings,
re-confirmed unchanged here) and the RETAIN `impl Lowered { first_
boundary_closure_path }` (boundary/value domain) and RETAIN `Structural
NatV1`/`BoundedNatV1`/`mod safe_byte_span`/`mod ac10_production_mint_
probe`/`DynamicConstructorV1`/`DynamicConstructorAlternativeV1`
(all different, already-established or independently-confirmed
domains): `mod effect_seat_group { pub(super) fn mint(...) ->
EffectSeatGroupId }` (8041-8057), `ClaimedEffectSeat` (8060-8072),
`EffectSeatClaimRoute` (8074-8084), `OpenEffectSeatGroup` (8086-8103),
`CommittedEffectSeatGroup` (8105-8111), `EffectSeatLedger` struct
(8113-8134), `EffectSeatClosure` (8136-8151), `impl EffectSeatLedger`
(8153-8427, 8 methods: `open_group`, `open_group_mut`, `claim`,
`close_group`, `discard_open_group_for_tests`, `drop_one_committed_
group_for_tests`, `commit_body`, `close`), `ObservedBytesSeat`
(8482-8493ish, zero external consumer -- the return type of `observe_
carried_bytes_span`, declared far from where it's used, matching this
campaign's established "type declared far from its use" pattern),
**`SiteOperandWitness` (enum, 8592) + `site_operand_witness` (fn,
8602) -- RETAIN at the `mod.rs` hub, corrected by the Architect's D0
vote (`evt_7nzxad9y75crk`), independently re-verified.** My first read
called this MOVE + widen-to-`pub(super)` on the theory that field
embedding in an already-moved sibling type is ordinary cross-sibling
consumption, not hub-stays -- **wrong application of the right
principle.** A crate-wide re-check (both symbols) finds **zero Effects
consumers**: `site_operand_witness`'s only callers are `aggregates.rs:
2878,3229,3230,3231,3238`, and the type's only embedding is
`aggregates.rs:672`'s `SiteOperandSource::Carried { projected:
SiteOperandWitness }` -- both entirely within item 15's already-landed
domain, none in `lower_process_host_effect` or any Cluster-1/2/3
mover. **The discriminator I mis-applied:** a type embedded as a field
of a sibling type and consumed ONLY by that sibling IS the hub-stays
signal (zero widening required, since `aggregates.rs` already reaches
it via ordinary descendant visibility from `mod.rs`) -- my prior
`SiteOperandWitness` reasoning inverted this by treating "the embedding
type already moved" as disqualifying hub-stays, when what actually
matters is which domain the CONSUMER evidence names, not which domain
the SYMBOL's name suggests ("site operand" reads Effects-adjacent but
the actual consumer is aggregates' synthesized-argument substitution).
Moving it would have forced exactly the widening BANNED SCOPE forbids
to make a move compile (`pub(super)` on a private-in-effects type
embedded in `aggregates.rs`'s own `pub(super)` struct triggers E0446)
-- the move would have manufactured the finding, not discovered one.
**Disposition: stays at the `mod.rs` hub, lowering-private, reached by
`aggregates.rs` via descendant visibility, zero widening.** (Non-
blocking note carried from the Architect, not part of this fix: the
tightest domain home is arguably `aggregates.rs` itself, since it is
the sole consumer -- out of this item's scope, since that would reopen
item 15's already-landed slice; a candidate for a future cleanup.)

**`ClaimedEffectSeats<'a>` (struct, 8501ish) + `impl<'a>
ClaimedEffectSeats<'a>` -- RETAIN, hub-stays, re-confirmed independently
rather than inherited from item 15's own "RETAIN, other domain"
verdict.** `core.rs:12921` constructs its fields DIRECTLY (`let seats =
ClaimedEffectSeats { claimed: &claimed, capability: ..., arguments: ...
}`, inside `lower_process_host_effect` itself) and `aggregates.rs` (6
sites) takes it as a parameter type. A RETAINED... except here the
"RETAINED" file doing the direct field construction is `core.rs`'s own
`lower_process_host_effect` -- which is THIS item's own mover. Re-examined
closely: `lower_process_host_effect` constructs `ClaimedEffectSeats`
directly as a **local, throwaway value** to pass to the (already-moved,
item 15's) `aggregates.rs` methods it calls (`site_operand_argument` and
five siblings) -- the type's OWN "home" is genuinely a shared parameter
type between this item's dispatcher and item 15's synthesized-argument
methods, constructed at the call boundary rather than owned by either
side's internal state. Kept RETAIN at the `mod.rs` hub (moving it would
force widening for the `aggregates.rs` reach either way, and it carries
no domain-specific behavior of its own beyond three trivial accessor
methods) -- flagged as a judgment call, the same class `carrier_out_slot`
was at item 15's own D0, for the Architect's read.

**Cluster 3 -- `mod.rs:12332-12872`, the byte-span/narrowing helper
family, all sole-called from `lower_process_host_effect`:** `wire_bytes_
seat` (12332-12439), `wire_bytes` (12441-12493), `narrow_native_int_u64`
(12495-12527), `record_capacity_phase_dispatch` `#[cfg(test)]`
(12529-12580 -- **NB: this span includes a pre-existing, misattributed
doc comment that reads as `narrow_carried_int_u64`'s own documentation
["the CARRIED exact-`Int` narrowing..."] but sits directly, with no
blank-line separation, above `record_capacity_phase_dispatch`'s own
`#[cfg(test)]` attribute; confirmed by reading, not assumed -- `narrow_
carried_int_u64`'s own declaration at 12748 has no doc comment of its
own. This is pre-existing source content, preserved verbatim at `D1`
exactly as it sits today; not something this transport-only move may
correct**), `record_capacity_phase_dispatch` `#[cfg(not(test))]`
(12582-12583), `observe_carried_bytes_span` (12585-12746, its OWN
correctly-attached doc comment), `narrow_carried_int_u64` (12748-12860,
bare, no doc comment -- see the note above), `lower_dynamic_small_int`
(12862-12872).

**`require_one_of_i64` (`mod.rs:12893-12919`) -- RETAIN, hub-stays,
independently confirmed (corrects a wrong first read).** Located and
traced crate-wide: called from `core/primitive.rs:354,418` (a genuinely
different domain, direct `Self::require_one_of_i64` calls with no
Effects involvement at all), from `lower_unsigned_u64_int` (`mod.rs:
13515`, itself RETAIN -- see below), and from my own `validate_resource_
io`/`validate_resource_error_reply`/`lower_process_host_effect`. Two
independent non-Effects call paths (primitive.rs direct, and via the
RETAIN `lower_unsigned_u64_int`) settle it RETAIN, grouped with its
siblings `require_i64`/`require_nonzero` (also confirmed RETAIN, used by
every sibling module: source/joins/aggregates/calls/units, fresh
crate-wide sweep, not inherited).

**Cluster 3b -- `mod.rs:12934-13235`, a validation-and-progress-minting
family, discovered only by exhaustively walking the file past what
Cluster 3's first pass covered (this is corrected scope, not new
territory: Cluster 3 previously stopped at 12872 without checking what
followed).** `require_u8` (12934-12948, sole caller `validate_resource_
error_reply`, mine), `require_true` (12949-12958) and `require_when`
(12959-12979) -- **re-examined and corrected**: my first pass called
these RETAIN on the theory that two call sites meant two domains, but
both call sites for each are themselves inside MY OWN candidates
(`mint_validated_progress_nat` and `validate_resource_error_reply`/
`lower_process_host_effect`) -- zero non-Effects callers exist for
either, so both are **MOVE**, not RETAIN. `mint_validated_progress_nat`
(12980-13047, sole caller `lower_process_host_effect`, plus the domain
test file `core/tests/effects.rs:151` -- internally calls the RETAIN
`BoundedNatV1::mint_after_reply_validation`, an ordinary cross-sibling
reference once it moves, no reclassification of `BoundedNatV1` itself).
`validate_resource_io` (13048-13076, sole caller `validate_resource_
error_reply`, mine) and `validate_resource_error_reply` (13077-13235,
sole caller `lower_process_host_effect`, mine) -- both newly found MOVE
candidates this file's own dispatcher calls, entirely missed by the
keyword sweep (neither name contains "effect"/"host_effect"/
"resource_seat" as a distinguishing token the way the rest of the
cluster's names do; "resource" alone is not Effects-distinguishing, and
this is the same naming-trap shape already flagged three times this
campaign).

**`mod.rs:13236-13826` (end of file) -- traced in full, ALL RETAIN,
different domain(s), zero Effects content.** A "value materialization"
cluster (`lower_value` 13236, `lower_seed_capture` 13296, `artifact_
static_payload` 13358, `lower_ground_value` 13374, `lower_big_int_
constant` 13422, `lower_unsigned_u64_int` 13483, `native_int_tag` 13526,
`ground_value` 13576, `intern_result` 13653) -- each independently
confirmed via a crate-wide caller grep to have at least one caller
outside `lower_process_host_effect`/Effects entirely (`lower_value`:
source.rs/units.rs/core.rs:11420 inside the general `lower_expr`
dispatcher; `lower_seed_capture`: called from `lower_expr` itself,
twice, plus `lower_declaration_ref`, see below; `native_int_tag`:
joins.rs/calls.rs/aggregates.rs/primitive.rs; `ground_value`/`intern_
result`: calls.rs; `lower_unsigned_u64_int`: core/primitive.rs). Then a
free-function classification cluster (`lowered_value_kind` 13672 --
called from essentially every sibling file, the most cross-domain
symbol found this item; `expect_two_args` 13697 -- sole external caller
`core/primitive.rs`, a different domain; `borrowed_constructor_identity`
13709 -- sole external callers joins.rs/source.rs). Then a thread-local
+ free-fn cluster with zero Effects-span callers at all (`PX8J_SOURCE_
TRACE`/`PX8J_DELETE_OWNED_SELECTED_SCOPE`/`PX8TR_TRAP_PROVENANCE`/
`PX8TR_DISABLE_DEFORESTED_ANSWER_ROUTE`/`NATIVE_INT_LOWERING_MUTATION`/
`PX8DS_RETIRED_FLAT_ORDER`/`LRC_D2B_*` thread_locals and their `lrc_d2b_
*` accessor fns, 13728-13826) -- consumed by test_objects.rs, calls.rs,
source.rs, `core/primitive/tests.rs`, `control.rs`, `constructors.rs`;
zero hits inside `lower_process_host_effect`'s own span, confirmed by
direct grep. **`mod.rs`'s own population is now closed to the end of
the file** -- no further un-swept region remains after 13826 (EOF).

**`core.rs`'s own population -- CLOSED, not merely the keyword-sweep
result previously flagged as open.** The naming-trap check flagged as
outstanding is now run: `lower_declaration_ref` (`core.rs:14147-14384`,
end of file) sits immediately after `lower_process_host_effect` and
contains `child_occurrence`/`lower_seed_capture` calls that a first
`self.method()` sweep of my own function's body could not see (they sit
past 14145, outside my own function's true end). Its sole caller is
`core.rs:12262`, inside `lower_expr` (the general expression
dispatcher, `pub(super)`, RETAIN by definition) -- `lower_declaration_
ref` is a general declaration/symbol-reference lowering routine, RETAIN,
different domain, not Effects. `core.rs`'s Effects population is
confirmed closed at exactly two functions: `lower_buffer_freeze_
resource_seat` (12780-12804) and `lower_process_host_effect`
(12806-14145).

### AC-2 -- test-property ledger, first pass (topology differs from item
### 15 -- state this plainly, don't force the constructors.rs template)

**`core/tests/effects.rs` already exists (3901 lines, 39 `#[test]`
fns) -- this is NOT item 15's shape.** Item 15 had to discover and move
tests INTO a newly-created inline `mod tests` inside its new production
file. Here, a prior initiative (its own header cites "RT-SPLIT slice 7"
and "RT-SPLIT §10.2 assigns these subjects to `effects`", and `core/
tests/mod.rs:3` says "Slice 4 populates `control`, `effects` and
`constructors`") already split the TEST tree, well before this
production-code campaign reached item 16. `effects.rs` reaches the
not-yet-moved production symbols via `use super::*` today; once `D1`
creates `lowering/effects.rs` and moves the production symbols there,
this test file's own `D2` need may be small (import-path fixups) rather
than a symbol-by-symbol test relocation -- **stated as a hypothesis to
verify at `D1`/`D2` time, not yet proven.**

**`control.rs` still holds the three tests item 8's own D2 flagged, all
independently re-verified at THIS item's pickup SHA (not inherited from
item 8's stale `34c0ef97a`):**
- `erasing_a_seat_key_axis_or_collapsing_the_contract_rejects`
  (`control.rs:16632`) -- uses `EffectSeatPlanMutation` exclusively (a
  **different, planner-owned** type from my own `EffectSeatVisitMutation`/
  `EffectSeatDispatchMutation`; the frozen stage predicate's "plan
  identities/validation" is exactly this). **Confirmed item 8's own
  domain, stays. Class 4/control-domain, not mine.**
- `an_incomplete_duplicate_discarded_or_misobserved_visit_rejects`
  (`control.rs:16701`) and `a_discarded_visit_refuses_before_its_body_
  is_defined` (`control.rs:16772`) -- both use `EffectSeatVisitMutation`
  (18 hits across the pair, `control.rs:16703-16840`), my own Cluster-1
  type. **Confirmed item 16's own domain -- movers**, matching item 8's
  lead, now independently re-verified rather than trusted.

**Two Class-4 (source-text oracle) sites in `control.rs` independently
confirmed, matching item 8's own D0 prediction of an anticipated
"effects" row addition at `D1` time (items 4-7's precedent):**
`correspondence_adds_no_emitted_unit_to_the_production_census`
(`control.rs:7243`, its own `Census { file: "planning/static_
transition/effects.rs", ... }` entry at 7549-7550 documents in-comment
that "the emitter-owned half (`EffectSeatGroupId`/`EffectSeatLedger`/
`EffectSeatClosure`) stays in `lowering/mod.rs`" -- literally describing
my own item's pre-`D1` state) and `the_backend_production_surface_
inventory_is_closed` (`control.rs:8162`, its own `("planning/static_
transition.rs", "effects")` row at 8279, same comment content), and
`the_identifier_census_survives_the_evasions_that_defeated_the_text_
scan` (`control.rs:7974`) -- a THIRD kind of oracle, keyed by **file
path** rather than domain label: it lists `("lowering/core.rs", ...)`,
`("lowering/core/primitive.rs", ...)`, `("lowering/mod.rs", ...)` and
`("planning/static_transition/effects.rs", ...)` as its census inputs
(8023-8110). Once `D1` creates `lowering/effects.rs`, this list needs a
new `("lowering/effects.rs", include_str!("../../effects.rs"))` entry,
mirroring the existing `core.rs`/`mod.rs` rows. **Item 8's "3-location"
prediction is now fully located and closed**, all three confirmed at
this item's own pickup SHA, not inherited from item 8's stale read.

**`control.rs`'s own MOVE-set population is confirmed CLOSED**: a
symbol-by-symbol sweep of every type/fn name in the closed MOVE set
against the whole file found hits only at the five sites above (the
three tests + the two census functions, `the_identifier_census_...`
folding into the same "Class 4" bucket as the other two) -- no stray
reference anywhere else in `control.rs`'s 30k+ lines.

### AC-2 -- `effects.rs`'s own 39 tests, program-matched against the
### closed MOVE set (mechanical pass, not yet a full prose read of each)

Every `#[test]` fn body in `core/tests/effects.rs` (39 total) matched
against the full closed MOVE set of type/fn/const names. **8 of 39**
name a MOVE-set symbol directly (`mint_validated_progress_nat`,
`EffectSeatDispatchMutation`/`EffectSeatVisitMutation` +
`set_effect_seat_*_mutation`, `capacity_phase_dispatch`/`reset_
capacity_phase_dispatch` via the `units.rs` accessor noted above,
`site_operand_substitution_hits`, `RESOURCE_ERROR_INVALID_BOUNDS`/
`RESOURCE_ERROR_MALFORMED_RESOURCE`). **The other 31 name none** --
this is expected, not a gap: they exercise the domain end-to-end
through `RuntimeExpr::Effect`/`host_effect_operation` construction and
compiled-output assertions, never touching the internal seat-ledger
symbols by name, which is the ordinary shape for an integration-style
lowering test.

**One finding that bears directly on the D2 shape, not the D0
population:** several of the 31 (`px8n_bounded_nat_observes_exact_
zero_successor_and_recursive_order` and its siblings) test `BoundedNatV1`
-- already independently confirmed **RETAIN** (a shared, general
primitive, not Effects-owned). They sit in `effects.rs` because their
fixture enters through the Effects-owned `mint_validated_progress_nat`
(the file's own header says as much: "Bounded-Nat, host-reply, IO,
borrowed-ingress and native-int lowering tests... assigns these
subjects to `effects`"). **The test-FILE grouping is by fixture entry
point, not a 1:1 mirror of the production-module boundary this ledger
draws** -- do not read "lives in `effects.rs`" as "tests an
Effects-owned symbol." This is consistent with, not a contradiction of,
concluding the whole file is already the domain's test home: the file
was assigned to this domain by an earlier initiative (`RT-SPLIT`) on
exactly this fixture-entry-point basis, before this campaign's
production-module boundary existed to compare it against.

**Working conclusion, not yet Architect-reviewed:** `core/tests/
effects.rs` needs no test *relocation* at `D2` -- unlike item 15's
`constructors.rs` pattern, the test-level split already happened. `D2`
here is more likely import-path fixups (matching whatever `D1` widens)
than a symbol-by-symbol test move. This reverses the AC-2 emphasis for
this item: the census work is in `control.rs` (closed above) and in
confirming `effects.rs` compiles clean against the moved production
module at `D1`, not in finding tests to relocate.

### Cluster 2 const/static gap -- closed (the D7_PAIR_CALLEE lesson,
### applied proactively this time)

A full declaration-selector re-pass over Cluster 2 (7250-8615) --
running functions/types/consts/statics together, not functions and
types first with consts as an afterthought -- found four consts my
first pass missed: `IO_ERROR_OTHER_DISCRIMINATOR` (`mod.rs:8436`),
`RESOURCE_ERROR_MALFORMED_RESOURCE` (8443), `RESOURCE_ERROR_INVALID_
OFFSET` (8447), `RESOURCE_ERROR_INVALID_BOUNDS` (8456). All four
checked crate-wide: every reference sits inside `mod.rs`'s own Cluster
2/3 functions (mine), `lower_process_host_effect` (mine), or `core/
tests/effects.rs` (my domain test file) -- zero RETAIN callers. **MOVE**,
all four. Cluster 1 re-swept the same way: no additional consts/statics
found beyond what Addendum 1's first pass already recorded.

### A NEW cross-file transport finding -- `CAPACITY_PHASE_DISPATCH`
### thread_local has an out-of-scope reader in `units.rs`

`CAPACITY_PHASE_DISPATCH` (`mod.rs:8575-8578` thread_local) is written
only by my own `record_capacity_phase_dispatch` (confirmed: its three
call sites, `core.rs:13179/13185/13190`, all sit inside `lower_process_
host_effect`) -- so the thread_local itself is **MOVE**. But its only
*readers* are two `#[cfg(test)]` accessor fns declared in **`units.rs`**
(`capacity_phase_dispatch`/`reset_capacity_phase_dispatch`,
`units.rs:5465,5470`, reached via `super::CAPACITY_PHASE_DISPATCH`) --
`units.rs` is **outside this item's two bound files** (item 9's own
domain, already split), so those two functions are not mine to move.
Their own sole caller, in turn, is my own `core/tests/effects.rs`
(`crate::cranelift_backend::lowering::units::capacity_phase_dispatch()`,
6 call sites, 3162-3344) -- so the full producer/consumer closure is
100% Effects, just with the accessor *body* physically parked in a
sibling file for reasons this ledger does not need to relitigate.
**Transport note for `D1`:** moving `CAPACITY_PHASE_DISPATCH` to
`effects.rs` breaks `units.rs:5466,5471`'s `super::CAPACITY_PHASE_
DISPATCH` reference -- it needs a qualified-path update
(`super::effects::CAPACITY_PHASE_DISPATCH` or equivalent) as part of
`D1`'s transport, even though `units.rs` itself is not a bound file and
carries no other Effects-domain content. This is a new shape for the
campaign: a cross-sibling reference landing in the *opposite* direction
from every prior case (a RETAINED, out-of-scope file's own code needs a
path fixup because of MY move, rather than my own moved code needing to
reach back to a RETAINED type).

### Hub-struct field embedding -- resolved, not a RETAIN case

**`EffectSeatLedger` IS embedded as a field of the retained `Lowering`
struct** (`mod.rs:2931`: `host_effect_seats: Option<EffectSeatLedger>`)
-- exactly the check the kickoff required. **This does not reclassify
it RETAIN.** The adjacent field one line above it, `aggregate_
allocations: Option<aggregates::AggregateAllocationLedger>`
(`mod.rs:2927`), is item 15's own already-moved type, referenced via
its **qualified module path** -- proving the precedent directly rather
than by analogy. `EffectSeatLedger` moves with the rest of its own
cluster; at `D1` the `host_effect_seats` field's value type becomes
`effects::EffectSeatLedger`, mirroring `aggregate_allocations` exactly.
The `Lowering` struct itself (and the field slot) stays in `mod.rs` --
only the type path qualifies.

### Blind spots / NOT YET CLOSED (stated, not closed -- do not read as a
### plan to skip them)

- **The four compiler-blind classes are only partially swept.** Class 2
  (cfg/attribute-gated) is implicitly covered by the `#[cfg(test)]`
  markers already named above, but not yet run as its own dedicated
  pass. Class 4 (source-text oracles) is closed -- all three `control.rs`
  locations found and confirmed (see above).
- **AC-2's `effects.rs` pass is mechanical (symbol-matched), not yet a
  full prose read of each of the 39 tests.** The programmatic sweep
  above is sound for population-closure purposes (does every test that
  names a MOVE-set symbol get accounted for) but has not been followed
  by an individual reading of each test's own assertions the way items
  11-15's AC-2 discipline calls for -- flagged so this is not silently
  read as done. `control.rs` itself IS confirmed swept end-to-end (see
  above): no `EffectSeat*`/`wire_bytes*`/`narrow_*_int_u64`/
  `SiteOperandWitness` reference exists there outside the five sites
  already classified.
- **Consts/statics/traits/repr classes -- closed for `mod.rs`.** A full
  declaration-selector re-pass (functions/types/consts/statics
  together) run over every cluster -- 1 (404-530), 2 (7250-8615,
  already reported above), 3 (12332-12872), and 3b/EOF (already covered)
  -- found no further consts/statics beyond what is already recorded in
  this ledger. `mod.rs`'s AC-1 population is now closed.
- **`ClaimedEffectSeats<'a>` RETAIN-at-hub -- Architect-affirmed**
  (`evt_7nzxad9y75crk`), independently re-verified: produced in Effects
  (`core.rs:12921`) and consumed by Effects' own `wire_bytes_seat`
  (`mod.rs:12388`), by `aggregates.rs` (6 param sites), and by item 15's
  `constructors.rs` tests (`::none()`, 4 sites) -- genuinely shared
  across effects.rs(new)+aggregates.rs+constructors.rs, LCA is `mod.rs`,
  zero-widening RETAIN.

### The closed MOVE set, restated for a single reference point

**Types/enums:** `EffectSeatGroupId`, `ClaimedEffectSeat`, `EffectSeatClaimRoute`,
`OpenEffectSeatGroup`, `CommittedEffectSeatGroup`, `EffectSeatLedger`,
`EffectSeatClosure`, `ObservedBytesSeat`, `EffectSeatVisitMutation`,
`EffectSeatDispatchMutation`. (`SiteOperandWitness` corrected OUT --
see RETAIN list below.)

**Consts:** `IO_ERROR_OTHER_DISCRIMINATOR`, `RESOURCE_ERROR_MALFORMED_RESOURCE`,
`RESOURCE_ERROR_INVALID_OFFSET`, `RESOURCE_ERROR_INVALID_BOUNDS`.

**Thread_locals:** `EFFECT_SEAT_VISIT_MUTATION`, `EFFECT_SEAT_VISIT_INDEX`,
`EFFECT_SEAT_DISPATCH_MUTATION`, `SITE_OPERAND_SUBSTITUTION_HITS`,
`CAPACITY_PHASE_DISPATCH` (has an out-of-scope reader in `units.rs`,
transport note above).

**Functions (`mod.rs`):** `open_host_effect_seat_group`, `claim_host_effect_seat`,
`close_host_effect_seat_group`, `effect_seat_group::mint`,
`EffectSeatLedger::{open_group, open_group_mut, claim, close_group,
discard_open_group_for_tests, drop_one_committed_group_for_tests,
commit_body, close}`, `wire_bytes_seat`,
`wire_bytes`, `narrow_native_int_u64`, `record_capacity_phase_dispatch`
(both `#[cfg(test)]` twins), `observe_carried_bytes_span`, `narrow_
carried_int_u64`, `lower_dynamic_small_int`, `set_effect_seat_visit_
mutation`, `effect_seat_visit_mutation`, `effect_seat_next_visit_index`,
`set_effect_seat_dispatch_mutation`, `effect_seat_dispatch_mutation`,
`site_operand_substitution_hits`, `require_u8`, `require_true`,
`require_when`, `mint_validated_progress_nat`, `validate_resource_io`,
`validate_resource_error_reply`. (`site_operand_witness` corrected OUT --
see RETAIN list below.)

**Functions (`core.rs`):** `lower_buffer_freeze_resource_seat`,
`lower_process_host_effect`.

**RETAIN, independently confirmed this item (not exhaustive -- see
prose above for the full reasoning per symbol):** `require_i64`,
`require_one_of_i64`, `require_nonzero`, `lower_unsigned_u64_int`,
`child_occurrence`, `lower_declaration_ref`, `lower_value`, `lower_seed_
capture`, `artifact_static_payload`, `lower_ground_value`, `lower_big_
int_constant`, `native_int_tag`, `ground_value`, `intern_result`,
`lowered_value_kind`, `expect_two_args`, `borrowed_constructor_identity`,
the `PX8J_*`/`PX8TR_*`/`NATIVE_INT_LOWERING_MUTATION`/`PX8DS_*`/
`LRC_D2B_*` cluster, `ClaimedEffectSeats<'a>` (Architect-affirmed,
above), `SiteOperandWitness`/`site_operand_witness` (Architect-
corrected from MOVE, above -- zero Effects consumers, sole
consumer/embedding is `aggregates.rs`), `BoundedNatV1`/`StructuralNatV1`,
the whole `emit_carrier_*`/`carrier_*` family, `mod safe_byte_span`,
`mod ac10_production_mint_probe`, `DynamicConstructorV1`,
`DynamicConstructorAlternativeV1`.

This is Addendum 1, corrected once by the Architect's D0 vote
(`evt_7nzxad9y75crk`, `SiteOperandWitness` MOVE -> RETAIN-at-hub --
applied and independently re-verified above; `ClaimedEffectSeats`
affirmed as-is). AC-1 (function/type/const/static population) is closed
for both bound files after two in-place corrections of my own (the
`require_true`/`require_when` misclassification, and this
`SiteOperandWitness` one) and two proactive re-passes (the Cluster-2
const gap, the Cluster-1/3 re-sweep that found nothing new). AC-2 has a
mechanical population match complete (39/39 `effects.rs` tests
classified, `control.rs` swept end-to-end) but not yet the individual
prose read items 11-15's discipline calls for. The hub-struct embedding
check and the Class-4 source-text-oracle sweep are both closed. Class
1/3 compiler-blind sweeps were checked ad hoc (zero found) but not run
as a final formal pass. Ready to re-request the Architect's D0
endorsement.

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

## `D1` transport manifest — executed against the endorsed D0 ledger @ `9081e56fd`

**Diff scope (7 files, +90/-2900):** new `lowering/effects.rs` (2957
lines); `lowering/mod.rs` (13826 -> 12319, -1507 net incl. new import/
`mod effects;` lines) and `lowering/core.rs` (14384 -> 13019, -1365 net
incl. one `pub(super)` widen) both shrink by the moved content;
`lowering/units.rs` and `lowering/aggregates.rs` each gain one
`#[cfg(test)]`-gated `use` (cross-sibling reach into `effects`, named
below); `core/tests/mod.rs` gains one `use` block (the not-yet-moved
`control`/`effects` test subjects' own reach into the moved production
symbols); `core/tests/control.rs` gets two absolute-path `use` fixups
inside its own two tests (`crate::…::lowering::{…}` ->
`crate::…::lowering::effects::{…}`) plus the three Class-4 census
additions (below); `core/tests/effects.rs` gets one analogous
absolute-path `use` fixup, split into two statements since its
prefix now diverges (`effects::{…}` vs `units::{…}`).

**Method:** a tokenizer-driven (string/comment-aware) span extraction
matching the exact methodology items 13-15's own D1s used (`rust_lex`
classify + brace-depth item-boundary finder), driven by the closed
D0 ledger's own def-line list. **One real tool bug found and fixed
before any file was touched:** the span-finder tracked only `{`/`}`
depth, so `validate_resource_error_reply`'s `[u64; 10]` array-type
parameter's own internal `;` was misread as the item's top-level
terminator, truncating its signature. Fixed to also track `(`/`[`
depth; re-verified against every other span (all identical except the
one corrected, confirming the fix was narrowly scoped) before
extracting.

**`impl<'a> Lowering<'a>` re-wrapping (mechanical, not part of AC-1's
own classification).** The D0 ledger's spans covered exactly the
closed MOVE-set *items*, each extracted independently of its
enclosing `impl` block. Two of the four original impl-block groupings
(the seat-group claim/close methods, and the Cluster-3/3b validation/
byte-span family plus the two moved `core.rs` methods) needed their
`self`-taking methods re-wrapped in a fresh `impl<'a> Lowering<'a> {
... }` in `effects.rs` — non-method items (types, thread_locals, free
fns, the nested `mod effect_seat_group`, `EffectSeatLedger`'s own
already-self-contained `impl` block) needed no wrapping. This is
mechanical re-assembly, not a design choice — matches `joins.rs`/
`aggregates.rs`'s own "each moved method sits in its own small `impl`
block" precedent (their headers, quoted in this file's own header).

**`pub(super)` widenings, each load-bearing (compiler-backstopped
E0624/E0425, not silent — AC-1's own "ordinary top-level RETAIN items"
disposition, no Architect vote needed for these, only for the D0
hub-stays judgment calls):**
- `lower_process_host_effect`, `lower_buffer_freeze_resource_seat` —
  sole caller `lower_expr`'s `RuntimeExpr::Effect` arm stays in the
  retained `core.rs` (exactly as the D0 ledger and kickoff predicted).
- `lower_dynamic_small_int` — retained `core/primitive.rs` (a
  descendant of `core.rs`) calls it directly; **not previously traced
  in the D0 census** (a genuine build-driven widening discovery, the
  same class item 15's own D1 retro named).
- `narrow_native_int_u64` — retained `calls.rs` calls it directly;
  likewise not previously traced.
- `observe_carried_bytes_span` — retained `aggregates.rs` calls it
  directly (its own `commit_aggregate_events`-adjacent code path);
  likewise not previously traced.
- `EffectSeatLedger::close` — retained `units.rs`'s `close_host_
  effect_seat_ledger` calls it directly.
- `EffectSeatLedger::commit_body` — retained `aggregates.rs`'s
  `commit_aggregate_events` calls it directly (a genuinely new
  cross-domain coupling this item's D0 did not have visibility into,
  since it required reading item 15's own retained code, not just this
  item's bound files).
- `EffectSeatLedger::drop_one_committed_group_for_tests` — retained
  `units.rs`'s `close_host_effect_seat_ledger`'s own `#[cfg(test)]`
  mutation check calls it directly.
- `mint_validated_progress_nat` — the not-yet-moved `core/tests/
  effects.rs`'s own tests call it as `Lowering::mint_validated_
  progress_nat(...)`.
- `EffectSeatDispatchMutation` (the enum itself, was fully private) —
  both the not-yet-moved `control.rs` tests and `aggregates.rs`'s own
  `#[cfg(test)]` mutation check need it.
- `effect_seat_visit_mutation`, `effect_seat_dispatch_mutation`,
  `site_operand_substitution_hits`, `SITE_OPERAND_SUBSTITUTION_HITS`,
  `RESOURCE_ERROR_MALFORMED_RESOURCE`, `RESOURCE_ERROR_INVALID_BOUNDS`
  — each needed by one or more of the same not-yet-moved test/
  cross-sibling-production consumers above; all widened to `pub(super)`
  -- **corrected by the Architect's D1 vote** (`evt_7m85d3q5sd4px`),
  independently re-verified before applying: my first pass widened
  these (plus `EffectSeatDispatchMutation`, `drop_one_committed_group_
  for_tests`, `mint_validated_progress_nat` above) to `pub(in
  crate::cranelift_backend)`, matching their already-`pub(in
  crate::cranelift_backend)` siblings (`EffectSeatVisitMutation`,
  `set_effect_seat_visit_mutation`, `set_effect_seat_dispatch_
  mutation`) -- but "matching a landed convention" was the wrong
  instinct here: those siblings' own wide visibility is itself a
  pre-existing over-widening (their only out-of-`lowering` reference is
  a `//!` doc-comment mention, which needs no visibility at all), and
  copying it onto genuinely new widenings propagates the over-widening
  rather than applying the minimal-sufficient rule fresh. **A
  crate-wide grep for every one of the 9 confirms zero real-code
  consumers outside `lowering/`** (the sole out-of-`lowering` hit,
  `EffectSeatDispatchMutation` in a `planning/static_transition/
  effects.rs` doc comment, is prose, not code) -- `pub(super)` is
  strictly wider than the private-at-base visibility every one of these
  9 already compiled under before the move, so the narrowing cannot
  break a consumer. The pre-existing `pub(in crate::cranelift_backend)`
  siblings are correctly left as-is (verbatim-moved, zero-widen, out of
  this slice's scope to re-narrow) -- the family is now
  intentionally mixed-visibility, which is correct, not an
  inconsistency to paper over. **Downstream consequence of the
  narrowing:** `core/tests/mod.rs`'s own re-export of these 9 (for
  `control`/`effects`'s benefit) had to narrow its own qualifier in
  lockstep, from `pub(in crate::cranelift_backend)` to `pub(in
  crate::cranelift_backend::lowering)` -- Rust rejects a re-export
  wider than the item's own visibility (E0364/E0365), caught
  immediately by the scoped test build.
- `masked_reply_response_bytes` — stays in the **retained** `core.rs`
  (its own sibling consumer is `core/tests/constructors.rs`'s `super::
  masked_reply_response_bytes`, item 15's own residual test), widened
  from private to `pub(super)` so this module's `lower_process_host_
  effect` can reach it — the one widening on the RETAIN side of the
  seam, not the MOVE side.
- `CAPACITY_PHASE_DISPATCH` — widened to `pub(super)` exactly as the
  kickoff's own carried non-move transport hunk specified; `units.rs`'s
  two accessor fns updated from `super::CAPACITY_PHASE_DISPATCH` to
  `super::effects::CAPACITY_PHASE_DISPATCH`.

**Judgment-call symbols confirmed NOT moved, exactly as the kickoff
specified:** `ClaimedEffectSeats<'a>` and `SiteOperandWitness`/
`site_operand_witness` both stay at the `mod.rs` hub, zero-widen.
`EffectSeatLedger`'s field slot on the retained `Lowering` struct
(`host_effect_seats`) stays at `mod.rs`; only its value type's path
qualifies, to `effects::EffectSeatLedger` — the `aggregate_
allocations`/`aggregates::AggregateAllocationLedger` precedent applied
verbatim. `mod.rs` gained a matching `use effects::{EffectSeatClosure,
EffectSeatLedger};` (mirroring its own existing `use aggregates::
{AggregateAllocationLedger, AggregateRelationClosure};`) so `units.rs`'s
retained ledger-lifecycle wrappers keep resolving these by bare name
via the same glob-inheritance mechanism.

**Class-4 (source-text oracle) updates — all three predicted `control.
rs` locations, now actually landed, not merely predicted:**
`correspondence_adds_no_emitted_unit_to_the_production_census` gets a
new all-zero `Census` row for `lowering/effects.rs` (confirmed zero
`FunctionBuilder::new`/`declare_function`/`define_function`/
`declare_data`/`define_data` occurrences by direct grep — no `mod
tests` block exists at `D1`, unlike `aggregates.rs`'s own D2-landed
non-zero row); `the_backend_production_surface_inventory_is_closed`
gets a new `("lowering/mod.rs", "effects")` row in its declared-module
list; `the_identifier_census_survives_the_evasions_that_defeated_the_
text_scan`'s own `BACKEND_PRODUCTION_SOURCES` list (shared by both of
the above tests) gets a new `("lowering/effects.rs", include_str!(...))`
entry. **All three found and fixed by running the test suite,** not
by re-deriving from the D0 prediction alone — the first full run
caught exactly these two failures (the third assertion shares the
same list, so one fix closed two of the three predicted sites at once).

**Gates:**
- **AC-4/4b** — `scripts/ken-cargo build -p ken-runtime --lib` and
  `scripts/ken-cargo test -p ken-runtime` both green: **926 passed / 0
  failed / 4 ignored** (lib), plus 26 passed / 0 failed
  (`value_depth_totality`) and 14 passed / 0 failed (doc-tests). New
  file `effects.rs` is 2957 lines, well under the 10k ceiling.
- **AC-5** — no adapters, no facades; nothing to ledger.
- **AC-6** — this is a slice-only transfer, not a phase-closure claim;
  item 16 remains `active` pending `D2`.
- Banned scope respected: no semantic change (build+test green,
  behaviour-preserving move only), no grouping, no facade, no tidiness
  renames, no line-count-driven extraction beyond the closed D0 set.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (effect-seat emission, host-call and
effect-operand controls). Place multi-leaf fixtures **once**,
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

> ### If `D2` mints a qualified `mod tests`, generalize the roster-strip idiom
>
> **Adversary latent finding on item 15 D2 (`evt_kpq5yn3w7n5d`), which this
> slice may re-trigger.** When a residual cross-subtree test must reach the
> moved cluster by path, `D2` writes a **qualified** module header
> (`pub(in crate::...) mod tests {`) rather than the bare `mod tests {` every
> other lowering file uses. The federation's test-region strip is a literal
> `.split_once("\n#[cfg(test)]\nmod tests {")`: a visibility qualifier between
> `#[cfg(test)]` and `mod tests {` defeats it, so the strip silently no-ops and
> the roster-iterating inventory pins fall back to the whole file. It was
> harmless on item 15 only because that test region named none of the guarded
> identifiers. **If this slice writes a qualified `mod tests`, generalize the
> strip to match `\n#[cfg(test)]\n` then `mod tests {` on the next line
> regardless of a visibility prefix** — a cheap owning-team fix that converts a
> latent false-positive into a closed one. If it does not, note that it stayed
> bare and this does not apply.

## `D2` executed — off `fe498769a` (`D1` merged), item 16 closes on this
## deliverable

**The "import-path fixups only" hypothesis is REFUTED for `effects.rs`
itself, CONFIRMED for the other 39 -- state both halves plainly, don't
average them into "mostly right."** `core/tests/effects.rs` needed zero
physical relocation for its own pre-existing 39 tests (the hypothesis
holds there, exactly as D0 predicted from the RT-SPLIT provenance).
But `control.rs` held two tests whose primary discriminated property
is this item's own domain, and those genuinely moved -- the "no
physical relocation" reading would have been wrong if applied to the
whole D2, only right for the file D0 flagged.

**Exhaustive individual read, not marker-sampled -- every one of the
39 pre-existing `effects.rs` tests read in full, function by function,
not just the ~8/39 D0's mechanical pass flagged as directly naming a
MOVE-set symbol.** Every single one discriminates a property squarely
inside the file's own declared scope (Bounded-Nat, host-reply, IO,
borrowed-ingress, native-int lowering) -- zero outliers, zero
misplaced tests found. The `px8n_bounded_nat_*`/`budget_eff_native_*`
family tests `BoundedNatV1` (independently confirmed RETAIN at D0) but
legitimately belongs here anyway: its fixture enters exclusively
through the Effects-owned `mint_validated_progress_nat`, matching the
file's own header ("assigns these subjects to `effects`" -- a
fixture-entry-point grouping, not a strict production-module mirror,
as flagged as a hypothesis at D0 and now confirmed by reading every
test rather than assumed from the file's existence).

**`control.rs`'s own three D0-flagged tests, re-verified independently
at this pickup (not inherited from item 8's stale read or this item's
own stale D0 read):**
- `erasing_a_seat_key_axis_or_collapsing_the_contract_rejects` --
  confirmed STAYS. Its own `use` names `planning::EffectSeatPlanMutation`,
  the planner's type (item 8's), never `EffectSeatVisitMutation`. Item
  8's own domain test, not touched.
- `an_incomplete_duplicate_discarded_or_misobserved_visit_rejects` and
  `a_discarded_visit_refuses_before_its_body_is_defined` -- confirmed
  MOVE. Both name `lowering::effects::{set_effect_seat_visit_mutation,
  EffectSeatVisitMutation}` directly (the second also reaches
  `lowering::units::{...}` for its own cross-domain body-definition-
  timing assertion -- a genuinely two-domain test, kept whole rather
  than split, matching how several of `effects.rs`'s own pre-existing
  tests already cross into `units.rs`). Relocated verbatim into
  `core/tests/effects.rs` (byte-identical bodies, mechanically
  confirmed against the pre-move extraction -- only the `use` block
  differs, and only because it must).

**Shared-fixture handling, the one non-trivial transport decision:**
`governed_nested_resource_bracket` (planning-domain, already `pub(in
crate::cranelift_backend)`) needed only an ordinary `use`, unaffected
by the move. `recursive_port_process_compiles` -- declared in
`control.rs`, 38 remaining call sites there after removing the 2 that
moved -- stays put and widens from private to `pub(in
crate::cranelift_backend::lowering::core::tests)`, the minimal
qualifier reaching a sibling test module (`effects`) under the same
`core::tests` parent; the two relocated tests reach it by the
qualified path `crate::…::lowering::core::tests::control::
recursive_port_process_compiles`. `set_effect_seat_visit_mutation`/
`EffectSeatVisitMutation` needed **no** import in their new home --
already ambient via `effects.rs`'s own ordinary `use super::*` chain
through `core/tests/mod.rs`'s existing re-export (the same one `D1`
already wired for this file's other tests).

**The qualified-`mod tests` roster-strip risk (frame's own carried
note, Adversary `evt_kpq5yn3w7n5d`) does NOT apply here -- checked, not
assumed.** That risk is specifically about a PRODUCTION file minting
its own inline `#[cfg(test)] mod tests { ... }` with a visibility
prefix (item 15's `aggregates.rs` shape). This item's production file
(`lowering/effects.rs`) has **no inline test module at all** -- zero
`FunctionBuilder`/`declare_function`/`define_function` calls, confirmed
at `D1` and unchanged here -- and its domain's tests live in the
wholly separate, pre-existing `core/tests/effects.rs` file. No
qualified `mod tests` header was written by this deliverable; the
roster-strip idiom is untouched and the generalization the frame notes
is not owed.

**Gates:**
- **AC-2** — discovery parity confirmed by exact `cargo test -- --list`
  name: both relocated tests now discover as `cranelift_backend::
  lowering::core::tests::effects::{name}` (previously `…::control::
  {name}`), each exactly once. Total discovered lib test count
  unchanged (930, matching this item's own D1 baseline) -- nothing
  lost, nothing duplicated. Bodies byte-identical to the pre-move
  extraction outside the `use` block (mechanically diffed, not
  eyeballed). The two tests are themselves mutation-restoration
  controls (`EffectSeatVisitMutation` enum-driven) and re-ran green
  unchanged after relocation -- the oracle re-point (the widened
  `recursive_port_process_compiles` reach) is exercised by every one of
  their own assertions, not a separate proof.
- **AC-3** — transport is fully named above: one visibility widening
  (`recursive_port_process_compiles`), zero adapters, zero facades.
- **AC-4/4b** — `scripts/ken-cargo build -p ken-runtime --lib` and
  `test -p ken-runtime` both green: **926/0/4** (lib) + 26/0
  (`value_depth_totality`) + 14/0 (doc-tests), re-run after the move.
  `core/tests/effects.rs`: 4083 lines. `core/tests/control.rs`: 30155
  lines (down from D1's unchanged baseline by exactly the two moved
  tests' span). Both well under any created/enlarged-file ceiling
  concern (`control.rs` shrank; `effects.rs`'s test file was already
  large before this item and gained ~180 lines, not newly created).
- **AC-5** — no adapters; nothing to ledger.
- **AC-6** — **this closes item 16.** `D0`+`D1`+`D2` together are the
  full slice-only transfer for the effects emitter family; no further
  deliverable is owed.

Banned scope respected: no semantic change (every test's own assertions
are byte-identical), no grouping beyond the two tests whose own
discriminated property already named this domain, no facade, no
line-count-driven extraction.


# ACCEPTANCE

**Amended on the Architect's whole-plan verdict `evt_14x1bqgrj4yze`.** The
first-cut acceptance did not prove the completeness or the preservation it
claimed; what follows is the corrected bar.

- **`AC-1` — an EXACT move ledger, closed over every Rust item class.**
  > **"Record the blind spots" is honest and it does not close them.** Stage A's
  > type selector sees 278 non-private types and does **not** see 694 `pub fn`,
  > 25 `pub const`, 7 `pub static`, 5 `pub mod`, private items, traits, impl
  > methods, macros, split-line declarations, or fields. A ledger built on it
  > alone is not exact, whatever it says about its own limits.

  Enumerate **every** moved item class: modules and re-exports; types with their
  fields and variants; traits, impls and methods; functions; consts and statics;
  cfg, attributes, derive, repr and visibility; and macro-produced owned items.
  **Each class needs its own fresh selector or syntax inventory, plus an explicit
  manual closure for what that selector cannot see.**

  **For the cfg / attribute / derive / repr / visibility class specifically:
  this records what the moved population carries TODAY. Preservation across the
  move is `AC-3`'s job at `D1`** — do not fold the two questions together.

  **A group label is not a ledger entry.** "ABI preflight helpers" names a set
  without enumerating it and does not discharge "exact".

  **PARTITION every declaration in the bound file(s).** Each one is either
  **moved to exactly one named owner**, or **EXPLICITLY RETAINED with its owning
  domain named**. A declaration that is neither is a **gap, not a non-event**.
  > **A moved-set universal is not the property that discharges "exact".** A
  > ledger can name four moved items perfectly and remain silent on the other
  > hundred-odd in the same files. Item 4's first candidate did exactly that —
  > 25 reconciled against 142 the selector returned — and it read as complete.

  Research `evt_1pwq0rssre6d8`: *"A selector count plus a blind-spot paragraph
  cannot discharge a universal."* **Declare the selector population for each
  class AND close its blind classes.** A declared population **bounds** the
  claim; it does not **discharge** it. Do not claim the universal on the strength
  of the count.
  > **The clause above is conjunctive, and the word "either" was the defect**
  > (Architect `evt_1dh3mj0janmfp`, revising its own correction on item 4's
  > evidence). Declaring the population is what makes "exact" a **well-formed**
  > universal rather than an unbounded one — so it is required *as well as* the
  > closure, never *instead of* it.

  **Source-text oracles and `include_str!` paths belong in the ledger** — Stage A
  found **49** such lines, and relocation can change what they mean without
  changing production behaviour.

- **`AC-2` — test identity and DISCOVERY, before the mutation proof.**
  > **Mutation restoration proves the discriminating tests that have mutations.
  > It does not prove that every moved test is still DISCOVERED** under the same
  > cfg and profile. A test that silently stops being collected passes every
  > mutation check that remains.

  Produce a **before/after test identity and discovery ledger for each relevant
  build profile**; execute directly and record a **nonzero selected-test count**;
  **then** the mutation proof — each moved mutation reds the **same reached
  property**, with the same **nonzero** denominator, restored. **Enumerate any
  source-oracle path or text rewrite as a non-move hunk.**

  **Each test-ledger row carries its CLASS and its exact old/new production
  INJECTION POINT.** Research `evt_1pwq0rssre6d8`, from the program report's
  four-way partition: **domain tests, shared fixtures, mutation controls at their
  production injection point, and end-to-end controls crossing planning through
  execution.**
  > **Class 4 legitimately REMAINS in the residual integration module.** A
  > ledger row without a class invites an end-to-end control to be converted into
  > a domain test, or moved by size — which is exactly what the report forbids.

- **`AC-3` — a TRANSPORT MANIFEST, not a line-pairing review aid.**
  > **Pairing removed lines with added lines is not a behaviour-preservation
  > control.** Attributes, cfg, visibility, field and variant order, derives,
  > imports and name resolution, re-export surfaces and diagnostics can all
  > change while every line still pairs.

  For **every** moved item record **old path, new path**, and an item comparison
  preserving **body, attributes, cfg, repr/derive, field and variant order,
  visibility, diagnostic text, hashes and serialization, and public/export
  profile**.

  **Permitted normalization, and nothing else:** module declarations, imports and
  path qualification, and **explicitly ledgered** adapter/re-export scaffolding.
  **Enumerate every other hunk as a non-move. A semantic hunk hard-stops the
  slice.** `git diff --color-moved` may support the review; **it cannot be the
  gate.**

- **`AC-4`** — the affected library configuration **and** the targeted test
  configurations both compile. **Control:** scoped `scripts/ken-cargo` runs only;
  the workspace gate is **CI's**, never a local run.

- **`AC-4b` — the TARGET CHILD's size is constrained, not just the root's.**
  Record the resulting line count of **every file this slice creates or
  enlarges**. **No move may CREATE OR ENLARGE any file past 10k**, and a move
  that would is a finding to route rather than a transfer to complete.
  > **"Create" alone did not match this criterion's own recording obligation**,
  > which already covers every file the slice *creates or enlarges*. The gap sat
  > on the most likely path in the plan: `lowering/core/tests/constructors.rs`
  > is **9,727** lines — 273 under the ceiling, in the very directory the fifteen
  > `D2` companion-test moves deposit into, and already **+436** with no test
  > moved yet.

  **Where a slice moves nothing this criterion is INAPPLICABLE, not satisfied**
  — `RT-PLANNER-ROOT-CLOSURE-SPLIT` under outcome 1, and the closure node, which
  deletes rather than moves. Restate it as inapplicable; do not tick it.
  > Research `evt_1pwq0rssre6d8`: none of the fifteen move frames constrained the
  > target child's size, so the phase could shrink every root while producing a
  > fresh violation.
- **`AC-5` — the ADAPTER AND FACADE DEBT LEDGER.** Any `D1` that introduces
  transitional scaffolding **appends an exact ledger** naming the symbol, why it
  is temporarily required, and **the final-closure deletion obligation**.
  > **[[RT-BACKEND-SPLIT-CLOSURE]] cannot prove it deleted "every adapter" if the
  > earlier slices never closed the population.** This criterion is what makes
  > that closure checkable, and it is owed by every slice that leaves scaffolding
  > behind.

- **`AC-6`** — this slice's own transfer is stated as complete, and **phase
  closure is explicitly NOT claimed.** Reporting a bound file's new line count as
  evidence the phase is done fails this criterion.

> ### LABEL THE THREE EVIDENCE SEATS IN THE LEDGER. Guardrail 7.
>
> **Research `evt_1pwq0rssre6d8`.** The common gate already says plans and
> commands never count as emitted evidence. The ledger must additionally label,
> per moved item, the **intention producer**, the **independent artifact
> observer / evidence decoder**, and the **closeout / publication seat** —
> **so a convenient emitter-family move cannot silently collapse them into one.**

# THE FROZEN STAGE PREDICATE — so `D0` cannot choose the boundary opportunistically

**Architect `evt_14x1bqgrj4yze`.** The per-domain symbol sets are deliberately
**not** pre-enumerated here — that would duplicate `D0` and go stale. What is
frozen is the total predicate:

- **The planner owns** plan identities, minting, relation and seat construction,
  validation and closure, and read-only projections.
- **The emitter owns** concrete CLIF/backend mutation that consumes a validated
  plan, and **may not mint or reshape planner identity**.
- **Aggregate, effect, and join/trap symbols are assigned EXACTLY ONCE across
  their planner/emitter pair.** The later `D0` **reconciles against the earlier
  LANDED ledger, not against its frame.**

That settles items 7/15, 8/16 and 9/14 as a boundary question. **The exact names
remain `D0`'s job.**

# BANNED SCOPE

- **No semantic change of any kind.** An exposed behavioural dependency **stops
  the move** and returns for a ruling; it is not repaired inside a pure move.
- **No grouping with another slice to reduce node count**, and no planner or
  lowering mega-diff. A census merge permits one frame with independently
  reviewable commits — it permits nothing else.
- **No facade that recreates the monolith**, and no widened visibility to make a
  move compile. If a symbol must widen, that is a finding.
- **No renaming for tidiness.** A move that also renames cannot be reviewed as a
  move.
- **No line-count-driven extraction.** The constraint is architectural soundness
  with a 10k ceiling, not equal-sized files.

# CONTENTION

**Bound files: `cranelift_backend/lowering/core.rs` and
`cranelift_backend/lowering/mod.rs`.**

> ### CHECK CONTENTION BY FILE INTERSECTION AT PICKUP, NOT BY THIS NODE LIST
>
> **Architect `evt_14x1bqgrj4yze`.** A frame that names today's live semantic
> nodes is **deliberately perishable** — the claim was true when written and
> decays silently.
>
> **The durable rule:** a **planner** slice checks active semantic candidates
> against `static_transition.rs` and `control.rs`; a **lowering or emitter**
> slice checks `core.rs`, `mod.rs` and `control.rs`. **A non-empty intersection
> holds the slice.**
>
> The sequencing preference stands — planner work first, lowering and emitter
> work only after semantic work has left those files.

> ### THE CHAIN'S WARRANT IS ARTIFACT DEPENDENCY, NOT SEAT COUNT
>
> **Corrected on the Architect's verdict.** This frame first justified the strict
> chain partly by there being one implementer seat. **Seat count is scheduling
> state, not architecture, and it must not be encoded as a dependency.**
>
> **The chain is nevertheless honest, for a real reason:** every `D2` reads and
> edits the same `lowering/core/tests/control.rs`, and each later `D0` must
> **remeasure the tree after the preceding production and test relocation**.
> Within the planner and the lowering/emitter groups the production roots also
> collide.
>
> ⇒ **If production and test moves were ever split into independent nodes**, the
> planner-production and lowering-production chains could **fork**, with final
> closure joining them. **With the current frames they cannot.**

**Re-derive every symbol by name at pickup**, never by line offset. `core.rs` was
20,413 lines and `mod.rs` 21,200 at `7509c77a7`; both are under active
pressure from this
phase itself.

# GATES BINDING EVERY STRUCTURAL FRAME IN THIS PHASE

These are not this slice's invention. They bind every child of
[[RT-BACKEND-MODULE-SPLIT]] and are reproduced here so a pickup does not have to
open the phase record to learn them.

- **Exact old/new symbol and test-property ledgers.**
- **No representation, diagnostic, hash, serialization, behaviour or trust
  change.** This phase is behaviour-preserving.
- **No widened production API, and no facade that recreates the monolith.**
- **Affected library and targeted test configurations both compile.**
- **Each moved mutation reds the same reached property**, with the same
  **nonzero** denominator, and is restored.
- **Plans and commands never count as emitted evidence.**
- **Source text is a census aid, not the only semantic oracle.**
- **Scoped local checks plus CI's workspace gate — never a local workspace run**
  (`COORDINATION section 12`).

> ### AN EXPOSED BEHAVIOURAL DEPENDENCY STOPS THE MOVE. It is not repaired here.
>
> If the move reveals that two regions are coupled by behaviour rather than by
> namespace, **return it for a semantic ruling.** Repairing it inside a "pure
> move" is what makes a structural slice unreviewable, because the diff then
> contains both a relocation and a change and neither can be checked against the
> other.

> ### THE THREE STANDING AMENDMENTS
>
> - **The graph foundation is not an `ids.rs` drawer.** `PredeclaredFunctionId`
>   stays unit-owned; `StaticOriginId` and source/child correspondence stay
>   occurrence-owned.
> - **`boundary_value_clif.rs` is not absorbed merely for size.** Its lifecycle
>   and consumers must be proven first.
> - **The source machine is relocation only in this phase**, never a transition
>   IR. Generated traps receive **no fabricated source origin**.

