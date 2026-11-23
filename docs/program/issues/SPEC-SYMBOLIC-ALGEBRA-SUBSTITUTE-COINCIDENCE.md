---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTE-COINCIDENCE
title: "Spec 66 §21 says eval depends only on the variables that occur, but substitution has no counterpart: §20 fixes one absent variable against the identity, and poly_eq_substitute_pointwise needs agreement at every variable and concludes only canonical equality. So a client whose two substitutions differ at several unused variables cannot conclude the results are equal as stored. Rule from a checked D0 at zero trust that substitute s e equals substitute t e whenever s and t agree at every variable occurring in e, and state it with a seed that separates agreement on the occurring variables from a disagreement at one of them"
status: ready
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). §21 gives evaluation's coincidence law; substitution has none. Steward-filed per COORDINATION section 2."
---

# Substitution depends only on the occurring variables

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law.
For every `e`, `s` and `t : Nat → Expr`, if `s w` equals `t w` as
`Equal Expr` for every `w` with `occurs w e` equal to `True`, then
`substitute s e` equals `substitute t e` as `Equal Expr`. It holds at
zero trust, with that premise only. Spec 66 states it by its public type.

## Settled inputs (main `aadce7362`)

- **The operations.** `substitute` (`NatPolynomial.ken.md` ~`:849`) maps
  `EVar v` to `s v` and rebuilds the other constructors. `occurs`
  (~`:1178`) is `nat_same v w` at `EVar w`, and `bool_or` over both
  children.
- **The template.** `eval_coincidence` (~`:1362`) has this premise
  shape: `(w : Nat) → Equal Bool (occurs w e) True → ...`. Its proof
  splits at `EAdd` and `EMul` with `or_left` and `or_right`, and closes
  `EVar` with `nat_same_complete v v Refl`. The substitution law follows
  the same recursion, with `cong` over `EAdd` and `EMul` in place of
  `add_cong` and `mul_cong`.
- **The neighbours.**
  - `substitute_absent` (~`:1300`, §20) fixes one absent `v` and compares
    `s` with `EVar`.
  - `poly_eq_substitute_pointwise` (~`:909`) needs `poly_eq (s v) (t v)`
    at every `v` and concludes only `poly_eq`.
  - No law relates two substitutions by their occurring variables and
    concludes `Equal Expr`.
- **Spec and tests.** Spec 66 ends at §21
  (`spec/50-stdlib/66-symbolic-algebra.md` ~`:1661`), so the new section
  is §22. The seed directory is `conformance/stdlib/symbolic-algebra/`.
  The test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_eval_coincidence.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §22, the catalog law with any private helper, a seed, a package
test and the status lines. No §1 to §21 operation or law type moves, and
there is no kernel or surface change.

## Deliverable

One candidate carrying the spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EVAL-COINCIDENCE.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the name and the binder order;
  - whether §20's `substitute_absent` is restated as a one-line corollary
    in §22's prose, or left as it is. It stays exported either way.

  Sweep consumers of the package's public roster by the sibling law.
- **AC-1.** §22 states the law by its public type. The Spec Leader and CV
  review it, with a Decision.
- **AC-2.** The seed's accepted block applies the law to
  `EAdd (EVar 0) (EConst 2)` with two substitutions: `s` sends 1 to
  `EConst 7`, 2 to `EConst 9` and is `EVar` elsewhere, and `t` is
  `EVar`. They differ at two variables, neither of which occurs, and the
  block concludes `substitute s e` equals `substitute t e`. A single
  `substitute_absent` cannot reach that conclusion.

  The refused block, on a fresh accepted prefix and in the same
  application form, makes variable 1 occur:
  `EAdd (EVar 1) (EConst 2)`. The agreement premise fails to check
  there, and the two results really differ (`EConst 7` against
  `EVar 1`). The two rows differ only in that one occurrence.
- **AC-T.** The package test asserts `trusted_base()` unchanged as an
  identity set on the assembled candidate. No Decision opens until it
  passes.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- The law needs an `Axiom`, a kernel or surface change, or a premise
  beyond the agreement on occurring variables.
- A change to an existing §1 to §21 operation or law type.

Checks: 3 fired. I listed every public `NatPolynomial` law on
`aadce7362`. The substitution laws are `substitute_absent`,
`substitute_compose`, `substitute_var`, `substitute_normalize`,
`substitute_expr_pow` and the two `poly_eq_substitute` laws. None takes
an occurrence-restricted agreement and concludes `Equal Expr`. 4 fired. I
read `substitute`, `occurs` and `eval_coincidence`. The obligation splits
at `EAdd` and `EMul` through `bool_or` exactly as evaluation's does, and
`EVar v` needs only `occurs v (EVar v) = True`. 8 fired. In the refused
row, a substitution pair that disagrees at an occurring variable gives
different expressions, so the law must refuse there, and it does at the
premise.
