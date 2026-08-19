---
id: RT-LOWERING-FUNCTION-STATE-SPLIT
title: "Move function-local lowering state out of lowering/mod.rs and lowering/core.rs into its own child -- the first lowering domain, and the point where the phase crosses from the planner files into the lowering files"
status: closed
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-ROOT-CLOSURE-SPLIT]
blocks: [RT-LOWERING-VALUES-BOUNDARY-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 10; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

# ARCHITECT DETERMINATION (dec_zkv16ra4nh9j, evt_2f3tkq8hgqa4a) — hub-retained, no material move. NODE CLOSED.

**Ruled 2026-08-19. This node is a determination, not a mover. There is no
`D1`/`D2` production move. `status: closed`.** The `D0` re-cut ledger below
(re-measured at `2be7f513c`, commit `85621309a`, Architect-confirmed SOUND) is
the durable per-type SCC-pinning inventory; it is carried forward as the
starting inventory for items 11-17's own `D0`s.

**THE RULING (the Architect's third path, rejecting both the ledger's offered
paths):** function-state IS the retained hub — the lowering-side
`StaticTransitionPlan`. `Lowering<'a>` and every frame/eliminator/scope type the
indivisible SCC consumes RETAIN at their LCA (`lowering/mod.rs`); the SCC stays
whole in `core.rs`, honored not relitigated. The size reduction of
`mod.rs`/`core.rs` comes from the DOMAIN METHOD-FAMILY slices (items 11-17:
values-boundary, source-machine, calls, joins/traps, aggregates, effects), whose
impl blocks move to descendant children reading the retained hub with zero
widening — the exact `construction.rs`/`closure.rs` shape from
`RT-PLANNER-ROOT-CLOSURE-SPLIT`.

- **`AC-4b` is INAPPLICABLE** (not satisfied), the same disposition
  `RT-PLANNER-ROOT-CLOSURE-SPLIT` used for its own no-move outcome.
- **Path 1 rejected** (trivial 55-line `AmbientBodyAuthority` move):
  non-material, line-count-driven, and it would pre-claim the permanent
  `function_state` module name for a sliver — the frame bans naming a module
  after a thin campaign node.
- **Path 2's framing rejected** (force an SCC-accessor-boundary representation
  change so function-state gets its own module): architecturally unsound and
  contradicts the item-9b precedent this node's own reasoning rests on — the hub
  stays at its LCA, domain method families move to descendants, not the reverse.
  You do not rewrite the SCC's dozens of direct field/variant accesses through
  accessors purely to relocate a 49-field struct; the struct is cheap at the
  LCA, the mass is in the methods.

**DOWNSTREAM FLAG (Steward/operator to track, NOT reopened here):**
`lowering/mod.rs` (21,200) is decomposable this way — its four non-SCC domain
impl blocks (`@:4815/:6500/:13631/:16777`) are the movable mass. But
`lowering/core.rs` (20,413) is dominated by the ~17k-line immovable SCC
(`:3090-20330`); **its own path under 10k is a separate question the domain
slices cannot solve while the SCC ruling stands.** Flagged so it is not
discovered late. Whether to pursue a representation change to the indivisible
SCC (vs. accepting `core.rs` stays over 10k) is the operator's call, framed from
this ruling — not decided here.

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

**Cut item 10 of [[RT-BACKEND-MODULE-SPLIT]]**, which is a **phase record and
will never merge** — it becomes `closed` when the cut is fully filed. This node
is complete for its own named transfer and **does not claim phase closure.**

Bound file for this slice:
`cranelift_backend/lowering/mod.rs` (21,200) and
`cranelift_backend/lowering/core.rs` (20,413).

# THE OWNER — a semantic lifecycle, never a line count or a campaign name

**Function-local state.** Per-function lowering state, scopes, frame
authorities, and the state machinery `core.rs` threads through emission.

