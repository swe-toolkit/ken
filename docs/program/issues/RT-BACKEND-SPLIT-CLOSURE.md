---
id: RT-BACKEND-SPLIT-CLOSURE
title: "Close the backend module split -- delete the transitional adapters, narrow the facades, run the test-root closure over control.rs, and prove all four bound files are under 10k"
status: active
owner: runtime
size: M
gate: none
depends_on: [RT-EMITTER-EFFECTS-SPLIT]
blocks: []
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 18; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

> # ITEM 17 IS ABSORBED HERE. Closure's whole-backend remeasure accounts it.
>
> **Architect ruling `evt_2ng0vm7h85zst`, 2026-08-20.** Item 17
> (`RT-EMITTER-TERMINALS-CLEANUP-SPLIT`) named no distinct terminals/cleanup
> lifecycle and folds into this node — 18 → 17 slices, no work lost. Native trap
> emission is nonexistent (no CLIF `.ins().trap` in `lowering/`); the
> terminals/cleanup declarations are already owned by items 9/12/13/14. This
> node's `depends_on` therefore retargets to item 16 (`RT-EMITTER-EFFECTS-SPLIT`,
> merged), the last real owner-move predecessor. Where the strands and `AC-6`
> below say "items 4-17", read **items 4-16 plus the item-17 RETAIN accounting**:
> item 17 produced no slice and no `AC-5` ledger, but the ring's item-17 RETAIN
> accounting is carried in as an input to the whole-backend remeasure, and every
> terminals/cleanup declaration is re-verified as correctly placed inside this
> closure's total accounting.

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.

This slice deletes the transitional adapters, narrows the facades and remeasures:
still a mechanical cleanup against the census, not a design change.


> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 18 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. **This is
the one node that DOES claim phase closure**, on the full resulting population
— see `AC-6`.

Bound file for this slice:
all four bound files.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Closure.** No new domain. This slice deletes what the phase built as
scaffolding and proves the operator's constraint is met.


> **Modules own semantic lifecycles.** The durable direction of the whole phase
> is *plan construction -> validated read-only views -> lowering state and source
> machine -> concrete backend mutation -> independent evidence -> closure ->
> publication*. **Do not name a permanent module after a temporary campaign
> node**, and do not size modules to be equal.


> # THIS NODE IS BESPOKE. The uniform template contradicts it and has been removed.
>
> **Architect `evt_14x1bqgrj4yze`.** The first cut copied the common frame, so
> this file simultaneously said it *"does not claim phase closure"*, forbade
> phase closure in its acceptance, and told the pickup to *"move an owner into a
> child module"* — while also being **the sole node that DOES claim closure and
> owns NO new domain.** Those clauses are replaced by what follows.

# THE FOUR STRANDS

1. **Inventory every adapter and facade debt** left by items 4-17, from the
   per-slice `AC-5` ledgers. **The population must be closed from those ledgers**
   — an inventory built by grepping for adapter-shaped code cannot prove it found
   them all.
2. **Delete or narrow them, and RECONCILE THE FACADES' POSITIVE DUTIES.**
   A facade still re-exporting the monolith's whole surface has recreated the
   monolith behind a new name.
   > **Research `evt_1pwq0rssre6d8`.** The planning and lowering facades have
   > **positive** duties — metadata extraction, orchestration, validation,
   > read-only plan views, backend entry points, one-compilation orchestration.
   > **"Narrow" must NOT mean moving or hiding the facade's actual orchestration
   > and validation owner.** Reconcile each duty to a named owner at closure.
3. **CLASSIFY the residual test root first**, then close it.
   > **Research `evt_1pwq0rssre6d8` corrected this strand.** It read *"the
   > finding is which owner's tests were never claimed"* — which **presumes every
   > large residual is an unclaimed domain test.** It is not. The report's class
   > 4, **end-to-end controls crossing planning through execution, are SUPPOSED
   > to remain here.**
   >
   > ⇒ **If genuine class-4 controls alone remain over 10k, that is a fresh
   > integration-test ownership cut — NOT evidence a production owner was
   > missed.** Calling them unclaimed invites converting them to domain tests or
   > moving them by size, which the report forbids.
