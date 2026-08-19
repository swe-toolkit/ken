---
id: RT-PLANNER-OCCURRENCES-SPLIT
title: "Move the occurrences domain out of planning/static_transition.rs -- StaticOriginId and source/child correspondence are occurrence-owned and must not be pulled into a shared identity drawer"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-UNITS-ABI-SPLIT]
blocks: [RT-PLANNER-CONTINUATIONS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 5; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 5 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Occurrences.** Occurrence minting, source/child correspondence, and the
occurrence-keyed views over the plan.

**`StaticOriginId` and source/child correspondence stay occurrence-owned.**
This is the second half of the graph-foundation amendment. The pull to hoist
`StaticOriginId` into a shared identity module is exactly the `ids.rs` drawer
that item 3's hard stop ruled against — **it has no owned lifecycle there.**

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
this slice just established** (occurrence minting, provenance and source/child
correspondence controls). Place multi-leaf fixtures **once**,
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

## D0 ledger, re-measured at `c4f79f7fa`

The Stage A inventories (taken at `4de486514`) were read as a starting point
and re-measured here at `c4f79f7fa` — the release commit, which already carries
item 4's D1 (`units.rs`) and D2 (test moves). Every count below is re-derived
at this SHA with the selector stated; the census pages are not cited as
authority.

### Boundary proposal — the design judgment this D0 discharges

The occurrences domain is the **occurrence lifecycle**: mint, record, validate,
and the read views over the occurrence table and source/child correspondence.
It is **not** every symbol that happens to carry a `StaticOriginId` as a field
— those are other domains' foreign keys, and they stay.

**MOVED → new `planning/static_transition/occurrences.rs` child module:**

| Item | Old path (line) | Class |
| --- | --- | --- |
| `StaticOriginId` | `semantic_ir.rs:35` | `pub(in crate::cranelift_backend)` newtype; the id |
| `PlannedOccurrence<'src>` | `static_transition.rs:390` | private occurrence record |
| `PlannedOccurrenceChildAuthority` | `static_transition.rs:490` | private authority record |
| `PlannedOccurrenceAuthority` | `static_transition.rs:498` | private authority record |
| `origin_of` | `static_transition.rs:185` | private fn; the sole mint site |
| `derive_occurrence_lifetime` | `static_transition.rs:3763` | private fn |
| `build_occurrence_authority_plan` | `static_transition.rs:3861` | private fn |
| `validate_occurrence_authority_plan` | `static_transition.rs:3902` | private fn |
| `occurrence_subtree_contains` | `static_transition.rs:4840` | private fn |
| `occurrence_authority` | `static_transition.rs:6074` | private fn |
| `record_source_occurrence` | `static_transition.rs:12776` | private `StaticTransitionPlan` method |
| `planned_occurrence_expr` | `static_transition.rs:13603` | private `StaticTransitionPlan` method |
| `source_occurrence` | `static_transition.rs:13643` | `pub(in …)` method |
| `source_occurrence_origin_at_ordinal_for_test` | `static_transition.rs:13673` | `pub(in …)` method |
| `child_static_origin` | `static_transition.rs:13729` | `pub(in …)` method |
| `root_static_origin` | `static_transition.rs:14661` | `pub(in …)` method |
| `validate_source_occurrence_table` | `static_transition.rs:17366` | private method |

**RETAINED — `StaticTransitionPlan` stays the parent container.** Its
occurrence-keyed fields stay on it (storage is the container's); only their
types move: `source_occurrences: Vec<Option<PlannedOccurrence<'src>>>`,
`occurrence_authorities: Vec<PlannedOccurrenceAuthority>`,
`root_occurrence: Option<StaticOriginId>`,
`declaration_occurrences: BTreeMap<String, StaticOriginId>`.

**The boundary line, stated once.** Occurrence identity + the occurrence table
+ the occurrence authority + the direct source/child-correspondence read views
are occurrence-owned. Origin-ENUMERATING views owned by other domains — the
join walks (`required_join_origins`, `source_join_origins_in_owner_subtree`,
`source_result_origins_in_owner_subtree`, `source_match_case_body_origins`),
the aggregate occurrence views (`source_aggregate_occurrence`,
`synthesized_aggregate_occurrence`, `unit_boundary_environment_occurrence`),
`declaration_occurrence_origin`, and the continuation/emitter origin views
(`worker_body_origin`, `closure_origin`, `body_origin`, `producer_*_origin`,
`continuation_origin`, …) — REMAIN with their domains. They consume
`StaticOriginId` as a foreign key; they do not mint, record, or validate
occurrences. `StaticNodeId`/`StaticEdgeId` (`static_transition.rs:191/:194`)
are graph/scheduling identity, NOT occurrence identity, and stay.

**No genuine fork found.** The boundary above is forced by the frame's owner
definition, the frozen predicate, and item 4's landed ledger (which already
retained the whole semantic-IR substrate including `SemanticPlane`,
`SemanticRecord`, `DenseRange` and the `child_origins` dense tables). The one
reading that would also move the dense semantic-IR `child_origins` table
contradicts item 4's landed exclusion, so it is not genuinely defensible; it is
recorded here as considered-and-rejected rather than silent.

### Symbol ledger — types (declared population: 117 non-private + 57 private)

Selector (non-private types), unchanged from item 4:

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(struct|enum|type)[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition/semantic_ir.rs
```

Returns **117** at `c4f79f7fa`: 90 in `static_transition.rs`, 27 in
`semantic_ir.rs`. Blind spot: private types, macro-produced decls, split-line
decls, traits, consts, fns, fields.

**Private types, closed rather than counted.** The private selector
`^(struct|enum|type)` (column 0) returns **56** top-level private types in
`static_transition.rs` and **1** in `semantic_ir.rs` (`OwnershipPartition`) —
57 total, all reconciled. None are macro-produced (macro_rules! count 0 in
both files). The body-less forward decl `struct RecursiveLoweringFrameGuard;`
(`static_transition.rs:73`) is a member of this class: it has impl+Drop at
`:76-93` and a production use at `:13004`
(`RecursiveLoweringFrameGuard::enter()`); it is **explicitly retained** with
the lowering-guard domain, not occurrence.

**Moved (4):**

| Type | Old → new | Visibility today (preserved) |
| --- | --- | --- |
| `StaticOriginId` | `semantic_ir.rs:35` → `occurrences.rs` | `pub(in crate::cranelift_backend)`; ordinal `pub(super)`; must NOT widen (standing amendment 3) |
| `PlannedOccurrence<'src>` | `static_transition.rs:390` → `occurrences.rs` | private |
| `PlannedOccurrenceChildAuthority` | `static_transition.rs:490` → `occurrences.rs` | private |
| `PlannedOccurrenceAuthority` | `static_transition.rs:498` → `occurrences.rs` | private |

**Excluded (113 non-private + 54 private), named individually and grouped by
owning domain — none moved:**

| Excluded because owned by | Non-private names (count) |
| --- | --- |
| Continuations + fusion (`RT-PLANNER-CONTINUATIONS-SPLIT`) | `ContinuationSpecializationId`, `ContinuationEmissionOwner`, `ContinuationContextId`, `PlannedContinuationContext`, `ContinuationContextView`, `ContinuationInputSource`, `ProducerLocalBinding`, `ProducerLocalLocator`, `ContinuationSourceCoordinate`, `ContinuationEnvironmentClaimOver`, `ContinuationEnvironmentClaim`, `ContinuationEnvironmentDraft`, `ContinuationFrameRequirement`, `ContinuationFrameIdentity`, `D3bFinalizationPerturbation`, `ContinuationAvailabilityOver`, `ContinuationAvailabilityViews`, `ContinuationAvailabilityDraft`, `ContinuationSourceSlotAuthority`, `ContinuationWorkerCaptureSource`, `ContinuationWorkerCaptureProvenance`, `ContinuationConsumingOccurrence`, `RequiredConsumerProjection`, `ContinuationCallIdentity`, `ContinuationUnitView`, `ContinuationOrdinaryEnvelopeRole`, `ComposedWorkerRouteEligibility`, `ComposedWorkerView`, `ComposedCallTarget`, `ComposedCallTargetDefect`, `ContinuationInputView`, `ContinuationCallView`, `ContinuationResultEdge`, `AdmittedContinuationDiscovery`, `RequiredConsumerProjectionDisposition`, `ContinuationRequiredConsumerObservation`, `RequiredConsumerProjectionMutation`, `EnvelopeDefect`, `CheckedCaseBinderRole`, `CheckedCaseBinderLayout`, `CheckedIhBinding`, `CheckedTransportCoordinate`, `StaticContinuationFusionId`, `StaticContinuationFusionKey`, `StaticContinuationFusionDescriptor`, `StaticContinuationFusionPlan`, `StaticContinuationFusionView`, `BodyEmissionDisposition`, `FusionOwnedBody`, `FusionRegionClaim`, `FusionClaimRefusal`, `FusionRegionClaimLedger`, `FusionClaimParameterMutation`, `FusionProducerCaptureMutation`, `StaticContinuationFusionCandidate`, `FusionComposedEdge`, `FusionOwnedOuterRealization`, `FusionCompositionLayer` (58) |
| Aggregates (`RT-PLANNER-AGGREGATES-SPLIT`) | `AggregateOccurrenceId`, `AggregateOccurrenceProducer`, `SynthesizedAggregateRole`, `PlannedAggregateShape`, `PlannedAggregateView`, `PlannedAggregateAllocation`, `PlannedAggregateChild`, `PlannedAggregateOwnership`, `SynthesizedAggregateRoot`, `SynthesizedAggregateStep`, `SynthesizedAggregatePath`, `SynthesizedAggregateNode`, `SynthesizedDynamicSet`, `SynthesizedHostResultTree` (14) |
| Effects (`RT-PLANNER-EFFECTS-SPLIT`) | `EffectSeatPhase`, `EffectSeatOperation`, `EffectSeatSlot`, `EffectSeatNeed`, `EffectSeatAvail`, `PlannedEffectSeat`, `EffectSeatPlanMutation` (7) |
| Joins/traps (`RT-PLANNER-JOINS-TRAPS-SPLIT`) | `JoinResultRepresentation`, `JoinPlanToken`, `PlannedTrapIdentity`, `D4bVerdict`, `D2jCause` (5) |
| Case-emission / declaration-call | `CaseEmissionStatus`, `DeclarationCallTargetClass` (2) |
| Root/parent-shared, stays with `StaticTransitionPlan` | `StaticTransitionPlan`, `ScaleBPlanCensus`, `PlannedResultFieldKindForTest`, `PlannedReferentLifetime` (4) |
| Semantic-IR shared substrate in `semantic_ir.rs` | `ConstructorIdentity`, `SynthesizedFixedConstructorRole`, `SynthesizedIoErrorRole`, `SynthesizedConstructorRole`, `FieldIdentity`, `SemanticProgramId`, `CaptureLayoutId`, `SemanticOwner`, `DenseRange`, `SemanticOpcode`, `RuntimeExprShape`, `SemanticSourceKind`, `SemanticSourceSeed`, `SemanticOperandElement`, `SemanticAtomKind`, `SemanticMaterialArena`, `CaptureSlot`, `RuledChild`, `SemanticRecord`, `SemanticProgram`, `CaptureLayout`, `PredeclaredFunction`, `SemanticDescriptor`, `SemanticPlane`, `D2aPopulationMutation`, `BodyOccurrenceMutation` (26) |

Non-private reconciliation: `90 = 58 + 14 + 7 + 5 + 2 + 4`;
`27 = 1 moved + 26 excluded`. `90 + 27 = 117`. Private reconciliation:
`56 + 1 = 57`, of which 3 move (the occurrence records) and 54 are retained.
The private class is NARROWED, not fully enumerated: the 3 moved records are
named in the moved table above; `RecursiveLoweringFrameGuard` (`st:73`) is
named retained (lowering-guard); `OwnershipPartition` (`sem:892`) is named
retained (semantic-IR substrate). The remaining 52 private types
(`StaticNodeId`, `StaticEdgeId`, `StaticSourceId`, `Planner`, `StaticNode`,
`StaticEdge`, `EdgeKind`, `TransitionKind`, `PlannedCaseEmission`,
`CaseProducerAuthority`, `CaseProducerFlowEdge`, `PlanContext`, and the
continuation/case-emission/fusion private records) are
graph/planner/case-emission/continuation-owned and stay in
`static_transition.rs`; their per-item ownership is the claiming slices' own
D0s, NOT this ledger's — AC-1's declared-population narrowing, stated rather
than asserted as "all reconciled".

### Symbol ledger — functions and methods (declared population: 293 `pub fn`)

Selector (per file, excluding `const fn` by `grep -v`):

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(async[[:space:]]+)?fn[[:space:]]+[A-Za-z_]' <file>
```

Returns **293** at `c4f79f7fa`: 263 in `static_transition.rs`, 30 in
`semantic_ir.rs`. (Item 4's D0 measured 317 across three files at `0542bfbbc`,
before its D1 moved 18 methods to `units.rs` and before the tree moved since;
this ledger's own selector at THIS sha is the authority, not a reconciliation
against that older number.)

**Moved (4 `pub` methods, the occurrence-keyed read views) + 9 private fns:**

`source_occurrence`, `source_occurrence_origin_at_ordinal_for_test`,
`child_static_origin`, `root_static_origin` (the 4 `pub`); `origin_of`,
`derive_occurrence_lifetime`, `build_occurrence_authority_plan`,
`validate_occurrence_authority_plan`, `occurrence_subtree_contains`,
`occurrence_authority`, `record_source_occurrence`, `planned_occurrence_expr`,
`validate_source_occurrence_table` (the 9 private).

**Closure for the remaining 289:** none moved. They are the other domains'
methods (continuation/aggregate/effect/join/case-emission/fusion inherent
impls on `StaticTransitionPlan` and on their own types) and `semantic_ir`'s
shared representation methods. They stay where they are today; their ownership
is their claiming slice's own D0, not this one's (the same narrowing item 4
applied, AC-1's declared-population clause).

### Symbol ledger — consts and statics (declared population: 4 consts + 31 `thread_local!` keys)

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+const[[:space:]]+' <file> | grep -v 'const fn'
grep -nE '^[[:space:]]*(pub(\([^)]*\))?[[:space:]]+)?static[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*:' <file>
```

True `const` items: **4** (3 `static_transition.rs`, 1 `semantic_ir.rs`) — none
moved. Corrected from a first pass that read 5: the phantom 5th was
`semantic_ir.rs:473` `pub(super) const fn control(...)` — a `const fn`, which
the selector's own `grep -v 'const fn'` excludes — and the `const { ... }`
blocks at `semantic_ir.rs:991`/`:1080` are `thread_local!` initializer
expressions, not `const` items. The four true consts
(`MAX_HELPERS_PER_STATIC_SOURCE:59`, `CRANELIFT_HOST_EFFECT_CONSUMERS_V1:5309`,
`D2J_DECLARATION:18851`, `SynthesizedFixedConstructorRole::ALL:90`) are none
occurrence-owned, and the moved fns' bodies carry zero references to
`MAX_HELPERS_PER_STATIC_SOURCE` — "none moved" holds.
`thread_local!`-scoped `static` keys: **31** (28 + 3) — none moved; the
occurrence domain has no mutation cell of its own (its `D2a`/`BodyOccurrence`
mutation cells live in `semantic_ir` and stay, per the substrate exclusion).

### Symbol ledger — modules and re-exports (declared population: 0 `pub mod`, 2 `pub use` blocks)

`static_transition.rs` has no `pub mod`. Its `pub use` statements
(`static_transition.rs:39-53`, `:18419-18428`) re-export `abi::*`,
`semantic_ir::*` and `units::*` names. The one effect on this move: the
`use semantic_ir::{ … StaticOriginId … }` re-export path changes (it resolves
through `occurrences.rs` instead of `semantic_ir.rs`). The other names are
unaffected. No `#[cfg(test)] use tests::{…}` seam is added by D0.

### Symbol ledger — traits (declared population: 0)

`grep -cE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+trait'` is 0 in both files.
The moved types' derives are `#[derive(...)]`-generated; no manual `impl Trait`.

### Symbol ledger — cfg / attributes / derive / repr / visibility (moved population)

`StaticOriginId`: `#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]`,
`#[repr(transparent)]`, `pub(in crate::cranelift_backend)`, no `#[cfg]`.
`PlannedOccurrence<'src>`: `#[derive(Clone, Copy)]`, private, no `#[cfg]`.
`PlannedOccurrenceChildAuthority` / `PlannedOccurrenceAuthority`:
`#[derive(Clone, Debug, Eq, PartialEq)]`, private, no `#[cfg]`.
The 4 moved `pub` methods and 9 private fns carry no `#[cfg]` (checked per
site). Preservation across the actual move is AC-3's job at D1.

### Source-text oracles and `include_str!` paths (Stage A's corpus-wide 49)

`include_str!` count in the two bound files: **3**, all in
`static_transition.rs`:

- `the_semantic_seed_api_accepts_only_occurrence_origins`
  (`static_transition.rs:26714`) `include_str!`s `static_transition.rs` and
  counts exact lines `children: &[StaticOriginId],` == 2 and
  `children: &[StaticNodeId],` == 0. **Occurrence-domain (moves with the owner
  in D2).** Moving `StaticOriginId`'s declaration does not change these counts;
  the test's own body moves, so its `include_str!` path stays valid (it reads
  the parent, not `occurrences.rs`).
- `b2r_ac6_the_abi_plane_declares_no_emission_construct` (:27577) and
  `b2r_ac7_the_abi_plane_adds_no_parser_and_no_dependency_edge` (:27628) both
  `include_str!` `static_transition/abi.rs`. `abi.rs` is untouched by this
  slice — inert.

**One oracle outside the bound files, flagged for D1:** the control.rs test
`the_owner_classification_has_a_closed_production_naming_inventory`
(`lowering/core/tests/control.rs:9790`) pins the production naming inventory
that names `StaticOriginId` (exactly `semantic_ir` + `abi`) AND pins the
exact declaration text `"pub(in crate::cranelift_backend) struct
StaticOriginId(pub(super) u32);"` (control.rs:9839). Moving `StaticOriginId`
to `occurrences.rs` will change both the naming inventory (a third file names
it) and the declaration site — this is a **non-move hunk D1 must re-anchor**,
ledgered here so it is not discovered at CI-red.

### Test-property ledger (for D2 — the companion test move)

Tests whose PRIMARY discriminated property is occurrence minting / provenance /
source/child correspondence, re-derived at `c4f79f7fa` by name (not the
census's pinned offsets). Class per the four-way partition (domain / fixture /
mutation control / end-to-end).

| Test | Old path | Class | D2 disposition |
| --- | --- | --- | --- |
| `content_equal_occurrences_resolve_to_distinct_occurrences` | `static_transition.rs` | domain | move with owner |
| `a_source_tree_at_a_different_address_yields_identical_origins` | `static_transition.rs` | domain | move with owner |
| `the_occurrence_table_is_total_over_every_planned_expression` | `static_transition.rs` | domain | move with owner |
| `occurrence_table_negative_controls_fail_at_named_artifacts` | `static_transition.rs` | domain | move with owner |
| `nested_computational_occurrences_stay_injective_under_a_shared_entry` | `static_transition.rs` | domain | move with owner |
| `a_body_occurrence_naming_an_in_range_control_node_is_refused` | `static_transition.rs` | mutation control | move with owner |
| `swapping_two_declaration_occurrences_is_refused` | `static_transition.rs` | mutation control | move with owner |
| `filing_two_occurrences_under_one_origin_is_refused` | `static_transition.rs` | mutation control | move with owner |
| `root_and_declaration_computational_bodies_take_the_resume_occurrence` | `static_transition.rs` | domain | move with owner |
| `the_semantic_seed_api_accepts_only_occurrence_origins` | `static_transition.rs` | domain (source oracle) | move with owner |
| `substrate_occurrence_owner_and_lifetime_are_exact` | `static_transition.rs` | domain | move with owner |
| `one_role_at_two_seats_is_two_non_aliasing_occurrences` | `static_transition.rs` | domain | move with owner |
| `unit_boundary_environment_record_has_a_structural_non_aliasing_occurrence` | `static_transition.rs` | domain | move with owner |
| `substrate_equal_case_lists_keep_distinct_occurrence_partitions` | `static_transition.rs` | domain | move with owner |
| `every_origin_to_expression_resolution_goes_through_the_single_route` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `exactly_one_plan_origin_to_expression_lookup_exists` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `every_expression_typed_field_is_a_reachable_positional_child_origin` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `swapping_two_same_shaped_children_swaps_their_derived_origins` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `perturbing_a_borrowed_address_does_not_move_any_derived_origin` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `every_source_term_carrier_holds_an_occurrence_and_never_a_bare_expression` | `lowering/core/tests/control.rs` | domain | move to LCA |
| `retained_closures_carry_a_static_origin_and_no_body_term` | `lowering/core/tests/control.rs` | domain | move to LCA |

**Borderline, classified as RETAINED with their domains (flag for the
Architect's D0 vote):** `boundary_b1_preserves_equal_occurrences_and_reuses_one_activation_program`
and `boundary_b1r_control_*` (boundary/activation, not occurrence minting);
`d8_join_plan_is_a_bijection_with_source_join_occurrences` (join);
`computational_match_is_the_sole_entry_occurrence_split` (computational-match
entry splitting). **Shakiest boundary, flagged:**
`nested_computational_occurrences_stay_injective_under_a_shared_entry` is
classified occurrence (in the move table above) on the ground that its
asserted property is occurrence injectivity, but a defensible reading puts it
with the computational-match entry-splitting population — it is named so the
vote can move it either way.

### Blind spots (stated, not closed)

The type selector cannot see private types (closed above by the private
selector), macro-produced decls (macro_rules! count 0 in both files),
split-line visibility/type declarations (none found by manual scan of the
moved surface), traits (0), consts/fns (separate selectors above), or fields
(the occurrence-keyed fields are enumerated in the boundary section, not
counted). A declaration silent in the tables above is a gap, not a non-event.

### Evidence seats (guardrail 7)

- **Intention producer:** this D0 slice (the ledger).
- **Independent artifact observer / evidence decoder:** `scripts/ken-cargo
  check -p ken-runtime --lib --tests` (compile is unaffected by D0 — no code
  moves; the ledger's selectors are the evidence, run against the tree at
  `c4f79f7fa`).
- **Closeout / publication seat:** D1's transport manifest (AC-3) carries
  preservation across the actual move; `[[RT-BACKEND-SPLIT-CLOSURE]]` (item 18)
  deletes any D1 re-export scaffolding this slice's move leaves behind.

This slice's own transfer (the ledger) is complete; **phase closure is NOT
claimed** — only item 18 discharges the 10k-file objective.
