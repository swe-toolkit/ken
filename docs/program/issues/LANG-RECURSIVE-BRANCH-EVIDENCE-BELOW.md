---
id: LANG-RECURSIVE-BRANCH-EVIDENCE-BELOW
title: "A constructor-branch equation on a recursive family has no evidence term, because Ken's one recursor shares its motive with the induction hypothesis. Carry it on a case-only Elim convoy and compile structural recursion through generated below and brecOn"
status: draft
owner: language
size: L
tier: T1
gate: architect
depends_on: [LANG-REFINEMENT-SUBSET-SIGMA]
blocks: []
github: null
origin: "Architect recut evt_7729d1mezkcxk of LANG-PATH-CONDITION-EVIDENCE (W2b), on Research advisory evt_2pjgy01xhz4db after the third advancing stop evt_35tcy3w6b3e4f. Operator 2026-10-07: refinements are real kernel types. Steward-filed per COORDINATION section 2."
---

# Recursive branches carry their equation

## Objective

Inside the `C r` branch of a match on a recursive family, the branch
equation `Eq S s (C r)` is supplied when a subset-Σ introduction
beneath it demands it (`LANG-REFINEMENT-SUBSET-SIGMA`, Architect
`evt_41fxwysr833vv`), without changing any induction hypothesis.

## Settled inputs

- Ken's `Elim` is one recursor. Kernel `method_type` forms each induction
  hypothesis by applying the motive to the recursive field
  (`ken-kernel/src/inductive.rs` ~:2149, fields from `recursive_shapes`
  ~:1384). A convoy motive `λ y. Eq S s y → T` therefore turns every
  hypothesis into `Eq S s r → T`, which the branch cannot supply
  (`evt_6vjfqzyn2bx7s`). Dropping the equation loses discharges
  (`evt_70rp9sn0axc7j`).
- **Design (Architect `evt_7729d1mezkcxk`, after the advisory).** As in
  Goguen, McBride and McKinna, and in Lean 4 and Coq Equations: the branch
  equation rides on a case-only use of `Elim`, whose hypotheses are unused,
  and recursion goes through `below_D` and `brecOn_D`, derived from the
  same recursor with the plain motive `T`, so `below (C r)` reduces to the
  result for `r` paired with `below r`. No kernel former, no trust change.
- Lean 4 compiles a recursive call by projecting from `below`; the
  advisory cites that lookup. It is prior art to understand, not code to
  copy.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

- For each recursive family, elaborator-generated, kernel-checked
  `below_D` and `brecOn_D` through `Elim` into `Type`.
- Structural recursion compiled by projecting from `below` instead of
  from the method's hypothesis, including in `compiler_driver`'s
  computational-hypothesis lowering.
- A recursive-family match with a demanding introduction beneath it
  becomes a case-only `Elim` convoy carrying the equation and the `below`
  dictionary. Without demand the match elaborates as on base.

## Acceptance

- **AC-0 (D0).** Probe that `below_D` for `Nat` and `List` checks through
  `Elim` into `Type` and reduces on a constructor (large elimination, spec
  14 section 3), then the Architect rules the cut.
- **AC-1.** An introduction in `pce_rec`'s `Zero` arm checks with the
  equation supplied, and an IH-using recursive method computes the same
  result.
- **AC-2.** Every landed package's emitted core is byte-identical to
  base, and catalog and corpus obligation outcomes are identical.
- **AC-3.** Native and reference runtime parity unchanged across the
  recursion corpus.
- **AC-4 (mutation).** Projecting the wrong `below` entry reddens AC-1 or
  AC-3.

## Stop conditions

- `below_D` cannot be defined through `Elim`, or needs a kernel former or
  trust change.
- Any AC-2 outcome change or AC-3 parity change.
