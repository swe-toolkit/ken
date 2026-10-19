---
id: SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION
title: "Spec 66 evaluates and decides equality of Nat polynomial expressions but defers substitution: nothing substitutes expressions for variables, or proves that evaluating a substitution is evaluating under the composed environment. Rule substitution and its laws from a checked D0 at zero trust, and state them as a section 3 with a conformance seed"
status: merged
owner: spec
size: S
tier: T1
gate: architect
depends_on: []
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 66 sections 1.4 and 2.3 list substitution among the deferrals. Symbolic algebra is the roadmap layer after linear algebra (docs/program/06-catalog-campaign.md); non-Lane-B layers are deepened in place. Same shape as SPEC-LINEAR-ALGEBRA-PRODUCT-COMPATIBILITY. Steward-filed per COORDINATION section 2."
---

# Substitution evaluates under the composed environment

## Objective

Spec 66 states a public `substitute` over
`Algorithm.SymbolicAlgebra.NatPolynomial`'s `Expr`, with checked laws
that evaluating a substitution is evaluating under the composed
environment, and that substitution preserves the §2 equality decision.

## Settled inputs (spec 66 §1 and §2, on `main`)

- **Carrier (§1.1).** `Expr = EConst Nat | EVar Nat | EAdd Expr Expr |
  EMul Expr Expr`, evaluated by `eval (env : Nat → Nat)`.
- **Laws today.** §1.3's five distribution and normalization laws, and
  §2.2's five canonical-form laws: `canonicalize_sound`,
  `canonicalize_complete`, `poly_eq_sound`, `poly_eq_complete` and
  `poly_eq_false_refutes`. No law mentions a map from variables to
  expressions.
- **Deferred today.** §1.4 and §2.3 list substitution, composition,
  subtraction, signed coefficients, division, factoring and derivatives
  as outside the contract.
- **Package.** `catalog/packages/Algorithm/SymbolicAlgebra/NatPolynomial.ken.md`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

A new spec 66 section stating `substitute` and each ruled law by its
public type, §1.4 and §2.3 corrected to drop substitution from the
deferrals, and a closed conformance seed under
`conformance/stdlib/symbolic-algebra/`.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  with rc 0 and no `Axiom`. It rules:
  - `substitute (s : Nat → Expr) (e : Expr) : Expr`, replacing each
    `EVar v` by `s v`;
  - `eval env (substitute s e)` equal to
    `eval (\v. eval env (s v)) e` over every `env`, `s` and `e`;
  - whether substitution preserves `poly_eq`'s `True` answer as a
    public law, and which helpers stay private.
- **AC-1.** The section states each ruled law by its public type, and
  §1.4 and §2.3 are corrected. The Spec leader and CV review it, with a
  Decision.
- **AC-2.** The seed checks the evaluation law on a closed substitution
  that maps one variable to a sum and another to a product, with a
  positive control on the identity substitution `EVar`, and has a
  refused block that claims evaluation under the uncomposed `env`.

## Stop conditions

- A law that needs subtraction, signed coefficients, function
  extensionality or a premise beyond §1's data, unless the D0 rules it
  as the stated premise.
- Any new primitive, kernel or surface feature.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.
