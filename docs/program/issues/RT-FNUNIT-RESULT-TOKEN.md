---
id: RT-FNUNIT-RESULT-TOKEN
title: "Broad starter shapes fail the result-token table on the FunctionizedUnits lane — pre-existing, unmasked by retiring SeedClosureCall"
status: merged
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: https://github.com/swe-toolkit/ken/pull/1892
origin: Measured 2026-08-08 by the Runtime ring during RT-SEED-CALL-PORT D3, evidence SHA d6fb593b. Campaign docs/program/16-recursive-descent-retirement.md Trap 2 — a newly reachable shape tripping a fail-closed invariant is routed as its own node. Steward-filed (agents cannot create tracked work per COORDINATION §2).
---

## What it is

`nc22_cranelift_agrees_with_runtime_ir_report_for_broad_starter_shapes`
(`crates/ken-runtime/src/cranelift_backend/artifact/api/tests.rs:95`) fails on
the `FunctionizedUnits` lane with:

```
native result token 265 is not in the result table
```

raised at `cranelift_backend/surface.rs:251` / `:315`.

## The attribution is measured, not argued

**This is NOT caused by the seed-closure port.** The Runtime implementer flipped
`nc22`'s callee from `RuntimeExpr::Closure` to `RuntimeExpr::LexicalClosure` —
the sibling arm live since [[RT-DECL-CLOSURE-PORT]], which
[[RT-SEED-CALL-PORT]]'s `D2` and `D3` never touched — and reproduced the
**identical** error.

⇒ **The shape was already unsupported on the functionized lane.**
`SeedClosureCall` was masking it: while the residual fired, the program routed
to the `RecursiveDescent` root and never reached the failing path. Retiring the
variant made it newly reachable.

**Discounted evidence, recorded so nobody re-counts it:** an earlier smaller
record-returning probe failed on both arms with a *different* error
(`BoundaryCarrier` unsupported). The implementer explicitly declined to
attribute this stop to it. Only the `nc22` callee flip attributes.

## Why it is its own node

Campaign Trap 2, verbatim: a newly reachable shape tripping a fail-closed
invariant is **expected** as classes retire, and is routed as its own node
rather than absorbed into the retiring node or worked around by adjusting the
lane.

## MERGED 2026-08-11 — PR #1892, exact `dbbf782f`, `origin/main` `8111fa06`

Four paths, `+804/-5` from declared base `cc56e460`. Every deliverable and every
acceptance criterion discharged; the frame records each.

**The quarantine is lifted.** `nc22` was skipped so [[RT-SEED-CALL-PORT]] `D3`
could land (Steward ruling, 2026-08-08). It is now **un-skipped and green on
`FunctionizedUnits`**, asserting the exact
`Record { ok: Bool(true), value: Int(7) }` — seen to fail first, under
`--ignored` at `D2` on this same node. The row runs; the skip was not tidied.

**`RT-DESCENT-RETIRE` is no longer blocked by this.** [[RT-DESCENT-RETIRE]]
deletes the `RecursiveDescent` emission lane, and any shape supported only there
would have stopped being supported at all. This shape now works on the
functionized lane, so deleting that lane no longer narrows what Ken can compile
through it.

## How the three opening questions came out

- **"What is result token 265, and why is it absent from the table?"** It was
  never absent from a table. `265` is a **well-formed `InvocationAggregate` tag
  (Record)** that reached the `Boundary` decoder's `_ =>` arm. There was no arm
  to receive it, and the table it was reported against was **empty**.
- **STRUCK — "is the gap the token's production or its registration?"** Neither.
  The tag is produced correctly and registered correctly. **The
  production/registration pair was the wrong fork to offer**, and offering it
  pointed the first hour at the two places the defect was not.
- **"How wide is the shape class?"** The enumerable family turned out to be
  **`Boundary` tags, not program shapes**. Record and bare constructor share one
  tag cell, Bool was already handled, and six tags remain deliberately
  unhandled. **No fixture family was authorable or needed, and `M` held.**

## What replaced the wildcard, which is the durable part

The ruling asked for one `InvocationAggregate` arm and for the `_ =>` refusal to
be **retained**. The implementer deleted the wildcard instead, spelling out
`None` plus all six unhandled tags. ⇒ **A future `BoundaryTag` is a compile
error at this decoder** rather than silently inheriting the wildcard's policy,
and widening the new arm requires deleting a named tag by hand.

## Carried out of this node

