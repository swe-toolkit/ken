---
id: RT-PLANNER-AGGREGATES-SPLIT
title: "Move the aggregates domain out of planning/static_transition.rs -- aggregate allocation events, relation closures and their planner-side lifecycle"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-CONTINUATIONS-SPLIT]
blocks: [RT-PLANNER-EFFECTS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 7; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 7 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Aggregates.** Aggregate allocation events, `AggregateRelationClosure`, and
the planner-side aggregate lifecycle.

The census's type-ownership ledger already shows this family declared across
`lowering/mod.rs` as well as the planner. **`D0` must state which declarations
are planner-owned and which are lowering-owned**, because the aggregate emitter
slice (item 15) takes the other half and the two must not both claim a symbol.

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
this slice just established** (aggregate allocation and relation-closure
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

## D0 ledger, re-measured at `afb726712`

The Stage A inventories (taken at `4de486514`) and the leader's pre-measured
envelope are read as a starting point and re-measured here at `afb726712` —
the item-6 release commit, which carries item 4's D1/D2 (units), item 5's
D1/D2 (occurrences), and item 6's D1/D2 (continuations + fusion). Every count
is re-derived at this SHA with the selector stated; neither the census pages
nor the kickoff's envelope are cited as authority.

Bound file: `cranelift_backend/planning/static_transition.rs`, **16528**
lines at `afb726712` (matches the leader's pre-measurement exactly).
`cranelift_backend/lowering/mod.rs` is **21200** lines at the same SHA — the
plausible lowering-owned half lives there.

### Boundary proposal — the design judgment this D0 discharges

**Aggregates.** Aggregate allocation events, `AggregateRelationClosure`, and
the planner-side aggregate lifecycle. The frozen predicate (frame, "THE
FROZEN STAGE PREDICATE"): **the planner owns** plan identities, minting,
relation and seat construction, validation and closure, and read-only
projections; **the emitter owns** concrete CLIF/backend mutation that
consumes a validated plan, and **may not mint or reshape planner identity**.

**Applying the predicate to the two candidate zones:**

- **`static_transition.rs` zone (this slice, `D1` moves it):**
  `AggregateOccurrenceId` (the identity), `AggregateOccurrenceProducer` (what
  the identity names), the shape/role/path/step/node vocabulary that derives
  and validates the lifetime meet, and `PlannedAggregateOwnership` (the
  closed, validated population) plus its read-only view
  (`PlannedAggregateView`). Every one of these is **minting, deriving, or
  read-only-projecting** a plan-side fact — squarely planner-owned by the
  predicate's own words.
- **`lowering/mod.rs` zone (item 15's half, NOT moved here):**
  `AggregateAllocationEvent` (keyed on `FuncId` + a raw CLIF
  `cranelift_codegen::ir::Value` — a concrete, per-compilation emission
  fact), `AggregateAllocationLedger` (the open/record/commit/close state
  machine over those concrete events, `open()`/`record_event()`/`relate()`/
  `commit()`/`close()` all mutate compilation-local emission bookkeeping),
  and `AggregateRelationClosure` (the whole-pass measurement `close()`
  returns). `close()`'s own signature —
  `fn close(&mut self, planned: &[PlannedAggregateOwnership]) -> Result<AggregateRelationClosure, _>`
  — is the predicate stated in code: it **consumes** the planner's validated
  population (`planned`) as a read-only input and **may not** mint or
  reshape it (the whole-pass law is `image(R) ⊆ P`, never widening `P`).
  `LocalAggregateEvents` (private) is `AggregateAllocationLedger`'s own
  per-body scratch state, same zone.

**No genuine fork.** Both zones apply the predicate's own words without
tension: the planner-zone types derive/validate/project a plan; the
lowering-zone types record what a live compilation actually emitted and
close a `population ⊇ image` relation against the planner's read-only
output. `CarrierAllocationRequest` and `GovernedAllocationMutation`
(lowering/mod.rs, both emitter types with `PlannedAggregate`/`NonAggregate`/
`SiblingAggregateProducer` variants) are a third thing entirely — carrier
allocation-request machinery that **references** `PlannedAggregateAllocation`
as an input variant name but is not itself in the aggregate-domain symbol
population; noted so their variant names are not mistaken for aggregate
types in a future grep.

**Items 7/15 settled**, matching the frame's own "THE FROZEN STAGE
PREDICATE" table.

### Symbol ledger — types (declared population: 33 non-private + 33 private)

Non-private selector (unchanged from items 4/5/6):

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(struct|enum|type)[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' \
  crates/ken-runtime/src/cranelift_backend/planning/static_transition.rs
```

Returns **33** at `afb726712` — exactly item 6's landed "Aggregates (14)"
exclusion row plus its other 19 excluded non-private names (`33 = 14 + 19`,
re-verified by name, not just by count). Private selector `^(struct|enum|
type)` (column 0) returns **33**. Blind spot: macro-produced decls
(`macro_rules!` count 0), split-line decls (none found in the aggregate
zone), traits (0, see below), consts/fns (separate selectors below), fields
(the aggregate-keyed `StaticTransitionPlan` field is named in the boundary
section, not counted here).

**Moved — non-private (14), by name, unchanged from item 6's exclusion row:**
`AggregateOccurrenceId` (:1674), `AggregateOccurrenceProducer` (:1685),
`SynthesizedAggregateRole` (:1728), `PlannedAggregateShape` (:1746),
`PlannedAggregateView` (:1757), `PlannedAggregateAllocation` (:1807),
`PlannedAggregateChild` (:1818), `PlannedAggregateOwnership` (:1864),
`SynthesizedAggregateRoot` (:1951), `SynthesizedAggregateStep` (:1968),
`SynthesizedAggregatePath` (:1998), `SynthesizedAggregateNode` (:2048),
`SynthesizedDynamicSet` (:2112), `SynthesizedHostResultTree` (:2336).

**Moved — private (2), CORRECTED against item 6's landed ledger:**
`SynthesizedTreeResolution` (:2392) and `FlattenedSynthesizedUse` (:2399).

> **Why this is a correction, not a fresh finding item 6 already made.** Item
> 6's D0 (`RT-PLANNER-CONTINUATIONS-SPLIT.md:488`) listed both names inside
> its 34-name "Private (34) are graph/planner/case/join/boundary-owned and
> stay" bucket — a **group label**, not a per-name domain call (the exact
> failure shape `AC-1` warns against: "a group label is not a ledger entry").
> Read against the actual tree: `SynthesizedTreeResolution` wraps
> `SynthesizedAggregateNode`, is produced and consumed only by
> `synthesized_tree_walk`/`synthesized_tree_node` (the aggregate recipe-tree
> walk), and `FlattenedSynthesizedUse` is the aggregate flattening's own
> per-use record (`path: SynthesizedAggregatePath`, `children: &'static
> [SynthesizedAggregateNode]`), produced only by
> `flatten_allocation_reachable_uses`/`collect_reachable_uses`. Neither name
> appears outside the aggregate zone (verified: grep hits are 100% within
> lines 2363-5750, all aggregate-domain call sites). Re-measuring by name,
> not by inherited bucket, is exactly what the frame's "the census is a
> starting point, not an authority" instruction is for — recorded here as a
> ledger correction, not a re-litigation of item 6's own moved set (which
> is untouched: nothing continuations/fusion-owned changes hands).
>
> **A second, inert discrepancy, noted for completeness:** item 6's private-34
> list also named `ProducerLocalKind`, which is no longer in this file at all
> — it moved to `continuations.rs` during item 6's own `D1`/`D2` (confirmed:
> `grep -rn ProducerLocalKind` finds it only under
> `planning/static_transition/continuations.rs`). Not aggregate-relevant;
> recorded so the private-type reconciliation below is exact rather than
> silently short by one name.

**Retained (19 non-private + 31 private = 50), grouped by owning domain —
none moved by this slice:**

| Excluded because owned by | Names (count) |
| --- | --- |
| Declaration-call | `DeclarationCallTargetClass` (1) |
| Joins/traps (`RT-PLANNER-JOINS-TRAPS-SPLIT`) | `JoinResultRepresentation`, `JoinPlanToken`, `PlannedTrapIdentity`, `D4bVerdict`, `D2jCause` (5) |
| Case-emission | `CaseEmissionStatus` (1) |
| Root/parent-shared, stays with `StaticTransitionPlan` | `StaticTransitionPlan`, `ScaleBPlanCensus`, `PlannedResultFieldKindForTest`, `PlannedReferentLifetime` (4) |
| Effects (`RT-PLANNER-EFFECTS-SPLIT`) | `EffectSeatPhase`, `EffectSeatOperation`, `EffectSeatSlot`, `EffectSeatNeed`, `EffectSeatAvail`, `PlannedEffectSeat`, `EffectSeatPlanMutation` (7) |
| Test-fixture enum (item 5's D2 seam) | `FixtureWitness` (1) |
| Private, graph/planner/case/join/boundary/ABI-worker-member-owned | `RecursiveLoweringFrameGuard`, `PlannedExpr`, `PlannedEntryBody`, `StaticNodeId`, `StaticEdgeId`, `StaticSourceId`, `PersistentNodeId`, `TransitionKind`, `EdgeKind`, `D4DeclarationTargetMutation`, `StoreKind`, `PlannedHelperKey`, `DynamicActivationFrame`, `PersistentStoreNode`, `StaticNode`, `StaticEdge`, `EdgeEvidence`, `PlanContext`, `PlannedJoinResult`, `CaseProducerSet`, `CaseProducerFlowKind`, `CaseProducerFlowEdge`, `CaseProducerAuthority`, `PlannedCaseEmission`, `BoundaryACensus`, `BoundaryB1Census`, `Planner`, `ResultPhase`, `ResultPhaseSummary`, `CaseProducerFact`, `StaticWorkerMemberMutation` (31) |

Reconciliation: `33 = 14 + 1 + 5 + 1 + 4 + 7 + 1`; `33 = 2 + 31`.
`33 + 33 = 66`; every name above appears in exactly one row, verified by
name against the two selector outputs (not by count alone).

### Symbol ledger — functions and methods (declared population: 157 `pub fn` + 41 private free fns)

Selector (unchanged from items 4/5/6, excluding `const fn` by construction —
none of the matched lines are `const fn`):

```sh
grep -nE '^[[:space:]]*pub(\([^)]*\))?[[:space:]]+(async[[:space:]]+)?fn[[:space:]]+[A-Za-z_]' <file>
grep -nE '^fn[[:space:]]+[A-Za-z_]' <file>   # private free fns, column 0
```

Returns **157** `pub fn` and **41** private free fns at `afb726712`. (These
counts are much smaller than item 6's pre-move 269/45 baseline because item
6's own `D1` already relocated 112 `pub fn` methods and moved `ProducerLocalKind`'s
former private-fn neighbours out with continuations — expected, re-derived
rather than assumed.)

**Moved — private free fns (16), by name:** `aggregate_child_referent_owners`
(:1919), `host_effect_recipe_tree` (:2187), `collect_site_operand_ordinals`
(:2363), `flatten_allocation_reachable_uses` (:2437),
`io_error_alternative_children` (:2465), `collect_reachable_uses` (:2479),
`node_referent_owners` (:2584), `site_operand_referent_owners` (:2655),
`dynamic_alternative_nodes` (:2689), `fixed_node_selected_owner` (:2718),
`fixed_node_selected_owner_of` (:2739), `unit_boundary_environment_fields`
(:3390), `build_aggregate_ownership_plan` (:3464),
`validate_aggregate_producers_are_unique` (:3664),
`validate_aggregate_ownership_plan` (:3679), `lifetime_referent_affinity`
(:3785).

**Moved — methods (15), the aggregate-owned `impl<'a> StaticTransitionPlan<'a>`
projections, by name:** `aggregate_allocation` (:5493),
`source_aggregate_occurrence` (:5521), `synthesized_aggregate_occurrence`
(:5549), `unit_boundary_environment_occurrence` (:5572),
`synthesized_aggregate_record` (private, :5602), `synthesized_tree_node`
(:5657), `synthesized_tree_walk` (private, :5695), `sibling_effect_seat`
(`#[cfg(test)]`, :5764 — despite its name, keyed on the aggregate
synthesized-occurrence lookup, not the `EffectSeat` plan domain; consumed by
`lowering/mod.rs::sibling_effect_seat_under_mutation` and
`lowering/core/tests/constructors.rs` for a wrong-seat-coordinate mutation
control), `aggregate_record_view` (:5808), `aggregate_ownership_records`
(:5828), `synthesized_dynamic_alternatives` (:5920),
`synthesized_alternative_population` (private, :5948),
`synthesized_root_alternative_population` (:6006),
`synthesized_aggregate_children` (:6032), `aggregate_allocation_at` (:6057).

**Moved — impl blocks (4):** `impl<'plan> PlannedAggregateView<'plan>`
(:1761), `impl SynthesizedAggregatePath` (:2003), `impl
SynthesizedAggregateNode` (:2125), `impl SynthesizedHostResultTree` (:2341).
The other 10 moved types have no dedicated `impl` block — their accessors
live entirely in the `StaticTransitionPlan` methods above.

**Explicitly retained, flagged (not silently kept) — three items the
selector would otherwise leave unaccounted:**

1. **`synthesized_seat_emission_owners`** (:2550) — returns
   `Vec<ContinuationEmissionOwner>` and walks `plan.continuation_contexts`; a
   **continuations-domain** enumeration (item 6's identity, not this
   slice's) whose only production callers today happen to be inside
   `build_aggregate_ownership_plan` (:3554, :3626). Consumed-by-aggregates is
   not owned-by-aggregates: claiming it would re-open item 6's landed
   boundary, which is banned scope. **Retained at the parent, continuations-
   owned**, referenced by aggregates.rs via `super::`.
2. **`host_effect_operation`** (:6015, private) — a small seat-to-operation
   lookup called by two aggregate methods (`synthesized_tree_node`,
   `synthesized_alternative_population`) **and** by
   `host_effect_site_operand_slots` (:5863, Effects-owned by return type —
   see below), which has **not yet been split out** (item 8 is a future
   slice). Moving this helper into `aggregates.rs` would force it to
   `pub(super)` or wider so the still-parent-resident Effects caller can
   reach it — a visibility widening solely to make the move compile, which
   `BANNED SCOPE` forbids. **Retained at the parent as a shared private
   helper**; `aggregates.rs`'s two callers reach it via `super::
   host_effect_operation`. Re-derive this call at item 8's own `D0` — moving
   Effects out first may let this helper follow it, or split cleanly the
   other way; not this slice's call to make unilaterally.
3. **`host_effect_site_operand_slots`** (:5863) — returns `BTreeSet<
   EffectSeatSlot>` (an Effects-domain type) and its stated purpose is "the
   exact argument slots [an] operation's planned synthesized result tree
   consumes" — an **Effects question** answered using the aggregate tree as
   an implementation detail. Classified **Effects-owned** by return type and
   purpose, not aggregate-owned by implementation; **retained**, not moved.

**Closure for the remainder:** none moved. `host_effect_seat_contract`
(:3048), `build_host_effect_seat_plan` (:3229), `mutate_planned_effect_seat`
(:3303), `validate_host_effect_seats_are_unique` (:3356),
`validate_host_effect_seat_plan` (:3371), `host_effect_seat_records`
(:5835), `host_effect_seat_slots` (:5845), `host_effect_seat` (:5882) are
Effects-owned (`PlannedEffectSeat`/`EffectSeatSlot`-typed, item 8/16).
`validate_substrate_preallocation_closure` (:3724) is case-emission/
occurrence-substrate-owned. `with_static_worker_member_mutation` (:3874),
`apply_static_worker_member_mutation` (:3892),
`closure_defines_a_planned_member` (:3958),
`validate_static_worker_member_population` (:4021) are the
`StaticWorkerMemberMutation`/ABI-worker-member domain (checkpoint `1c`, a
different `D7`-tagged obligation than the aggregate one — the file reuses
the `RT-DECL-CLOSURE-PORT D7` tag across two unrelated checkpoints, and only
the aggregate-ownership one is this slice's). `runtime_value_lifetime`
(:1624) is occurrence-owned (item 5; imported by `occurrences.rs`).
`reset_recursive_lowering_frame_count`, `max_recursive_lowering_frame_count`,
`dense_slice`, `planner_error`, `planner_capacity_error`, `is_source_join`,
`summarize_result_phase`, `result_phase_environment_for_owner`,
`build_join_result_plan`, `derive_case_producer_fact`,
`build_case_emission_plan`, `validate_case_emission_plan`,
`runtime_expr_tag` are shared/other-domain-owned, unrelated to aggregates.
Their per-item ownership is their claiming slice's own `D0`.

### Symbol ledger — consts and statics (declared population: 2 module-level consts + 7 `thread_local!` blocks / 11 keys)

True module-level `const` items (column 0): **2** —
`MAX_HELPERS_PER_STATIC_SOURCE` (:94, static-source domain) and
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1` (:3019, Effects-owned — the admitted-
operation inventory `host_effect_seat_contract` is total over). **Neither is
aggregate-owned; none moved.** (A third name, `D2J_DECLARATION`, lives
**inside** `mod tests` at :8351, indented — not module-level; joins/traps-
owned, not aggregate, not counted here.)

`thread_local!` blocks: **7**, **11** keys total —
`ACTIVE_RECURSIVE_LOWERING_FRAMES`/`MAX_RECURSIVE_LOWERING_FRAMES` (:97, lowering-frame domain), `D4_DECLARATION_TARGET_MUTATION` (:284, declaration-call domain), `D8_FORCE_VARIABLE_SPECIALIZED`/`D8_REMOVE_VARIABLE_CALLABLE_SEED` (:815, join domain), `EFFECT_SEAT_PLAN_MUTATION` (:3000, Effects), `D4B_ADMISSION`/`D4B_ADMISSION_ARMED` (:3815, worker-member-checkpoint-`1c` domain), `STATIC_WORKER_MEMBER_MUTATION` (:3863, same), `AC4_RESOLUTIONS`/`AC4_ROUTE_INVOCATIONS` (:5124, source-occurrence-resolution domain). **None aggregate-owned; none moved.** The aggregate zone has no test-mutation `thread_local!` cell of its own at this SHA — its A/B controls (the `sibling_effect_seat` route) reuse a `StaticTransitionPlan` method, not a cell.

### Symbol ledger — traits (0) · modules/re-exports (0 `pub mod`, re-export paths unchanged) · macro-produced items (0)

`pub trait` count 0; `pub mod` count 0; `macro_rules!` count 0 in the bound
file. The parent's re-export surface is unchanged by this ledger (no
aggregate name is re-exported through the parent today — checked, not
assumed).

### Source-text oracles and `include_str!` paths

`include_str!` count in the bound file: **2** at `afb726712` — `b2r_ac6`
(:13358) and `b2r_ac7` (:13409), both reading `static_transition/abi.rs`.
Both are ABI-plane tests (`b2r_ac6_the_abi_plane_declares_no_emission_
construct`, `b2r_ac7_the_abi_plane_adds_no_parser_and_no_dependency_edge`) —
**inert to this move**, same finding item 6 made for the identical two
lines. The control.rs census pins
`the_owner_classification_has_a_closed_production_naming_inventory` and
`the_backend_production_surface_inventory_is_closed` (also present in
`planning/static_transition/semantic_ir.rs`) will need `D1` re-anchoring
when `aggregates.rs` becomes a production module — the module-inventory
census adds a row; ledgered here, re-anchored in `D1`, same mechanism as
item 6's `continuations`/`fusion` rows.

### Test-property ledger (for `D2` — the companion test move)

Tests whose PRIMARY discriminated property is aggregate-ownership /
allocation-lane / synthesized-tree, re-derived at `afb726712` by reading
each candidate test body (not by a keyword-count heuristic — a raw
`[Aa]ggregate` grep over-counts on incidental fixture-variable names, see
below).

- **`static_transition.rs` `mod tests`** (opens :7727, **81** `#[test]`
  total, matching `D2`'s landed `st.rs retained 81`): a **contiguous
  12-test block**, :14812-:15945, all reading a real `Effect` seat fixture
  and asserting a fact about `PlannedAggregateOwnership`/
  `SynthesizedAggregateNode`/the allocation-lane derivation: `a_scalar_
  nodes_owner_set_comes_from_its_exact_spill_disposition` (:14812),
  `a_site_bound_child_is_resolved_against_the_seat_not_pruned` (:14892),
  `an_absent_node_is_never_a_child` (:14964), `a_dynamic_childs_owners_are_
  the_union_of_its_alternatives` (:14998), `the_flattening_reproduces_the_
  measured_tree` (:15112), `a_repeated_role_at_one_seat_gets_distinct_real_
  records` (:15349), `a_path_step_kind_is_load_bearing_not_an_index`
  (:15438), `an_abandoned_eager_template_contributes_neither_records_nor_
  allocation` (:15540), `a_records_lookup_key_includes_its_path_not_only_
  its_role` (:15613), `the_planner_owns_the_ordered_alternative_population`
  (:15704), `a_lawful_non_dynamic_root_is_not_a_failed_lookup` (:15792), and
  `substrate_same_shape_aggregates_keep_distinct_lifetimes` (:15945, the
  leader's pre-flagged test — confirmed, does not use the shared fixture
  below). All 12 classify **domain** (four-way partition): each asserts a
  property of the planner's own aggregate-ownership derivation, not a
  cross-cutting fixture, a mutation control at production injection, or an
  end-to-end transport check.
  > **Boundary note, not a move:** the immediately preceding test,
  > `substrate_case_emission_open_ingress_prunes_nothing` (:14738), is
  > case-emission-owned, **not** aggregate — a first crude keyword scan
  > mis-flagged it because the shared fixture immediately below it
  > (`d7_seat_fixture`, next paragraph) references
  > `synthesized_seat_emission_owners` and the scan window bled across the
  > function boundary. Corrected by reading the test's own body: it asserts
  > only `CaseProducerSet::Open`/`CaseEmissionStatus::Reachable`, no
  > aggregate symbol.
- **Shared fixtures (2), single-leaf — move with their sole user per the
  frame's fixture rule:** `d7_seat_fixture` (:14765, `pub(super) fn`) and
  `d7_three_operands` (:14791, `pub(super) fn`). Grepped: both are called
  exclusively by the 11 of the 12 tests above that need a live `Effect`
  seat (all but `substrate_same_shape_aggregates_keep_distinct_lifetimes`,
  which builds its own fixture inline) — zero call sites outside this
  block. Move once, into `aggregates.rs`'s own `#[cfg(test)] mod tests`.
- **`lowering/core/tests/control.rs`** (**32789** lines, **247** `#[test]`
  total at `afb726712`): a keyword scan finds **57** `[Aa]ggregate` hits,
  but **no dedicated aggregate-domain test family** (no `d5a`/`d6`-style
  prefix cluster the way item 6 found for continuations). Reading the hit
  sites: the large majority are a local variable literally named
  `aggregate` standing for a generic `RuntimeExpr::Construct` fixture in
  tests whose primary property is a **different** domain (worker binding,
  declaration-call ordering, closure capture); a smaller cluster (:5730-
  5804, :9955-10039, :18165, :22544-22566) touches the **lowering-owned**
  half (`AggregateAllocationLedger`/`commit_aggregate_events`/
  `open_aggregate_events`, item 15's territory) as an emitter-side Class-4
  end-to-end check, not a planner-domain test. **No test moves from
  `control.rs` in this slice's `D2`.** This is a by-name-scan negative
  finding over 247 tests at prefix-scan depth, stated as such — re-verify
  at `D2` pickup rather than trusting this ledger's count if `control.rs`
  has moved by then (per the frame's own "no upfront split" instruction,
  this was never this D0's job to fully resolve).

### Blind spots (stated, not closed) + evidence seats

The type selector cannot see private types (closed by the private
selector, corrected against item 6's stale bucket above), macro-produced
decls (0), split-line decls (none in the moved surface), traits (0),
consts/fns (separate selectors above), fields (the aggregate-keyed
`StaticTransitionPlan.aggregate_ownership: Vec<PlannedAggregateOwnership>`
field is named here, not counted as a moved item — storage is the
container's, only the field's declared TYPE moves). A declaration silent in
the tables above is a gap, not a non-event.

- **Intention producer:** this `D0` slice (the ledger).
- **Independent artifact observer / evidence decoder:** the selectors above,
  run against the tree at `afb726712`; `D1`'s `scripts/ken-cargo` compile is
  the decode for the actual move.
- **Closeout / publication seat:** `D1`'s `AC-3` transport manifest +
  `[[RT-BACKEND-SPLIT-CLOSURE]]` (item 18).

This slice's own transfer (the ledger) is complete; **phase closure is NOT
claimed**.