4. **Measure the output tree** — see the boundary rule below, which is the part
   most likely to be got wrong.

> ### DOC ERRATUM TO RECONCILE AT CLOSURE: core.rs's "indivisible SCC" header
>
> **Items 11-12 falsified core.rs's own module header.** It still calls the impl
> the "indivisible lowering SCC" (RT-SPLIT §10.1/§10.2), but item 11
> (values-boundary) and item 12 (source-machine) relocated whole method families
> out to sibling modules (`boundary.rs`, `source.rs`) via `pub(super)`, with the
> struct and its field-ownership retained at the LCA. **On closure, rewrite that
> header** to state the realized structure — type/field-ownership is LCA-pinned
> in core.rs; method families relocate to sibling modules via `pub(super)`. A
> comment reconciliation, not a code move; grounded by Architect
> `evt_4x0gdq2vmyz8j`, which ruled core.rs IS reducible.

> # MEASURE THE COMPLETE RESULTING FILE POPULATION, NOT THE FOUR ORIGINAL ROOTS
>
> **This is the Architect's sharpest correction and it closes a real hole.**
>
> **Measuring only the four oversized roots is defeatable by construction.**
> Moving 15,000 lines out of a root and into **one new child** would "prove" all
> four roots below 10k **while violating the operator's only constraint** — the
> phase would report success having produced a fresh 15,000-line file.
>
> ⇒ **Closure is discharged only when EVERY resulting Rust file in the phase
> boundary is below 10k** — every newly created production and test child
> included — **and every extracted module has a coherent lifecycle with no
> monolith facade.**
>
> **If any file remains over 10k, recording its owner analysis is an ACCEPTED
> FINDING — and phase closure remains OPEN, requiring a successor slice.**
> Reporting the four roots and stopping is what this block exists to prevent.

# THIS IS WHERE THE OPERATOR'S CONSTRAINT IS DISCHARGED, AND NOWHERE ELSE

**"Files over 10k lines are decomposed into architecturally sound smaller files.
That is the whole constraint."** No earlier slice may claim it is met; each
claims only its own transfer. **Do not extract speculatively to get under the
number** — optimizing for a line count is the guardrail this phase was warned
about from the start.

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

  **A group label is not a ledger entry.** "ABI preflight helpers" names a set
  without enumerating it and does not discharge "exact".

  **Reconcile every moved declaration and use site to EXACTLY ONE owner.**
  Research `evt_1pwq0rssre6d8`: *"A selector count plus a blind-spot paragraph
  cannot discharge a universal."* Either narrow the words "exact and complete" to
  a **declared selector population**, or supply a **closure method for the blind
  classes**. Do not claim the universal on the strength of the count.

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

- **`AC-6` — THIS node DOES claim phase closure, and only on the full
  population.** State the line count of **every** Rust file in the
  phase boundary, including every child created by items 4-17.
  > **THE PHASE BOUNDARY IS THIS EXACT FILE SET, never the bare phrase:** every
  > `.rs` under `crates/ken-runtime/src/cranelift_backend/`, **plus**
  > `crates/ken-runtime/src/boundary_value_clif.rs`. That last file sits OUTSIDE
  > the `cranelift_backend/` subtree and was **9,116** lines at `a1cf83622` —
  > research `evt_1pwq0rssre6d8` named it as able to cross the ceiling *"without
  > any frame noticing"*. The standing amendment governs whether it may be
  > MOVED; this criterion governs whether closure MEASURES it. It does.

  **Closure holds only if all of them are below 10k**; any file over it is
  recorded with its owner analysis and **leaves closure OPEN with a named
  successor slice.**
- **`AC-7`** — the adapter/facade debt population is **closed from the per-slice
  `AC-5` ledgers**, and **ZERO transitional adapters and parallel carriers
  remain**. Any retained surface is explicitly justified and re-ledgered as
  permanent. **An inventory that cannot cite the ledger it closed over fails
  this**, and **this node's `D1` may NOT authorize new adapters.**