**This is where the phase changes files**, and the contention picture changes
with it — see CONTENTION below. Everything up to item 9b runs in
`static_transition.rs`, which no live semantic node touches. From here on the
slices contend with the runtime lane's own semantic work.

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
this slice just established** (function-state, scope and frame-authority
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

## D0 ledger, re-measured at `2be7f513c` (RE-CUT — supersedes `60ae7cb8f`)

**This re-cut responds to Architect CHANGES REQUESTED (`dec_1xqze0vcr7rdg`,
`evt_1an5avts6vm91`) on the withdrawn D0 `60ae7cb8f`.** That candidate's
"zero widening required" claim was false: it proposed relocating the moving
population to a NEW SIBLING of `core` (`lowering::function_state`), which
severs `core.rs`'s existing ancestor-private access to items declared in
`lowering` (mod.rs) — `core` reaches them today only because it is a
**descendant** of `lowering`, not because of anything about siblinghood. A
sibling module gets no such privilege. This re-cut re-derives the boundary
per-type, by direct SCC-consumption grep rather than by textual adjacency
to the struct, exactly as directed.

Bound files unchanged: `lowering/mod.rs` **21200** lines, `lowering/core.rs`
**20413** lines at `2be7f513c`.

### The corrected mechanism (why the withdrawn D0 was wrong)

`pub(super)` on an item declared *inside* `core` already means `pub(in
lowering)` — moving such an item to a **different child of `lowering`**
(a new sibling of `core`) changes nothing about what it means, since the
new host's own `super` is still `lowering`. That relocation is genuinely
zero-widening. But a **bare-private** item declared *at* `lowering` (i.e.
in `mod.rs` itself, or in `core.rs` with no `pub` qualifier at all) is
visible today to `lowering` and every one of its descendants **only
because it is declared there** — relocating it to a *different* descendant
of `lowering` requires adding a `pub(in lowering)`-equivalent qualifier to
restore the reach it already had, and the Architect's ruling is explicit
that doing so **to make a move compile** is the banned widening, full stop,
regardless of whether the net reachable audience is unchanged in the
abstract. This re-cut applies that literally: a type moves only if its
**existing** consumers already reach it through a channel unaffected by
which sibling of `lowering` hosts it (a `pub(super)`-qualified associated
function call) or have **no** existing consumers requiring private access
at all.

### Per-type SCC-consumption and test-tree-consumption census (the corrected
### method)

`core.rs`'s indivisible SCC is `compile_expr_into_module` (`:2040-3089`)
plus `impl<'a> Lowering<'a>` (`:3090-20330`) — the module doc's own "29-method
SCC plus `compile_expr_into_module`." Both spans were extracted and grepped
per candidate type (not sampled). **A second, equally binding population
was checked this time and was missing from the withdrawn D0: `lowering/core/
tests/{control.rs,constructors.rs,mod.rs,effects.rs}`** — these are
descendants of `core`, not of any prospective new sibling, and several
construct the moving population via bare struct literals (private-field
access), which is `D2`'s population to relocate, not this `D0`'s, and `D2`
has not run.

**Pinned by the indivisible SCC's own production code (direct private
field/variant/method access, confirmed by grep — cannot move under any
sequencing without reopening the SCC ruling):**

`Lowering<'a>` itself (49 fields, dozens of direct field reads:
`self.function_local` 25x, `self.process_symbols` 52x,
`self.static_transition_plan` 23x, `self.defining_emission_owner` 15x,
`self.consumed_subcontinuation_frames` 14x, `self.defining_function_id`
12x, more); `FunctionLocalRefs` (**correction to the withdrawn D0**: a
naive name-grep found 0 textual mentions of the type name in the SCC and
was read as "unconsumed" — wrong, because the SCC reaches its fields
through `self.function_local.<field>` without ever naming the type; 25+
such field accesses confirmed: `.worker_calls`, `.raw_worker_calls`,
`.defining_abi_operands`, `.unit_calls`, `.declaration_calls`, more);
`TrapExitAuthority` (`FunctionLocalRefs`'s own `trap_exit` field type — no
field access to `trap_exit` specifically found in the SCC body, but it
travels with `FunctionLocalRefs`, which is itself pinned, so splitting it
out alone buys nothing); `EliminatorFrame` (97 real hits — variant
construction/destructuring); `EliminatorRole` (8 hits); `ActiveContinuationFrame`
(11); `ComputationalEliminatorFrame` (9); `OrdinaryEliminatorFrame` (4);
`PendingLetContinuationFrame` (1, struct-literal construction);
`InvocationTemplateRef` (4, variant construction/destructuring);
`CheckedRecursiveInvocationInstance` (2, struct-literal destructuring);
`OwnedSelectedScope` (3, struct-literal construction); `CarriedInvocationCoordinates`
(4 hits, including calls to its own `fn of` — a **private**, not `pub`,
inherent method at `mod.rs:14596`, so the SCC needs ancestor privilege to
call it at all); `CheckedFrameBranchScope` (`core.rs`-bare-private, 8 real
hits, genuinely core-internal).

