---
id: SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE-ABSENT-VARIABLE
title: "Spec 66 §7 proves a derivative zero when its variable is semantically unused, under a premise quantified over every environment that agrees off the variable, and says independence is not syntactic absence; but no checked law gives the case a rewriting client can discharge structurally, a variable that does not occur in the expression. Rule from a checked D0 at zero trust a public occurrence decision and an absent-variable derivative law, with a conformance seed"
status: active
owner: spec
size: S
tier: T1
gate: architect
depends_on: [SPEC-TIME-MERGE-SEPARATED-APPEND]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Non-Lane-B layers are deepened in place (docs/program/06-catalog-campaign.md); Symbolic algebra follows Time in the layer rotation. Steward-filed per COORDINATION section 2."
---

# A derivative in an absent variable is zero

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public occurrence
decision `occurs v e`, and a public checked law: for every `v : Nat`,
`e : Expr` and `env : Nat → Nat`, if `occurs v e` is `False`, then
`eval env (deriv v e)` is `Zero`. Spec 66 states both by their public
types.

## Settled inputs (main `a3f9b487b`)

- **The derivative.** `deriv` (`NatPolynomial.ken.md:971`) sends a
  variable through the private `var_deriv (nat_same v w)` (`:2553`,
  `nat_same` at `:1390`). Its sum and product cases recurse on both
  operands.
- **The semantic law.** `deriv_independent` (`:1127`) needs a premise
  quantified over every `env2` that agrees with `env` off `v`. A client
  cannot discharge that for an open `e` without its own induction.
- **The claim's gap.** Spec 66 §7 (`66-symbolic-algebra.md:722`) says
  independence is semantic, "not absence of `EVar v` in the syntax", and
  `:754` to `:756` says no law for syntactic occurrence follows from §7.
  No section states the syntactic case. Spec 66 runs §1 to §13 (file end
  `:1202`), and the package exports 29 public theorems.
- **The enablers.** `nat_same_sound` (`:2576`) and `nat_same_complete`
  (`:2611`). `deriv_var_deriv_eval` (`:2559`) is the pattern for matching
  an arbitrary Boolean, not a computed `nat_same`. `add_cong` (`:1414`)
  and `mul_cong` (`:1425`). `bool_or`
  (`Core/Classes/LawfulClasses.ken.md:107`) is not imported yet.
  `(proof zero_l for mul)` (`Data/Numeric/Nat/Arithmetic.ken.md:70`)
  closes the product case's `mul Zero (eval env b)`.
- **Seeds.** Under `conformance/stdlib/symbolic-algebra/`, next to
  `seed-derivative-independent-variable.md`. The package test pattern is
  `crates/ken-elaborator/tests/cat_symbolic_algebra_deriv_constant_factor.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

The ruled operation and law in NatPolynomial, a spec 66 section stating
them by their public types, a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`, a named package test, and spec
66's status lines, all in one candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep and `trusted_base()` as an identity set before
  and after. The Architect rules from that report, and does not build
  or test. It rules:
  - the operation's and law's names, argument order and placement;
  - the section number;
  - the conclusion's form: evaluated `Zero` at every `env`, or the
    `poly_eq` form against `EConst Zero`;
  - whether `bool_or` is imported or `occurs` is written by direct
    match.

  It confirms that no §1 to §13 operation or law type changes.
- **AC-1.** The section states the ruled operation and law by their
  public types. The Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client is generic over an open
  expression `x` with `occurs` false for the variable, and shows a
  derivative built from `x` is zero.
  - Its refused block claims the zero derivative of §7's example
    `EAdd (EVar Zero) (EMul (EVar (Suc Zero)) (EConst Zero))` in
    `Zero`, which occurs.
  - Its accepted same-form control makes the same claim in
    `Suc (Suc Zero)`, which does not occur, so the refusal is the
    occurrence, not the form.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond non-occurrence.
- A change to an existing §1 to §13 operation or law type.

Checks: 4 fired: the law is written from `deriv` at `:971`,
`var_deriv` at `:2553`, `deriv_independent`'s premise at `:1127` and the
`nat_same` laws at `:2576` and `:2611`, each read at its line. 3 fired:
NatPolynomial's 29 public theorems and spec 66 §1 to §13 were read for
an occurrence law; none exists, and §7 says none follows. 8 fired: the
refused row and its control share the expression and the claim, and
differ only in whether the variable occurs.
