---
id: RT-PLANNER-EFFECTS-SPLIT
title: "Move the effects domain out of planning/static_transition.rs -- effect seats, seat groups and effect-seat closures on the planner side"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-AGGREGATES-SPLIT]
blocks: [RT-PLANNER-JOINS-TRAPS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 8; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 8 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Effects.** Effect seats, `EffectSeatGroupId`, `EffectSeatClosure`, and the
planner-side effect lifecycle.

Same split-ownership caution as aggregates: the effect **emitter** family is
item 16. **`D0` states the planner/emitter boundary for every effect symbol**
before anything moves.

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
this slice just established** (effect-seat, seat-group and effect-closure
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

## D0 ledger, re-measured at `0f96b5b99`

Bound file: `cranelift_backend/planning/static_transition.rs`, **13,420 lines**
at this SHA. All line numbers below are re-measured at `0f96b5b99` and are not
inherited from any prior slice's frame or census row.

### Boundary proposal (frozen-predicate application)

The planner-owned Effects family is the **host-effect seat authority**: derive
one `PlannedEffectSeat` per capability/argument seat of every admitted `Effect`
occurrence, validate the population is the exact closed rebuild plus pairwise
distinct, and expose read-only projections of it. This is `plan
identities/minting/relation/seat construction/validation/closure/read-only
projections` verbatim — the planner half of the frozen predicate.

The emitter-owned Effects family — `EffectSeatGroupId`, `EffectSeatLedger`,
`EffectSeatClosure`, `EffectSeatVisitMutation`, `EffectSeatDispatchMutation` —
all live in `lowering/mod.rs` already, all consume a validated
`PlannedEffectSeat` population by claiming/closing seats, and none of them
mint or reshape a seat's identity. That is item 16's territory; this ledger
does not touch it and does not re-litigate it.

### THE CARRY-FORWARD — `host_effect_operation` and `host_effect_site_operand_slots`, re-derived from first principles

Both are `StaticTransitionPlan` methods still resident at the parent
(`host_effect_operation` @4182, private; `host_effect_site_operand_slots`
@4143, `pub(in crate::cranelift_backend)`). Item 7's D0 correctly identified
both as Effects-owned and item 7's D1 correctly left them at the parent,
because item 7 was not the Effects slice. **That deferral reason no longer
applies — this is the Effects slice.** Re-deriving from scratch against this
slice's own frozen predicate:

- **`host_effect_operation`** answers "what host operation does this
  occurrence represent" by matching `self.source_occurrences` against
  `RuntimeExpr::Effect` — a read-only projection of one seat's identity.
  Effects-owned, unambiguously. **Verdict: Effects-owned, moves in this
  slice's `D1`.**
- **`host_effect_site_operand_slots`** returns `BTreeSet<EffectSeatSlot>` — an
  Effects-typed read-only projection — by consulting
  `host_effect_recipe_tree`/`collect_site_operand_ordinals` (both
  **aggregates-owned**, already landed in `aggregates.rs` by item 7's `D1`,
  reached via `super::aggregates::{...}`). The internal dependency on another
  domain's helper is ordinary cross-domain consumption (the same shape as
  `lowering` depending on both `aggregates` and `effects`), not a reason to
  retain — the method's own identity (its name, its return type, its role as
  a planner-side effect-seat view) is Effects-owned. **Verdict: Effects-owned,
  moves in this slice's `D1`.**