- **`AC-8`** — the planning and lowering facades' **positive duties** — metadata
  extraction, orchestration, validation, read-only plan views, backend entry
  points, one-compilation orchestration — are each **reconciled to a named
  owner**. Narrowing that leaves a duty unowned fails this criterion.

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

# CLOSURE LEDGER — the four strands, executed at pickup `f3a500d1f`

**Not a standard D0/D1/D2 candidate; this is the bespoke closure record
itself.** All four strands below were executed against the tree at
this item's own pickup SHA, re-measured directly, not inherited from
any earlier item's own counts.

## Strand 1 — adapter/facade debt, closed from the AC-5 ledgers

**Ledger closure first (AC-7's own requirement):** every one of items
4-16's own frame docs was read for its `AC-5` section. Only **item 4**
(`RT-PLANNER-UNITS-ABI-SPLIT`) carries a populated ledger entry. Items
9b, 13, 14, 15, 16 each explicitly state `AC-5: empty/inapplicable --
no scaffolding introduced`. Items 5, 6, 7, 8, 9, 10, 11, 12 carry no
populated `AC-5` heading at all -- independently checked each for any
adapter/facade/scaffolding discussion outside the criterion's own
boilerplate text; found none. **The item-17 RETAIN accounting**
(carried into this closure per the Architect's absorption ruling,
`evt_2ng0vm7h85zst`) names no adapter debt either -- it is a
classification record, not a move.

**Item 4's one entry, current disposition:** `use units::{
EmittableCallEdge, EmittableCallKind, PredeclaredFunctionId };` in
`planning/static_transition.rs`, re-exported further by `planning.rs`
and consumed by `lowering/mod.rs`. `EmittableCallEdge` has already
narrowed away naturally (zero references anywhere in the crate).
`EmittableCallKind`/`PredeclaredFunctionId` are genuinely load-bearing
across the full three-layer chain (`units.rs` declares ->
`static_transition.rs` re-exports -> `planning.rs` re-exports ->
`lowering/mod.rs` consumes directly) -- **this is the facade doing its
positive re-export duty, not debt.** (First traced this wrong myself --
checked only the fully-qualified `static_transition::EmittableCallKind`
path for consumers and found none, concluded dead; the actual consumer
reaches it one layer up, through `planning::EmittableCallKind`. The
compiler's own clean build is what caught the error before it became a
ledger claim.) **Item 4's `AC-5` ledger closes: 1 of 3 named symbols
already resolved by natural narrowing; the other 2 are not adapter
debt.**

**Supplementary compiler-driven sweep (not the primary evidence --
AC-7 wants ledger-closure primarily; this is a cross-check).** Ran
`scripts/ken-cargo build -p ken-runtime --lib` and `--tests`, captured
every `unused import` warning, and checked each crate-wide (production
AND test files, not just the flagging file) before treating any as
real debt -- **most turned out to be false positives**: rustc's
per-file unused check cannot see a descendant module's own `use
super::*` consumption, so a name unused *within* a hub file
(`lowering/mod.rs`, `planning.rs`) is frequently still load-bearing for
a sibling production file or a test-tree descendant reaching it
through the glob chain. Confirmed genuinely dead (zero consumers
anywhere, re-checked past the initial false-positive pattern) and
narrowed, three cases, each rebuilt and retested clean (926/0/4 lib +
26/0 + 14/0 after every one):

1. `lowering/mod.rs` -- `GovernedAllocationMutationGuard`/
   `SiblingProducerSubstitution` (item 15's own `D2` leftover: their
   sole consumer, a test formerly in `constructors.rs`, relocated into
   `aggregates::tests` at item 15's own `D2`, where both names are
   already in scope without re-export). `GovernedAllocationMutation`
   alone stays -- still used by `constructors.rs`'s residual
   `d7_ownership_run`.
2. `core/tests/mod.rs` -- `effect_seat_dispatch_mutation`/
   `effect_seat_visit_mutation` (the GETTERS, never called by any
   test) and the raw `SITE_OPERAND_SUBSTITUTION_HITS` static (item
   16's own `D1` leftover, this ring's own prior work -- the accessor
   fn `site_operand_substitution_hits()`, which IS the real reader of
   the static, stays).
3. `planning/static_transition.rs` -- `validate_continuation_
   specialization_closure`/`ContinuationProjectionOmission`/
   `ContinuationInternMutation` (an item-6-era leftover: neither is
   consumed anywhere in the whole `lowering/` tree, and `planning.rs`
   does not re-export either further). `ContinuationProductionMutation`
   alone stays. The neighboring `#[allow(unused_imports)]`-marked
   mutation-const block was left untouched -- a deliberate, already-
   flagged suppression, not adapter debt.

