---
id: RT-PLANNER-CONTINUATIONS-SPLIT
title: "Move the continuations domain out of planning/static_transition.rs -- sequenced deliberately after the continuation and evidence churn, so the domain is not re-homed against a surface that is still moving"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-OCCURRENCES-SPLIT]
blocks: [RT-PLANNER-AGGREGATES-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 6; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 6 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Continuations.** Continuation keys, continuation-seat construction, and the
evidence surfaces keyed on them.

> ### THIS SLICE IS ORDERED LAST AMONG THE EARLY PLANNER DOMAINS ON PURPOSE
>
> **Architect: continuations move only after the live continuation/evidence
> churn is gone.** A domain moved while its own surface is under semantic edit is
> re-homed twice, and the second move is the expensive one because the ledger
> from the first is stale.
>
> **Check the churn is actually gone before starting** — re-derive it, do not
> inherit this sentence. The `RT-CONTKEY-*` and `RT-CONTINUATION-*` families are
> where to look.

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
this slice just established** (continuation-key, continuation-seat and
continuation-evidence controls). Place multi-leaf fixtures **once**,
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

## D0 ledger, re-measured at `11a590363`

The Stage A inventories (taken at `4de486514`) were read as a starting point
and re-measured here at `11a590363` — the item-6 release commit, which already
carries item 4's D1/D2 (units) and item 5's D1/D2 (occurrences). Every count is
re-derived at this SHA with the selector stated; the census pages are not cited
as authority.

### Churn gate, re-derived (the frame's own instruction)

The frame orders this slice last among the early planner domains so the
continuation/evidence surface is not re-homed while it is still moving. The
Steward's ruling (`evt_30wcfm1bpswwj`) holds the gate: the two ready-but-parked
continuation nodes — `RT-CONTKEY-REFUSAL-PROFILE-SPLIT` and
`RT-CONTSRC-CALLABLE-CONTRACT` — are parked (not dispatched) under the
operator's single-lane priority, so neither can land while item 6 is in flight.
"No continuation churn lands mid-slice" is guaranteed by the parking, not by the
nodes being merged. They re-anchor to the moved child module when unparked; that
cost falls on them, not on this slice. Recorded, not silent.

### Boundary proposal — the design judgment this D0 discharges

The continuations domain is **continuation keys, continuation-seat
construction, and the evidence surfaces keyed on them** — plus the
static-continuation **fusion** identity plane, which item 5's landed ledger
grouped under this same owner ("Continuations + fusion",
`RT-PLANNER-UNITS-ABI-SPLIT.md` exclusion table). `StaticTransitionPlan` stays
the parent container; only the continuation-owned types, records, validations
and views move.

**MOVED → new `planning/static_transition/continuations.rs` child module:**

- 58 non-private types (the whole `Continuation*`/`Composed*`/`Required*`/
  `Envelope*`/`Checked*`/`StaticContinuationFusion*`/`Fusion*` vocabulary plus
  `BodyEmissionDisposition`) — full name list in the type ledger below.
- 19 private records/keys (`ContinuationInputProjection`,
  `ContinuationEmitterFrame`, `ContinuationValueSourceAuthority`,
  `ContinuationProducerEnvironment`, `ContinuationProjectionOmission`,
  `ContinuationInternMutation`, `ContinuationProductionMutation`,
  `ContinuationWorkerProvenance`, `ContinuationSpecializationKey`,
  `PlannedContinuationSpecialization`, `ContinuationSpecializationCallToken`,
  `PlannedContinuationSpecializationCall`,
  `ContinuationConsumingOccurrenceSeed`, `ContinuationConsumingOccurrenceSeeds`,
  `ContinuationRequiredConsumingOccurrence`, `ContinuationDiscovery`,
  `ContinuationConsumingOccurrenceSeedMutation`, `CheckedBinderProvenance`,
  `CheckedTransportScope`).
- The 45 continuation-owned free functions (name list in the fn ledger) and
  the inherent methods on the 58 moved types (each moved type's `impl` block
  moves with it).
- The Steward's D0 ruling (`evt_30wcfm1bpswwj`), carried verbatim:
  `consuming_occurrence_from_seed` (~:10991), `rederive_consuming_occurrence`
  (~:11148), and `ContinuationSourceSlotAuthority` (:1434) are part of the
  continuations domain and move WITH it.

**RETAINED — `StaticTransitionPlan` stays the parent.** Its continuation-keyed
FIELDS (`continuation_specializations`, `continuation_specialization_calls`,
`required_consumer_projections`, `continuation_contexts`,
`static_continuation_fusions`, …) stay on it (storage is the container's); only
the field TYPES move. The origin-enumerating / aggregate / effect / join /
case-emission views remain with their domains; `StaticOriginId`/`StaticNodeId`
remain occurrence/graph-owned (item 5, not re-opened).

**No genuine fork found.** The one boundary that could have forked — is fusion
continuations-owned or a separate later slice? — is settled by item 5's landed
ledger, which already grouped `StaticContinuationFusion*`/`Fusion*` under this
owner. Recording it here as considered-and-settled-by-precedent, not re-derived.

### Symbol ledger — types (declared population: 91 non-private + 53 private)

Non-private selector, unchanged from items 4/5:

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(struct|enum|type)[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs
```

Returns **91** at `11a590363`. Private selector `^(struct|enum|type)` (column 0)
returns **53**. Blind spot: private types (closed below), macro-produced decls
(macro_rules! count 0), split-line decls, traits, consts, fns, fields.

**Moved (58 non-private + 19 private = 77):**

Non-private (58), by name: `ContinuationSpecializationId`,
`ContinuationEmissionOwner`, `ContinuationContextId`, `PlannedContinuationContext`,
`ContinuationContextView`, `ContinuationInputSource`, `ProducerLocalBinding`,
`ProducerLocalLocator`, `ContinuationSourceCoordinate`,
`ContinuationEnvironmentClaimOver`, `ContinuationEnvironmentClaim`,
`ContinuationEnvironmentDraft`, `ContinuationFrameRequirement`,
`ContinuationFrameIdentity`, `D3bFinalizationPerturbation`,
`ContinuationAvailabilityOver`, `ContinuationAvailabilityViews`,
`ContinuationAvailabilityDraft`, `ContinuationSourceSlotAuthority`,
`ContinuationWorkerCaptureSource`, `ContinuationWorkerCaptureProvenance`,
`ContinuationConsumingOccurrence`, `RequiredConsumerProjection`,
`ContinuationCallIdentity`, `ContinuationUnitView`,
`ContinuationOrdinaryEnvelopeRole`, `ComposedWorkerRouteEligibility`,
`ComposedWorkerView`, `ComposedCallTarget`, `ComposedCallTargetDefect`,
`ContinuationInputView`, `ContinuationCallView`, `ContinuationResultEdge`,
`FusionComposedEdge`, `FusionOwnedOuterRealization`, `FusionCompositionLayer`,
`AdmittedContinuationDiscovery`, `RequiredConsumerProjectionDisposition`,
`ContinuationRequiredConsumerObservation`, `RequiredConsumerProjectionMutation`,
`EnvelopeDefect`, `CheckedCaseBinderRole`, `CheckedCaseBinderLayout`,
`CheckedIhBinding`, `CheckedTransportCoordinate`, `StaticContinuationFusionId`,
`StaticContinuationFusionKey`, `StaticContinuationFusionDescriptor`,
`StaticContinuationFusionPlan`, `StaticContinuationFusionView`,
`BodyEmissionDisposition`, `FusionOwnedBody`, `FusionRegionClaim`,
`FusionClaimRefusal`, `FusionRegionClaimLedger`, `FusionClaimParameterMutation`,
`FusionProducerCaptureMutation`, `StaticContinuationFusionCandidate`.

Private (19): the name list in the boundary section above.

**Excluded (33 non-private + 34 private), grouped by owning domain — none
moved:**

| Excluded because owned by | Names (count) |
| --- | --- |
| Declaration-call | `DeclarationCallTargetClass` (1) |
| Joins/traps (`RT-PLANNER-JOINS-TRAPS-SPLIT`) | `JoinResultRepresentation`, `JoinPlanToken`, `PlannedTrapIdentity`, `D4bVerdict`, `D2jCause` (5) |
| Case-emission | `CaseEmissionStatus` (1) |
| Root/parent-shared, stays with `StaticTransitionPlan` | `StaticTransitionPlan`, `ScaleBPlanCensus`, `PlannedResultFieldKindForTest`, `PlannedReferentLifetime` (4) |
| Aggregates (`RT-PLANNER-AGGREGATES-SPLIT`) | `AggregateOccurrenceId`, `AggregateOccurrenceProducer`, `SynthesizedAggregateRole`, `PlannedAggregateShape`, `PlannedAggregateView`, `PlannedAggregateAllocation`, `PlannedAggregateChild`, `PlannedAggregateOwnership`, `SynthesizedAggregateRoot`, `SynthesizedAggregateStep`, `SynthesizedAggregatePath`, `SynthesizedAggregateNode`, `SynthesizedDynamicSet`, `SynthesizedHostResultTree` (14) |
| Effects (`RT-PLANNER-EFFECTS-SPLIT`) | `EffectSeatPhase`, `EffectSeatOperation`, `EffectSeatSlot`, `EffectSeatNeed`, `EffectSeatAvail`, `PlannedEffectSeat`, `EffectSeatPlanMutation` (7) |
| Test-fixture enum (item 5's D2 seam) | `FixtureWitness` (1) |

Private (34) are graph/planner/case/join/boundary-owned and stay:
`RecursiveLoweringFrameGuard` (lowering-guard), `PlannedExpr`, `PlannedEntryBody`,
`StaticNodeId`, `StaticEdgeId`, `StaticSourceId`, `PersistentNodeId`,
`TransitionKind`, `EdgeKind`, `D4DeclarationTargetMutation`, `StoreKind`,
`PlannedHelperKey`, `DynamicActivationFrame`, `PersistentStoreNode`,
`StaticNode`, `StaticEdge`, `EdgeEvidence`, `PlanContext`, `PlannedJoinResult`,
`CaseProducerSet`, `CaseProducerFlowKind`, `CaseProducerFlowEdge`,
`CaseProducerAuthority`, `PlannedCaseEmission`, `BoundaryACensus`,
`BoundaryB1Census`, `Planner`, `ResultPhase`, `ResultPhaseSummary`,
`CaseProducerFact`, `SynthesizedTreeResolution`, `FlattenedSynthesizedUse`,
`ProducerLocalKind`, `StaticWorkerMemberMutation`.

Reconciliation: `91 = 58 + 1 + 5 + 1 + 4 + 14 + 7 + 1`;
`53 = 19 + 34`. `91 + 53 = 144`, every name above appears in exactly one row.

### Symbol ledger — functions and methods (declared population: 269 `pub fn` + 45 private free fns)

Selector (per file, excluding `const fn` by `grep -v`):

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(async[[:space:]]+)?fn[[:space:]]+[A-Za-z_]' <file>
```

Returns **269** at `11a590363`. Plus 45 top-level private free functions owned
by continuations (moved, name list below).

**Moved free functions (45, private), by name:** `finalize_continuation_frame`,
`finalize_continuation_claim`, `finalize_continuation_availability`,
`finalize_continuation_availability_plan`, `continuation_input_view`,
`continuation_result_origins`, `build_continuation_worker_provenance`,
`continuation_owner_entry_sources`, `producer_local_source`,
`producer_local_value`, `walk_continuation_value_environment`,
`validate_continuation_source_slot`, `continuation_owner_source_root`,
`exact_continuation_source_environment`,
`continuation_emission_seat_environment`, `current_lexical_availability`,
`exact_continuation_projection`, `exact_continuation_ordinary_parameters`,
`continuation_keys_equal_under_mutation`, `intern_specialization`,
`with_continuation_consuming_occurrence_seed_mutation`, `envelope_defect`,
`derive_checked_transport`, `build_checked_transport`, `primary_fusion_key`,
`rederive_fusion_key`, `fusion_claim_error`,
`fusion_unique_static_body_triple`, `fusion_resolved_binder_body`,
`fusion_through_checked_wrappers`, `enumerate_live_fusion_candidates`,
`enumerate_live_fusion_candidates_with_input_size`,
`fusion_root_source_for_future_enumerator`, `admitted_continuation_discoveries`,
`initial_continuation_discoveries`, `continuation_result_constructor_identities`,
`consuming_occurrence_from_seed`, `required_consuming_occurrence_for_alternative`,
`derive_required_consumer_occurrence`, `rederive_consuming_occurrence`,
`validate_continuation_consuming_occurrences`,
`build_continuation_specialization_plan`,
`validate_continuation_specialization_closure`,
`validate_required_consumer_projections`,
`validate_continuation_specialization_plan`.

**Moved methods:** the inherent `impl` blocks of the 58 moved types move with
their types (each accessor/projection is enumerated in D1's transport manifest,
per item 4's method-ledger discipline). The `StaticTransitionPlan`
continuation-owned view methods (the `pub(in crate::cranelift_backend)`
`continuation_*` / `composed_*` / `fusion_*` projections, e.g.
`continuation_units`, `continuation_context_for`, `continuation_fusions`,
`continuation_inputs`, `continuation_result_edges_owned_by`,
`ordinary_continuation_targets`, `install_static_continuation_fusions`,
`install_fusion_owned_bodies`, `observed_fusion_definition_count`,
`rederive_continuation_consuming_occurrence`) move too — the full list is D1's
AC-3 manifest, gated on this D0's boundary.

**Closure for the remainder:** none moved. They are the aggregate/effect/join/
case-emission/root-shared methods and free functions, and the occurrence/
semantic-IR substrate items that items 4/5 already assigned. Their per-item
ownership is their claiming slice's own D0 (the same narrowing items 4/5 applied).

### Symbol ledger — consts and statics (declared population: 3 consts + 28 `thread_local!` keys)

True `const` items: **3** (`MAX_HELPERS_PER_STATIC_SOURCE`,
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1`, `D2J_DECLARATION`) — none moved.
`thread_local!`-scoped `static` keys: **28** — none individually moved; the
continuation/fusion mutation cells (`D3_C4_MATCHES_CLOSURE_BODY_ONLY` in
`abi.rs`, the `D2a`/`BodyOccurrence` cells in `semantic_ir`) are test seams and
stay with their files. The continuation-owned mutation cells in
`static_transition.rs` (`CONTINUATION_PRODUCTION_MUTATION`,
`CONTINUATION_INTERN_MUTATION`, …) move with their domain in D1.

### Symbol ledger — traits (0) · modules/re-exports (0 `pub mod`, re-export paths unchanged) · macro-produced items (0)

`pub trait` count 0; `macro_rules!` count 0. The parent's `pub use`
re-export blocks are unchanged by this move (no continuation name is re-exported
through the parent today — checked, not assumed).

### Source-text oracles and `include_str!` paths

`include_str!` count in the bound file: **3** (item 5's ledger) — the
occurrence-owned `the_semantic_seed_api…` (:26714-equivalent, moved to
`occurrences.rs` in item 5), and `b2r_ac6`/`b2r_ac7` reading `abi.rs`. All inert
to THIS move (continuations touch none of them). The control.rs
`the_owner_classification_has_a_closed_production_naming_inventory` and
`the_backend_production_surface_inventory_is_closed` pins (item 5's ledger) will
need D1 re-anchoring when `continuations.rs` becomes a production module — the
module-inventory census adds a row; ledgered here, re-anchored in D1.

### Test-property ledger (for D2 — the companion test move)

Tests whose PRIMARY discriminated property is continuation-key / continuation-
seat / continuation-evidence / fusion, re-derived at `11a590363`. Class per the
four-way partition (domain / fixture / mutation control / end-to-end).

- static_transition.rs `mod tests`: **87** continuation-family tests (the
  `d2f_*`, `d2i_*`, `d2j_*`, `d2g_*`, `d3_*`, `d5a_*`, `d6a_*`, `d6b_*`,
  `d6c_*`, `d8*_*`, `contspec_*`, `contsrc_*`, `contkey_*`, `r3_*`,
  `required_consumer_*`, `composed_*`, `envelope_*`, `checked_transport_*`
  families) → move to `continuations.rs` `#[cfg(test)] mod tests` in D2. The
  exact per-test list + class is D2's re-measurement (by name, per the frame's
  "re-derive every symbol by name" rule), bounded here to the 87-name prefix set.
- lowering/core/tests/control.rs: the continuation/fusion emission and
  closeout controls (`d5_c4_*`, `d5a_*`, `d6a_*`, `d8*_*`, `contsrc_*`,
  `d4a_*`/`d4b_*` continuation-frame controls, `r3_*` fusion) → move to the LCA
  in D2, except the Class-4 end-to-end emission controls that stay.
- Borderline flagged for the Architect's D0 vote: the `d5a_*` marker-family and
  the `d8*_*` composed/transport families sit on the planner/emitter boundary —
  their primary property is planner-side continuation evidence (move) but a
  defensible reading treats the `d8m_*`/`d8p_*` transport rows as Class-4
  end-to-end (stay). Named so the vote can move them.

### Blind spots (stated, not closed) + evidence seats

The type selector cannot see private types (closed by the private selector),
macro-produced decls (0), split-line decls (none in the moved surface), traits
(0), consts/fns (separate selectors above), fields (the continuation-keyed
`StaticTransitionPlan` fields are named in the boundary section, not counted).
A declaration silent in the tables above is a gap, not a non-event.

- **Intention producer:** this D0 slice (the ledger).
- **Independent artifact observer / evidence decoder:** the selectors above, run
  against the tree at `11a590363`; D1's `scripts/ken-cargo` compile is the
  decode for the actual move.
- **Closeout / publication seat:** D1's AC-3 transport manifest +
  `[[RT-BACKEND-SPLIT-CLOSURE]]` (item 18).

This slice's own transfer (the ledger) is complete; **phase closure is NOT
claimed**.
