---
id: RT-C5-PRIMITIVE-TYPE-ARGUMENTS
title: "Give closed primitive type constants a comparable interpreter value, so C5 cast regularity fires on applied types with Int, String or other primitive arguments instead of returning Unknown on a closed, hole-free program"
status: merged
owner: runtime
size: M
gate: architect
tier: T1
depends_on: []
blocks: []
github: null
origin: "Adversary advisory 2026-09-27 on M8 e41f7589e (evt_7wtyyhd1yw5m5), source mechanism confirmed by the Steward. Spec 40-runtime/42-evaluation.md §3.6 canonicity; the interpreter is the reference semantics (spec 00 §3). Operator 2026-09-27: sequenced in L1 right after RT-OWNER-VIS-RETURN-PROTOCOL, ahead of RT-IGNORED-ROWS-NEXT-GROUP (option b). Steward-filed per COORDINATION section 2."
---

# C5 on primitive type arguments

## Objective

A closed, hole-free, well-typed program whose cast crosses an applied type
with primitive arguments (`Vec Int n`, `Vec String n`) evaluates to a
value, as it already does for `Vec Nat n`.

## Settled inputs (measured on `e41f7589e`)

- **Primitive type constants evaluate to `Neutral`.** `eval.rs:1906`:
  `PrimReduction::OpaqueType => EvalVal::Neutral`, the same value as an open,
  stuck index.
- **`eq_type_eq` has no arm for them.** `eval.rs:1220-1242`: no
  `(Neutral, Neutral)` arm and no scalar pairs, so the match falls to
  `_ => false`. `cast_reduce` (`eval.rs:1155-1167`) then returns `Unknown`,
  even for a literal `refl`.
- **Repro** (Adversary, not re-run by the Steward): the un-ignored
  `two_vector_zip_recursive_step_convoy_fixture` with `Int` or `String`
  elements evaluates to `Unknown`; with `Nat` it evaluates to the expected
  `Ctor`.
- **A durable pin freezes the conflation.**
  `neutral_inductive_type_app_index_does_not_cast` (`eval.rs:1310`,
  "open indices fail closed"). The repair must keep open indices failing
  closed while letting closed primitive constants compare.
- Not a regression, and fail-closed: both cases were `Unknown` before
  `e41f7589e`.

## Deliverable

A value for closed primitive type constants that `eq_type_eq` compares by
identity, distinct from `Neutral`. Scalar index values (`Int`, `Bool`,
`Str`) compare too, if AC-0 shows a reachable family indexed by them.

## Acceptance

- **AC-0 (D0).** Enumerate every consumer of `EvalVal::Neutral` that a
  closed primitive type now reaches (Check 3), and propose the value shape.
  The Architect rules before any build.
- **AC-1.** The convoy fixture over `Int` and over `String` evaluates to the
  expected value on both engines.
- **AC-2 (controls).** An open index still fails closed (the durable pin
  holds, or is re-stated by Architect ruling), and removing the new arm
  returns each AC-1 case to `Unknown`.

## Stop conditions

- Any kernel, `trusted_base()` or spec change (an operator question).
- **Held work:** never move `4b4c8565c`, `21c039918`, `7f1a04a40`,
  `wp/RT-BRACKET-PRODUCER-AUTHENTICITY` or the child-2 checkpoint.

## Closeout

Landed at `235b8cefe` (PR #4319, candidate `cf915732e`). `eq_type_eq` in
`crates/ken-interp/src/eval.rs` gained `Str` and `Float`/`Float32` arms,
the floats compared by bits. So a closed reflexive cast over an applied type
with a primitive argument now evaluates to its value on the interpreter.
Trust is unchanged.

Carries:
- **Native half deferred** (Architect `evt_23q2tzd5xxghf`). The native
  engine refuses the convoy at the `CheckedCoreBodyView`
  `UnsupportedDependentMotive` gate, before it reaches C5.
  `crates/ken-cli/tests/rt_c5_primitive_type_native_gate.rs` pins that
  refusal. The native `Int` and `String` convoy rows are owed by whichever WP
  lifts that gate; no node owns it yet.
- **Compound and higher-order indices** (stop 1, `evt_7nf6ds3er67t1`) are
  placed in `RT-C5-COMPOUND-HIGHER-ORDER-INDICES`.
