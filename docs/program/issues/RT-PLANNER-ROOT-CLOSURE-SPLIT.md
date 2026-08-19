---
id: RT-PLANNER-ROOT-CLOSURE-SPLIT
title: "Remeasure the planner residue after the six domain moves and close static_transition.rs -- a fresh node, not a renamed item 3, and if the parent is already under 10k it records that and extracts nothing"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-JOINS-TRAPS-SPLIT]
blocks: [RT-LOWERING-FUNCTION-STATE-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 9b; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.

Caveat specific to this node: if the remeasure finds the residue needs a
representation change — nested storage or a new accessor boundary — that is not
part of this move. The parent frame requires it be framed explicitly as its own
node with old-to-new storage/accessor/visibility ledgers, and that design work is
T1. This estimate covers the remeasure-and-relocate path and the pure "record it
is under 10k and extract nothing" outcome; a surfaced representation change is a
separate T1 node.


> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 9b of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/planning/static_transition.rs`.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**The residue.** Whatever remains in `static_transition.rs` after items 4-9.

> # THIS IS A FRESH NODE, NOT A RENAMED ITEM 3. Do not reuse its thread.
>
> **Architect `evt_6r403ez3m2m69`, stated explicitly.**
> [[RT-PLANNER-GRAPH-FOUNDATION-SPLIT]]
> is **CLOSED** — it ran its `D0`, the subtraction proof came back empty, and it
> gates nothing. **That is the node working, not failing.** This node asks a
> different question against a different tree.

## THE THREE OUTCOMES, and the first is the most likely

1. **The parent is already under 10k.** ⇒ **Record that and do no speculative
   extraction.** This is a complete, mergeable result, and there is
   nothing to move. **It satisfies the constraint FOR THIS BOUND FILE only** —
   the operator's constraint is phase-wide and is discharged at item 18, never
   here.
2. **A cohesive graph construction/validation lifecycle is now visible.** ⇒ Move
   **that whole lifecycle**, not a vocabulary drawer.
3. **The residue needs nested storage or a new accessor boundary.** ⇒ **Frame
   that representation change explicitly, before moving anything**, with
   old-to-new storage, constructor/writer, accessor, derive/layout, visibility
   and cfg ledgers. A representation change is not a move and does not get a
   move's review.

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

> # THIS NODE DOES NOT CARRY THE UNIFORM `D1`/`D2` TEMPLATE
>
> **Architect `evt_14x1bqgrj4yze`.** An unconditional "move the owner into a
> child module" deliverable contradicts this node's own semantics — it may
> correctly move **nothing**. **`D0` selects exactly one of three outcomes**, and
> what follows from it depends on which:
>
> | `D0` outcome | what follows |
> |---|---|
> | **1. parent already below 10k** | **The measured no-move record IS the complete accepted result.** `D1` and `D2` are **not applicable** and are not written. |
> | **2. a cohesive graph lifecycle exists** | `D1` moves **that whole lifecycle**; `D2` moves **only its companion tests**. |
> | **3. a representation or accessor change is required** | **Record the hard stop and frame that semantic change separately. Perform NO "relocation" under this node.** |
>
> **Outcome 1 is the most likely and it is a success.** Do not treat an empty
> move as a failure to find work, and do not extract speculatively to produce a
> deliverable — that is the size-driven extraction this phase bans.
>
> **Outcome 3 requires the full ledger set BEFORE anything moves:** old-to-new
> storage, constructor/writer, accessor, derive/layout, visibility and cfg. A
> representation change is not a move and does not get a move's review.

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

## D0 ledger, re-measured at 460141f5f

**SHA measured:** `460141f5f`. `git rev-parse HEAD` confirmed at pickup.
**File measured:** `cranelift_backend/planning/static_transition.rs`, **11630
lines** total -- over the 10k ceiling. **Outcome 1 (already under 10k) is OFF**,
matching the frame's own statement of that fact at item 9's merge.

**Region split** (the boundary that shapes this whole ledger): `mod tests {`
opens at line **4465** and runs to EOF. So the file is **4464 lines of
production/root code** followed by **7166 lines of `mod tests`** -- the test
region is now 1.6x the production region.

### THE DETERMINATION: OUTCOME 2, with a required internal sub-split

**A cohesive graph construction/validation/closure lifecycle is visible in the
residue**, and it is not a vocabulary drawer -- it is exactly the frozen stage
predicate's own words, applied to the two types the predicate is *about*:

> "The planner owns plan identities, minting, relation and seat construction,
> validation and closure, and read-only projections."

That sentence has two verbs-worth of a lifecycle, both still resident here:
- **"minting, relation and seat construction"** -- `Planner<'src>`'s own impl:
  the raw graph-construction machine (node/edge/store allocation, the
  recursive `plan_expr` descent, declaration-call-target resolution).
- **"validation and closure, and read-only projections"** -- most of
  `StaticTransitionPlan<'src>`'s own impl: `validate`, `validate_source_
  return_topology`, `census`, `semantic_census`, and the case-emission /
  case-producer / static-worker-member / substrate-preallocation validators
  that back `validate`.

**Why not outcome 1:** stated above -- 11630 > 10000.

**Why not outcome 3 (a representation change):** every cross-sibling consumer
of a residue symbol already reaches it through the *identical* visibility
mechanism items 4-9 used throughout -- `pub(in
crate::cranelift_backend::planning::static_transition)` for sibling reach,
`pub(super)` for grandparent reach, and the "many `impl` blocks for one type
across many files" pattern (item 4's `units.rs` precedent, already extended
to `Planner` itself in item 9's `intern_trap`). Moving `Planner`'s and
`StaticTransitionPlan`'s impls into new child modules changes **which file**
grants that visibility, not **whether** it can be granted without new nested
storage or a new accessor. No field needs to move behind a new accessor
boundary; no storage needs to be renested. **This is an ordinary move**, not a
representation change.

**But a single combined child would itself violate the ceiling this campaign
exists to enforce**, so the move is not a flat "one new file" the way items
4-8 were. Rough accounting: `Planner`'s impl (974 production lines) +
`StaticTransitionPlan`'s validate/closure-side methods (roughly 1000 of its
1557 impl lines, see the method-level split below) + their owned types and
free functions (roughly 700-900 more production lines) is already
**~3000-3200 production lines** before a single companion test moves: 60
`#[test]` fns plus roughly 79 non-test helper/fixture fns spanning most of
the 7166-line `mod tests` region would push either a combined child, or even
one badly-drawn half of a two-way split, well past 10k on its own. **This
requires the item-6 sub-split precedent (`continuations.rs` +
`continuations/fusion.rs`) applied here**, not a flat move.

**Proposed boundary — split along the predicate's own two clauses, not by
line count:**

- **`construction.rs`** — "minting, relation and seat construction." Owns:
  `Planner<'src>` (struct + its full 974-line impl, lines 1576-2549:
  `new`, `source`, `push_node`, `control_node`, `expression_node`,
  `expression_seed`, `connect_declaration_calls`, `declaration_call_target`,
  `edge`, `store`, `frame`, `plan_sequence`, `plan_cases`,
  `register_static_body`, `plan_expr`); the `StaticTransitionPlan` methods
  that are Planner's own registration write-path, confirmed by call site
  (not by guess) -- `register_scheduling_entry` (called from `plan_static_
  transition_graph_with_symbols` at line 4250/4258, and self-called from
  `record_planned_entry_body`), `record_planned_entry_body` (called from
  `Planner::plan_expr` at line 2080), `planned_entry_body` (the paired read
  accessor); `runtime_expr_tag` (called only from `plan_expr`, line 2095);
  `D4DeclarationTargetMutation` and its thread_local (the
  `connect_declaration_calls` mutation cell).
- **`closure.rs`** — "validation and closure, and read-only projections."
  Owns: the rest of `StaticTransitionPlan`'s big impl -- `helper_key_for_
  activation` (called from inside `validate` at line 3693, not from
  construction), `validate` (3547-3796), `validate_source_return_topology`,
  `activation_successor`, `require_only_outgoing_edge`, `require_only_
  incoming_edge`, `census`, `semantic_census`, plus the second, small `impl
  StaticTransitionPlan` block at 2551-2588 (`process_parameter_slot`, a
  read-only ABI projection); the case-producer-authority / case-emission
  family in full -- confirmed by internal call site, not by external-consumer
  count alone: `derive_case_producer_fact` and `build_case_emission_plan` are
  both called from inside `Planner::finish` (line 2453) and from `validate`
  (line 3778/2666), so despite `aggregates.rs`/`continuations.rs` also
  calling `derive_case_producer_fact` externally, its primary role is
  closure-lifecycle-internal -- `CaseProducerSet`, `CaseProducerFlowKind`,
  `CaseProducerFlowEdge`, `CaseProducerAuthority`, `CaseProducerFact`+impl,
  `CaseEmissionStatus`, `PlannedCaseEmission`, `validate_case_emission_plan`,
  `D4bVerdict`, `d4b_arm_admission`/`d4b_take_admission`; the static-worker-
  member mutation family (`StaticWorkerMemberMutation`, `with_static_worker_
  member_mutation`, `apply_static_worker_member_mutation`, `validate_static_
  worker_member_population` -- called from `finish` at 2545 and `validate` at
  3562); `validate_substrate_preallocation_closure` (called from `finish` at
  2457 and `validate` at 3782); the boundary/scale census types
  (`BoundaryACensus`, `ScaleBPlanCensus`, `BoundaryB1Census`) that `census`/
  `semantic_census` return; `PlannedResultFieldKindForTest` (paired with the
  `#[cfg(test)]` `planned_result_field_kinds_for_test` method).
- **EXPLICITLY RETAINED at the root (`static_transition.rs` itself), neither
  child** -- confirmed by a zero- or symmetric-internal-caller check, not by
  the earlier census's external-consumer count alone:
  - `plan_static_transition_graph` / `plan_static_transition_graph_with_
    symbols` (4210-4294) -- the two top-level entry points. This is the root
    orchestration function class the frame anticipates staying: it sequences
    `Planner::new` -> the construction child -> `.finish()` -> the closure
    child, and owns none of either child's internals. After this move
    `static_transition.rs` becomes a thin orchestrator over **nine**
    children (the 8 existing plus these two new ones), the same shape it
    already has one level up (`planning.rs` orchestrating
    `static_transition.rs`).
  - `planner_error` / `planner_capacity_error` (804, 808) -- universal error
    constructors with an internal caller count in the hundreds across *every
    one of the 8 sibling domains*; moving them into either new child would
    just relocate the same wide fan-out problem, not solve it.
  - `PlannedReferentLifetime` (the enum) and its sole constructor
    `runtime_value_lifetime` (1155) -- **zero internal caller** in
    `static_transition.rs` itself (checked: neither `Planner`'s nor
    `StaticTransitionPlan`'s methods call it); its only call sites are
    `occurrences.rs` (the constructor) and `aggregates.rs`/`continuations.rs`/
    `lowering/mod.rs`/the grandparent `planning.rs` (the type, read across
    the planner/emitter boundary itself). This is cross-cutting shared
    vocabulary, not lifecycle-owned.
  - `dense_slice` (521) -- zero internal caller; consumed only by
    `continuations.rs` and `continuations/fusion.rs`.
  - `synthesized_seat_emission_owners` (1216) -- zero internal caller;
    consumed only by `aggregates.rs`.
  - `AC4_RESOLUTIONS` / `AC4_ROUTE_INVOCATIONS` thread_locals plus `ac4_open_
    route_window` / `ac4_note_route_invocation` / `ac4_route_counts`
    (2614-2644) -- written from `occurrences.rs`'s `source_occurrence`
    (confirmed: that function is not defined in this file at all), read by
    the grandparent `planning.rs` and `lowering/core.rs`. Cross-cutting
    instrumentation that happens to be textually declared here; not
    construction- or closure-owned.
  - `governed_nested_resource_bracket` (4325) and the shared `trap()` fixture
    inside `mod tests` -- already-established shared cross-domain fixtures
    per item 8's own D0 negative-finding pattern; same treatment here.
  - `D2jCause` and its 3 external consumers (`planning.rs`,
    `lowering/core/tests/control.rs`, `continuations/fusion.rs`), and the
    `contspec_nested_fixture` re-export -- pre-existing shared fixtures,
    re-exported out of `mod tests` at lines 4455/4460; untouched by this
    node regardless of outcome.
  - `MAX_HELPERS_PER_STATIC_SOURCE` (139) -- used only inside `validate`
    (closure-side) and inside `mod tests`; **this one should actually travel
    with `closure.rs`**, not stay at root -- flagging the correction here
    rather than silently fixing the classification above, since D0's job is
    to expose exactly this kind of reclassification for D1 to execute
    against, not to have gotten every item right on the first pass.

**Graph-primitive types** (`StaticNodeId`, `StaticEdgeId`, `StaticSourceId`,
`PersistentNodeId`, `TransitionKind`, `EdgeKind`, `StoreKind`,
`PlannedHelperKey`, `DynamicActivationFrame`, `PersistentStoreNode`,
`StaticNode`, `StaticEdge`, `EdgeEvidence`, `PlanContext`, `PlannedExpr`,
`PlannedEntryBody`) are minted by `Planner` and read by `StaticTransitionPlan`
alike -- **their exact per-child assignment is D1's job**, resolved the same
way every ambiguous item above was resolved (by which side's methods
primarily construct vs. primarily read them), not asserted here.

### AC-1 -- closed math, by item class

| class | production region | `mod tests` region | notes |
|---|---|---|---|
| `mod` declarations | 8 (existing children) + 0 new yet | `mod tests` (1) | D1 adds 2: `mod construction;`, `mod closure;` |
| re-exports (`use`/`pub(...) use`) | ~30 lines (18-137), all import FROM the 8 children back into root scope | 2 re-exports OUT of `mod tests` (`D2jCause` family, `contspec_nested_fixture`) | closed by direct read of the import block, not sampled |
| types (struct/enum) | **32** declared directly in this file (2 are function-local to `governed_nested_resource_bracket` and excluded as non-items) | **5** module-level (2 more are function-local `Restore` RAII guards, excluded) | see full name/line list folded into the boundary proposal above; `StaticTransitionPlan` (529-682, 33 fields) and `Planner` (796-802, 5 fields) are the two field-holding structs the file exists to define, confirmed **still declared here**, not previously moved |
| `impl` blocks on file-local types | 6 blocks total: `RecursiveLoweringFrameGuard`+`Drop` (2), `PlannedHelperKey` (1), `CaseProducerFact` (1), `Planner<'src>` (1, 974 lines), `StaticTransitionPlan<'_>`/`StaticTransitionPlan<'src>` (2 blocks, 38 + 1557 lines) | 0 (test fixtures are free fns, not impls, in this region) | the `Planner` + `StaticTransitionPlan` impls together are 2569 of the file's 4464 production lines (58%) |
| free functions | ~25 production-region free fns enumerated by name/line/consumer in the boundary proposal above | ~79 non-`#[test]` helper/fixture fns (counted, not individually named here -- see blind spots) | |
| consts/statics/`thread_local!` | 1 `const` (`MAX_HELPERS_PER_STATIC_SOURCE`), 6 `thread_local!` blocks, **all** `#[cfg(test)]`-gated | (thread_locals live in production region even though cfg-test; none new in `mod tests` itself) | |
| traits | **0**, whole file | | |
| macros (`macro_rules!`) | **0**, whole file | | |
| `include_str!` / source-text oracles | 0 | **2** (lines 10096, 10147), both `include_str!("static_transition/abi.rs")`, feeding `b2r_ac6_*`/`b2r_ac7_*` -- negative source-text controls on the already-moved `abi.rs`, not on the residue | |
| `#[test]` fns | -- | **60**, exact names/lines extracted by a `#[test]` -> next-`fn` scan (not sampled); family breakdown below | |
| visibility distribution (production region) | `pub(in crate::cranelift_backend)`: ~14 items (the cross-into-`lowering` surface). `pub(super)`: 1 (`MAX_HELPERS_PER_STATIC_SOURCE`). bare-private: the large majority, including `Planner` itself and most of `StaticTransitionPlan`'s methods | | |

**Test family -> lifecycle-side mapping** (closes AC-2's discovery
requirement's prerequisite -- which side each family belongs to; the exact
per-test move is D1/D2's job, not asserted line-by-line here):

| family | count | production injection point | side |
|---|---|---|---|
| `boundary_*` (b1/b1r/c1/a) | 17 | `census`/`semantic_census`/`validate` | closure |
| `b2o_*` | 6 | `semantic_census().function_units` | closure |
| `d2h_*` | 4 | `PlannedHelperKey`/`helper_key_for_activation` | closure |
| `d7_1c_*` | 3 | `StaticWorkerMemberMutation`/`validate_static_worker_member_population` | closure |
| `substrate_*` | 3 | `build_case_emission_plan`/`validate_substrate_preallocation_closure` | closure |
| `b2r_*` | 2 | `include_str!` oracle on `abi.rs` | closure (source-text control on the validated plan's own inertness claim, not construction) |
| `d2a_*`/`d2_*`/`d4_*` (excluding `d4b_*`) | mixed within the 8-count family reported earlier | `connect_declaration_calls`/`declaration_call_target` (`Planner`) | construction |
| `d4b_*` | mixed within the same 8-count family | `D4bVerdict`/case-emission admission | closure |
| 17 singly-named tests | 17 | mixed -- some exercise `Planner::plan_expr`/`connect_declaration_calls` directly (construction), others `validate`/`validate_source_return_topology` (closure); the one Class-4 fixture (`ac3_emit`, line 8557, driving real lowering end-to-end through 3 of these 17) legitimately stays wherever its 3 consuming tests land, per the frame's own Class-4 exception |

**Closed math total:** 17+6+4+3+3+2 = 35 tests confidently closure-side by
direct production injection point; the remaining 25 (the `d2a`/`d2`/`d4`/
`d4b`-family 8 plus the 17 singly-named) split construction/closure by
per-test read at D1/D2 -- **not claimed closed here**, honestly bounded
instead: both sides get a substantial, non-trivial companion population
either way (nothing close to a 60-0 split), which is itself evidence this is
a real two-lifecycle boundary and not a line-count-driven cut.

**Blind spots (AC-1's own requirement):**
- The ~79 non-`#[test]` fixture/helper fns inside `mod tests` are counted,
  not individually named here; D1/D2 pickup needs a fresh per-fn pass to
  assign each to construction/closure/shared, same as every prior item's D0
  named its population and D1 closed the last mile.
- No `#[doc(hidden)]` items and no `macro_rules!`-produced items were found
  (0 macros, 0 hits), so this blind class is empty for this file, not
  unchecked.
- The two long single-line `pub(in ...) use continuations::{...}` re-export
  blocks (lines 74, 79) pack many names onto one physical line; a future
  selector assuming one identifier per line would miss members of those
  blocks. Flagging for D1, not re-deriving the full member list here since
  those two lines are untouched by this move (they import FROM
  `continuations`, already landed, not part of the residue being split).
- Whole-workspace (not just `cranelift_backend`) consumer greps were not run
  for every retained free function -- crate-privacy already bounds every
  `pub(in crate::cranelift_backend)` item to inside this crate's module tree,
  so a `cranelift_backend`-scoped grep is sufficient for correctness, but is
  narrower in principle than a workspace-wide sweep.

### AC-3 note (transport manifest) -- deferred to D1

D0 is docs-only; no item has moved. The construction/closure boundary and
per-item classification above is the manifest's *plan*, not its execution --
AC-3's actual old-path/new-path/attribute-preservation table is produced at
D1, against whichever child each item lands in.

### AC-4b -- anticipated child sizes

Rough production-line accounting from the boundary proposal (before any
companion tests move): `construction.rs` ≈ 974 (Planner's impl) + ~150
(registration-path `StaticTransitionPlan` methods + `runtime_expr_tag` +
`D4DeclarationTargetMutation`) ≈ **1100-1200 production lines**.
`closure.rs` ≈ 1557-150 (the non-registration remainder of the big
`StaticTransitionPlan` impl) + 38 (`process_parameter_slot`) + ~500
(case-producer/case-emission/static-worker-member/substrate-preallocation
families' types+fns) + `MAX_HELPERS_PER_STATIC_SOURCE` ≈ **1900-2000
production lines**. Both comfortably clear of 10k on production code alone.

**The real ceiling risk is the companion test population**, not the
production split: even an even test split (~30/30 of the 60 named `#[test]`
fns, plus roughly half of the ~79 helper fns each) would land each child in
the low-to-mid thousands of test lines on top of its production lines --
**both children should land safely under 10k**, but this is D1/D2's number
to confirm by direct measurement once the exact per-test assignment is made,
not asserted here as a proof.

If D1's actual measurement shows either child approaching the ceiling, the
frame's own item-6 precedent (`continuations.rs` + `continuations/fusion.rs`)
is already the sanctioned escape hatch -- a further sub-split within
whichever child needs it, not a re-opening of this D0's boundary.

### AC-5 -- adapter/facade debt

None anticipated. Every retained-at-root symbol above is retained because it
is genuinely shared (multiple sibling consumers, or the top-level entry
point itself), not because a temporary facade was introduced to make a move
compile. No scaffolding ledger entry is owed by this D0.

### AC-6 -- this transfer's own completeness, phase closure NOT claimed

This D0 is complete for its own question: the residue's outcome is
determined (**outcome 2**, with a required internal sub-split), and the
construction/closure boundary is proposed with named populations closed
per item class above. **This does not close item 9b**, which needs its own
`D1`/`D2` execution and QA/Architect gate, and it explicitly **does not
claim phase closure** for `RT-BACKEND-MODULE-SPLIT` -- that remains item 18's
job, per this frame's own `AC-6`.

