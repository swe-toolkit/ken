---
id: RT-PLANNER-JOINS-TRAPS-SPLIT
title: "Move the joins and traps domain out of planning/static_transition.rs -- the last named planner domain, and generated traps receive no fabricated source origin"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-EFFECTS-SPLIT]
blocks: [RT-PLANNER-ROOT-CLOSURE-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 9; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
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

**Cut item 9 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Joins and traps.** Join disposition, planned trap seats, trap provenance
events, and their planner-side lifecycle.

> ### GENERATED TRAPS RECEIVE NO FABRICATED SOURCE ORIGIN
>
> **Standing amendment, and this is the slice where the temptation appears.**
> A trap that has no source origin must keep not having one. **Do not mint an
> origin to make a moved type's invariant look uniform** — that is a
> representation change wearing a relocation's clothes, and it is banned by the
> phase gates above.

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
this slice just established** (join-disposition, planned-trap-seat and
trap-provenance controls). Place multi-leaf fixtures **once**,
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

## D0 ledger, re-measured at `de402e255`

Bound file: `cranelift_backend/planning/static_transition.rs`, **12,726 lines**
at this SHA. All line numbers below are re-measured at `de402e255` and are not
inherited from any prior slice's frame or census row.

### THE BEHAVIORAL WATCH-ITEM — result FIRST, since it gates everything else

**No fork. Traps in this domain are identified purely by VALUE, never by
source origin.** `PlannedTrapIdentity(u32)` is a dedup index into
`trap_catalog: Vec<RuntimeTrap>`; `intern_trap`/`trap_identity` both take
`trap: &RuntimeTrap` and return `PlannedTrapIdentity` — no `StaticOriginId`
or any origin-bearing parameter appears anywhere in this domain's signatures,
today or after the move. `intern_trap` is called from `expression_seed` (a
`Planner` method that stays at root, handles every occurrence kind) at a
point where an origin IS in scope (the enclosing node), but `intern_trap`
itself never receives or needs it — confirmed by reading its full body
(`static_transition.rs:2243-2261`): it dedups on `RuntimeTrap` equality
alone. **No function in the moved population would need to mint an origin to
compile**, so the hard-stop this watch-item exists to catch does not apply.

### THE FROZEN STAGE PREDICATE, applied — a boundary the frame's own prose
### does not settle, so `D0` settles it

The frame's `THE OWNER` names *"planned trap seats, trap provenance events"*
alongside join disposition. **Neither exists in the bound file.**
`Px8trTrapProvenanceEvent` and `PlannedTrapSeat` are both declared and fully
implemented in `lowering/mod.rs` (`pub(crate) enum Px8trTrapProvenanceEvent`
at `lowering/mod.rs:515`; `pub(crate) enum PlannedTrapSeat` at `:605`; the
recorder `px8tr_record_trap_provenance` at `:622`) — **zero presence in
`static_transition.rs`**, confirmed by a full-file grep. This is the
emitter's half of the pair (item 14's, per the frozen predicate: "the
emitter owns concrete CLIF/backend mutation... assigned EXACTLY ONCE"), and
it already lives at its correct final home — there is nothing here for
`D1` to claim or move. **This slice's actual planner-owned population is
narrower than the frame's own prose suggests: join disposition (which
representation a source join's result takes) and trap IDENTITY (a
value-keyed dedup catalog) — not trap seats and not trap provenance.**

### Symbol ledger — closed over every Rust item class

**Types (6):**

| type | lines | kind | fields/variants | vis |
|---|---|---|---|---|
| `JoinResultRepresentation` | 449-452 | enum | `NativeScalarPair`, `CarrierWord` | `pub(in crate::cranelift_backend)` |
| `JoinPlanToken` | 460-464 | struct | `origin`, `representation`, `has_continuing_predecessor` (all `pub(in crate::cranelift_backend)`) | `pub(in crate::cranelift_backend)` |
| `PlannedJoinResult` | 467-470 | struct | `representation`, `has_continuing_predecessor` | private |
| `PlannedTrapIdentity` | 551 | tuple struct `(u32)`, `#[repr(transparent)]` | 1 field, private | `pub(in crate::cranelift_backend)` |
| `ResultPhase` | 843-847 | enum | `SpecializedOnly`, `CarrierRequired` | private |
| `ResultPhaseSummary` | 855-866 | struct | `phase`, `continues`, `callable_result` | private |

