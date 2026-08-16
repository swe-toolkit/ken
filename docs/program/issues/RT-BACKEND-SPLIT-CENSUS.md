---
id: RT-BACKEND-SPLIT-CENSUS
title: "Stage A of the backend module split — five inventories over the post-retirement tree, before any code moves"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-DESCENT-RETIRE, RT-CENSUS-CAVEAT-GUARD, RT-CALL-EDGE-EXECUTABILITY-AXIS, RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]
blocks: [RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]
github: null
origin: Architect ruling evt_54zvaqbrm752x (2026-08-10) decomposing RT-BACKEND-MODULE-SPLIT into independently mergeable slices, cut item 1. Enclave pass anchored at evt_104nz8cedzyat on operator instruction 2026-08-10. Stage A is research/compiler-refactoring-program.md §5.1. Steward-filed per COORDINATION §2.
---

> # DRAFT UNTIL [[RT-DESCENT-RETIRE]] MERGES. THAT IS THE POINT OF IT.
>
> This node **is** the post-retirement remeasure. It cannot be framed against a
> tree the capstone is about to change, and it is the reason every other #8
> child stays `draft` too: **the census supplies their binding paths, counts and
> sizes.**

> # OPERATOR RULING 2026-08-16 — THREE CONTROL FIXES LAND BEFORE THIS CENSUS
>
> **`depends_on` now names three campaign spinouts**, and the edge is on
> `depends_on` rather than their `blocks` because `scripts/gen-progress.sh`
> reads only `depends_on`; a `blocks` edge alone is invisible to every generated
> view.
>
> | node | size | region |
> |---|---|---|
> | [[RT-CENSUS-CAVEAT-GUARD]] | S | `lowering/core/tests/control.rs` |
> | [[RT-CALL-EDGE-EXECUTABILITY-AXIS]] | S | `planning/static_transition.rs` |
> | [[RT-SRCMACHINE-DISPATCH-REACHABILITY-CONTROL]] | S | `control.rs`, `core.rs`, `mod.rs` |
>
> **The discriminator was file contention, not tidiness.** All three are
> semantic edits inside this node's own scope
> (`crates/ken-runtime/src/cranelift_backend/` plus `boundary_value_clif.rs`),
> and a split cannot run concurrently with semantic work on the files it
> partitions — campaign §4 ground 3. Landing them first means one rebase rather
> than a re-home followed by a fix.
>
> **`RT-CENSUS-CAVEAT-GUARD` has the stronger reason, and it is specific to this
> node.** Inventory 4 is a **test-property ledger that becomes binding on all
> seventeen later slices.** That node's defect is a staleness guard which cannot
> detect the drift it was written to catch. **Census it as-is and the ledger
> records a broken guard as the expected property**, after which every slice is
> checked against a wrong expected value. A faithful census can freeze a design
> defect; that is what this edge prevents.
>
> **Two other campaign spinouts were considered and deliberately NOT sequenced
> here** — [[RT-GROUNDVALUE-RECURSIVE-DROP]] and
> [[RT-FRONTEND-REACHABILITY-TRIPWIRE]]. Neither lives in this scope: the first
> is `RuntimeGroundValue` decode/drop, the second is a grammar and elaborator
> instrument by construction. **Gating the phase on them would hold
> [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] and the nineteen transitive ABI
> dependents behind it for work that does not contend.** Do not add them.
>
> **Sequencing within lane 1**, Steward's call under the ruling: the capstone
> first (it makes the largest change to these same files), then the three fixes,
> then this census. Putting them ahead of the capstone would have the deletion
> re-open work already done.

## What it is

The first work package of campaign node #8, and the only one that moves no
code. It produces five inventories over the post-`RT-DESCENT-RETIRE` tree:

1. **Type ownership** — every public-in-backend type, its minting module, and
   its consumers.
2. **Lifecycle, evidence and closeout** — each authority and the exact
   lifecycle it governs.
3. **Re-export surface** — every existing path and visibility class, in **both**
   the library and test builds.
4. **Test property** — each test, fixture, mutation, counter, denominator and
   source oracle, with its property class and production injection point.
5. **Co-change baseline** — the post-retirement version of the four-file churn
   matrix.

**The census is the refactor's plan.** A directory sketch without these
inventories is insufficient, because it cannot show which semantic edges a move
must preserve.

## Why it is a node and not a preamble

Every later slice's acceptance rests on a ledger this produces — the exact
old/new symbol ledger and the test-property ledger are binding on every
structural frame (Architect `evt_54zvaqbrm752x` §3). A slice that carries its
own census would be asserting the map it is being checked against.

It also holds one **fail-closed verification gate**: it revalidates the
primitive-lowering call graph that [[RT-BACKEND-PRIMITIVE-LOWERING-SPLIT]] was
chosen on. If #7 created a new shared owner or a cycle there, this node **stops
with the exact contradiction** rather than widening the slice.

## Scope

`crates/ken-runtime/src/cranelift_backend/` and `boundary_value_clif.rs`.

⛔ **No code movement, no renames, no visibility changes, no test relocation.**
An inventory only. Source text is a census aid, not the only semantic oracle.

## Sequencing

Campaign node #8, cut item 1 — first of the phase, gating every other child.
The phase record and the full 18-item cut are in
[[RT-BACKEND-MODULE-SPLIT]].