**Pinned by coupling to one of the above (not itself SCC-consumed, but
depends on a type that is, or is depended on by one):**

`ConsumedSubcontinuationFrame` (`core.rs:1681`, a bare-private tuple-type
alias) is `CheckedFrameBranchScope`'s own field type (`baseline`/`union:
BTreeSet<ConsumedSubcontinuationFrame>`) — since `CheckedFrameBranchScope`
is SCC-pinned and cannot change its own qualifier, the alias its fields
depend on must stay reachable from `core.rs`, i.e. it stays too.
`CheckedFrameFunctionScope` (`core.rs:1906`, `pub(super)`) is **not**
SCC-consumed directly (its only real callers are `units.rs:2366/2520/2741/
2960/3379/3576/5878/6251` — an *existing sibling* of `core`, reached via its
already-adequate `pub(super)` qualifier) — but its own field
(`enclosing_consumed: BTreeSet<ConsumedSubcontinuationFrame>`) depends on
the now-pinned alias above. Moving `CheckedFrameFunctionScope` alone would
leave it unable to name its own field's type from a different module, so
it stays too, co-located with `ConsumedSubcontinuationFrame`.

**Pinned by not-yet-relocated test-tree construction (`D2`'s population, not
yet moved — a second binding constraint the withdrawn D0 never checked):**

`RecursorFrameProvenance`, `RecursorInvocationSegment`, `RecursorUnwindStack`,
`ComputationalRecursorLayer` — the Architect's own four named candidates for
the SCC-independent residual, and **correctly zero-hit in the SCC body
itself** — but all four are constructed via bare struct literals from
`lowering/core/tests/{control.rs,constructors.rs,mod.rs}` (`core/tests/mod.rs`
itself references `RecursorInvocationSegment`, meaning these are **shared
fixtures threaded through the test tree**, not one narrow test family — e.g.
`control.rs:641/795/1134/2296/6808/11091/11126`, `constructors.rs:4243-4262`).
Those test files are descendants of `core`, not of any prospective new
sibling module, so relocating the production types now — ahead of their own
`D2` — breaks `core/tests/*` compilation immediately, before `D2` ever runs.
An atomic `D1`+`D2` pair could in principle rescue this family, but the
population is woven through shared fixtures across the whole test tree
(`control.rs` alone is 33,969 lines), not a narrow, cleanly severable slice —
combining them now would mean exactly the kind of pre-emptive, broad
`control.rs` test relocation the Architect's standing ruling
(`evt_6r403ez3m2m69`) forbids doing ahead of a settled production boundary.
Not pursued as an atomic pair in this `D0`.

**The one type confirmed genuinely free to move, zero widening, no
atomic pairing needed:**

`AmbientBodyAuthority` (`core.rs:1822`, `pub(super)`) plus its own `bind`/
`release` methods (`core.rs:1827-1877`). Grep-confirmed: **zero** struct-literal
construction anywhere in the tree outside its own `impl` block (its private
fields `enclosing_owner`/`enclosing_unit` are touched only by `bind`/
`release` themselves); **zero** occurrences in `core/tests/*`; its only
external callers (`units.rs:2372/2793/3511/5823` and `core.rs:5982/10426/
12052/12198`, the latter *inside* the SCC) all go through the already-`pub(super)`
`AmbientBodyAuthority::bind` call, which needs no ancestor privilege — only
visibility to *name* the type and call its associated function, which
`pub(super)` already grants from any sibling of `core`, unchanged by
relocation.

### Population reconciliation

23 originally-proposed types: 15 pinned by direct SCC production
consumption, 2 pinned by coupling, 4 pinned by not-yet-relocated test-tree
consumption, 1 (`AmbientBodyAuthority`) confirmed free. 9 originally-proposed
methods: `Lowering::new`/`FunctionLocalRefs::bind_unit_trap_frame`/
`CheckedFrameFunctionScope::open`/`close`/`CheckedFrameBranchScope::capture`/
`start_successor`/`merge_successor`/`finish`/`harness` are all pinned
(bound to a pinned type); `AmbientBodyAuthority::bind`/`release` (2) are
free.

### THE OUTCOME DETERMINATION, STATED EXPLICITLY

> RESOLVED by the Architect (dec_zkv16ra4nh9j) — see the ARCHITECT
> DETERMINATION banner at the top of this frame. The outcome-3 finding below is
> CONFIRMED, but the two paths this ledger offered were BOTH rejected in favour
> of a third: hub-retained / no material move. The text below is the ledger's
> original reasoning, preserved as evidence; the disposition is the banner's.

**The SCC-independent residual is one type and two methods
(`AmbientBodyAuthority` + `bind`/`release`, roughly 55 lines).** That is
**not** "a coherent child that materially reduces `mod.rs`" — extracting 55
lines from a 21,200-line file changes nothing material, and the frame's own
guardrails forbid exactly this shape of move ("no line-count-driven
extraction... the constraint is architectural soundness... not equal-sized
files"; a module created to hold one small type is tidiness, not a
lifecycle boundary). Every other candidate is pinned either by the
indivisible SCC's own direct field/variant/private-method access (permanent,
short of reopening the SCC ruling — out of scope) or by construction sites
in `core/tests/*` that have not yet relocated (a `D2` dependency this `D0`
cannot discharge without pre-empting the Architect's own `evt_6r403ez3m2m69`
ruling against broad, ahead-of-boundary `control.rs` decomposition).

**This is `OUTCOME 3`, not `OUTCOME 2`.** Creating a genuine, materially-sized
function-state child module — one that actually holds `Lowering` and the
frame/eliminator/scope vocabulary named as this owner in the kickoff — would
require converting the indivisible SCC's *direct* private field and variant
access into a *deliberate accessor boundary* (methods the SCC calls instead
of touching fields/variants directly). That is a representation change: it
alters how the SCC's own code is written, not merely where a declaration
lives, and it is out of scope for a behaviour-preserving "pure move" `D1`.
Per the frame's own hard-stop instruction, this is exactly the boundary the
`D0` cannot settle by the frozen predicate.

**HARD-STOP.** Routing back for a ruling on which of two paths this node
takes:
1. Accept the residual as-is: a trivial `D1` moving only `AmbientBodyAuthority`
   (+ `bind`/`release`) into a new child, explicitly documented as **not**
   materially advancing the file-size constraint, with the true function-state
   population (the struct, the frame/eliminator/scope core) remaining
   permanently resident in `mod.rs`/`core.rs` pending a future
   representation-change node; or
2. Defer item 10's production move entirely and frame the representation
   change (SCC accessor boundary) as its own node, with this `D0`'s per-type
   census carried forward as its starting inventory — item 10 would then
   close on the census alone, or wait on that node.

Not authorized to choose between these unilaterally — a strategic choice
between materially different futures, per COORDINATION §6.

### Corrections carried from the withdrawn D0

The two open questions it flagged (`GeneratedContextCaptures`, the
environment-construction helper cluster `env_with`/`bound_values`/
`specialized_bindings_at`/`extend_specialized`/`extend_captures`) are now
moot under either outcome above — neither is part of the tiny confirmed-free
residual, and both would need the same per-type SCC/test-tree consumption
check if outcome 1 or a future node revisits them. `include_str!` = 0 both
files, unchanged. Test-property ledger: still deferred to `D2`, unchanged;
the `FrameScopeHarnessWitness`/`FrameScopeHarnessMutation` harness pair
(`core.rs:2022-2037`) is itself part of `CheckedFrameBranchScope`'s own
harness and is therefore now also pinned, not free — correcting the
withdrawn D0's listing of it as moving.

