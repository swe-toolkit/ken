---
id: CAT-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY
title: "Spec 66 section 2 specifies a canonical Horner form and a proved Boolean decision of polynomial equality for Algorithm.SymbolicAlgebra.NatPolynomial, but the package delivers only section 1. Deliver section 2 from the Architect's checked D0 at zero trust, with the constructor exports a client needs, and its seed as an acceptance target"
status: active
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 66 section 2 from SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY (Architect D0 evt_44wcmv83mbfzh). Steward-filed per COORDINATION section 2."
---

# Deciding polynomial equality, delivered and proved

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` gains spec 66 section 2 token
for token: `Poly`, `eval_poly`, `canonicalize`, `poly_eq` and five laws,
all proved with no Axiom. A client outside the module decides that two
syntactically different expressions are equal.

## Settled inputs (Architect `evt_44wcmv83mbfzh`, on `bc711b1e0`)

- **The contract.** Spec 66 section 2, as landed by
  SPEC-SYMBOLIC-ALGEBRA-CANONICAL-EQUALITY:
  - the carrier `Poly = PConst Nat | PHorner Poly Poly`, with
    `export PConst, PHorner`;
  - the operations `eval_poly`, `canonicalize` and `poly_eq`;
  - the laws `canonicalize_sound`, `canonicalize_complete`,
    `poly_eq_sound`, `poly_eq_complete` and `poly_eq_false_refutes`.
- **A checked development exists.** The D0 added section 2 as a fence
  inside the module itself: `ken check` rc 0, no Axiom, postulate or
  admit, `trusted_base()` 114 before and after, and all five laws
  `Decl::Transparent`. The source is
  `notes/symbolic-canonical-equality-d0-bc711b1e0.ken` on
  `architect/work`, and the closed cases are in
  `notes/symbolic-canonical-equality-d0-seed-cases-bc711b1e0.ken`
  (branch head `d93f73529`). The proofs use the module's private
  `var_product`, so section 2 goes in the same module.
- **Imports.** The module gains `Core.Classes.LawfulClasses (IsTrue,
  bool_and, leq_nat)` and `Data.Numeric.Nat.Order (sub,
  leq_nat_add_left_bound, leq_nat_add_right_bound)`. Import `leq_nat`
  from `Core.Classes.LawfulClasses`: through the `Data.Numeric.Nat.Order`
  facade it fails with `UnboundName ...leq_nat::trans`.
- **Consumers with section 2 present.** `cat_symbolic_algebra_nat_polynomial`
  is 3/3, the strict-resolution census row for NatPolynomial is unchanged
  (the same 8 pinned names), and the other strict-resolution tests are
  4/4.
- **The seed** is the CV's section 2 seed under
  `conformance/stdlib/symbolic-algebra/`, landed with the spec.
- **Known limits.** A dependent `match` refines only its scrutinee as
  written (CHECKS 11); every `match` in the D0 is on a bound variable.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. Section 2 in `catalog/packages/Algorithm/SymbolicAlgebra/
   NatPolynomial.ken.md`: the D0's declarations and proof bodies, the
   constructor export line, and the five laws as public theorems. The
   D0's private `add_cancel_right` may stay private here; do not change
   another package.
2. An acceptance target in `crates/ken-elaborator/tests/` that loads the
   package, runs the section 2 seed as a client import, refuses its
   negative, restates the five law types generically, and records
   `trusted_base()` before and after loading.

## Acceptance

- **AC-1.** The package checks, every section 2 law is a checked proof
  body, and `trusted_base()` does not grow on loading it.
- **AC-2.** The seed's positive cases pass from a client module,
  including `x₀+x₀` against `2·x₀` decided True, and the unequal-claim
  negative is refused with the same diagnostic class as in the D0.
- **AC-3 (mutation, QA).**
  - Making `canon` keep the empty tail `PHorner (PConst c) (PConst Zero)`
    reddens a load-bearing proof or seed row; QA names the row and the
    assertion that fires.
  - Removing `export PConst, PHorner` reddens the client import.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public signature
  that differs from spec 66.
- A census row, a section 1 declaration, or a catalog package other than
  this one changes.
