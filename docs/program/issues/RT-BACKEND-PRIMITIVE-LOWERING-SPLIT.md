---
id: RT-BACKEND-PRIMITIVE-LOWERING-SPLIT
title: "Move the primitive-lowering family to its own module — the first production slice of the backend split, and the architectural release point for NATIVE-HANDLE-CARRIER"
status: merged
owner: runtime
size: M
gate: none
depends_on: [RT-BACKEND-SPLIT-CENSUS]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: Architect ruling evt_54zvaqbrm752x (2026-08-10) §5, answering the campaign's standing question of whether an early subset of #8 releases NATIVE-HANDLE-CARRIER. Answer yes, on a bounded ownership proof. Enclave pass anchored at evt_104nz8cedzyat on operator instruction 2026-08-10. Steward-filed per COORDINATION §2.
---

> # MERGED 2026-08-17 — the complete `D0`-`D4` move. 19 DEPENDENTS RELEASED.
>
> **Landed squash `7b05136bda5e586649dd03331888321f110fbfb4`, PR #2545**, from
> reviewed candidate exact `23601bdc98d619ff6a5a602f5e2b00a06f76669f`; range
> `3001cd431d1b84ddfc3b3d9e59120d161ba59773...23601bdc9`. One non-merge commit,
> six `crates/ken-runtime` paths, `+882/-844`. Decision `dec_h838ht881t7y`
> resolved APPROVED; QA `evt_1yem11kv1gc4v`. All six paths verified by blob
> identity from the declared merge-base.
>
> **`AC-6` WAS DISCHARGED BY A COMPLETED MOVE, NOT BY THIS MERGE EVENT.** The
> node was deliberately held `active` through review so that this flip could
> assert the move is whole rather than let a merge event stand in for the
> property. It is whole: `lower_primitive_call` moved with its entire
> responsibility, all twelve helpers and `lowered_char_list` came with it, and
> the domain test family moved with its subject. **Nineteen transitive dependents
> are released by this flip**, so the distinction was the whole reason for the
> criterion.
>
> **`AC-1` is the criterion that earned its cost, and the diff is why.**
> `+882/-844` on a pure move reads as deleted behaviour — `values.rs` became
> `primitive/tests.rs` at 98% similarity alongside a new production child — and
> **no reading of the diff can separate a faithful move from a rewrite.** Three
> independent instruments did:
>
> 1. the ring extracted every item's body at base and candidate via
>    `rust-analyzer symbols` node ranges, normalized whitespace and the one
>    forced visibility change, and compared: **14/14 production, 13/13
>    test/helper exact**;
> 2. QA reproduced that by its own independent extraction;
> 3. the Architect used two instruments repeating neither — a whitespace-
>    normalized **line-multiset** comparison (825 removed, 834 added) finding
>    **exactly one line that left the tree without returning**, `fn
>    lower_primitive_call(`, which returns as `pub(super) fn`, the other nine new
>    lines being scaffolding; and then, **because a multiset check is order-blind
>    and cannot see a statement moved between two function bodies**, a
>    per-function pass closing that gap at 14/14 identical.
>
> **`AC-3` came in at exactly one non-private item.** The child exposes only
> `pub(super) lower_primitive_call`, forced by the parent's `PrimitiveCall` arm;
> `core.rs` gains only `mod primitive;`. The test-only `core::tests::big` seam is
> forced jointly by the moved `primitive::tests` and the retained
> `core::tests::effects`.
>
> **`expect_two_args` stayed parent-owned, and that was a choice.** `D6`
> permitted it either way; leaving it puts the boundary exactly at what was
> warranted rather than one step past it.
>
> **`AC-2` verified at the mechanism:** zero `int_to_uint64_raw` anywhere under
> `cranelift_backend`, and zero occurrences in the diff in either direction. The
> arm is [[NATIVE-HANDLE-CARRIER]]'s and now has a durable home to land in —
> `lowering/core/primitive.rs`, with the dispatcher at `:43`.
>
> ### THE ADVERSARY CLOSED THE HOLE I FLAGGED, AND ITS ARGUMENT IS BETTER THAN MINE
>
> **Hunt `evt_32xzh27atv6ee`, no finding.** In the M8 notification I named a
> shared blind spot — **all three fidelity instruments normalize whitespace and
> the one forced visibility change before comparing, so three agreements on the
> same normalization are one chance to catch what it hides, not three** — and
> said I had no candidate for what could hide there. The Adversary supplied the
> shape and then measured it.
>
> **The blind spot has a name: path-relative resolution.** Normalization
> preserves text, so what survives it is **text that means something different
> in the new location.** For a module move that is dominated by two things, and
> both are measurable:
>
> | axis | measured |
> |---|---|
> | `#[cfg]` context | 2 attrs in `primitive.rs`, 1 removed from parents — `#[cfg(test)] mod tests;` at `:8` is new scaffolding for the renamed test file, `#[cfg(test)]` at `:306` is the one that moved. **1 moved + 1 new = 2; no moved code changed compilation context.** |
> | path-relative refs | `crate::` 11, `super::` 1, `self::` 0. **Eleven of twelve are absolute, so their meaning cannot depend on where the code lives**; the twelfth is the conventional `use super::*` a split-out child uses. |
>
> ⇒ **This is a structural argument rather than a comparison, and that makes it
> stronger than the three instruments.** They establish that the text did not
> change. **This establishes that the text's meaning could not have.**
>
> **It is nearly empty, not empty, and the residual is stated rather than
> waved off.** `use super::*` now binds against `core` where this code
> previously sat *inside* `core`, so a name reachable in both `core` and
> `lowering` could in principle bind differently. **Rust errors on ambiguous
> glob resolution at the use site**, so a silent rebind needs a name present in
> exactly one of them at each site — which is why the verdict is *nearly*
> empty.
>
> **A second note worth keeping, because it changes how the zero row may be
> edited.** The new module's all-zero census row is a live sentinel: a later
> artifact-level definition in `primitive.rs` makes some count `1 ≠ 0` and reds.
> **Its correctness rests on those counters detecting that pattern at all, which
> only the roster's non-zero rows establish** — so the zero row is meaningful
> *because* the roster has non-zero rows, never on its own.