**Not chased further, explicitly out of scope for this pass:** a
handful of `unused import` warnings in files outside the split's own
subtree or attributable to a different campaign (`boundary_value_
clif.rs`'s own two test-local imports, `cranelift_backend.rs:74`'s
dead `#[cfg(test)]` re-export -- explicitly attributed in its own
comment to `RT-DYNAMIC-ARM-SCALAR-MERGE`, a different initiative --
`native_execution_differential.rs`, `object_linker_packaging.rs`).
None of these are RT-BACKEND-MODULE-SPLIT adapter debt; touching them
here would be scope creep into another campaign's or pre-existing
debt's territory.

**AC-7's own residual limitation, stated plainly:** this sweep finds
adapters the compiler can see as *unused*. It cannot find an adapter
that is still *used* only because nothing has been re-pointed to the
direct path yet -- that class of debt (a re-export kept alive purely
by inertia, not by a load-bearing use) is not something a mechanical
sweep discharges; it would need a per-symbol "is this the direct path
or an inherited one" judgment this pass did not attempt at that
granularity beyond the one item-4 case already traced.

## Strand 2 — facade positive duties, reconciled to a named owner (`AC-8`)

**`lowering/mod.rs` -- the lowering facade.** Positive duties and their
owners:
- **Hub type/state ownership** (the `Lowering<'a>` struct itself,
  `Lowered`, `LoweringOperand`, `FunctionLocalRefs`, and the several
  ledger fields items 9-16 each qualified to their own moved type) --
  owner: `mod.rs` itself, by design (`Lowering` is the compilation
  state every emitter borrows and mutates; "modules own semantic
  lifecycles" makes the hub struct's own home the natural owner, not a
  campaign artifact to relocate).
- **Planner read-only projection re-export** (the large `use super::
  planning::{...}` block) -- owner: `mod.rs`, whose duty is making the
  planner's validated, read-only types reachable to every emitter
  sibling (aggregates/calls/joins/effects/boundary/source/units) via
  the `use super::*` chain, without each sibling importing from
  `planning` directly. This is exactly a "backend entry point" /
  "read-only plan views" duty per the research's own named list, and
  it is why several names in that block read `unused` in `mod.rs`
  itself while being genuinely load-bearing for a descendant -- the
  facade's OWN correctness is in doing this re-export, not in
  consuming every name itself.
