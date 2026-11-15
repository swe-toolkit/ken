---
id: SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-CONGRUENCE
title: "Spec 66 §16-§17 give expr_pow exponent laws but no law relating it to canonical equality of its base or to substitution, so a client cannot rewrite under a power or push a substitution through one. Rule from a checked D0 at zero trust that expr_pow preserves poly_eq in its base, and that substitute distributes over expr_pow, and state both with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md:376). expr_pow has exponent laws (spec 66 §16-§17) but no base congruence or substitution law. Steward-filed per COORDINATION section 2."
---

# Powers under canonical equality and substitution

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports two public checked
laws on `expr_pow` at zero trust, and spec 66 states them by their
public types:

- **Base congruence:** from `IsTrue (poly_eq a b)`, conclude
  `IsTrue (poly_eq (expr_pow a n) (expr_pow b n))`.
- **Substitution:** `substitute s (expr_pow e n)` against
  `expr_pow (substitute s e) n`, as a structural `Equal Expr` if the D0
  admits it, otherwise by canonical `poly_eq`.

## Settled inputs (main `fd523a842`)

- **The operation.** `expr_pow` (`NatPolynomial.ken.md:1328`) is
  `Zero ↦ EConst (Suc Zero); Suc k ↦ EMul (expr_pow e k) e`. Its public
  laws are `deriv_expr_pow` (`:1334`), `expr_pow_add` (`:1345`),
  `expr_pow_mul` (`:1353`) and `expr_pow_emul` (`:1373`). The last two
  recurse on `n` through `poly_eq_trans` and `poly_eq_mul`.
- **Substitution.** `substitute` (`:843`) maps `EConst c` to `EConst c`
  and `EMul a b` to `EMul (substitute s a) (substitute s b)`, so both
  `expr_pow` cases compute under it. Its laws include `eval_substitute`
  (`:851`) and `poly_eq_substitute` (`:875`).
- **The equality tools.** `poly_eq_refl` (`:775`), `poly_eq_trans`
  (`:782`), `poly_eq_mul` (`:821`) and `poly_eq_complete` (`:753`).
- **The gap.** No public law has `expr_pow` of a `poly_eq`-premised base,
  or `substitute` applied to `expr_pow`.
- **Spec and tests.** Spec 66 ends at §17 (`66-symbolic-algebra.md:1378`),
  so the new section is §18. The test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_expression_power_mul.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Scope

Spec 66 §18, the two catalog laws with any private helper, a seed, a
package test and the status lines. No §1 to §17 operation or law type
moves, and no kernel or surface change.

## Deliverable

One candidate carrying spec, seed and catalog halves, as for
SPEC-SYMBOLIC-ALGEBRA-EXPR-POWER-MUL.

## Acceptance

- **AC-0 (D0, Architect, before spec text).** A checked development on
  the landed base with rc 0, no `Axiom`, and `trusted_base()` unchanged
  as an identity set. It rules:
  - the names and binder order;
  - the substitution law's form, structural `Equal` or `poly_eq`;
  - the proof route for each, induction on `n` or evaluation through
    `poly_eq_complete`.
- **AC-1.** §18 states each ruled law by its public type. The Spec
  Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted block applies each law at closed
  arguments:
  - congruence at `n = 2`, with `a` and `b` syntactically distinct but
    `poly_eq`-equal, for example `EAdd (EVar 0) (EVar 1)` and
    `EAdd (EVar 1) (EVar 0)`;
  - substitution at `n = 2`, with `s` sending variable 0 to a non-variable
    expression.

  Its refused block pins, in the same form as an accepted row and on a
  fresh accepted prefix:
  - congruence claimed for two bases that are not `poly_eq`-equal, such
    as `EVar 0` and `EVar 1` at `n = 2`;
  - substitution claimed as `expr_pow e n` on the right, with the
    substitution dropped.
- **AC-3.** Full-workspace CI is green on the exact candidate.

## Stop conditions

- A law needs an `Axiom`, a kernel or surface change, or a premise beyond
  the congruence law's `poly_eq` hypothesis.
- A change to an existing §1 to §17 operation or law type.

Checks: 4 fired. For congruence, the `Suc` step is
`EMul (expr_pow a k) a` against `EMul (expr_pow b k) b`, which
`poly_eq_mul` closes from the hypothesis and the induction hypothesis.
For substitution, `substitute s (EMul (expr_pow e k) e)` computes to
`EMul (substitute s (expr_pow e k)) (substitute s e)`, so the step is a
congruence on the induction hypothesis. 3 fired: the `expr_pow` grep
covered `catalog/packages`, `spec/`, `conformance/` and `crates/`, which
gave 6 files. 8 fired: `EVar 0` and `EVar 1` squared are distinct
canonical polynomials, so a premise-free congruence is refused. Under
`s 0 = EConst 2`, the substituted square of `EVar 0` contains no
variable, while the unsubstituted `expr_pow (EVar 0) 2` contains
variable 0. So the dropped-substitution row separates.