**Impls (2):**

- `impl PlannedTrapIdentity` (553-557): 1 method (`abi_word`, `pub(in
  crate::cranelift_backend)`).
- `impl ResultPhaseSummary` (868-914): 2 assoc consts (`TRAP`, `SPECIALIZED`,
  private) + 4 methods (`carrier`, `callable`, `join`, `sequence`, all
  private).

**Free functions (5):**

| fn | lines | vis | consumed at |
|---|---|---|---|
| `is_source_join` | 916-925 | private | same block + `source_join_origins_in_owner_subtree`/`validate_join_result_plan` (both move) + the `D2` test at `:12060` (currently resident in this file's own `mod tests`) |
| `planned_partiality_trap` | 927-950 | `pub(in crate::cranelift_backend)` | `lowering/core/primitive.rs:75,83` (cross-boundary) |
| `summarize_result_phase` | 955-1229 | private | only within this block |
| `result_phase_environment_for_owner` | 1231-1281 | private | only within this block |
| `build_join_result_plan` | 1283-1315 | private | root builder (`static_transition.rs:3048`, stays put) |

**Thread-locals (2, `#[cfg(test)]`, 850-852):** `D8_FORCE_VARIABLE_SPECIALIZED`,
`D8_REMOVE_VARIABLE_CALLABLE_SEED` — both `Cell<bool>`, module-private. Read
only inside `summarize_result_phase`'s `Var` arm (`:1170,1173`); set only by
two `D2`-population tests currently resident in this file's own `mod tests`
(`:11995,11999,12010,12014`).

**A `Planner` method (1) — the first instance of this specific shape in the
campaign:**

- `intern_trap` (2243-2261), private, inside `impl<'src> Planner<'src>`
  (the plan-construction BUILDER, a different type from
  `StaticTransitionPlan` — declared at `:2080`). Mints a `PlannedTrapIdentity`
  by deduplicating `trap_catalog`. Called from `expression_seed` (another
  `Planner` method, general-purpose node registration for every occurrence
  kind, stays at root) at `:2228,2232`. **This is not architecturally novel**
  — `Planner`, like `StaticTransitionPlan`, may hold multiple `impl` blocks
  across files, so `intern_trap` moves the same way a `StaticTransitionPlan`
  method does: a new `impl<'src> Planner<'src> { fn intern_trap(...) }`
  fragment in the child module, reading `Planner`'s ancestor-private `plan`
  field under the same standing child-module pattern. Flagged explicitly
  because it is the first time this campaign moves a `Planner`-impl fragment
  rather than a `StaticTransitionPlan`-impl fragment or a free function.

**`StaticTransitionPlan` methods (7):**

| method | lines | vis | consumed at |
|---|---|---|---|
| `join_plan_token` | 3251-3257 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:7048,7312` + `control.rs:8942,8959` (cross-boundary) |
| `join_plan_token_if_planned` | 3263-3276 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:7285` (cross-boundary) |
| `required_join_origins` | 3284-3308 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:7329,7421` (cross-boundary) |
| `source_join_origins_in_owner_subtree` | 3318-3353 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:7102,7149` (cross-boundary) |
| `trap_identity` | 3749-3766 | `pub(in crate::cranelift_backend)` | `lowering/mod.rs:19316` (cross-boundary, production) + `control.rs:9071` (test) |
| `trap_catalog` | 3768-3770 | `pub(in crate::cranelift_backend)` | `lowering/core.rs:2958`, `lowering/mod.rs` (production) + `lowering/core/tests/{effects,constructors}.rs` (cross-boundary) |
| `validate_join_result_plan` | 4446-4486 | private | root's own `StaticTransitionPlan::validate` (`:4442`, stays put) |

**Explicitly retained at the parent (not moved, owning domain named):**

- `trap_catalog: Vec<RuntimeTrap>` (field, `:633`) and `join_results:
  Vec<Option<PlannedJoinResult>>` (field, `:649`) — private fields of
  `StaticTransitionPlan` itself, which stays at the parent (same rule as
  every prior slice). Only the field element types' homes change.
- `build_join_result_plan`/`validate_join_result_plan` call sites at `:3048`
  (the plan builder's `finish`-like method) and `:4442`
  (`StaticTransitionPlan::validate`) — root orchestration that calls into
  every domain's build/validate function, exactly the item 7/8 pattern.
- `intern_trap`/`planned_partiality_trap` call sites inside `expression_seed`
  (`:2228,2231-2232`) — general node-registration machinery for every
  occurrence kind, not domain-specific.
- `RuntimeTrap`/`RuntimeTrapCode` — crate-level types (`use
  crate::{..., RuntimeTrap, RuntimeTrapCode};` at `:25`), not declared in
  this bound file at all; referenced, not owned.
- `occurrence_authority`, `StaticOriginId`, `PredeclaredFunctionId` —
  occurrence-owned and unit-owned respectively, referenced only.

### Two negative findings — surface in the keyword scan, not this domain

**`CaseProducerFact::join`** (`:1373`) is a SECOND, unrelated `.join()`
method in this file — a lattice-join over `CaseProducerSet`
(open/closed-constructor-set union), part of the already-settled
case-emission/substrate domain (`CaseProducerSet`, `CaseProducerFlowKind`,
`CaseProducerFlowEdge`, `CaseProducerAuthority`, `PlannedCaseEmission`, all
declared at `:472-523`, immediately after this domain's type block and
immediately before `PlannedTrapIdentity`). **Not mine** — `.join()` on
`ResultPhaseSummary` (`:897`) is the only join-disposition-domain method of
that name; a naming grep alone cannot tell the two apart, only reading each
site's receiver type can.

**The `trap` test-fixture helper** (`:6529`, `pub(in
crate::cranelift_backend) fn trap(message: &str) -> RuntimeTrap`, declared
inside this file's own `mod tests`) surfaces in a `trap` keyword scan and
constructs `RuntimeTrap` VALUES — but it is a **shared cross-domain test
fixture**, the exact shape of item 8's `governed_nested_resource_bracket`
finding. It is called 18 times within this file's own tests alone, spanning
fixtures for domains that have nothing to do with join disposition or trap
identity (every domain's tests need SOME trap value for a `Match`'s
`default:` arm). Zero uses in `control.rs`. It has no interaction with
`PlannedTrapIdentity`/`intern_trap`/`trap_catalog` at all. **Explicitly
retained wherever the campaign's shared-fixture convention places it — not
claimed by this slice.** (A second, textually-nested `trap()` inside
`governed_nested_resource_bracket`'s own body, at `:5050`, is a local item
scoped to that function and not independently reachable; irrelevant here for
the same reason it was irrelevant to item 8.)

**A third near-miss, resolved the same way:** `trap_terminal_id`/
`terminal_id` (`:9031-9046`, a test-only `impl StaticTransitionPlan<'_>`
fragment nested inside `mod tests`) look up nodes by `TransitionKind::
Terminal`/`TransitionKind::TrapTerminal` — a **graph-topology** concept (a
node's transition KIND in the static graph), unrelated to trap VALUE
identity. Confirmed by its sole consumer (`:8920`, an edge-rewrite/topology
validation test) asserting nothing about `PlannedTrapIdentity` or
`trap_catalog`. **Not this domain.**

### THE CARRY-FORWARD CHECK — items 4-8's retained-and-flagged items

**None belong to joins/traps.** `host_effect_operation` and
`host_effect_site_operand_slots` (item 7's two flags) were confirmed
Effects-owned and moved by item 8's `D1`. `governed_nested_resource_bracket`
(item 8's flag) is the shared cross-domain fixture already discussed above —
no domain claims it, including this one; none of this domain's own tests
(the `D8` family below) even use it, they use their own `d8_*` fixtures
instead. `synthesized_seat_emission_owners` (the false-positive item 7's own
kickoff warned against) is continuations-owned (item 6's). Checked each by
name against this domain's actual population; none reclassify.

### Source-text oracles

- **LIVE**: `lowering/core/tests/control.rs`'s
  `the_backend_production_surface_inventory_is_closed` (module-inventory
  vec), `BACKEND_PRODUCTION_SOURCES` (the `include_str!` roster), and
  `correspondence_adds_no_emitted_unit_to_the_production_census` (the
  `Census` array) will need the same 3-location addition for the new child
  module's name (items 4-8's precedent) once `D1` creates it — ledgered here
  as an anticipated non-move hunk, not executed in this docs-only `D0`.
- **INERT**: `lowering/core/tests/control.rs:12424`,
  `d8_join_helpers_have_the_closed_typed_caller_population`, is gated
  `#[cfg(any())]` — permanently disabled per its own retirement comment ("RETIRED
  by the RT-FNSPLIT-RECUR-PORT successor repair: caller-name counts over
  repository text are not a behavioral representation proof"). Its
  `helpers.matches("plan: &JoinPlanToken")` census is dead code; `D1` owes it
  nothing.

### Test-property ledger

**`static_transition.rs`'s own `mod tests`: one contiguous, genuinely
domain-scoped family, lines 11677-12077.** By-name scan for every
joins/traps symbol (`JoinResultRepresentation`, `JoinPlanToken`,
`is_source_join`, `join_plan_token`, `PlannedTrapIdentity`) confirms a clean
boundary — `:11609` ends a `D4` (declaration-call) test, `:11677` begins this
family, `:12077` ends it, `:12080` begins a `substrate_*` (case-emission)
fixture. Contents:

- 6 fixture-producer fns: `d8_mixed_join` (11677), `d8_functionized_plan`
  (11711 — a leaf helper, used only within this block, not a shared
  cross-domain fixture like `governed_nested_resource_bracket`),
  `d8_environment_join` (11723), `d8_bound_callable_join` (11776),
  `d8_abi_parameter_join` (11829), `d8_abi_parameter_join_origin` (11861).
- 2 assertion helpers: `assert_d8_environment_join_is_carrier` (11758),
  `assert_d8_bound_callable_join_is_carrier` (11811).
- 9 `#[test]` fns, all domain-scoped (none crosses into emission —
  none calls `recursive_port_process_compiles` or any lowering/JIT
  entry point): `d8_mixed_join_plan_is_carrier_and_arm_order_independent`
  (11870), `d8_let_environment_provenance_reaches_the_exact_nested_join`
  (11900), `d8_bound_lexical_callable_provenance_reaches_the_exact_nested_join`
  (11916), `d8_abi_parameter_provenance_reaches_the_exact_nested_join` (11932),
  `d8_inert_abi_slots_do_not_change_recursive_descent_join_storage` (11960),
  `d8_variable_seed_mutation_reds_at_the_plan_boundary` (11994),
  `d8_callable_seed_removal_reds_at_the_plan_boundary` (12009),
  `d8_trap_predecessors_do_not_create_a_result_edge` (12022),
  `d8_join_plan_is_a_bijection_with_source_join_occurrences` (12053).

**This is a real, non-empty population for `D2` to move** — the mirror image
of item 8's zero finding. `D2`'s own re-measurement at pickup must re-derive
these line numbers fresh, per the standing discipline; they will have
shifted once `D1` moves the production block.

**`control.rs` (32,851 lines at `de402e255`): every joins/traps-symbol hit is
either Class-4 end-to-end or emitter-owned (item 14's). Nothing moves.**
Full census by symbol:

- **6 Class-4 end-to-end controls** (all call `recursive_port_process_compiles`,
  crossing planning through execution, over the campaign's shared fixtures):
  `a_trap_arm_and_its_trap_free_twin_both_functionize` (`:8855`, asserts
  `join_plan_token(...).representation`/`.has_continuing_predecessor` directly
  but is architecturally an end-to-end compile control);
  `d8_mixed_host_result_uses_one_uniform_carrier_conversion_per_predecessor`
  (`:12321`), `d8_dynamic_host_result_merge_enters_materialized_dead_cfg_population`
  (`:12348`), `d8_all_trap_host_result_emits_no_merge_or_predecessor_conversion`
  (`:12370`), `d8_unsupported_carrier_production_publishes_no_unit_function`
  (`:12387`), `d8_every_required_join_plan_is_consumed_exactly_once`
  (`:12727`, the largest — consumes 8 supporting fixture fns at
  `:12283-12726`, all leaf helpers of this one test). Per `AC-2`'s four-way
  partition, Class 4 "legitimately REMAINS in the residual integration
  module" — moving any of these into a joins/traps-only test module would
  sever them from the emission-level assertions that make them behavioral
  controls rather than unit checks.
- **3 emitter-owned tests** (item 14's, not this slice's domain):
  `typed_trap_exit_preserves_the_planner_identity_across_two_unit_calls`
  (`:9056`), `typed_trap_exit_rejects_a_deleted_or_root_misclassified_unit_lane`
  (`:9087`), `typed_trap_exit_identity_and_caller_protocol_mutations_are_discriminating`
  (`:9113`) — all three drive `TrapFrameBindingMutation`/
  `TrapIdentityMutation`/`TrapCallerProtocolMutation`, all declared in
  `lowering/mod.rs` (confirmed by definition-site grep), and all execute via
  JIT (`run_example_with_seed_observation`). One of the three incidentally
  calls `plan.trap_identity(...)` as a pre-condition sanity check
  (`:9071`) before driving the emitter mutation — a single accessor call
  inside an otherwise emitter-domain test does not reclassify the test.

### The three evidence seats

- **Intention producer**: `build_join_result_plan` (join disposition — mints
  `join_results` from `summarize_result_phase`'s recursive walk) and
  `intern_trap` (trap identity — mints `PlannedTrapIdentity` by dedup).
- **Independent artifact observer / evidence decoder**:
  `validate_join_result_plan` (the planner's own closed-form density/
  bijection check against `is_source_join`, at both plan-build and
  `StaticTransitionPlan::validate`). Trap identity has no independent
  re-derivation step of its own in this domain — `trap_identity` re-looks-up
  by value (not a separate derivation), and `.abi_word()`'s own doc comment
  frames the ABI word as the thing consumed at the emitter's boundary, not
  independently re-proved here.
- **Closeout/publication seat**: for join disposition, the emitter's
  `join_plan_token`/`required_join_origins` consumption at end-of-function
  (`lowering/mod.rs:7048,7329` etc.) — outside this slice. For trap identity,
  the emitter's `identity.abi_word()` consumption at CLIF emission
  (`lowering/mod.rs:19316-19368`) — also outside this slice. Both closeout
  seats are item 14's (or already-landed emitter machinery), named here so a
  later slice cannot silently collapse producer and closer into one.

### Blind spots inherited

- The Stage A type-ownership selector cannot see private types — 4 of this
  domain's 6 types (`PlannedJoinResult`, `ResultPhase`, `ResultPhaseSummary`,
  and the assoc consts on `ResultPhaseSummary`) are private and were found
  only by hand-reading the file.
- Neither `backend-split-census-lifecycles.md` nor
  `backend-split-census-reexports.md` carries a row distinguishing "trap
  identity" from "trap seats/provenance" — the frame's own prose conflated
  them, and this ledger's "frozen predicate, applied" section above is the
  closure for that gap.
- Macro-produced items and traits: none exist in this domain's population
  (confirmed by direct reading).

### Anticipated child size

Summing the closed line ranges above (types+impls `449-914` minus the
case-emission interior `472-523`, the free-function block `916-1315`, the
`Planner` method `2243-2261`, and the `StaticTransitionPlan` method block
`3251-3353` + `3749-3770` + `4446-4486`) is approximately 690 raw source
lines for `D1` before module boilerplate — close to item 8's `effects.rs`
(670 raw lines, landed at 733). Adding the `D2` test population
(`11677-12077`, ~400 lines) if a later measurement combines them would still
land well under the 10k ceiling; the exact counts are `D1`'s and `D2`'s to
report.