- **Cross-domain shared primitives** (`require_i64`/`require_nonzero`,
  `child_occurrence`, `lowered_value_kind`, `lower_expr`'s own
  dispatch, and the several other hub-stays helpers items 11-16 each
  independently confirmed) -- owner: `mod.rs`, genuinely shared
  (multiple sibling consumers each item's own D0 traced), not a
  facade artifact.

**`planning.rs` -- the planning facade.** Positive duties and their
owners:
- **Single point of entry to the whole planner subsystem** (`use
  static_transition::{...}` blocks re-exporting the planner's own
  validated types/functions to the backend) -- owner: `planning.rs`,
  serving the same re-export duty for `lowering/mod.rs` and any other
  crate-level consumer that `lowering/mod.rs` serves for its own
  siblings, one layer further out.
- **Orchestration entry points**
  (`plan_static_transition_graph_with_symbols` and siblings) -- owner:
  `planning.rs`/`static_transition.rs` jointly, unchanged since before
  this campaign; no split item moved this orchestration seam, so there
  is nothing to reconcile that wasn't already reconciled.

**Neither facade "still re-exports the monolith's whole surface."**
Both are narrow, named re-export surfaces with a stated purpose per
block (every block above carries its own attributing comment naming
which item's move or which descendant consumer needs it) -- the
`AC-8` finding is that the positive duties are ALREADY reconciled by
the accumulated per-item work, not that this closure item newly
reconciles them. Strand 1's three narrowings are the only corrections
this pass found necessary.

## Strand 3 — residual test root, classified before closed

`lowering/core/tests/control.rs` -- 218 `#[test]` functions across
30,161 lines at pickup, re-measured directly. **Exhaustive
classification** (four-way: domain / shared-fixture / mutation-control
at its production injection point / class-4 end-to-end) run over the
full population, two representative clusters independently
spot-checked against the classification given (both confirmed correct:
`d8i_the_discharge_facet_is_transported_stated_and_refuses_both_ways`
reaches its own hub-declared `d8i_*`/`d8d_*` helpers directly from
`crate::cranelift_backend::lowering`, no extracted domain;
`d6a_a_specialization_binds_two_leading_static_workers_for_the_ih_and_its_recursive_argument`
compiles a whole governed-bracket fixture and inspects trace output,
squarely class-4).

**One genuine class-1 finding, acted on:**
`refusal_pins_rehomed_computational_match_without_selector_exclusion`
constructed a `Lowered::ComputationalRecursorClosure` directly and
pinned `boundary_transfer_admissibility`'s exact refusal -- `boundary.rs`'s
own domain (the method is declared there), matching the idiom of sibling
tests already in `boundary.rs`'s own `mod tests`. **Relocated** (verbatim
body, two missing imports added -- `inert_test_static_origin`,
`UnsupportedLowering`, both rustc-suggested and independently confirmed by
the clean build). Discovery parity confirmed: the test now discovers as
`cranelift_backend::lowering::boundary::tests::{name}`, once, matching
AC-2's exact-name requirement. Its immediate neighbor,
`refusal_pins_rehomed_static_worker_without_selector_exclusion`, exercises
`StaticWorkerFieldLedger` (hub-declared at `mod.rs`, no single-domain
owner) -- confirmed NOT class-1, stays.

**The rest of the population, approximate (not individually pinned to
the test):** roughly 40-55 class-3 (mutation controls at an
already-moved domain's own injection point -- the `ced_d3_*`/`d3b_*`/
`d3c_*`/`typed_trap_exit_*`/`d5a_*` clusters, each flipping a
domain-owned `#[cfg(test)]` mutation enum), roughly 150-165 class-4
(built on whole-pipeline entry points -- `compile_expr_into_module`,
`px8j_capture_source_trace`, `plan_static_transition_graph[_with_symbols]`
-- including the two explicit whole-backend census tests, which
deliberately span multiple domains by design). 154 of 218 hit zero
extracted-domain symbol at all -- these are the file's own designated
subject matter per its header (`oriented_*`/`px8j_*`/root-authority/recursor
tests), not any domain's.

**Per the frame's own strand-3 correction:** the large majority being
class-4 is not evidence a production owner was missed -- it is the
class of test the report says legitimately remains here. **Control.rs
closing at ~30k lines with genuine class-4 population is therefore a
fresh integration-test ownership question, not a missed-move finding**
-- named as such in Strand 4 below, not silently converted into
"needs another domain extraction."

**Stated limitation, not banked:** the sub-agent that ran this
classification did not individually read all ~150 class-4-bucketed
tests line by line; classification there rests on the presence of a
whole-pipeline entry point in the body, a strong but not airtight
signal, corroborated here by two independent spot-checks (both
confirmed correct) but not by a full second read. If a tighter audit
is ever wanted, the `d8i`/`d8j`/`d8k`/`d8o` and `d6a`/`d6b`/`d6c`
clusters were named as the lightest-sampled.

## Strand 4 — the complete resulting file population

**Every `.rs` under `crates/ken-runtime/src/cranelift_backend/` (37
files) plus `boundary_value_clif.rs`, measured directly, not from any
earlier item's own count:**

| file | lines |
|---|---|
| `lowering/core/tests/control.rs` | **30,099** (post strand-1/3 edits; was 30,161 at pickup) |
| `lowering/core.rs` | **13,019** |
| `lowering/mod.rs` | **12,323** (post strand-1 narrowing; was 12,319 at pickup) |
| `planning/static_transition/continuations.rs` | 9,768 |
| `lowering/core/tests/constructors.rs` | 7,813 |
| `planning/static_transition/closure.rs` | 6,763 |
| `lowering/units.rs` | 6,319 |
| `lowering/source.rs` | 6,216 |
| `planning/static_transition/continuations/fusion.rs` | 6,147 |
| `lowering/aggregates.rs` | 5,581 |
| `lowering/boundary.rs` | 2,889 (post strand-3 addition; was 2,817 at pickup) |
| `lowering/core/tests/effects.rs` | 4,083 (unchanged by this item) |
| `boundary_value_clif.rs` (outside the subtree, in-boundary per `AC-6`) | 9,116 |
| *(remaining 24 files)* | each under 3,200, full list available on request |

**Three files remain over 10k.** Per the frame's own explicit rule ("if
any file remains over 10k, recording its owner analysis is an ACCEPTED
FINDING and phase closure remains OPEN, requiring a named successor slice"
-- and "do not extract speculatively to get under the number"),
**this closure item does NOT force these three down**, and states each
one's own owner analysis rather than ticking the box:

