---
id: SPEC-SYMBOLIC-ALGEBRA-NORMALIZE-CONGRUENCE
title: "Spec 66 makes poly_eq a congruence for EAdd, EMul, substitute and deriv, but not for normalize, the package's own canonicalizer: two canonically equal expressions are not known to normalize to canonically equal results, so a client that normalizes both sides of a proved equation must re-derive it through soundness. Rule from a checked D0 at zero trust the public law that normalize preserves poly_eq, and state it with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Symbolic algebra follows Time in the L3 layer rotation (docs/program/06-catalog-campaign.md). Steward-filed per COORDINATION section 2."
---

# Normalization respects canonical equality

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports a public checked law:
for all `e1`, `e2 : Expr`, `IsTrue (poly_eq e1 e2)` implies
`IsTrue (poly_eq (normalize e1) (normalize e2))`. Spec 66 states the law
by its public type.

## Settled inputs (main `1a14eff00`)

- **The canonicalizer.** `normalize` (`NatPolynomial.ken.md:82`) is
  `reify (monomials e)`. `normalize_poly_eq` (`:1298`) relates one
  expression to its own normal form. `deriv_normalize` (`:1301`) and
  `substitute_normalize` (`:1306`) push it through the other operations.
- **The congruences.** `poly_eq_add` (`:794`), `poly_eq_mul` (`:814`),
  `poly_eq_substitute` (`:868`) and `poly_eq_deriv` (`:1024`) are public,
  and each preserves `poly_eq`. `poly_eq_sym` (`:770`) and
  `poly_eq_trans` (`:775`) are public too. No `normalize` congruence
  exists under any name.
- **The spec.** Spec 66 runs §1 to §14. §14 starts at
  `66-symbolic-algebra.md:1185`, and the file ends at `:1251`. The
  normalize laws are §9.1 (`:843`). Spec §9.2 warns against an
  `Equal Expr (normalize e) e` refusal, so the seed must not use one.
- **Seeds and tests.**
  - Seeds go under `conformance/stdlib/symbolic-algebra/`. The nearest
    neighbours are `seed-nat-polynomial-normalize-transparent.md` and
    `seed-nat-polynomial-congruence.md`.
  - The package test pattern is
    `crates/ken-elaborator/tests/cat_symbolic_algebra_normalize_transparent.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The ruled law in NatPolynomial, a spec 66 section stating it by its
public type, a closed conformance seed and a named package test, in one
candidate.

## Acceptance

- **AC-0 (D0).** The Architect specifies a development on the landed
  base: the source, and the named-binary commands with a bounded
  timeout. The Foundation implementer runs it on scratch and reports
  rc, the `Axiom` grep, and `trusted_base()` as an identity set before
  and after. The Architect rules from the report and does not build or
  test. It rules:
  - the name and placement;
  - that no §1 to §14 operation or law type changes.
- **AC-1.** The section states the ruled law by its public type. The
  Spec Leader and CV review it, with a Decision.
- **AC-2.** The seed's accepted client applies the law to
  `EAdd (EVar 0) (EVar 1)` and `EAdd (EVar 1) (EVar 0)`, with the premise
  `Proved`. Its refused block applies the law to `EVar 0` and `EVar 1`,
  where the premise's `Proved` fails. The accepted same-form control is
  the commuted-sum application. The refusal therefore comes from the
  failed premise, not the form. Each refusal is appended to a fresh,
  fully accepted prefix.

## Stop conditions

- The law needs an Axiom, a kernel or surface change, or a premise
  beyond the one `poly_eq`.
- A change to an existing §1 to §14 operation or law type.

Checks: 4 fired: `normalize`, `normalize_poly_eq`, the four congruences
and `poly_eq_sym`/`poly_eq_trans` were read at their lines on
`3a2900b03` (unchanged at `1a14eff00`); the law's type uses only that
vocabulary. 3 fired: grepped catalog/, spec/, conformance/ and crates/
for `normalize_cong`,
`poly_eq_normalize` and `poly_eq (normalize`; the only hits are the
single-expression `normalize_poly_eq`. 8 fired: the refused application
and its control differ only in the expressions, and the failed premise
is the one `poly_eq`.
