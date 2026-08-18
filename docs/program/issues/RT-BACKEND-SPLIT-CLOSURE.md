---
id: RT-BACKEND-SPLIT-CLOSURE
title: "Close the backend module split -- delete the transitional adapters, narrow the facades, run the test-root closure over control.rs, and prove all four bound files are under 10k"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-EMITTER-TERMINALS-CLEANUP-SPLIT]
blocks: []
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 18; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 18 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
all four bound files.

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Closure.** No new domain. This slice deletes what the phase built as
scaffolding and proves the operator's constraint is met.

## THE FOUR DELIVERABLE STRANDS

1. **Delete the transitional adapters** every earlier slice was permitted to
   leave behind.
2. **Narrow the facades.** A facade that still re-exports the monolith's whole
   surface has recreated the monolith behind a new name — that is banned by the
   phase gates and this is where it is checked.
3. **The test-root closure.** Remeasure `lowering/core/tests/control.rs` and
   **prove it under 10k.** The companion test moves in items 4-17 are what make
   this reachable; if it is not reachable, the finding is which owner's tests
   were never claimed.
4. **Remeasure all four bound files** and state each one's line count against
   the 10k constraint.

> # THIS IS WHERE THE OPERATOR'S CONSTRAINT IS DISCHARGED, AND NOWHERE ELSE
>
> **"Files over 10k lines are decomposed into architecturally sound smaller
> files. That is the whole constraint."** No earlier slice may claim the
> constraint is met; each one claims only its own transfer.
>
> **A file still over 10k here is a result, not a failure to hide.** Report it
> with the residue's owner analysis and route it, rather than extracting
> speculatively to get under the number — **optimizing for a line count is the
> guardrail violation this phase was warned about from the start.**

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
  enlarges**. **No move may create a file over 10k**, and a move that would is a
  finding to route rather than a transfer to complete.
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
  population.** State the line count of **every** Rust file in the phase
  boundary, including every child created by items 4-17. **Closure holds only if
  all of them are below 10k**; any file over it is recorded with its owner
  analysis and **leaves closure OPEN with a named successor slice.**
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

