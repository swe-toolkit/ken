---
id: KERNEL-LEQ-INT-LITERAL-REDUCTION
title: "The kernel cannot compute leq_int on two Int literals, so a closed refinement obligation such as isScalar 55295 or PosInt 5 has no proof term. Extend ADR 0013 Layer 2 to leq_int only: leq_int (IntLit m) (IntLit n) reduces to True or False by the same BigInt comparison the interpreter runs"
status: draft
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
github: null
origin: "Architect recut evt_5qg2098zmhd20 on hard stop evt_4g6d57z5x6wqz (LANG-REFINEMENT-INTRODUCTION-OBLIGATION, §1b entry 1). A kernel TCB extension: held at draft for the operator. Steward-filed per COORDINATION section 2."
---

# Kernel decides leq_int on literals

## Objective

In kernel whnf, `leq_int (IntLit m) (IntLit n)` reduces to `True` when
`m <= n` and to `False` otherwise. It stays neutral when either operand is
not a literal after whnf. Closed refinement goals over `leq_int` then reduce
to `Equal Bool True True` or `Equal Bool False True`.

## Settled inputs (Architect `evt_5qg2098zmhd20`, read at `032bc7b75`)

- **Today `leq_int` is stuck.** It is declared with `PrimReduction::Op
  { symbol: "leq_int" }` (`numbers.rs:421`), which `env.rs:94` documents as
  awaiting its reduction (K3).
- **The precedent is ADR 0013 Layer 2.** The kernel's only literal
  computation is `obs.rs::eq_at_registered_literal` (`:110`), which reduces
  `Eq ty (IntLit m) (IntLit n)` to Top or Bottom. Its comparison is the
  same `num_bigint::BigInt` operator as the runtime decider, pinned by
  `ken-interp/tests/ds6b_intlit_eq_reduction_cross_layer.rs`.
- **The runtime decider.** `ken-interp/src/eval.rs:1999`
  (`("leq_int", [a, b])` over `eval_to_bigint`).
- **The consumers it unblocks.**
  - `isScalar 55295` δ-unfolds to four `leq_int` literal tests and closes
    with `tt`. `55296` stays `Equal Bool False True`, which is open, as the
    seed-numbers row requires.
  - The guide's `PosInt` (`library/guide/surface-reference.ken.md:108`)
    closes `const five : PosInt = 5`.
- **Scope.** Only `leq_int`. The other Int ops stay K3.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- One kernel reduction arm for `leq_int` on two literals, using the same
  `BigInt` `<=` as the interpreter.
- An ADR 0013 amendment recording Layer 2's extension to `leq_int`.

## Acceptance

- **AC-1.**
  - `Equal Bool (leq_int 0 5) True` checks by `Proved`.
  - `isScalar 55295` closes.
  - A cross-layer test pins agreement between the kernel and
    `ken-interp` on a boundary set that includes negatives, equality and
    values beyond `i64`.
- **AC-2 (controls).**
  - `leq_int 1 0` does not convert to `True`. The rejection fence is
    asserted with `Proved`, not `Refl`.
  - `leq_int x 0` with `x` a variable stays neutral.
  - Removing the arm reddens the AC-1 rows.
  - `eq_int` and the other ops are unchanged.
- **AC-3.** These suites stay green:
  - `ds6b_intlit_eq_reduction`;
  - the ds6b cross-layer test;
  - kernel lib.

## Stop conditions

- Any trusted-base change beyond this one arm and its ADR amendment: stop
  to the Architect.
- A comparison that is not the interpreter's operator: stop to the
  Architect.
