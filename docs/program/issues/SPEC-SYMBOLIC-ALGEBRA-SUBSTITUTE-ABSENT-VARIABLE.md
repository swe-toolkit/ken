---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-ABSENT-VARIABLE
title: "Spec 66 §19 says evaluation ignores a variable that does not occur, but no law says substitution does, so a client that rewrites one variable cannot conclude that an expression without it is unchanged. Rule from a checked D0 at zero trust that, when occurs v e is False and s is the identity on every other variable, substitute s e equals e as stored, and state it with a seed that separates absent from present"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). eval_absent and deriv_absent cover evaluation and derivation, with no substitution counterpart. Steward-filed per COORDINATION section 2."
---

# Substitution leaves an expression without the variable unchanged

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law
that, for every `v`, `e` and `s : Nat → Expr`, if `occurs v e` is `False`
and `s w` equals `EVar w` for every `w` other than `v`, then
`substitute s e` equals `e` as `Equal Expr`. It holds at zero trust, with
the two stated premises only. Spec 66 states it by its public type.

## Settled inputs (main `a0a4df8d3`)

- **The operations.** `substitute` (`NatPolynomial.ken.md:846`) maps
  `EVar v` to `s v` and rebuilds the other constructors. `occurs`
  (`:1175`) is `nat_same v w` at `EVar w` and `bool_or` over both
  children.
- **The neighbours.** `eval_absent` (`:1227`) is the evaluation form,
  with the same premise shape: `occurs v e` is `False`, and the two
  environments agree off `v`. `deriv_absent` (`:1183`) is the derivation
  form. `substitute_var` (`:959`) is the identity-substitution case,
  proved by structural match.
- **The gap.** `NatPolynomial.ken.md` and spec 66 relate `substitute` to
  `occurs` nowhere. A client holding `occurs v e = False` can conclude
  that evaluation ignores `v`, but not that substituting for `v` leaves
  `e` unchanged.
- **Spec and tests.** Spec 66 ends at §19
  (`spec/50-stdlib/66-symbolic-algebra.md:1503`), so the new section is
  §20. The seed directory is `conformance/stdlib/symbolic-algebra/`. The
  test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_eval_absent.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §20, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §19 operation or law type moves, and
no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EVAL-ABSENT-VARIABLE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules the name, the binder order, and whether
  the off-`v` premise is stated in `eval_absent`'s form (`Equal Nat v w →
  Bottom`).
- **AC-1.** §20 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law at closed
  expressions, with `v = 1` and an `s` that sends 1 to `EConst 7` and is
  `EVar` elsewhere:
  - `EAdd (EVar 0) (EConst 2)`, which equals itself after substitution;
  - `EMul (EVar 0) (EVar 2)`, which is unchanged with two other
    variables present.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - `substitute s (EAdd (EVar 1) (EConst 2))` claimed `Equal Expr` to
    `EAdd (EVar 1) (EConst 2)`. Variable 1 occurs, so the result is
    `EAdd (EConst 7) (EConst 2)`, and the claim is false.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the two stated.
- A change to an existing §1 to §19 operation or law type.

Checks: 3 fired. I grepped `NatPolynomial.ken.md` and spec 66 for
`substitute` with `occurs` or absence and found no law; `eval_absent` and
`deriv_absent` are the only absence laws. 4 fired. I wrote the
obligation in the delivered vocabulary. At `EVar w`, `occurs v (EVar w) =
False` gives `v ≠ w` through `nat_same`, as `eval_absent` already derives
it, and the premise then gives `s w = EVar w`. The other arms follow by
`cong` on the children, with `bool_or` false splitting into both sides.
8 fired. The refused row changes only whether `v` occurs, so it fails
exactly when the absence premise is dropped, while both accepted rows
hold.
