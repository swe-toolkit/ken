---
id: CAT-SYMBOLIC-ALGEBRA-NAT-POLYNOMIAL
title: "Spec 66 section 1 specifies Algorithm.SymbolicAlgebra.NatPolynomial with five proved laws, but no catalog package delivers it. Deliver the package from the Architect's checked D0 at zero trust, with the constructor exports a client needs, and its seed as an acceptance target"
status: ready
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-CONTRACT]
blocks: []
github: null
origin: "Operator 2026-10-08: \"start l3 on the establish catalog program\"; operator 2026-09-13: every catalog package fully proved. Spec 66 section 1 from SPEC-SYMBOLIC-ALGEBRA-CONTRACT (Architect D0 evt_v6ba82swws6p, review evt_gw4ep13cbrd6). Steward-filed per COORDINATION section 2."
---

# Natural-coefficient polynomials, delivered and proved

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exists in `catalog/`, matches
spec 66 section 1 token for token, and proves all five laws with no Axiom.
A client outside the module can build and evaluate expressions.

## Settled inputs (on `bc092bdd5` plus the spec chapter)

- **The contract.** Spec 66 section 1 fixes the following.
  - The carriers are `Expr` (`EConst`, `EVar`, `EAdd`, `EMul`) and
    `Monomial` (`Mono Nat (List Nat)`).
  - The six operations are `eval`, `eval_monomial`, `eval_monomials`,
    `monomials`, `reify` and `normalize`.
  - The five laws are `normalize_sound`, `normalize_idempotent`,
    `monomials_sound`, `reify_sound` and `monomials_reify`, with exact
    signatures in section 1.3.
- **A checked development exists.** The Architect's D0 checks all five
  generic proofs together, rc=0, with no Axiom, postulate or admit in the
  source. It uses Nat `add` and `mul`, distributivity, `list_append` and
  transport helpers that are on main.
- **Constructor exports are required.** `pub data` exports only the type
  (`33 §4.1`). Without `export EConst, EVar, EAdd, EMul` and `export Mono`,
  the seed client fails with `UnboundName ...EConst`. With them, the module
  and the client both check (Architect `evt_gw4ep13cbrd6`). The precedent
  is `Capability/Parsing/Cursor.ken.md`.
- **The seed** is
  `conformance/stdlib/symbolic-algebra/seed-nat-polynomial.md`. Its
  positive block has 19 assertions. Its two negatives, a dropped monomial
  and the canonical converse, are refused at the `Refl` span.
- **Known limits.**
  - Pass `map` a named helper, not a λ
    (`LANG-ELABORATED-BETA-REDEX-AT-KERNEL-INFERENCE`).
  - A dependent `match` refines only its scrutinee as written (CHECKS 11).

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

1. The package at `catalog/packages/Algorithm/SymbolicAlgebra/
   NatPolynomial.ken.md`. It holds the D0's declarations and proof
   bodies, the two export lines, and the section 1.3 laws as public
   theorems.
2. An acceptance target in `crates/ken-elaborator/tests/` that:
   - loads the package;
   - runs the seed's positive block as a client import;
   - refuses both negatives;
   - restates the five law types generically;
   - records `trusted_base()` before and after loading.
3. Optionally, the Architect's wording nit on the seed's
   `reify-sound-instance` line.

## Acceptance

- **AC-1.** The package checks, every section 1.3 law is a checked proof
  body, and `trusted_base()` does not grow on loading it.
- **AC-2.** The seed's 19 positive assertions pass from a client module,
  and both negatives are refused with the same diagnostic class as in the
  Architect's run.
- **AC-3 (mutation, QA).**
  - Dropping one `EAdd` arm's right operand in `monomials` makes the
    package's own proofs fail to load.
  - Removing the constructor exports reddens the client import.

## Stop conditions

- A law needs an Axiom, a kernel or surface change, or a public signature
  that differs from spec 66.
- A census row, or a catalog package other than this new one, changes.