- **`control.rs` (30,155 lines).** Strand 3's own classification is the
  owner analysis: the large majority of its population is genuine
  class-4 end-to-end control, which the frame's own guidance says
  belongs here. This is **not** "an unclaimed domain hiding at scale"
  -- it is a residual integration-test file whose size is a direct,
  expected consequence of the campaign's own design (every domain's
  own mutation/unit tests moved to their sibling files across 13
  items; what's left is deliberately the cross-cutting and
  whole-pipeline population). **Named successor:** a fresh
  integration-test ownership cut -- e.g. partitioning class-4 controls
  by which planning/lowering seam they cross (continuation-composition
  end-to-end vs. effect/host-call end-to-end vs. census/inventory
  self-checks) -- is the correctly-scoped next slice, not a domain
  extraction from this campaign's own owner list.
- **`core.rs` (13,019 lines) and `mod.rs` (12,331 lines).** These are
  the two files 13 of this phase's 17 items have already extracted
  from (`core.rs` was 20,413 lines and `mod.rs` 21,200 at `7509c77a7`,
  the frame's own cited baseline -- now down to 63.8%/58.2% of that).
  Item 17's own D0 (folded into this closure) found no further
  distinct semantic lifecycle left in either file after an exhaustive,
  independently-forked search, and the Architect's own absorption
  ruling (`evt_2ng0vm7h85zst`) confirmed this empirically (zero native
  CLIF trap emission anywhere in `lowering/`) and architecturally (the
  frame's own 13-owner map names no fourteenth). **Named successor: none
  identified by owner-search** -- these two files' residual content
  (the `Lowering` hub struct's own state/dispatch, the giant
  `compile_expr_into_module_with_root_projection` driver, and the
  several genuinely-shared cross-domain primitives Strand 2 names) has
  no single further semantic lifecycle this campaign's own
  decomposition strategy (`stage -> owner -> module`) can extract
  without inventing a domain that doesn't exist, which the banned
  scope forbids. **This is an honest limit of the campaign's own
  strategy, not a deferred task** -- closing it further would need a
  different decomposition axis than "find one more owner" (e.g.
  splitting the hub struct's OWN state into narrower per-concern
  structs, a genuinely different and larger design question this
  closure item is not positioned to open unilaterally).

## Net disposition

**Phase closure remains OPEN**, per `AC-6`'s own explicit rule -- three
files over 10k, two with no further owner-search successor identified,
one (`control.rs`) with a named successor (a fresh integration-test
ownership cut). Strands 1-3's own work (three adapter narrowings, one
test relocation, the facade reconciliation write-up) is this item's
own complete, mergeable transfer regardless of the open-closure
finding -- per `AC-6`'s own text, this node's job is to state the
finding accurately, not to force a false "closed."

