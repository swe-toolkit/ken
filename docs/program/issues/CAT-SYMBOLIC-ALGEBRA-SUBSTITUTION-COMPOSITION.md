---
id: CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION
title: "Algorithm.SymbolicAlgebra.NatPolynomial exports spec 66 section 4: the builder compose_subst and the structural laws substitute_compose and substitute_var, proved at zero trust from the Architect's D0 development, with the composition seed run by a package test"
status: merged
owner: foundation
size: S
tier: T2
gate: architect
depends_on: [SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION]
blocks: []
github: null
origin: "Operator 2026-09-13: every catalog package fully proved. SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION states section 4 (fd15baf37) and records its laws as checked in the Architect's D0 scratch development, not yet exported. Same shape as CAT-SYMBOLIC-ALGEBRA-SUBSTITUTION. Steward-filed per COORDINATION section 2."
---

# NatPolynomial exports substitution composition

## Objective

`Algorithm.SymbolicAlgebra.NatPolynomial` exports `compose_subst`,
`substitute_compose` and `substitute_var` with exactly spec 66 §4's
public types, each law a checked proof.

## Settled inputs (spec 66 §4, on `main`; Architect D0)

- **The surface.** `pub fn compose_subst s t v = substitute s (t v)`
  (`t` first, then `s`). `substitute_compose` is structural `Equal
  Expr` for every `s`, `t` and `e`. `substitute_var` uses the
  constructor `EVar` as the identity map. No evaluation corollary is
  exported (§4).
- **The D0.** `local/arch-ssc-d0/d0-catalog-fcd05e7eb.patch` (primary
  checkout), on base `fcd05e7eb`: the three public names plus the
  private `expr_add_cong` and `expr_mul_cong`, by induction on `e`.
  Checked rc 0 with no Axiom, `trusted_base()` 114 before and after.
- **Seed.**
  `conformance/stdlib/symbolic-algebra/seed-nat-polynomial-substitution-composition.md`,
  from SPEC-SYMBOLIC-ALGEBRA-SUBSTITUTION-COMPOSITION AC-2. It must be
  on `main` before this WP starts.
- **Package.** `catalog/packages/Algorithm/SymbolicAlgebra/NatPolynomial.ken.md`.
  §3 is tested by
  `crates/ken-elaborator/tests/cat_symbolic_algebra_canonical_equality.rs`.

Treat anchors as perishable. If a settled input is false on the landed
base, stop and report the mismatch.

## Deliverable

The three exports and the two private helpers in `NatPolynomial.ken.md`,
the spec 66 §4 status lines updated to say the exports are delivered,
and a new
`crates/ken-elaborator/tests/cat_symbolic_algebra_substitution_composition.rs`
that root-loads each new public name and runs the seed.

## Acceptance

- **AC-1.** `ken check` and `ken fmt --check` pass. The test root-loads
  the three names as package-owned and transparent, keeps the helpers
  private, and runs the seed's positive cases. Its refused block is
  pinned to the reversed-order proposition on the seed's closed
  fixture, so an unrelated false equation does not satisfy it. The §1
  to §3 suites still pass. The status lines change with the exports
  (Steward `evt_5rtkgkdf42qb`), and the Spec leader reviews those spec/
  paths.
- **AC-2.** `trusted_base()` is equal before and after, with no Axiom or
  primitive. `lang_mod_strict_resolution_d0`'s
  `catalog_ambient_passthrough_migration_census` row for the package is
  unchanged, measured on the candidate.
- **AC-3 (mutation, QA).** Each mutant fails, and is restored:
  - `compose_subst` in the reversed order, `substitute t (s v)`, fails
    `substitute_compose` or the seed's forward case;
  - replacing an inductive arm of `substitute_var` with `Refl` fails to
    check;
  - an unrelated false closed equation in the refused block fails the
    proposition pin.

## Stop conditions

- A law needs an Axiom, function extensionality, a decidable equality on
  `Expr`, a kernel or surface change, or a public type other than §4's.
- A census row, or a consumer of `NatPolynomial` outside this package,
  changes.
