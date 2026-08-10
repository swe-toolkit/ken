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

## Sequencing

Campaign node #8, cut item 2 — the first production slice of the phase, and the
only one that is not a planner domain. The phase record and the full 18-item cut
are in [[RT-BACKEND-MODULE-SPLIT]].