> **One should-fix carried, not respun.** The child's module doc says it owns
> "symbol dispatch, and **emission**", while the census row added in this same
> candidate asserts the move creates "no second **emission authority**". Both are
> true in different senses — 30 `.ins()` instruction-emission sites, zero
> artifact-level declare/define sites — **but that reconciliation is written
> nowhere in the candidate**, and a reader resolving the conflict could conclude
> the census row is wrong and weaken or delete it. Qualifying the doc as
> *instruction* emission forecloses that.
>
> **Fix it in the DOC, not the row — the direction matters and the Adversary's
> hunt is why.** The row is the live sentinel described above. **A reader who
> resolves the conflict by deleting the row removes the detector**, and the
> census then reads complete while `primitive.rs` sits unmeasured. The doc
> sentence has no such load.

> # THIS NODE IS WHY 19 NODES DO NOT WAIT FOR THE WHOLE PHASE
>
> [[NATIVE-HANDLE-CARRIER]] is the head of the entire remaining Linux ABI
> completion program — `NATIVE-HANDLE-CARRIER` → [[PX8-F-CAP-41]] → `PX8` →
> {`ABI-R3`, `PX9`} → Tracks A/M/S/T, **19 transitive dependents.** Its other
> three dependencies are merged, so campaign node #8 alone held it.
>
> **The Architect ruled that a clean early subset exists, and this is it.** The
> merge of this node — cut item 2 of 18, immediately after the census — is the
> architectural release point. The ABI program no longer waits on the other 16
> slices.

## The ownership proof this was chosen on

Measured on current `main`, to be revalidated by [[RT-BACKEND-SPLIT-CENSUS]] on
the post-#7 SHA:

**Exclusively primitive, and therefore moves:**

- one dispatcher, `lower_primitive_call`, at `lowering/core.rs:17977-18208`;
- twelve exclusively primitive helper methods in the contiguous
  `lowering/mod.rs:18470-19032` family;
- the exclusive recursive `lowered_char_list`;
- the primitive/value subject tests, which form a domain property family.

**Shared, and therefore stays with its existing owner:** source-child lookup,
`lower_expr`, `specialized_operands_at`, carrier projection, and dynamic
small-`Int` lowering all have non-primitive consumers. `PX8-F-CAP-41`'s
six-axis matrix stays end-to-end evidence.

> **`lower_primitive_call` is not merely an emitter, and the frame must not
> treat it as one** (Architect amendment). It owns argument evaluation,
> partiality, carried projection, symbol/arity/representation checks, dispatch,
> and identity conversions **that emit nothing**. Its structural home is a
> `primitive` lowering family **nested under the current core/source-machine
> owner** — not a premature typed command interpreter.

## Shape

Initial home a child such as `lowering/core/primitive.rs`, calling the existing
source-machine evaluator and shared value services through narrow `pub(super)`
seams. **That states today's dependency honestly** rather than minting a second
evaluator or widening planner state to avoid admitting it.

The move is byte-for-behaviour: dispatcher, exclusively owned helpers, and the
domain tests and mutations.

## What this node must NOT do

⛔ **It must not add `int_to_uint64_raw`.** That arm is
[[NATIVE-HANDLE-CARRIER]]'s, and it lands **after** this move, once, in the
durable home. A slice that carries the arm would be a pure move plus a semantic
change, which every structural frame in this phase forbids.

⛔ No change to supported symbols or partiality. No splitting rule selection
from emission. No widened production API, no facade recreation.

## The fail-closed gate ahead of it

[[RT-BACKEND-SPLIT-CENSUS]] revalidates this call graph on the post-#7 tree.
**If #7 created a new shared owner or a cycle, this node stops with the exact
contradiction** rather than widening silently. Present measurement is enough to
choose and frame it; the census supplies the binding paths and counts.

## Deliverables

