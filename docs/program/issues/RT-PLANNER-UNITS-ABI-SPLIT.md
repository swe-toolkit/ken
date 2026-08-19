---
id: RT-PLANNER-UNITS-ABI-SPLIT
title: "Move the units and ABI domain out of planning/static_transition.rs into its own child module -- abi.rs, predeclared ids, descriptors, slots, call-edge views, pre-emission validation and the read-only EmittableUnit boundary form the phase's strongest closed seam, and it is the first planner domain"
status: merged
owner: runtime
size: L
gate: none
depends_on: [RT-BACKEND-SPLIT-CENSUS]
blocks: [RT-PLANNER-OCCURRENCES-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 4; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 4 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs` (34,883 lines).

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Units and ABI.** `abi.rs`, predeclared ids, descriptors, slots, call-edge
views, pre-emission validation, and the **read-only `EmittableUnit` boundary**.

**Why this domain is first, and it is not arbitrary.** The Architect ruled it
the strongest closed seam: it is **less coupled to live source-machine work**
than continuations are, and it gives every later emitter slice a **stable
unit/call vocabulary** to move against. Moving it first means the emitters are
not re-homing against a vocabulary that is itself still moving.

**`PredeclaredFunctionId` stays unit-owned.** That is one of the three standing
amendments and it is this slice's to honour, not a later slice's.

> **Modules own semantic lifecycles.** The durable direction of the whole phase
> is *plan construction -> validated read-only views -> lowering state and source
> machine -> concrete backend mutation -> independent evidence -> closure ->
> publication*. **Do not name a permanent module after a temporary campaign
> node**, and do not size modules to be equal.
> ### THE SEAM ALREADY EXISTS. This slice extends it rather than inventing it.
>
> `static_transition.rs` **already has a sibling `static_transition/` directory**
> holding `semantic_ir.rs` and `abi.rs`. This is not the original interlude,
> which had to create `cranelift_backend/` from a monolith. **Extend the
> established seam; do not design a new one.**

> ### `StaticTransitionPlan` STAYS IN THE PARENT. Through all six domain moves.
>
> **Architect `evt_6r403ez3m2m69`, from item 3's hard stop.** It is the parent
> container. A child domain module **may own its types and domain-specific
> inherent impls while reading ancestor-private root state** — that is the
> pattern for every planner slice, established here.
>
> Item 3 tried to extract a graph foundation ahead of the domains and the
> subtraction proof came back **empty**: the genuinely shared identities already
> live in `semantic_ir`, and the apparent residual was **data vocabulary with no
> owned lifecycle**. Moving it alone yields the `ids.rs` drawer the research
> report warns against. **That node is CLOSED and gates nothing.**

# `D0` — THE LEDGER. No code moves in this deliverable.

> ## RE-CUT AT `0542bfbbc`, closing the AC-1 gaps QA found. 2026-08-18.
>
> **History, for a pickup that does not want to re-derive it.** The original
> `D0` candidate (`0fd56146f`) was measured at base `7509c77a7`, which predates
> `40cb6b3b5` — the publish that applied the Architect's and research's
> whole-plan amendments and rewrote this file. Rebasing alone (to `1a7ecc8e1`,
> onto `main` at `0542bfbbc`) was clean and produced no semantic delta — QA
> confirmed the apparent 2343-line/18-frame two-way diff against `main` was
> base skew, not a candidate defect.
>
> **QA then resolved the routed fork on its own authority, textually, not as a
> judgment call:** `AC-2` ("each moved test"), `AC-3` ("every moved item") and
> `AC-4b` ("every file this slice creates or enlarges") all condition on a
> moved or created artifact. `D0` moves nothing, so none of the three can be
> evaluated against an artifact that does not yet exist — **they bind `D1`/`D2`,
> not `D0`.** The Steward confirmed this stands and is not gated on the
> Architect.
>
> **What QA found instead, independent of that fork, is what this re-cut
> closes: the rebased ledger failed `AC-1` on its own terms.** Its own selector
> command, run across all three bound files and presented as "checked against
> the entries below," in fact returns 142 type declarations against a table
> that reconciled only 25; only the type class had a selector at all; one
> method row named a group ("ABI preflight/read-only-view helpers") instead of
> enumerating members; and the ledger never stated whether it checked for
> source-text oracles. The ledger below is the recut against those four points,
> re-measured at `0542bfbbc`.

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

# `D1` — THE MOVE. Behaviour-preserving, and reviewable as a relocation.

Move the owner into its own child module, extending the established seam.
Adapters are permitted **as transitional scaffolding only**, and item 18 deletes
them.

# `D2` — THE COMPANION TEST MOVE. Separate accepted partial.

`lowering/core/tests/control.rs` was **33,969 lines at
`a1cf83622`** and is **in scope** — the
operator's constraint says large files and excepts nothing, and a test file is
not exempt. **It is a companion axis, not a phase of its own.**

**Move only the tests whose primary discriminated property belongs to the owner
this slice just established** (unit and ABI controls, call-edge views,
pre-emission validation). Place multi-leaf fixtures **once**,
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


## `D0` ledger, re-measured at `0542bfbbc`

The Stage A inventories were read as a starting point and re-measured at this
SHA. The owner is a lifecycle, not every name that happens to mention a unit:
validated semantic ownership -> ABI descriptor/frame -> read-only emission
view -> pre-emission validation. `StaticOriginId` and source/child
correspondence remain occurrence-owned; `StaticTransitionPlan` remains the
parent container.

**Declared selector population, per `AC-1`'s conjunctive population clause.**
"Exact and complete" is bounded by a stated population for every class below,
and every declaration that population contains is reconciled to exactly one owner —
either this slice's Units/ABI owner (and named as a `D1` move target), or a
named other owner it is excluded to. No class is closed by a count-plus-blind-
spot paragraph alone.

### Symbol ledger — types (declared population: 142)

Selector used for non-private type declarations, unchanged from the prior
ledger:

```sh
rg -n '^\s*pub(?:\([^)]*\))?\s+(?:struct|enum|type)\s+[A-Za-z_][A-Za-z0-9_]*' \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition/abi.rs \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition/semantic_ir.rs \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs
```

At `0542bfbbc` this returns **142** declarations: 21 in `abi.rs`, 28 in
`semantic_ir.rs`, 93 in `static_transition.rs`. The selector cannot see
private types, macro-generated declarations, split-line declarations, traits,
constants, functions, or fields — those classes are ledgered separately
below, not folded into this count. **Private-type blind spot, checked rather
than assumed:** `rg -n '^\s*(struct|enum)\s' <range>` over the exact source
range of every method this `D1` moves (`static_transition.rs:13513`–`18171`)
returns zero private `struct`/`enum` declarations — the moved surface
declares no private type the `pub` selector could have hidden.

**Every one of the 142 is reconciled below.** Four are this slice's owner and
move in `D1`. The other 138 are named individually and excluded, grouped by
the other domain that owns them — this is a closure of the full selector
population, not a group label standing in for one.

**Moved (4):**

| Symbol group | Old path | New path in D1 | Lifecycle reason |
| --- | --- | --- | --- |
| `AbiCarrier`, `AbiStorageOwner`, `AbiOwnership`, `AbiSlotKind`, `AbiCaptureProvenance`, `AbiUnitDefinition`, `AbiRootIngress`, `AbiSchedulingIngress`, `AbiProcessParameter`, `AbiSlot`, `AbiFrameHeader` | `planning/static_transition/abi.rs` | unchanged: `planning/static_transition/abi.rs` | closed carrier, ownership, provenance, and frame-slot vocabulary already has its owner — **not counted in the "4 moved," listed here because it is the other half of `abi.rs`'s 21** |
| `AbiDescriptorShape`, `AbiDescriptor`, `AbiContinuationDescriptor`, `AbiContinuationInputAuthority`, `AbiContinuationInputProvenance`, `AbiContinuationContextDescriptor`, `AbiStaticContinuationFusionDescriptor`, `PlannedStaticContinuationFusionAbi`, `AbiPlane`, `AbiBoundarySignature` | `planning/static_transition/abi.rs` | unchanged: `planning/static_transition/abi.rs` | descriptor and pre-emission validation plane stays private to planner siblings — **also unmoved, the rest of `abi.rs`'s 21** |
| `PredeclaredFunctionId` | `planning/static_transition/semantic_ir.rs` | `planning/static_transition/units.rs` | unit identity belongs with descriptors and call-edge views; the standing amendment, not a graph-id extraction. **Correction from the prior ledger: it has no `impl PredeclaredFunctionId` block anywhere in the crate** (`grep -rn "impl PredeclaredFunctionId" crates/ken-runtime/src/cranelift_backend/` returns nothing) — "and its unit-identity methods" overclaimed a method surface that does not exist. It is a bare `#[repr(transparent)]` newtype, moved by construction/pattern-match sites only. |
| `EmittableCallEdge`, `EmittableCallKind`, `EmittableUnit` | `planning/static_transition.rs` | `planning/static_transition/units.rs` | planner-minted, lowering-readable unit/call vocabulary with private fields and no lowering constructor; their inherent methods are enumerated in the functions/methods ledger below, not summarized as a group |

`abi.rs`'s 21 are listed above (unmoved) so the "moved (4)" count is
auditable against the full 142 rather than presented as a bare number: **4
moved** = `PredeclaredFunctionId` + `EmittableCallEdge` + `EmittableCallKind`
+ `EmittableUnit`; the 21 `abi.rs` types are unmoved members of the same
file-unchanged group already covered by "unchanged: `abi.rs`" in the prior
ledger, restated here for completeness rather than double-counted.

**Excluded (138), named individually and grouped by owning domain — none
moved by this `D1`:**

| Excluded because owned by | Symbols (all individually named; none moved) |
| --- | --- |
| Continuations (`RT-PLANNER-CONTINUATIONS-SPLIT`) | `ContinuationSpecializationId`, `ContinuationEmissionOwner`, `ContinuationContextId`, `PlannedContinuationContext`, `ContinuationContextView`, `ContinuationInputSource`, `ProducerLocalBinding`, `ProducerLocalLocator`, `ContinuationSourceCoordinate`, `ContinuationEnvironmentClaimOver`, `ContinuationEnvironmentClaim`, `ContinuationEnvironmentDraft`, `ContinuationFrameRequirement`, `ContinuationFrameIdentity`, `D3bFinalizationPerturbation`, `ContinuationAvailabilityOver`, `ContinuationAvailabilityViews`, `ContinuationAvailabilityDraft`, `ContinuationSourceSlotAuthority`, `ContinuationWorkerCaptureSource`, `ContinuationWorkerCaptureProvenance`, `ContinuationConsumingOccurrence`, `RequiredConsumerProjection`, `ContinuationCallIdentity`, `ContinuationUnitView`, `ContinuationOrdinaryEnvelopeRole`, `ComposedWorkerRouteEligibility`, `ComposedWorkerView`, `ComposedCallTarget`, `ComposedCallTargetDefect`, `ContinuationInputView`, `ContinuationCallView`, `ContinuationResultEdge`, `AdmittedContinuationDiscovery`, `RequiredConsumerProjectionDisposition`, `ContinuationRequiredConsumerObservation`, `RequiredConsumerProjectionMutation`, `EnvelopeDefect`, `CheckedCaseBinderRole`, `CheckedCaseBinderLayout`, `CheckedIhBinding`, `CheckedTransportCoordinate`, `StaticContinuationFusionId`, `StaticContinuationFusionKey`, `StaticContinuationFusionDescriptor`, `StaticContinuationFusionPlan`, `StaticContinuationFusionView`, `BodyEmissionDisposition`, `FusionOwnedBody`, `FusionRegionClaim`, `FusionClaimRefusal`, `FusionRegionClaimLedger`, `FusionClaimParameterMutation`, `FusionProducerCaptureMutation`, `StaticContinuationFusionCandidate`, `FusionComposedEdge`, `FusionOwnedOuterRealization`, `FusionCompositionLayer`, `CaseEmissionStatus`, `DeclarationCallTargetClass` (case/dispatch surface — hedged toward continuations over joins/traps on `ContinuationEmissionOwner`'s adjacency; the claiming slice's own `D0` is the final word) |
| Aggregates (`RT-PLANNER-AGGREGATES-SPLIT`) | `AggregateOccurrenceId`, `AggregateOccurrenceProducer`, `SynthesizedAggregateRole`, `PlannedAggregateShape`, `PlannedAggregateView`, `PlannedAggregateAllocation`, `PlannedAggregateChild`, `PlannedAggregateOwnership`, `SynthesizedAggregateRoot`, `SynthesizedAggregateStep`, `SynthesizedAggregatePath`, `SynthesizedAggregateNode`, `SynthesizedDynamicSet`, `SynthesizedHostResultTree` |
| Effects (`RT-PLANNER-EFFECTS-SPLIT`) | `EffectSeatPhase`, `EffectSeatOperation`, `EffectSeatSlot`, `EffectSeatNeed`, `EffectSeatAvail`, `PlannedEffectSeat`, `EffectSeatPlanMutation` |
| Joins/traps (`RT-PLANNER-JOINS-TRAPS-SPLIT`) | `JoinResultRepresentation`, `JoinPlanToken`, `PlannedTrapIdentity`, `D2jCause`, `D4bVerdict` |
| Root/parent-shared, stays with `StaticTransitionPlan` (not this slice's to move; no successor slice claim implied) | `StaticTransitionPlan`, `ScaleBPlanCensus`, `PlannedResultFieldKindForTest`, `PlannedReferentLifetime` (allocation-lifetime vocabulary with no single clean domain owner at this reading — flagged, not resolved, for whichever of aggregates/root-closure claims it) |
| Semantic-IR shared substrate in `semantic_ir.rs` (root/parent-shared — the same "no owned lifecycle" finding item 3 made for the graph-id extraction, applied here to the representation types) | `ConstructorIdentity`, `SynthesizedFixedConstructorRole`, `SynthesizedIoErrorRole`, `SynthesizedConstructorRole`, `FieldIdentity`, `SemanticProgramId`, `CaptureLayoutId`, `SemanticOwner`, `DenseRange`, `SemanticOpcode`, `RuntimeExprShape`, `SemanticSourceKind`, `SemanticSourceSeed`, `SemanticOperandElement`, `SemanticAtomKind`, `SemanticMaterialArena`, `CaptureSlot`, `RuledChild`, `SemanticRecord`, `SemanticProgram`, `CaptureLayout`, `SemanticDescriptor`, `SemanticPlane`, `D2aPopulationMutation`, `BodyOccurrenceMutation` |
| Occurrence-owned (standing amendment; explicitly not this slice's to move) | `StaticOriginId` |
| Occurrence-adjacent record, excluded despite touching this owner's id | `PredeclaredFunction` — the `{id, planned_node, body_occurrence, program}` record binding a `PredeclaredFunctionId` to its scheduling seed (`StaticNodeId`) and body occurrence (`StaticOriginId`, occurrence-owned). Only the bare id type is this slice's owner; the binding record couples units to occurrence lifecycle and is excluded, deferred to whichever slice claims the occurrence/lifecycle table. |

**Reconciliation check, by file, against the selector counts above** (the "4
moved" are a subset of these totals, not additional to them):
`abi.rs` 21 = 11 + 10, both rows unmoved (the two "moved" table rows above
are `abi.rs`'s full population, restated from the prior ledger, not new
moves — `abi.rs` itself does not move); `static_transition.rs` 93 = 3 moved
(`EmittableCallEdge`, `EmittableCallKind`, `EmittableUnit`) + 90 named in the
exclusion table (60 Continuations + 14 Aggregates + 7 Effects + 5 Joins/traps
+ 4 root/parent-shared — the Continuations row's list is 60 distinct names,
not the 62 backtick occurrences in it: `ContinuationEmissionOwner` is named
once in the list and once more in the row's own hedge parenthetical, and
`D0` inside that same parenthetical is prose, not a symbol); `semantic_ir.rs`
28 = 1 moved (`PredeclaredFunctionId`)
+ 27 named in the exclusion table (25 semantic-IR substrate + `StaticOriginId`
+ `PredeclaredFunction`). `21 + 93 + 28 = 142`, and every name in every file's
selector output above appears in exactly one row of this ledger — checked by
diffing the selector's raw name list against every backtick-quoted identifier
in this section, not asserted.

No visibility widening is authorized. `AbiPlane`, descriptors, builders, and
validators remain planner-private; `EmittableUnit` and `EmittableCallEdge` stay
read-only with private fields. This is an old/new relocation ledger, not an API
change or a facade proposal.

**Use-site re-measurement at `0542bfbbc`**, per "re-measure every count you
rely on" — `rg -l '\b<Symbol>\b' crates/ken-runtime/src/` for each of the 4
moved types: `PredeclaredFunctionId` in `lowering/core.rs`,
`lowering/core/tests/{constructors,control}.rs`, `lowering/mod.rs`,
`lowering/units.rs`, `planning.rs`, `planning/static_transition.rs`,
`planning/static_transition/{abi,semantic_ir}.rs`; `EmittableCallEdge` in
`lowering/core/tests/control.rs`, `planning/static_transition.rs`;
`EmittableCallKind` in `lowering/mod.rs`, `lowering/units.rs`,
`planning.rs`, `planning/static_transition.rs`; `EmittableUnit` in
`lowering/core/tests/control.rs`, `planning/static_transition.rs`. Consistent
with the Stage A census, with `abi.rs` added as a current `PredeclaredFunctionId`
consumer (a use-site the pinned-SHA census did not carry) — every consumer
file's import path is re-pointed at `units.rs` in `D1`, not silently left
stale.

### Symbol ledger — functions and methods (declared population: 317 `pub fn`)

Selector, run per file and excluding `const fn` (a naming trap: an earlier
draft of this recut's own selector matched `const fn` as a const item before
this correction — `rg` has no look-ahead, so the exclusion is a manual
`grep -v`, stated so the next pickup does not repeat it):

```sh
rg -n '^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+[A-Za-z_]' <file>
```

Counts at `0542bfbbc`: `abi.rs` 13, `semantic_ir.rs` 30, `static_transition.rs`
274 — **317 total**, inherent methods and free functions together (the
selector cannot distinguish them from the pattern alone; the split below is
by manual read of each impl block).

**Moved (18: 5 + 7 + 5 + 1), individually named — this is the enumeration
that replaces the prior ledger's "ABI preflight/read-only-view helpers"
group label:**

| Method | Owner type | Notes |
| --- | --- | --- |
| `caller`, `callee`, `callee_origin`, `call_site_origin`, `kind` | `EmittableCallEdge` (inherent impl, `static_transition.rs:13513`) | read-only accessors, no lowering constructor |
| `function`, `body_occurrence`, `entry_origin`, `definition`, `header`, `slots`, `slot_offsets` | `EmittableUnit<'plan>` (inherent impl, `static_transition.rs:13555`) | read-only accessors; `slot_offsets` delegates to `abi::slot_offsets`, not re-derived |
| `emittable_call_edges`, `executable_units`, `executable_call_edges`, `emittable_units`, `root_emittable_unit` | `StaticTransitionPlan` (inherent impl, `static_transition.rs:13721`–`18171`) | validated read-only projections into the `Emittable*` vocabulary; **`root_emittable_unit` is a real addition over the prior ledger's five-name list, found by grepping every `StaticTransitionPlan` method whose return type mentions `Emittable*`** |
| `validate_emitted_transfers_are_representable` | `StaticTransitionPlan` (same impl block) | pre-emission validation named directly in this slice's OWNER description; delegates to `abi::validate_emitted_transfers`; was entirely absent from the prior ledger's grouped row |

`PredeclaredFunctionId` contributes zero methods (see the symbol-ledger
correction above).

**Near-misses checked and excluded — named because a substring match on
"abi"/"unit"/"predeclared"/"emittable" would have wrongly caught them, and
each was read in full before exclusion:**

| Method | Why it looks like this owner | Why it is not |
| --- | --- | --- |
| `expect_entry_abi` | Returns a tuple including `PredeclaredFunctionId`; name contains "abi" | Method on a Continuations-domain enum variant matcher (`ContinuationInputSource`-family `Self::EntryAbi`/`Self::ProducerLocal`); `#[cfg(test)]`-only assertion helper for continuation input source |
| `PlannedTrapIdentity::abi_word` | Method name is `abi_word` | Inherent method on `PlannedTrapIdentity`, a Joins/traps-owned type (see the type-exclusion table); the word format happens to be ABI-shaped, the type is not this owner's |
| `verify_current_lexical_availability` | Takes `emission_owner: PredeclaredFunctionId` as a parameter | Validates a `ContinuationProducerEnvironment` lexical-availability relation; Continuations-domain, `PredeclaredFunctionId` is a foreign key here, not the subject |
| `verify_predeclared_entry_frame_membership` | Name contains "predeclared"; takes `frame: PredeclaredFunctionId` | Validates continuation-source-coordinate membership (`RT-CONTSRC-PRODUCER-LOCAL D3b`); Continuations-domain despite the vocabulary overlap |
| `unit_boundary_environment_occurrence` | Name contains "unit" | Aggregate-occurrence lookup keyed on `SynthesizedAggregateRoot::UnitBoundaryEnvironment`; "unit" here names an aggregate root kind, a false-friend collision with this owner's "unit" |
| `continuation_units` | Adjacent to `root_emittable_unit`; revalidates ABI descriptor/specialization agreement | Returns `Vec<ContinuationUnitView<'_>>`, a Continuations-owned type; it is a cross-boundary *consumer* of this owner's ABI data after `D1`, not a member of it — flagged below as a `D1` consumer to watch, not excluded silently |
| `required_join_origins` | Takes `function: PredeclaredFunctionId` | Joins/traps-domain (join origin computation); `PredeclaredFunctionId` is a foreign key |
| `static_body_source_bindings` | Returns a tuple containing `PredeclaredFunctionId` | Source/body-binding tracking is occurrence-domain vocabulary; the id is a foreign key, not the subject |

**Cross-boundary consumer to watch at `D1`, not a blocker:** `continuation_units`
(Continuations-domain, unmoved) reads this owner's ABI descriptor count and
the plan's continuation-specialization population to revalidate agreement
between them. After `D1` this becomes a read across the `units.rs`/parent
boundary — permitted by the standing "child domain module may own its types
while reading ancestor-private root state" pattern (`evt_6r403ez3m2m69`), not
an exposed behavioural dependency requiring a stop.

**Closure for the remaining 299 (317 declared − 18 moved):** none are moved
by this `D1`. They remain declared exactly where they are today — either
other planner domains' not-yet-extracted inherent methods on
`StaticTransitionPlan`, free functions serving those domains'
validation/construction, or `semantic_ir.rs`'s shared representation methods
(e.g. `SemanticMaterialArena`, `SemanticPlane`). This `D0` does not
pre-enumerate their ownership by the same discipline the frame's frozen
stage predicate applies to per-domain symbol sets: doing so here would
duplicate and stale the claiming slices' own `D0` ledgers. This is the
"narrow the declared population" resolution AC-1 permits, applied to a class
this slice never claimed complete in the first place — unlike the type
class above, where the prior ledger's own wording claimed a completeness it
did not have.

### Symbol ledger — consts and statics (declared population: 4 true consts, 35 `thread_local!` keys)

Two selectors, corrected to exclude `const fn` (see the functions section
above for why a naive `const|static` regex over-matches):

```sh
rg -n '^\s*pub(?:\([^)]*\))?\s+const\s+' <file> | grep -v 'const fn'
rg -n '^\s*(pub(?:\([^)]*\))?\s+)?static\s+[A-Za-z_][A-Za-z0-9_]*\s*:' <file>
```

**True `const` items (4), none moved:** `SynthesizedFixedConstructorRole::ALL`
(`semantic_ir.rs:89`, Aggregates-owned type), `MAX_HELPERS_PER_STATIC_SOURCE`
(`static_transition.rs:55`, source-machine/occurrence-scoped),
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1` (`static_transition.rs:5305`,
Effects-owned), `D2J_DECLARATION` (`static_transition.rs:19194`,
Joins/traps-owned test fixture literal).

**`thread_local!`-scoped `static` keys (35), none individually moved:** 4 sit
in `abi.rs` (`D2_IGNORE_DECLARATION_OWNERSHIP`,
`D2_CLAIM_ALL_BODIES_DECLARATION_OWNED`, `D3_C4_MATCHES_CLOSURE_BODY_ONLY`,
`SKIP_CONTINUATION_ABI_PREFLIGHT`) and travel with `abi.rs` unchanged — this
owner's mutation-test cells, already covered by the "unchanged: `abi.rs`"
disposition above, not a new move. The remaining 31 are mutation-test cells
for the other five planner domains (continuation, aggregate, effect, fusion,
join/worker) declared in `semantic_ir.rs` and `static_transition.rs`; none
belong to this owner and none move.

### Symbol ledger — modules and re-exports (declared population: 2 `pub use` blocks in scope, 0 `pub mod`)

`static_transition.rs` has no `pub mod`. Its `pub use` statements
(`static_transition.rs:39-53`, `:18419-18428`):

| Re-export | Contents | D1 effect |
| --- | --- | --- |
| `use abi::{...}` (two blocks) | `AbiCaptureProvenance`, `AbiCarrier`, `AbiFrameHeader`, `AbiOwnership`, `AbiProcessParameter`, `AbiRootIngress`, `AbiSchedulingIngress`, `AbiSlot`, `AbiSlotKind`, `AbiStorageOwner`, `AbiUnitDefinition`, `expected_capture_slot` | None — `abi.rs` is unchanged, this re-export line is untouched |
| `use semantic_ir::{...}` | `ConstructorIdentity`, `FieldIdentity`, `PredeclaredFunctionId`, `StaticOriginId`, `SynthesizedConstructorRole`, `SynthesizedFixedConstructorRole`, `SynthesizedIoErrorRole` | Only the `PredeclaredFunctionId` re-export path changes (it now resolves through `units.rs`, not directly from `semantic_ir`); the other six names are unaffected |
| `#[cfg(test)] use semantic_ir::{with_last_io_error_role_omitted, with_d2a_population_mutation, D2aPopulationMutation}` | test-only | Aggregates/occurrence-domain (`D2a`), unaffected |
| `#[cfg(test)] use tests::{contspec_nested_fixture, d2j_checked_fixture_under, d2j_installed_plan_under, D2jCause, D2J_DECLARATION}` | test-only | Joins/traps-domain (`D2j`), unaffected |

### Symbol ledger — traits (declared population: 0)

`rg -c '^\s*pub(?:\([^)]*\))?\s+trait\s+[A-Za-z_]'` returns 0 across all
three files. No locally declared trait exists to reconcile. The moved types'
derives (below) are the only trait surface they carry, and all are
`#[derive(...)]`-generated, not manual `impl Trait for`.

### Symbol ledger — cfg, attributes, derive, repr and visibility

**Corpus-scale occurrence counts, for the full three-file population:**
`#[derive(...)]` 20 (`abi.rs`) + 29 (`semantic_ir.rs`) + 140
(`static_transition.rs`) = 189; `#[cfg(...)]` 9 + 16 + 218 = 243. **Blind
spot, stated rather than closed by this count:** the selector counts
attribute *occurrences*, not distinct attributed items — one item can carry
several derives, and a doc comment or `#[repr]`/visibility modifier is not a
`derive`/`cfg` match at all.

**Per-item closure for the moved population — what AC-1 asks of `D0` on its
own, independent of any move.** Read from source, not inferred:

| Item | Derive | `#[repr]` | Visibility | `#[cfg]` |
| --- | --- | --- | --- | --- |
| `PredeclaredFunctionId` (`semantic_ir.rs:243-245`) | `Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd` | `transparent` | `pub(in crate::cranelift_backend)` | none |
| `EmittableCallEdge` (`static_transition.rs:13498-13499`) | `Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd` | none | `pub(in crate::cranelift_backend)` | none |
| `EmittableCallKind` (`static_transition.rs:13507-13508`) | `Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd` | none | `pub(in crate::cranelift_backend)` | none |
| `EmittableUnit<'plan>` (`static_transition.rs:13544-13545`) | `Clone, Copy, Debug` | none | `pub(in crate::cranelift_backend)` | none |
| The 18 moved methods (`EmittableCallEdge`'s 5, `EmittableUnit`'s 7, `StaticTransitionPlan`'s 6) | n/a (methods, not derived) | n/a | uniformly `pub(in crate::cranelift_backend)` — checked individually at each of the 18 sites, not inferred from one | none carry a `#[cfg(...)]` — checked individually at each site |

This is a present-fact statement about what the moved population carries
today, not a preservation check — preservation across the actual move is
`AC-3`'s transport manifest at `D1`, which this table does not pre-empt or
duplicate.

### Symbol ledger — macro-produced owned items

`macro_rules!` count: 0 in all three files. The only macro-produced owned
items are the 35 `thread_local!`-scoped `static` keys already reconciled in
the consts-and-statics section above; there is no second macro-produced
population to close.

### Source-text oracles (Stage A's corpus-wide 49)

`include_str!` count in the three bound files: 0 in `abi.rs`, 0 in
`semantic_ir.rs`, **3** in `static_transition.rs` — all of Stage A's
corpus-wide 49 that fall inside this slice's bound files.

- `b2r_ac6_the_abi_plane_declares_no_emission_construct` and
  `b2r_ac7_the_abi_plane_adds_no_parser_and_no_dependency_edge`
  (`static_transition.rs:28229`, `:28280`) both `include_str!` `abi.rs`.
  `abi.rs`'s path is unchanged by this `D1`, so these are inert to this move
  — already named in the test-property ledger below (`b2r_ac6_*`, `b2r_ac7_*`).
- `the_semantic_seed_api_accepts_only_occurrence_origins`
  (`static_transition.rs:27058`) `include_str!`s `static_transition.rs`
  itself and counts exact-line occurrences of `children: &[StaticOriginId],`.
  This is Occurrence-domain (AC-12, not this owner), and its target line
  occurs only at `static_transition.rs:12558,12575` — outside every method
  this `D1` moves (`13513`–`18171`). Checked, not assumed: `grep -n
  'children: &\[StaticOriginId\],' static_transition.rs` returns exactly
  those two lines. Moving this owner's methods out does not change this
  test's count.

### Test-property ledger

| Test property | Current location | Mutation / discriminating surface | Fixture owner | D1/D2 disposition |
| --- | --- | --- | --- | --- |
| descriptor/unit bijection and caller-independent descriptor | `planning/static_transition.rs`: `b2r_ac1_*`, `b2r_ac2_*` | descriptor population and irrelevant caller binding | `b2r_plan`, closure fixtures | D1 with units/ABI tests |
| capture provenance, carrier choice, ABI payload/depth invariance | `planning/static_transition.rs`: `b2r_ac3_*`, `b2r_ac4_*`, closed-ground-carrier control | capture slots, carrier and implicit-tail validation | `b2r_seed_closure`, `b2r_lexical_closure` | D1 with units/ABI tests |
| imported-capture refusal and named validation arms | `planning/static_transition.rs`: `b2r_ac5_*`, `b2r_ac11_*` | imported edge and validation-arm mutations | `b2r_plan` | D1 with units/ABI tests |
| ABI plane remains inert and parser-free | `planning/static_transition.rs`: `b2r_ac6_*`, `b2r_ac7_*` | source-topology inventory only | no runtime fixture | remain planner-wide: not a behavioural D2 move candidate |
| descriptor population over fixtures | `planning/static_transition.rs`: `b2r_ac10_*` | fixture descriptor prediction | shared B2R fixtures | D1 with units/ABI tests |
| continuation ABI slot count, affinity/lifetime refusal, allocation-free preflight | `planning/static_transition.rs`: `contspec_abi_*` and `contspec_parameter_affinity_*` | capture addition and owner/lifetime/affinity disagreement | contspec fixtures | D1 with units/ABI tests |
| pre-emission ABI-domain refusals reach before a call emits | `lowering/core/tests/control.rs`: `d5_c4_abi_domain_mutations_each_refuse_before_any_call_is_emitted` | ABI-domain mutation guards and nonzero reached refusal | shared lowering control fixture | D2: primary discriminated property is this owner, fixture remains at the lowest common ancestor |

The table deliberately leaves source-topology controls lowering-wide: they do
not establish a moved runtime property and would make D2 a production-path or
line-range test split. D0 makes no phase-closure claim; only the later closure
node can discharge the 10k-file objective.

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

**Bound file: `cranelift_backend/planning/static_transition.rs`.**

**A split and semantic work on the same files cannot run concurrently** —
campaign section 4, ground 3. That is the constraint that orders this whole
phase, and it is the reason the planner domains come first:

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

**Re-derive every symbol by name at pickup**, never by the line offsets in any
frame or census row. `static_transition.rs` was 34,883 lines at
`a1cf83622`, and every slice
moves some of them.

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

---

## D1 AC-5 adapter/facade debt ledger — `RT-PLANNER-UNITS-ABI-SPLIT` D1

**Candidate:** `45e622243` (post-fix SHA recorded at handback).
**Slice:** D1 — the move (this slice).

### Scaffolding introduced

| Symbol | Location | Why temporarily required | Closure deletion obligation |
| --- | --- | --- | --- |
| `pub(in crate::cranelift_backend) use units::{EmittableCallEdge, EmittableCallKind, PredeclaredFunctionId};` | `planning/static_transition.rs` (parent re-export block, immediately after the `semantic_ir` re-export) | The parent's `pub(in crate::cranelift_backend) use semantic_ir::{...}` block previously re-exported `PredeclaredFunctionId` and the parent directly defined `EmittableCallEdge`/`EmittableCallKind`; consumers in `planning.rs` and `lowering/` resolve these names through the parent namespace. D1 moved the definitions into `units.rs`, so this re-export preserves the parent-path surface those consumers already import from — a behaviour-preserving move must not force every consumer to re-point simultaneously. `EmittableUnit` is **not** re-exported: the compiler proved it unused outside `units.rs` (sole references are doc comments), so the facade is already narrowed to the three consumed names. | `[[RT-BACKEND-SPLIT-CLOSURE]]` (item 18) narrows this facade when consumers re-point at `units.rs` directly. The closure slice deletes this re-export block (or the now-dead names in it) once no consumer resolves through the parent. |

### Evidence seats (guardrail 7)

- **Intention producer:** this D1 slice (the move).
- **Independent artifact observer / evidence decoder:** `scripts/ken-cargo -p ken-runtime` (scoped build + lib tests; the unused-import warning on `EmittableUnit` was the compiler's evidence the facade was over-broad, acted on in this slice).
- **Closeout / publication seat:** `[[RT-BACKEND-SPLIT-CLOSURE]]` (item 18).

### Non-scaffolding normalization (permitted, no ledger debt)

Module declaration (`mod units;`), the `use super::units::PredeclaredFunctionId` re-pointing in `abi.rs` and `semantic_ir.rs`, and the one test-only import re-pointing in the parent's `mod tests` are path-qualification normalization, not adapters — they carry no closure obligation.