**A load-bearing widening this move requires, flagged rather than silently
applied (`BANNED SCOPE`: "if a symbol must widen, that is a finding"):**
`host_effect_operation` is called not only by its own sibling
`host_effect_site_operand_slots` but also by **`aggregates.rs`** — a SIBLING
child module, at two call sites (`aggregates.rs:1614`, `aggregates.rs:1833`,
inside the `impl StaticTransitionPlan` wrapper item 7's `D1` installed there).
Today it is private to the file it is textually in
(`static_transition.rs`), which is visible to `aggregates.rs` only because
Rust privacy makes a parent module's private items visible to all its
descendants. Once `host_effect_operation` moves into a NEW child
(`effects.rs`, a sibling of `aggregates.rs`), plain module-privacy no longer
reaches `aggregates.rs` — the two are siblings, not ancestor/descendant. It
must widen to at least `pub(in
crate::cranelift_backend::planning::static_transition)` (equivalently
`pub(super)` from `effects.rs`, since `effects.rs`'s parent module IS
`static_transition`) to remain callable from `aggregates.rs`. This is a
widening **driven by real, already-existing cross-sibling consumption**, not
a convenience widening to make an unrelated move compile — it is `D1`'s to
apply, not `D0`'s, and is recorded here so `D1` does not have to rediscover
it from a compile error.

### THE NEGATIVE FINDING — `governed_nested_resource_bracket` is NOT Effects-owned

`governed_nested_resource_bracket` (`static_transition.rs:5712`,
`#[cfg(test)]`, `pub(in crate::cranelift_backend)`) surfaces in an `effect`
keyword scan — its body constructs `RuntimeExpr::Effect` nodes
(`BufferAllocate`/`BufferFreeze`) and one of its heaviest consumers is an
Effects-domain mutation control. **That is the same shape of trap this
frame's own kickoff names for `synthesized_seat_emission_owners`: present in
an effect-keyword scan, owned by neither domain the scan suggests.**

Its own doc comment states the real disposition: *"The governed nested-bracket
source shared by the planning and emission controls. Keeping one constructor
prevents the emission gate from silently measuring a trap-free or
non-recursive surrogate."* A full-crate grep of its call sites
(14 in `lowering/core/tests/control.rs` alone, spanning worker-construction,
capture, and bracket/scope tests entirely unrelated to Effects, plus the two
genuinely Effects-adjacent D7 controls) confirms it: this is a **general
cross-cutting fixture generator**, reached through `planning.rs`'s
general-fixture re-export group (`contspec_nested_fixture,
governed_nested_resource_bracket, plan_static_transition_graph`) — a
**different** re-export group than the Effects-domain group two lines below
it (`host_effect_seat_contract_of, EffectSeatNeed, ...`). It happens to be
resident in `static_transition.rs` (the bound file) and happens to build
`Effect`-shaped occurrences as example content, but it is not itself part of
the host-effect seat authority and does not become part of it by proximity.

**Verdict: EXPLICITLY RETAINED at the parent, owning domain "shared
cross-domain test fixture" — not claimed by this slice.** Moving it would
also be a `BANNED SCOPE` violation on its own terms (it is not this domain's
to move), and — checked, since the frame's `D2` fixture-placement rule
applies in spirit — it already sits at its own lowest common ancestor (the
bound file every consuming domain's tests reach through one facade), so there
is no placement defect to fix.

### Symbol ledger — closed over every Rust item class

**Types (7, all already `pub(in crate::cranelift_backend)`, all
`#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]` except
`EffectSeatPlanMutation` which derives only `Clone, Copy, Debug, Eq,
PartialEq` — no `Ord`/`PartialOrd`):**

| type | lines | kind | fields/variants | cfg |
|---|---|---|---|---|
| `EffectSeatPhase` | 1752-1757 | enum | `SpecializedTemplate`, `CarriedWord` | none |
| `EffectSeatOperation` | 1761-1773 | enum | `SelectClosedTag`, `ProjectBytesSpan`, `ObserveResourceHandle`, `ObserveCapabilityToken`, `NarrowExactInt` | none |
| `EffectSeatSlot` | 1784-1789 | enum | `Capability`, `Argument(u32)` | none |
| `EffectSeatNeed` | 1806-1823 | enum | `ConstructorTag`, `BytesPointerLength`, `ResourceScalar`, `CapabilityTokenScalar`, `ExactIntU64` | none |
| `EffectSeatAvail` | 1835-1838 | struct | `specialized: bool`, `carried: bool` (both `pub(in crate::cranelift_backend)`) | none |
| `PlannedEffectSeat` | 1872-1908 | struct | 9 fields (`effect_origin`, `child_origin`, `position`, `operation`, `slot`, `producer_owner`, `consumer_owner`, `semantic_operation`, `need`, `avail` — all `pub(in crate::cranelift_backend)`) | none |
| `EffectSeatPlanMutation` | 1959-1972 | enum | `Exact`, `EraseOperation`, `EraseOrdinal`, `EraseNeed`, `CollapseContract` | `#[cfg(test)]` |

**Impls (2):**

- `impl EffectSeatAvail` (1840-1862): 2 assoc consts (`SPECIALIZED_ONLY`,
  `EITHER_PHASE`, both module-private `const`) + 1 method (`admits`,
  1856-1861, `pub(in crate::cranelift_backend)`).
- `impl PlannedEffectSeat` (1918-1946): itself `#[cfg(test)]`; 1 method
  (`for_observer_control`, 1930-1945, doubly-gated `#[cfg(test)]` on both the
  impl and the fn — the exact pattern a prior fix (item 7's D1 notes)
  recorded as load-bearing, since the `--lib` profile cannot see a
  wrongly-scoped single gate).

**Free functions (7):**

| fn | lines | vis | cfg | called from (outside its own domain family) |
|---|---|---|---|---|
| `set_effect_seat_plan_mutation` | 1981-1985 | `pub(in crate::cranelift_backend)` | `#[cfg(test)]` | `lowering/core/tests/control.rs` (cross-boundary) |
| `host_effect_seat_contract` | 2023-2187 | private | none | (only its own domain family) |
| `build_host_effect_seat_plan` | 2204-2275 | private | none | root builder (`static_transition.rs:3675`, stays put) |
| `mutate_planned_effect_seat` | 2278-2308 | private | `#[cfg(test)]` | (only its own domain family) |
| `host_effect_seat_contract_of` | 2318-2323 | `pub(in crate::cranelift_backend)` | none (production) | `lowering/mod.rs:13162` (cross-boundary, the emitter's independent close-time recompute) |
| `validate_host_effect_seats_are_unique` | 2331-2344 | private | none | (only its own domain family) |
| `validate_host_effect_seat_plan` | 2346-2356 | private | none | root builder + root validator (`static_transition.rs:3676`, `:5127`, both stay put) |

**Consts/statics (2):**

- `CRANELIFT_HOST_EFFECT_CONSUMERS_V1` (1994-2009): `pub(in
  crate::cranelift_backend) const`, `[ken_host::HostOpV1; 13]`, no cfg.
  Cross-boundary: `lowering/mod.rs:278`, `planning.rs:73`.
- `EFFECT_SEAT_PLAN_MUTATION` (1975-1978): `thread_local!` `static`, module
  private, `Cell<EffectSeatPlanMutation>`, `#[cfg(test)]`.

**`StaticTransitionPlan` methods (5, all `impl` fragments in the shared root
impl block — a method-level move, not a block move, same discipline as item
7):**

| method | lines | vis | consumed at |
|---|---|---|---|
| `host_effect_seat_records` | 4114-4117 | `pub(in crate::cranelift_backend)` | `lowering/units.rs:5414` (cross-boundary) |
| `host_effect_seat_slots` | 4119-4134 | `pub(in crate::cranelift_backend)` | `lowering/core.rs:17992`, `lowering/mod.rs:9587` (cross-boundary) |
| `host_effect_site_operand_slots` | 4136-4153 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:9649` (cross-boundary) |
| `host_effect_seat` | 4155-4176 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:9629` (cross-boundary) |
| `host_effect_operation` | 4181-4197 | private | `aggregates.rs:1614`, `:1833` (sibling — the flagged widening above) |

**Explicitly retained at the parent (not moved, owning domain named):**

- `host_effect_seats: Vec<PlannedEffectSeat>` — private field of
  `StaticTransitionPlan` itself (declared 700, initialized `Vec::new()` at
  2728). `StaticTransitionPlan` stays at the parent (same rule as every prior
  slice); only the field's element type's home changes, via an import.
- `build_host_effect_seat_plan`/`validate_host_effect_seat_plan` call sites
  at 3675-3676 (the plan builder's `finish`) and 5127 (`StaticTransitionPlan::
  validate`) — root orchestration that calls into every domain's
  build/validate function; stays at the parent exactly as
  `build_aggregate_ownership_plan`'s call sites did for item 7.
- `governed_nested_resource_bracket` (5712-5843) — the negative finding
  above: a shared cross-domain fixture, not Effects-owned.
- `occurrence_authority` (consumed at `build_host_effect_seat_plan:2224`) —
  already occurrences-owned (item 5, `occurrences.rs:253`,
  `pub(super)`), a cross-domain dependency only.
- `StaticOriginId`, `PredeclaredFunctionId` — occurrence-owned and
  unit-owned respectively per the frame's own "THREE STANDING AMENDMENTS";
  referenced, not owned, by this domain.

### Source-text oracles

- **LIVE**: `lowering/core/tests/control.rs`'s
  `the_backend_production_surface_inventory_is_closed` (module-inventory
  vec), `BACKEND_PRODUCTION_SOURCES` (the `include_str!` roster), and
  `correspondence_adds_no_emitted_unit_to_the_production_census` (the
  `Census` array) already carry rows for `"abi"`, `"aggregates"`,
  `"continuations"`, `"occurrences"` (items 4-7's precedent, each a 3-location
  addition). **`D1` will need the same 3-location addition for `"effects"`**
  once `effects.rs` exists — ledgered here as an anticipated non-move hunk,
  not executed in this docs-only `D0`.
- **INERT**: `lowering/core/tests/mod.rs:978-1135`,
  `exactly_one_plan_origin_to_expression_lookup_exists`, is gated
  `#[cfg(any())]` — permanently disabled, never compiled. Its `let planner =
  include_str!("../../../planning/static_transition.rs")` census would, if
  live, be sensitive to any Effects-domain `pub(in crate::cranelift_backend)
  fn` moving out of `static_transition.rs`'s own text — but it is dead code
  (its own exported-list is already stale against the live tree, a further
  sign of its disablement), so `D1` owes it nothing.

### Test-property ledger

**Zero tests in `static_transition.rs`'s own `mod tests` have Effects as
their primary discriminated property.** A by-name scan for every
Effects-domain symbol (`PlannedEffectSeat`, `EffectSeat*`, `host_effect_*`,
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1`) inside the test module (starting
5852) returns exactly 3 hits, all `ken_host::HostOpV1::BufferAllocate` /
`::BufferFreeze` used as **fixture content** inside
`d2h_ac2_the_three_expressible_refusals_mint_nothing` (7129) and
`substrate_case_emission_open_ingress_prunes_nothing` (12862) — both
case-emission/substrate-domain tests (a different, already-settled owner)
that happen to use an `Effect`-shaped occurrence as example data, exactly the
same shape as `governed_nested_resource_bracket`'s own false-positive above.
Neither test asserts anything about seat derivation, contract lookup, or
seat-plan validation. **This slice's `D2` therefore moves NO tests out of
`static_transition.rs`'s own test module** — the population is empty, stated
as a fact rather than left silent (per `AC-2`'s discovery-before-mutation
discipline, an empty population is still a population and is recorded as
one).

**Control.rs re-verification (required at this SHA, not inherited from any
prior slice's scan):** a full keyword census of `Effect` in
`lowering/core/tests/control.rs` (32,820 lines at `0f96b5b99`) returns 27
hits, closing as follows:

- 3 hits (`7365`, `7437`, `12271`, `18308` — 4 lines, one is prose) are
  generic `RuntimeExpr::Effect` fixture construction, unrelated to this
  domain's own symbols (`RuntimeExpr` is not owned by `static_transition.rs`
  at all).
- 13 hits (`18066`-`18105`) are `erasing_a_seat_key_axis_or_
  collapsing_the_contract_rejects` — a genuinely **planner-Effects**-domain
  mutation control (`set_effect_seat_plan_mutation`/`EffectSeatPlanMutation`,
  which exercise `build_host_effect_seat_plan`'s rebuild-equality
  validation). **Its primary discriminated property is Effects-planner, but
  the test itself is a Class-4 end-to-end control** — it compiles a whole
  process via `recursive_port_process_compiles`, crossing planning through
  execution, over the shared `governed_nested_resource_bracket` fixture.
  `AC-2`'s four-way partition states Class 4 "legitimately REMAINS in the
  residual integration module." **Verdict: found, not a mover.**
- 11 hits (`18135`-`18261`, two tests:
  `an_incomplete_duplicate_discarded_or_misobserved_visit_rejects` and
  `a_discarded_visit_refuses_before_its_body_is_defined`) use
  `set_effect_seat_visit_mutation`/`EffectSeatVisitMutation`, both
  **lowering/emitter**-owned symbols (`lowering/mod.rs`, item 16's
  territory). **Verdict: not this slice's domain at all.**

**Net: control.rs carries one true positive for the Effects-planner domain by
symbol, and that one test is architecturally excluded from `D2` by the
frame's own Class-4 rule. This slice's `D2` moves zero tests from either
`static_transition.rs` or `control.rs`.** State this now so `D2`'s kickoff
does not have to re-derive it, and so it is not mistaken for an unexamined
gap.

### The three evidence seats

- **Intention producer**: `build_host_effect_seat_plan` — mints the planned
  seat population from `plan.source_occurrences` and
  `host_effect_seat_contract`.
- **Independent artifact observer / evidence decoder**: two, at different
  points in the lifecycle — `validate_host_effect_seat_plan` (the planner's
  own closed-form rebuild-equality plus uniqueness check, at plan-build and
  at `StaticTransitionPlan::validate`) and `host_effect_seat_contract_of`
  (the emitter's INDEPENDENT recomputation of the same contract from nothing
  but `(operation, slot)`, consulted at `lowering/mod.rs:13162` during the
  ledger's own close — its own doc comment names this: *"Without it `need`
  would be diagnostic text... erasing it would change no decision."*).
- **Closeout/publication seat**: `EffectSeatGroupId::close`/
  `EffectSeatLedger::close` → `EffectSeatClosure`, in `lowering/mod.rs` —
  item 16's territory, outside this slice, but the seat this domain's
  evidence is produced FOR. Named here so a later slice cannot silently
  collapse producer and closer into one.

### Blind spots inherited

- The Stage A type-ownership selector cannot see private types (none of this
  domain's *types* are private, but every retained free fn/method that IS
  private was found only by hand-reading the file, not by the selector).
- **`backend-split-census-lifecycles.md` has no row for the planner-side seat
  lifecycle** — its only Effects row (line 55) is the emitter's
  mint/claim/close cycle. The planner-side mint
  (`build_host_effect_seat_plan`) / validate-close
  (`validate_host_effect_seat_plan`) cycle is undocumented there; this
  ledger's "three evidence seats" section above is the closure for that gap.
- The reexports census (`backend-split-census-reexports.md:210`) already
  names the exact Effects-domain re-export group at `planning.rs:71-73`
  (`host_effect_seat_contract_of, EffectSeatNeed, EffectSeatOperation,
  EffectSeatPhase, EffectSeatSlot, PlannedEffectSeat,
  CRANELIFT_HOST_EFFECT_CONSUMERS_V1`) plus the `#[cfg(test)]` group at
  `:381` (`set_effect_seat_plan_mutation, EffectSeatPlanMutation`) — both
  re-verified against the live tree above and unchanged.
- Macro-produced items, traits, and split-line declarations: none exist in
  this domain's population (confirmed by direct reading, not by a selector
  that would miss them).

### Anticipated child size

Summing the closed line ranges above (types 1752-1972, the const/statics
block 1975-2009, the free-function block 2011-2356, and the method block
4114-4197) is approximately 670 raw source lines before module boilerplate
(header doc comment, `use` block). Compare item 7's `aggregates.rs`, whose
much larger population (16 types + 16 fns + 15 methods + 4 impls) landed at
1929 lines pre-`D2`. This domain's population (7 types + 7 fns + 5 methods +
2 impls + 2 consts/statics, roughly a third of item 7's count) is expected to
land in the **1000-1300 line range**, well under the 10k ceiling. `AC-4b`
applies and is not a blocking concern at this scale; the exact count is
`D1`'s to report.

## D2: no movable tests

Re-verified by name at `34c0ef97a` (`D1` merged) — not copied forward from
`D0`'s numbers at `0f96b5b99`. `D1` moved production code and the `control.rs`
structural census only; it never touched a test body, so this re-scan is a
genuine independent measurement, not an inherited assumption.

**`static_transition.rs`'s own `mod tests` (now starting at line 5158, shifted
by `D1`'s move): still zero tests with Effects as their primary discriminated
property.** A fresh by-name scan for every Effects-domain symbol
(`PlannedEffectSeat`, `EffectSeat*`, `host_effect_*`,
`CRANELIFT_HOST_EFFECT_CONSUMERS_V1`) inside the test module returns the same
3 hits `D0` found, at the same relative offsets — `ken_host::HostOpV1::
BufferAllocate`/`::BufferFreeze` used as fixture content inside
`d2h_ac2_the_three_expressible_refusals_mint_nothing` and
`substrate_case_emission_open_ingress_prunes_nothing`, both
case-emission/substrate-domain tests that use an `Effect`-shaped occurrence as
example data. Neither asserts anything about seat derivation, contract
lookup, or seat-plan validation. **Nothing to move here; unchanged from `D0`.**

**`control.rs` (32,851 lines at `34c0ef97a`): the same three-way finding
holds, re-verified fresh.** A full `Effect` keyword census now returns 32
hits (up from `D0`'s 27) — the +5 delta is fully attributable to `D1`'s own
three census-comment additions naming the emitter-owned family
(`EffectSeatGroupId`/`EffectSeatLedger`/`EffectSeatClosure`/
`EffectSeatVisitMutation`/`EffectSeatDispatchMutation`, at
`control.rs:7776-7777` and `:8365-8367`), not to any new or changed test.
Closing the same three buckets `D0` established:

- **`erasing_a_seat_key_axis_or_collapsing_the_contract_rejects`**
  (`control.rs:18097`, byte-identical to `D0`'s reading) is the one true
  positive by symbol for this slice's own domain
  (`EffectSeatPlanMutation`/`set_effect_seat_plan_mutation`, which exercise
  `build_host_effect_seat_plan`'s rebuild-equality validation — now living in
  `effects.rs` after `D1`, reached transparently through the unchanged
  `planning.rs`/`static_transition.rs` re-export chain the test's own `use
  crate::cranelift_backend::planning::{...}` already goes through).
  **Why it stays, not just that it does:** the test compiles a WHOLE process
  via `recursive_port_process_compiles`, crossing planning through execution
  over the shared `governed_nested_resource_bracket` fixture — it is a
  Class-4 end-to-end control in `AC-2`'s four-way partition (domain tests,
  shared fixtures, mutation controls at their production injection point, and
  end-to-end controls crossing planning through execution), and Class 4
  "legitimately REMAINS in the residual integration module" by the frame's
  own words. Moving it into `effects.rs`'s own test module would sever it
  from the very thing that makes it a *behavioral* control rather than a unit
  check on `build_host_effect_seat_plan` alone: its assertions read the
  compiled process's refusal message, not `effects.rs`'s internal state
  directly, and its shared fixture is deliberately resident at the LCA of the
  planning and emission controls rather than owned by either.
- **`an_incomplete_duplicate_discarded_or_misobserved_visit_rejects`**
  (`control.rs:18166`) and **`a_discarded_visit_refuses_before_its_body_is_
  defined`** (`control.rs:18237`) both use
  `EffectSeatVisitMutation`/`set_effect_seat_visit_mutation` —
  **lowering-owned symbols** (`lowering/mod.rs`, item 16's emitter-side
  territory), confirmed unchanged from `D0`. Not this slice's domain.

**Conclusion: `D2` moves zero tests from either `static_transition.rs` or
`control.rs`.** This is the recorded closure of the finding `D0` stated and
the Architect's `D1` vote asked to have written down rather than left to read
as an unexamined gap. No code changes; this docs-only commit is item 8's
final deliverable.