[[RT-GROUNDVALUE-RECURSIVE-DROP]] — filed as a draft. `RuntimeGroundValue` is
itself recursive, so a deep aggregate that *decodes successfully* overflows in
its own `drop`. The ruling's "deep valid data uses no recursive host stack"
holds for the traversal and cannot hold end to end. **The depth at which it
happens is unmeasured**, and that node's first work is the bisect.

## THE SIZING QUESTION IS SCOPING, NOT MEASUREMENT

**Measured by the Adversary on the closing merge `91435b89`:**

1. **`nc22` is a single composite program, not a loop over shapes.** Its body is
   one nested `Let` / `Call{callee: Closure}` / `Match` / `Construct` / `Record`
   / `If` tree — "broad starter shapes" names breadth *within* one program. The
   skip suppresses **one row**, not a corpus.
2. **STRUCK — *"`nc22` is the ONLY one of 21 `nc` fixtures carrying a `Call`
   whose callee is a `Closure` or `LexicalClosure`. It is alone."*** Falsified
   by `D1` at `be54d47f`, 2026-08-11.

   `nc5_seed_examples` carries `closure-capture-application` with `callee:
   RuntimeExpr::Closure` at `crates/ken-runtime/src/ir.rs:1084` — a `pub fn`,
   **production, not a test fixture.**

   **The defect was scope, not staleness.** The claim held *within*
   `cranelift_backend/artifact/api/tests.rs`, which does contain exactly one
   `callee: Closure`, at `:106`. **One file was measured and "the corpus" was
   written.** Everything below followed from that width.

⇒ **STRUCK — the family-width question is NOT unestablishable.** Two instances
of the closure-call shape exist, and comparing them establishes the axis with
nothing authored.

⇒ **STRUCK — the corpus does NOT have zero live coverage of that shape.** `nc5`
is green on `FunctionizedUnits` under two existing committed controls. **This
one is struck conditionally:** `D1`'s census observes IR only, so whether `nc5`
reaches native emission is not yet established. The frame's `AC-1a` owns that,
and if the answer is no, this claim becomes substantially true again.

## What `D1` established instead — the axis is RETURN SHAPE

| fixture | returns | on `FunctionizedUnits` |
|---|---|---|
| `nc5` closure-capture-application | `Int 7` | green |
| `nc22` | `Record { ok: Bool(true), value: Int(7) }` | fails, `NativeResultDecode` token `265` |

**The closure call was never the wall.** `nc22` remains live and still
reproduces the failure.

**`M` stands and no re-cut is owed** (Steward ruling 2026-08-11,
`evt_6y341v6jsqwe3`). The node's structure is unchanged: `D2` identifies
`nc22`'s actual `ResultDecoder` arm, and only then does `AC-2` decide whether
the uninstantiated Bool / bare-constructor / Boundary cells need authored
fixtures or a report.

**`D1` worked as designed.** It was written as a gate that could resize the
node and it returned "the sizing survives, for a reason nobody had" — which is
a better outcome than agreement, and the reason the deliverable was a gate.

## Post-merge adversary pass — what it covered, and what it did NOT

**Structural pass at `evt_6tfhakf962cer`, measured on `76dd2022`. No finding.**
Recorded here rather than left in the thread, because **a merge with a clean
adversary pass reads as swept unless the unswept surface is written down.**

**What it closed.** The Steward's notified gap was whether every path into the
new decoder reaches it through a sealed store. `decode_invocation_ground` has
**exactly one production caller** — `compiled.rs:213`, with
`.finish(&mut store, None)` immediately above it — and twenty test call sites,
all inside the `#[cfg(test)]` module. ⇒ The enumeration closes at the **caller
count**, not at an argument, which is stronger than checking the argument at
each site.

**The residual that was considered and deliberately not filed.** The seal is a
**caller-side invariant**: the decoder's signature does not require a sealed
store, `adopt` refuses one at runtime. A future second production caller could
therefore omit it. Not filed, on two grounds — it is findable in **one
permanent grep** while the production caller count stays at one, and it **fails
loudly** (`BOUNDARY_ERR_SEALED`, which is how the ring's own control found the
ordering) rather than adopting silently. **Fail-closed in the direction that
matters.**

**UNHUNTED, and that is not the same as clear.** The pass executed nothing and
read only the ordering at the single production site. It did **not** examine:

- the iterative traversal;
- the grey/black cycle handling;
- the node-admission predicate;
- the identity resolution through `carrier_symbol`;
- whether the written-out arms actually exhaust `BoundaryTag`'s variants — the
  claim that a new tag is now a compile error here was taken from the handback,
  **not verified by counting arms against the enum.**

⇒ **Those five are the merge's substance.** They were gated by QA and the
Architect, which is the real assurance; the adversary pass adds nothing to them
and must not be read as if it did.
