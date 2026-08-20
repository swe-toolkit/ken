---
id: RT-LOWERING-VALUES-BOUNDARY-SPLIT
title: "Move the values and boundary domain out of the lowering files -- and boundary_value_clif.rs is NOT absorbed merely because it is large; its lifecycle and consumers must be proven first"
status: ready
owner: runtime
size: M
gate: none
depends_on: [RT-LOWERING-FUNCTION-STATE-SPLIT]
blocks: [RT-SOURCE-MACHINE-TYPES-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 11; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

## Model-capability estimate (steward.md §4h): T2 — mechanical

Behaviour-preserving move executing this slice's pre-built D0 symbol and
test-property ledgers: the T2 (cheap coder) row of steward.md §4h. This records
per-WP the phase's standing seat ruling — RT-BACKEND-MODULE-SPLIT "runs T2, and
only this phase" (operator 2026-08-10, agent/MODELS.md) — not a fresh per-slice
judgment. The design judgment — the domain ownership boundary — is discharged in
the D0 and its Architect vote, not by the implementer executing the D1/D2 moves.

# LOWERING-DOMAIN DISCIPLINE (established by item-10 ruling evt_2f3tkq8hgqa4a) — READ FIRST

Item 10 (`RT-LOWERING-FUNCTION-STATE-SPLIT`) determined that **`Lowering<'a>` is
the retained hub — the lowering-side `StaticTransitionPlan`.** It and every
frame/eliminator/scope type the indivisible SCC (`impl<'a> Lowering<'a>`,
`core.rs:3090-20330`) consumes STAY at their LCA (`lowering/mod.rs`); the SCC
stays whole in `core.rs`, honored not relitigated. Size reduction comes from the
DOMAIN METHOD-FAMILY slices — of which this is one.

⇒ **This slice moves the values-boundary domain's METHOD FAMILY (impl blocks)
into a DESCENDANT child of `lowering` that reads the retained hub via descendant
visibility with ZERO widening — the `construction.rs`/`closure.rs` shape.** You
do not move `Lowering`, its fields, or any SCC-consumed type; you do not widen a
visibility to make a move compile (that is a finding, stop and route).

**`D0` MUST classify by PER-TYPE SCC-CONSUMPTION GREP, never textual adjacency.**
Item 10's withdrawn first cut mis-scoped 23 types by adjacency; its re-cut
corrected the method and self-caught a real defect — `FunctionLocalRefs` had 0
bare type-name hits in the SCC but 16 `self.function_local.<field>` NESTED
accesses that never name the type. Grep for nested field access, not just the
type name. Check a SECOND pinning population the first cut missed: struct-literal
construction in `core/tests/*` (descendants of `core`, not of a prospective
sibling) pins a type until its `D2` relocates it, and `evt_6r403ez3m2m69` forbids
pulling that forward.

**Starting inventory:** item 10's per-type SCC-pinning census (in
`RT-LOWERING-FUNCTION-STATE-SPLIT.md`, landed with this node's release) records
what is already known pinned to the hub. Do not re-derive it; extend it for the
values-boundary population.

**If the values-boundary method family is itself SCC-pinned** (as function-state
turned out to be), this node is ALSO a hub-retained determination, not a forced
move — state `OUTCOME 2` (coherent descendant child, proceed) vs `OUTCOME 3`
(pinned, hub-retained determination) explicitly in `D0`, and hard-stop the fork
to the Architect rather than resolving it with a zero-widening claim. That is the
fork item 10's first cut should have stopped on.

> # THE OPERATOR'S CONSTRAINT, AND IT IS THE ONLY ONE
>
> **2026-08-18: "Files over 10k lines are decomposed into architecturally sound
> smaller files. That is the whole constraint."** How that is accomplished — the
> factorization and the sequencing — is the Steward's and the Architect's.
>
> ⇒ **Nothing in this frame is an operator constraint** beyond that sentence.
> Re-derive a constraint at each use rather than inheriting it.

**Cut item 11 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/mod.rs`, `cranelift_backend/lowering/core.rs`, and
possibly `boundary_value_clif.rs` (9,116).

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Values and boundary.** Boundary outcomes, boundary value construction and
decode, and the value vocabulary the emitters consume.

> ### `boundary_value_clif.rs` IS NOT ABSORBED MERELY FOR SIZE
>
> **Standing amendment.** It is 9,116 lines and therefore **under the operator's
> 10k constraint already** — it is not in the bound population and nothing
> requires it to move.
>
> ⇒ **`D0` must prove its lifecycle and its consumers before any part of it is
> touched.** Absorbing it because it is adjacent and large is the "optimize for
> equal-sized files" failure the research program report names as a guardrail
> violation. **If the proof does not come back positive, leave it alone and say
> so** — that is a result.

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
this slice just established** (boundary-outcome, boundary-value and
value-vocabulary controls). Place multi-leaf fixtures **once**,
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

## D0 ledger, re-measured at `0c5bb765a`

Bound files: `lowering/mod.rs` **21200** lines, `lowering/core.rs`
**20413** lines — unchanged from item 10 (both closed with no code motion).
SCC boundaries unchanged: `compile_expr_into_module` (`core.rs:2040-3089`)
plus `impl<'a> Lowering<'a>` (`core.rs:3090-20330`), re-extracted fresh at
this SHA and byte-identical to item 10's extraction. `core.rs` carries
**zero** `Boundary`/value-vocabulary declarations of its own — the whole
domain lives in `mod.rs`.

### `boundary_value_clif.rs` — the proof comes back negative

It is **not a `cranelift_backend` file at all**: `lib.rs:32` declares
`mod boundary_value_clif;` at the **crate root**, a sibling of
`cranelift_backend`, not a descendant of `lowering`. It is already
under the 10k ceiling (9,116 lines), already correctly separated (its own
module doc: *"the executable half of the boundary-value ABI"*, paired
1:1 with `crate::boundary_value`, the constants/layout half), and consumed
from deep inside the immovable SCC itself (`core.rs:2636`,
`crate::boundary_value_clif::emit_boundary_value_local_graph`) plus a
`mod.rs` struct field (`:842`) and multiple `core/tests/*` sites. Moving
any part of it into a descendant of `lowering` would not shrink
`mod.rs`/`core.rs` (it is not counted in either today) and would move it
in the wrong direction — out of a stable, already-scoped, crate-root
position and into a nested one, reversing its own module doc's stated shape
for no size benefit.
**Left untouched; not in the bound population.**

### The domain's actual shape: a movable vocabulary half and a pinned
### emission half

"Values and boundary" splits cleanly into two halves with different
Rust-visibility profiles, discovered by reading every candidate's own
declared qualifier, not by assuming adjacency:

- **The disposition/classification/lifecycle vocabulary** — `Lowered`'s
  own variant tag and its static (non-emitting) properties, plus the phase
  and policy machinery. Declared `pub(in crate::cranelift_backend)`, on both
  the **type and every field/variant**, throughout. This qualifier is an
  **absolute, crate-relative path** — unlike `pub(super)` (relative to
  whichever module currently hosts the item) it names `cranelift_backend`
  directly, so relocating a `pub(in crate::cranelift_backend)` item
  **anywhere within `cranelift_backend`'s own subtree** (including a
  brand-new descendant child of `lowering`) does not change what it means,
  to any caller, anywhere in the facade — SCC included, test-tree included.
  This is a stronger and simpler zero-widening case than item 10's
  `pub(super)` finding: item 10 found `pub(super)` survives moving between
  **existing siblings of the same parent**; here the qualifier is already
  anchored to the parent two levels up and is untouched by relocation to a
  **new** child.
- **The carrier-emission machinery** — the CLIF-level code that actually
  constructs and decodes a boundary word (`transfer_into_carrier` and the
  ~30-method `emit_carrier_*`/`carrier_*` family it dispatches to). These
  are bare-private or `pub(super)`-relative-to-`mod.rs`, and the entry
  point (`transfer_into_carrier`) is called **directly from inside the
  indivisible SCC** dozens of times (`core.rs:3603/3786/14506/14824/17687`,
  plus more) and **directly from `core/tests/constructors.rs`** dozens more
  times — both populations item 10 established as binding. This machinery is
  pinned to the LCA the same way item 10's frame/scope family was, for the
  same reason: it is bare-private, and both consumers (the immovable SCC
  and the not-yet-relocated test tree) reach it today only through
  descendant-of-`mod.rs` privilege that a new sibling of `core` does not
  inherit.

### Per-item evidence

**Movable — `pub(in crate::cranelift_backend)` on type and fields/variants,
confirmed by direct read of each declaration (not assumed from the
qualifier on one member):**

| item | kind | location |
|---|---|---|
| `BoundaryTransferInvokingSite` | enum (facade-qualified) | `mod.rs:2482-2609` |
| `Lowered::variant` | method (facade-qualified; `Lowered` itself stays — method-level move, hub-stays/method-moves, per item 10's precedent) | `mod.rs:10937` |
| `LoweredVariant` + `::ALL` const + `::boundary_disposition` | enum + const + method | `mod.rs:10794-10992`, `:11949-12133` |
| `StaticEncodingPolicy` + `::ALL` const + `::policy` (on `BoundaryDisposition`) | enum + const + method | `mod.rs:10976-11051` |
| `HandleIdentity` | enum | `mod.rs:11092-11116` |
| `BoundaryOutcome` + `::permitted_by` + `::requires` + `::phase_closure` | enum + 2 impls | `mod.rs:11117-11144`, `:11349-11430`, `:11616-11713` |
| `BoundaryInput` + `::all` + `::outcome` (+ its private helper `handle_identity`, self-contained, called only from `outcome` in the same impl block) | struct + impl | `mod.rs:11145-11348` |
| `BoundaryDisposition` + `::policy` | enum + impl | `mod.rs:11774-11809` (decl), `:11002-11016` (impl, textually earlier in the file — forward reference, harmless) |
| `LifecyclePhase` + `::ALL` const + `::index` | enum + impl | `mod.rs:11431-11465`, `:11466-11573` |
| `PhaseBinding` | enum | `mod.rs:11574-11592` |
| `PhaseClosure` + `::binding` | struct + impl | `mod.rs:11593-11615` |
| `Lowered::boundary_disposition`, `Lowered::source_aggregate_producer`, `Lowered::boundary_transfer_admissibility` | methods only (`Lowered` the type stays pinned — SCC-consumed 238×; these three methods are individually `pub(in crate::cranelift_backend)` and need only descendant-of-`mod.rs` access to match `Lowered`'s bare-private variants, which a new child of `lowering` has) | `mod.rs:11798-11948` |
| `impl crate::boundary_value::BoundaryEmissionPlan { fn derive() }` | inherent impl on a type declared in the **already-`pub`, crate-root** `boundary_value` module; `derive()` itself is `pub(crate)` (crate-wide, a fortiori portable); called once from the SCC (`core.rs:2627`) but the call survives relocation regardless, same absolute-qualifier reasoning | `mod.rs:21028-21120` |

**Approximate moving total: ~1,576 lines** (128 + 1,355 + 93, the three
spans above) — **not yet exact**, to be confirmed precisely at `D1`
handback per this campaign's standing convention (item 10's own AC-4b
note). Comparable to already-landed children of this campaign
(`joins_traps.rs` 1,155, `effects.rs` 733) — a coherent, materially-sized
child, not a token extraction.

**Pinned by direct SCC production consumption (bare-private/`pub(super)`,
reached today only via descendant-of-`mod.rs` privilege the SCC has and a
sibling of `core` would not):**

`Lowered` (238 hits — type stays; its facade-qualified methods above move);
`LoweringOperand` (324 hits — type stays; its own 4-method impl block at
`mod.rs:3971-4060`, `effect_seat_phase`/`specialized_at`/
`specialized_join_arm`/`specialized_ref_at`, reads as
Effects/Continuation-join territory by name, not values-boundary's to claim
— flagged, not pursued, left with the type); `CarriedBoundaryWord` (19 hits
— type stays, no separate methods); `transfer_into_carrier`
(`mod.rs:6521-6529` and `:7010-7030`ish — direct SCC calls at
`core.rs:3603/3786/14506/14824/17687`) and `emit_carrier_transfer` (its
private recursive step, `mod.rs:8879-9002`ish); the ~30-method
`emit_carrier_*`/`carrier_*` family it dispatches to
(`mod.rs:9369-10780`, confirmed by grep against the freshly re-extracted
SCC: 13 of the ~30 have direct call sites — `carrier_arena`,
`carrier_identity_immediate`, `emit_carrier_immediate`,
`emit_carrier_store_tag_id`, `emit_carrier_store_field`, `emit_carrier_tag`,
`emit_carrier_class`, `emit_carrier_host_success`,
`emit_carrier_host_payload`, `emit_carrier_scalar`,
`emit_carrier_field_count`, `emit_carrier_field`, `emit_carrier_record_field`);
the remaining ~17 members of that family are pinned by **internal coupling**
to the SCC-consumed members (`carrier_refs`/`carrier_arena` are each called
19× by their own siblings in the same impl block) or by test-tree
consumption below, except `emit_public_carrier_scalar` (`pub(super)`
**declared at `mod.rs`**, so its current meaning is already
`pub(in cranelift_backend)` — but per the corrected mechanism, relocating a
`pub(super)` item **out of the module where it is declared and into a
child** re-anchors it to `pub(in lowering)`, which still covers its one
real external caller `units.rs:3949`; grouped here anyway rather than split
out, since it is one call inside a thoroughly-pinned 30-method family and
splitting one member out is not materially different from moving it, for a
family this tightly coupled through shared private state
(`self.function_local.boundary_carrier`));
`BoundaryTransferInvokingSiteGuard` + its `Drop` impl
(`mod.rs:2610-2660`ish, constructed only inside the pinned
`carry_call_input`) and the two thread_locals it/`transfer_into_carrier`
read (`D2K_OWNER_TRACE`, `D2K_BOUNDARY_TRANSFER_INVOKING_SITE`,
`mod.rs:2600-2609`ish) — test-instrumentation co-located with pinned code.

**Pinned by coupling (not itself SCC-consumed, but a bare-private field
type of a pinned struct declared in `mod.rs`):**

`BoundaryCarrierRefs` (`mod.rs:3432-3450`ish, bare-private struct) is
`FunctionLocalRefs`'s own field type (`boundary_carrier:
Option<BoundaryCarrierRefs>`, `mod.rs:1014`, constructed at `mod.rs:908`)
— `FunctionLocalRefs` is item 10's SCC-pinned hub-companion and stays at
`mod.rs`; a bare-private field type it names must stay reachable **from
`mod.rs`**, which a descendant-of-`mod.rs` sibling declaration is not
(privacy does not flow upward from child to parent). Its own accessor,
`fn carrier_refs(&self) -> Result<BoundaryCarrierRefs, ...>`
(`mod.rs:9369-9376`), is the shared low-level read every member of the
pinned carrier-emission family calls (19 internal call sites) — pinned
twice over.

**Pinned by not-yet-relocated test-tree construction, checked directly
(second binding population, per item 10's discipline):**

`emit_carrier_alloc` — bare-private, called directly from `lowering/core/
tests/constructors.rs:7190/7206/7223`, a descendant of `core`'s own
module, not of a prospective new sibling — pins it independently of its 9
in-`mod.rs` callers. `transfer_into_carrier` itself — the entry point is
called from `core/tests/constructors.rs` at more than 20 additional sites
(`:2073, 2098, 2709, 2750, 2882, 3087, 3340, ...`), the single heaviest
test-tree consumer found in this domain. Neither is pursued as an atomic
`D1`+`D2` pair, for the same reason item 10 declined one: the population
is woven through shared fixtures across the whole
`control.rs`/`constructors.rs` test tree, and pulling it forward pre-empts
`evt_6r403ez3m2m69`'s standing ban on ahead-of-boundary `control.rs`
decomposition.

**A confirmed-negative check, not a pin:** `BoundaryInput` was initially
flagged a test-tree-pinning candidate (7 struct-literal constructions in
`core/tests/control.rs:10088-10220`) — but since `BoundaryInput`'s type
**and every field** are `pub(in crate::cranelift_backend)`, that
construction does not pin it: the absolute qualifier reaches the new child
regardless of where the type is declared, so those test sites keep
compiling either way. Recorded so a future reader does not re-flag the same
false positive.

### Source-text oracle found

`lowering/core/tests/control.rs:9573` reads mod.rs's own source text and
matches the literal string
`"fn boundary_disposition(self) -> BoundaryDisposition {"`
via `.split_once(...)`. `Lowered::boundary_disposition` moves in this
slice (it is one of the movable methods above); this oracle's expected
substring must move with it, or be re-pointed at the new file, at `D1` —
recorded here so `D1` does not discover it as a surprise
compile-clean-but-logic-broken regression. No other
`include_str!`/source-text-oracle hits found scoped to this domain's
population; `include_str!` = 0 in both bound files, unchanged from item 10.

### Population reconciliation

**Moving to the new descendant child (facade-portable, zero widening):** 10
types/enums + 3 consts + roughly a dozen methods, ~1,576 lines (exact count
at `D1`).

**Retained at the LCA, extending item 10's hub census (this domain's own
companions, pinned by SCC/coupling/test-tree — not this slice's to move):**
`Lowered`, `LoweringOperand`, `CarriedBoundaryWord`, `BoundaryCarrierRefs`,
`BoundaryTransferInvokingSiteGuard`, `transfer_into_carrier`,
`emit_carrier_transfer`, the ~30-method `emit_carrier_*`/`carrier_*`
family, 2 thread_locals, `LoweringOperand`'s own 4-method impl block
(flagged as foreign-domain-shaped, not claimed here).

**Explicitly out of scope, deferred to its own claiming slice's `D0`
(item 10 precedent — per-item attribution belongs to whichever item claims
that domain):** every other declaration in `mod.rs`/`core.rs` not named
above — the Effects/Aggregates/Calls/Joins/Continuations/source-machine
method families riding on the retained `Lowering` hub (items 12-17), and
`ProductionAnchor` (`mod.rs:11485-11573`ish, its own `derived_witness`/
`CONTROL_CLOSED` — a phase-closure production-tracking type physically
interleaved with this domain's vocabulary cluster but semantically a
different, broader closure/attestation concern; not values-boundary's to
claim, flagged rather than swept in by adjacency).

### THE OUTCOME DETERMINATION, STATED EXPLICITLY

**This is `OUTCOME 2`, not `OUTCOME 3`.** Unlike item 10, this domain is
**not** wholly SCC-pinned: a genuinely facade-portable vocabulary/
classification/lifecycle-phase sub-population exists, confirmed
type-by-type and field-by-field (not assumed from one member's qualifier),
and it is material — ~1,576 lines, larger than two of this campaign's own
already-landed children. The domain's other half (the CLIF-level
carrier-emission machinery) is genuinely pinned, by the same mechanisms
item 10 established (direct SCC consumption, test-tree consumption,
coupling to a pinned field type) — and it stays at the LCA alongside item
10's retained hub, which is architecturally consistent: the low-level
emission machinery is exactly the kind of code the indivisible SCC (a prior
campaign's own construct) was built to hold. No hard-stop is warranted; the
fork the frame asked me to check for (wholesale SCC-pinning, as happened to
function-state) did not materialize here.

### Test-property ledger

Deferred to `D2`, per the standing rule (`control.rs` 33,969 lines,
`evt_6r403ez3m2m69` forbids pre-emptive decomposition). The two test-tree
findings above (`emit_carrier_alloc`'s and `transfer_into_carrier`'s
`constructors.rs` call sites, and the `control.rs:9573` source-text oracle)
are flagged now because they bear on **production** pinning and the oracle
inventory, not because `D2`'s own test-classification is being pre-empted
here.

### Evidence seats

**Intention producer:** this `D0` (the per-type/per-method visibility and
consumption census above). **Independent artifact observer:** the fresh SCC
re-extraction (byte-identical to item 10's, confirmed by `md5sum`) plus the
whole-tree greps against the current checkout at `0c5bb765a`, not against
any cached/stale extraction. **Closeout/publication seat:**
runtime-leader's object-store verify, then the Architect's `D0` DESIGN
vote.