**`D0` — read the census's `D6` verdict FIRST, and cite it.**
[[RT-BACKEND-SPLIT-CENSUS]] revalidates the ownership proof this node was
chosen on. **If `D6` reported a
new shared owner or a cycle, this node stops with that contradiction and the
re-cut is the Steward's** — do not widen the slice to absorb it. If `D6` is
clean, quote the SHA it was measured at; that SHA is this node's base.

**`D1` — create the child and move the exclusively-primitive family.** The
dispatcher `lower_primitive_call`, the twelve contiguous helpers, and the
exclusive recursive `lowered_char_list`, into `lowering/core/primitive.rs` (or
the child the census's inventory 3 shows is correct). **Bodies move unchanged.**

**`D2` — move the domain tests and mutations with the code they cover.** They
are a domain property family, not a file's worth of tests; a move that leaves
them behind separates the property from its subject.

**`D3` — enumerate the seams the move requires**, each as narrow `pub(super)`,
each with the consumer that forced it. The move calls the existing
source-machine evaluator and shared value services; **stating that dependency
honestly is the deliverable**, not routing around it.

**`D4` — the move-fidelity evidence.** See `AC-1`: this is what makes the
candidate reviewable as a move rather than as a rewrite.

## Acceptance criteria

- **`AC-1` — fidelity is DEMONSTRATED, not asserted.** For every moved item,
  show the body is unchanged apart from the mechanical adjustments the move
  forces — module path, `use` lines, indentation, visibility. **A reviewer must
  be able to check this without reading the logic**, so produce the normalized
  comparison rather than a claim that one would pass. **"The suite is green" is
  not evidence of a faithful move**: a move that also changes behaviour is
  exactly as green as one that does not, wherever the tests do not discriminate.

- **`AC-2` — `int_to_uint64_raw` does not appear.** That arm is
  [[NATIVE-HANDLE-CARRIER]]'s and lands after this move, once, in the durable
  home. A slice carrying it is a move plus a semantic change.

- **`AC-3` — no widened production API.** Every new `pub(super)` is listed with
  its forcing consumer. No facade recreation, no second evaluator, no widened
  planner state. **A seam that exists to avoid admitting a dependency is the
  failure this node's Shape section is written against.**

- **`AC-4` — supported symbols and partiality are unchanged**, and rule
  selection is not split from emission. `lower_primitive_call` owns argument
  evaluation, partiality, carried projection, symbol/arity/representation
  checks, dispatch, and identity conversions that emit nothing. **It moves as
  that whole thing** — decomposing it into a typed command interpreter is a
  different node and is not authorized here.

- **`AC-5` — no-regression, in CI** (`COORDINATION §12`).

- **`AC-6` — THE RELEASE CONDITION IS THE COMPLETED MOVE, NOT THIS NODE'S
  MERGE EVENT.** Read this before proposing any partial.

  > **[[NATIVE-HANDLE-CARRIER]]'s `depends_on` names this node, and
  > `gen-progress.sh` keys eligibility on `status: merged`.** Nineteen
  > transitive dependents sit behind it — the whole remaining Linux ABI
  > completion program.
  >
  > **The accepted-partial policy means a partial WP merges.** So a partial
  > that flips this node to `merged` releases all nineteen **on an incomplete
  > move**, and nothing reds to say so: the tracker cannot distinguish "merged
  > because done" from "merged because a partial landed."
  >
  > ⇒ **Partials may land, but this node stays `active` until `D1`-`D3` are
  > complete.** The flip to `merged` is a separate, deliberate act that asserts
  > the move is whole. **A release condition keyed on a merge event fires the
  > moment the first accepted partial lands, which is not when the property it
  > gates becomes true.**

## Sequencing

Campaign node #8, cut item 2 — the first production slice of the phase, and the
only one that is not a planner domain. The phase record and the full 18-item cut
are in [[RT-BACKEND-MODULE-SPLIT]].

**`ready` as of 2026-08-17. The census merged at squash `8ebc2467d` (PR #2541)
and its `D6` returned the positive verdict**, which was this node's only bar.

**`D0` is already half-answered, and the answer is in the tree rather than a
thread.** `D6` revalidated the ownership proof this node was chosen on:
`lower_primitive_call` has one definition and one caller, every call site of
the twelve selected helpers lies inside that dispatcher, `lowered_char_list` is
definition plus one self-recursive call plus the dispatcher, and
`expect_two_args` is an acyclic shared arity seam confined to selected methods
rather than a second lowering owner. **No new shared owner and no cycle** —
so `D0` cites [[RT-BACKEND-SPLIT-CENSUS]]'s record and the measurement SHA
pinned in it, and does not re-derive the graph.

**The five inventories are the binding reference for `AC-3`'s seam enumeration
and `AC-1`'s fidelity comparison.** `backend-split-census-type-ownership.md`
gives the visibility partition, `backend-split-census-reexports.md` gives the
re-export surface **per build profile** — 29 default library and test, 4
named-feature only, 2 test-default, 22 test only — and
`backend-split-census-tests.md` names the 716 tests and 127 mutation surfaces
that `D2` moves with their subject.
