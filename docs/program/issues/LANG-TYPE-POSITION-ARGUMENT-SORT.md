---
id: LANG-TYPE-POSITION-ARGUMENT-SORT
title: "A declaration with an unannotated Type binder used under a Type 0 former, such as (A : Type) (o : Option A), is refused at its obligation with Type 0 vs Type u0: elab_type builds type-position applications unchecked, so the binder's level meta is consistent only through the final zonk's Zero default, and the obligation hole generalizes it. Record the argument's sort equation at the producer, and solve level metas without defaulting"
status: merged
owner: language
size: S
tier: T1
gate: architect
depends_on: [LANG-OBLIGATION-HOLE-LEVEL-META-CLOSURE]
blocks: []
github: null
origin: "Adversary evt_2wwkj8s5eycm0 on a6ea5035c (class finding on LANG-OBLIGATION-HOLE-LEVEL-META-CLOSURE); Architect ruling evt_1tqan5f034h8n: real, fails closed, producer repair, one S node after LANG-ASCRIPTION-INTRODUCTION-INFERENCE. Steward-filed per COORDINATION section 2."
---

# A type-position argument's sort is recorded where it is built

## Objective

A declaration whose unannotated `Type` binder is used under a former
checks and reports its obligation, as its `(A : Type 0)` twin does.

## Settled inputs (Architect `evt_1tqan5f034h8n`, on `a6ea5035c`)

- **Repro.** `fn k (A : Type) (x : A) (h : Equal A x x) (n : Int) : Int =
  use5 n` and its `(o : Option A)` twins are KernelRejected `TypeMismatch
  { expected: Type 0, found: Type u0 }` at the obligation. This holds
  through refinement, Requires, PartialPrim and match-arm mints, and with
  `(xs : List A)` or two binders.
- **Mechanism.** `elab_type`'s `RType::RApp` arm (elab.rs ~:1261) builds
  the application unchecked, so `Option A` records nothing about `A :
  Type ?u`. Only the final zonk's Zero default makes it well-typed, and
  `obligation_level_params` (:6066) generalizes the unsolved meta. A
  second instance: `unify_levels` (:320) solves with `zonk_level`, which
  defaults an unsolved partner to Zero; a local former `fn k (F : Type →
  Type) (A : Type) (y : F A) (n : Int) : Int = use5 n` is refused with
  `Type 0 vs Type u2`.
- **Ruled code.** S1: the RApp arm calls a new unconditional
  `unify_type_position_argument_sort(cx, &f_k, &a_k)` that reads the
  former's domain sort and the argument's sort with
  `kernel_infer_in_context_open` and unifies them; any failed read
  returns early. S2: both `unify_levels` arms solve with
  `zonk_level_with(_, false)` and skip a value that mentions the meta,
  through a new `level_mentions_var`. `obligation_level_params` is
  unchanged. The ruling's text is the code.
- **Measured with S1 and S2.** r1-r5 and r7 are Ok with 1 hole and 0
  params; two- and three-binder rows keep only the free binder; `List A`
  0 params; local former n1 2 params; `F : Type 1 → Type 1` with `(A :
  Type)` now Ok; the WP row and solved-after row keep 1 param;
  `(A : Type 1) (o : Option A)` stays refused. The 11 suites named in the
  ruling are green.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

S1 and S2 in `crates/ken-elaborator/src/elab.rs`, with the ruling's rows
as a test target. No kernel, trust or spec change.

## Acceptance

- **AC-1.** The ruling's table as exact rows, each with its hole count,
  parameter count and goal head.
- **AC-2.** The 11 suites the ruling names stay green.
- **AC-3 (mutation, QA; Architect `evt_75p7jq6a4w9hf`).**
  - Deleting the S1 call reddens r1-r7 and c6 with `Type 0 vs Type u0`, and
    n1 with `Type u0 vs Type u2`. n3 and n4 may also go red.
  - Reverting both S2 zonks to `zonk_level` reddens only n1 among the 17
    rows, with `Type 0 vs Type u2`.

## Stop conditions

- A previously green elaborator or catalog row changes outcome under S2:
  stop to the Architect.
- Any kernel, conversion or `trusted_base()` change.

Out of scope: `(y : {v : A | Equal A v x})` fails with UnresolvedCon "A"
on base, a separate gap.
