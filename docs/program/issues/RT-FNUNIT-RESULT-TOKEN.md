---
id: RT-FNUNIT-RESULT-TOKEN
title: "Broad starter shapes fail the result-token table on the FunctionizedUnits lane — pre-existing, unmasked by retiring SeedClosureCall"
status: active
owner: runtime
size: M
gate: none
depends_on: []
blocks: [RT-DESCENT-RETIRE]
github: null
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

## It owns the quarantine

`nc22` is **skipped** so [[RT-SEED-CALL-PORT]] `D3` can land (Steward ruling,
2026-08-08, under the operator's 2026-08-06 CI-gate policy). **This node owns
un-skipping it.** A skipped row measures nothing, so closing this node means
`nc22` runs green on the functionized lane, not that the skip is tidied.

## Why it blocks `RT-DESCENT-RETIRE`

[[RT-DESCENT-RETIRE]] **deletes the `RecursiveDescent` emission lane.** Any
shape supported only there stops being supported at all. This shape is
currently supported only there, so it must work on the functionized lane before
that lane is deleted — otherwise the retirement silently narrows what Ken can
compile.

## First questions for whoever picks this up

- **What is result token 265, and why is it absent from the table?** Start at
  `surface.rs:251`/`:315` and work back to who populates the table on the
  functionized path.
- **Is the gap the token's production or its registration?** Those route
  differently.
- **How wide is the shape class?** `nc22` is one fixture. **The `M` was set
  without that answer, and the answer cannot come from reading this corpus** —
  see below.

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
