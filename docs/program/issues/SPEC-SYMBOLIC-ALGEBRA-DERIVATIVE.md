---
id: SPEC-SYMBOLIC-ALGEBRA-DERIVATIVE
title: "Spec 66 defers derivatives, so a client cannot differentiate a Nat polynomial expression and rely on the result depending only on the polynomial function it denotes. Rule a formal derivative on Expr with checked laws at zero trust from a D0, including that semantically equal expressions have semantically equal derivatives, and state it as a section 5 with a conformance seed"
status: merged
owner: spec
size: M
tier: T1
gate: architect
depends_on: []
blocks: [CAT-SYMBOLIC-ALGEBRA-DERIVATIVE]
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. Spec 66 §1.4, §2.3 and §4 list derivatives as outside §§1 to 4. Symbolic algebra is the next layer in the in-place rotation after time/events. Same shape as SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION. Steward-filed per COORDINATION section 2."
---

# A formal derivative that respects the polynomial function

## Objective

Spec 66 states a formal partial derivative `deriv : Nat → Expr → Expr`
with checked laws. The headline law: if two expressions evaluate
equally at every environment, so do their derivatives in any variable.

## Settled inputs (spec 66 §§1 to 4, on `main`)

- **Carrier (§1.1).** `Expr = EConst Nat | EVar Nat | EAdd Expr Expr |
  EMul Expr Expr`. `eval env e : Nat` takes a total `env : Nat → Nat`.
- **Canonical form (§2.2).** `canonicalize_complete` turns agreement at
  every environment into `Equal Poly` of canonical forms. The
  `poly_eq_*` laws decide and refute. That is the expected route for
  the headline law, but the D0 decides.
- **The gap.** §1.4, §2.3 and §4 say derivatives "remain outside §§1–4".
- **Package.** `catalog/packages/Algorithm/SymbolicAlgebra/NatPolynomial.ken.md`.
  The seeds are under `conformance/stdlib/symbolic-algebra/`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch; do not build around it.

## Deliverable

A new spec 66 §5 that states each ruled law by its public type, with
the deferral text in §1.4, §2.3 and §4 corrected, plus a closed
conformance seed under `conformance/stdlib/symbolic-algebra/`. The
status lines change in the same candidate.

## Acceptance

- **AC-0 (D0, Architect).** A checked development on the landed base,
  rc 0, with no `Axiom` and `trusted_base()` measured before and after.
  It rules:
  - `deriv`'s defining equations, structural on `Expr`, with the
    product rule producing `EAdd` of two `EMul`;
  - the headline law, by name and public type, for example
    `deriv_respects_eval (v : Nat) (e1 e2 : Expr) (h : (env : Nat → Nat)
    → Equal Nat (eval env e1) (eval env e2)) (env : Nat → Nat) : Equal
    Nat (eval env (deriv v e1)) (eval env (deriv v e2))`;
  - whether any structural law, such as one on constants or a
    variable, is public, or stays a definitional consequence;
  - the private helpers, and which §2 laws the proof reuses.
- **AC-1.** §5 states each ruled law by its public type. The deferral
  text is corrected. The Spec leader and CV review it, with a Decision.
- **AC-2.** The seed's closed positive client differentiates two
  syntactically different but semantically equal expressions, for
  example `x·(x+1)` and `x·x + x`, and gets evaluation-equal results.
  Its refused block claims that the derivative of `x·x` in `x` equals
  `x` at every environment, and a same-shape positive control states
  `x + x`.

## Stop conditions

- The headline law needs function extensionality, decidable equality
  beyond `poly_eq`, a new primitive, or a kernel or surface feature.
- Any `trusted_base()` change.
- An ambient name in the package beyond its current census row
  (`lang_mod_strict_resolution_d0`): import it explicitly.

Checks: 4 fired: the headline proposition is written in the package's
vocabulary (`eval`, total `Nat → Nat` environments, `Equal Nat`) and its
premise has `canonicalize_complete`'s exact shape. 8 fired: AC-2's
refused block (d/dx of x·x is x) is rejected by a correct product rule
but accepted by one that drops a term.
