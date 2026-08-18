---
id: RT-PLANNER-EFFECTS-SPLIT
title: "Move the effects domain out of planning/static_transition.rs -- effect seats, seat groups and effect-seat closures on the planner side"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-PLANNER-AGGREGATES-SPLIT]
blocks: [RT-PLANNER-JOINS-TRAPS-SPLIT]
github: null
origin: "Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 8; boundary and companion-test-axis ruling evt_6r403ez3m2m69 (2026-08-18). Framed by the Steward on the operator's 2026-08-18 directive making RT-BACKEND-MODULE-SPLIT the runtime lane's priority. Binding inputs are the five Stage A inventories from RT-BACKEND-SPLIT-CENSUS (merged 8ebc2467d). Steward-filed per COORDINATION section 2."
---

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

`lowering/core/tests/control.rs` is **33,969 lines and in scope** — the
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

- **`AC-1`** — the old/new **symbol ledger** is exact and complete for this
  owner, and every entry names its old path and its new path. **Control:** the
  ledger's symbol count reconciles against a fresh selector run at the stated
  SHA, and the selector and its blind spots are both recorded.
- **`AC-2`** — the **test-property ledger** is exact, and every moved test names
  the property it discriminates. **Control:** each moved mutation reds **the same
  reached property**, with the same **nonzero** denominator, and is restored. A
  mutation whose denominator is zero has not shown the test reaches anything.
- **`AC-3`** — **no representation, diagnostic, hash, serialization, behaviour or
  trust change**, and **no widened production API**. **Control:** the diff is
  reviewable as a relocation — a reviewer can pair every removed line with an
  added one, or the hunk is called out as not a move.
- **`AC-4`** — the affected library configuration **and** the targeted test
  configurations both compile. **Control:** scoped `scripts/ken-cargo` runs only;
  the workspace gate is **CI's**, never a local run.
- **`AC-5`** — this slice's own transfer is stated as complete, and **phase
  closure is explicitly NOT claimed.** A frame that reports the bound file's new
  line count as evidence the phase is done fails this criterion.

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

> ### THE PLANNER CHAIN IS CONTENTION-FREE WITH THE LANE'S LIVE SEMANTIC WORK
>
> **Measured 2026-08-18.** The runtime lane's live semantic nodes —
> [[RT-BRANCH-LOCAL-DECLARED-CALLABLE]] and the residuals behind it — contend on
> `lowering/core.rs`, **not** on `static_transition.rs`. So items 4 through 9b
> can run while that work is still in flight.
>
> **Items 10 onward cannot**, and that is stated in their own frames. Verify the
> claim at pickup rather than inheriting it; it is a statement about which nodes
> are live, and that changes.

**The planner slices contend with EACH OTHER** — they all edit one file — so
they are a strict chain, not a fan-out. That is what `depends_on` encodes here.

**Re-derive every symbol by name at pickup**, never by the line offsets in any
frame or census row. `static_transition.rs` is 34,883 lines and every slice
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

