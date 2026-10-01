---
id: KERNEL-LEQ-INT-LITERAL-REDUCTION
title: "The kernel cannot compute leq_int on two Int literals, so a closed refinement obligation such as PosInt 5 has no proof term. Extend ADR 0013 Layer 2 to leq_int only: leq_int (IntLit m) (IntLit n) reduces to True or False by the same BigInt comparison the interpreter runs"
status: active
owner: kernel
size: S
tier: T1
gate: architect
depends_on: []
blocks: [LANG-REFINEMENT-INTRODUCTION-OBLIGATION]
github: null
origin: "Architect recut evt_5qg2098zmhd20 on hard stop evt_4g6d57z5x6wqz (LANG-REFINEMENT-INTRODUCTION-OBLIGATION, §1b entry 1). A kernel TCB extension, approved by the operator 2026-10-01. Steward-filed per COORDINATION section 2."
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
  - `isScalar 55295` closes once `inRangeBool` is respelled by transparent
    Bool elimination, which `LANG-REFINEMENT-INTRODUCTION-OBLIGATION` carries.
    Today it composes the tests with `and_bool`/`or_bool`, also stuck Ops
    (Architect `evt_pawgvbeevyg2`, after stop `evt_n6w1rkyqkhw2`).
  - The guide's `PosInt` (`library/guide/surface-reference.ken.md:108`)
    closes `const five : PosInt = 5`.
- **Scope.** Only `leq_int`. The other Int ops stay K3.

Treat anchors as perishable. If a settled input is false on the landed base,
stop and report the mismatch.

## Deliverable

- One kernel reduction arm for `leq_int` on two literals, using the same
  `BigInt` `<=` as the interpreter.
- An ADR 0013 amendment recording Layer 2's extension to `leq_int`.
- A spec 16 §2.2 amendment. Its "Primitive type" paragraph says an `Op`
  application on literals stays neutral under conversion (K3-deferred), so
  this is a spec change and needs the Spec vote (Architect
  `evt_4r2mqaavb9gbh`).
- Companion corrections so the spec states both kernel-WHNF `Op` rules:
  this `leq_int` arm and the landed `string_to_list_char` view on a checked
  `String` literal (`conv.rs:216`, bfdbb9789). Every other registered `Op`
  stays K3-deferred. Wording only; no code, test or other rule change (CV
  `evt_7sa4v62w5xqkj`, `evt_44a12zma9rzdn`; Architect `evt_4j4109frj9gq6`).
  - `spec/10-kernel/17-conversion.md` §1 (a **prim** row, the `Op` bullet,
    the neutral list) and the `whnf` pseudocode, per the Architect's text.
  - `spec/10-kernel/18-judgments.md` §4.2 and §5, and
    `spec/10-kernel/18a-primitive-registry.md` introduction, §5 and
    §5.2.2(3): name the pair, never "sole", and keep the interpreter's
    tested-not-trusted arm separate from the kernel arm.
  - The general opacity sentences the Architect lists:
    `spec/10-kernel/14-inductive.md` item 5,
    `spec/30-surface/30-taxonomy.md:86`, `spec/30-surface/35-numbers.md`
    (`:15-16`, §6.1), `spec/30-surface/37-strings-collections.md` (`:109`,
    `:142`, `:154`, `:756`), `spec/30-surface/38-ffi-io.md:103-105` and
    `spec/40-runtime/42-evaluation.md:268-270`. Their examples
    stay; only the general claim is qualified.
- Conformance for both kernel-WHNF `Op` rules (CV `evt_27q51k32zhqs`):
  - in `conformance/kernel/conversion/seed-conversion.md`, one discriminating
    kernel-conversion case per rule. `leq_int` on two literals converts to the
    `Bool` its `BigInt <=` gives, with a false-side fence and a
    variable-operand neutral control. `string_to_list_char` on a checked
    `String` literal converts to its `List Char`, with a non-literal operand
    neutral control;
  - in `conformance/surface/numbers/seed-decimal-char-demote.md`,
    `char-extraction-computes-scalar-proof` no longer calls
    `string_to_list_char` a `Neutral` stub. It distinguishes the
    installed-IDs `apply` path from the direct `prim_reduce` fallback, as
    §42 now states. Its deferred runtime face is otherwise unchanged.

## Acceptance

- **AC-1.**
  - `Equal Bool (leq_int 0 5) True` checks by `Proved`.
  - `leq_int 55295 55295` reduces to `True`.
  - Kernel-local row: the closed term `match (leq_int 0 55295) { True |->
    leq_int 55295 55295 ; False |-> False }` whnfs to `True`. No prelude
    edit (`evt_pawgvbeevyg2`).
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
