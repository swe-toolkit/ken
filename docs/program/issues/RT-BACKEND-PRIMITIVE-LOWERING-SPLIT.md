---
id: RT-BACKEND-PRIMITIVE-LOWERING-SPLIT
title: "Move the primitive-lowering family to its own module — the first production slice of the backend split, and the architectural release point for NATIVE-HANDLE-CARRIER"
status: draft
owner: runtime
size: M
gate: none
depends_on: [RT-BACKEND-SPLIT-CENSUS]
blocks: [NATIVE-HANDLE-CARRIER]
github: null
origin: Architect ruling evt_54zvaqbrm752x (2026-08-10) §5, answering the campaign's standing question of whether an early subset of #8 releases NATIVE-HANDLE-CARRIER. Answer yes, on a bounded ownership proof. Enclave pass anchored at evt_104nz8cedzyat on operator instruction 2026-08-10. Steward-filed per COORDINATION §2.
---

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

**`draft` until the census returns.** Not framing debt: the frame above is
shovel-ready and the ownership proof is measured. What is missing is `D6`'s
verdict, which can re-cut this node — and per the node's own Shape section,
*"present measurement is enough to choose and frame it; the census supplies the
binding paths and counts."* **Flip to `ready` when the census merges with a
clean `D6`.**
